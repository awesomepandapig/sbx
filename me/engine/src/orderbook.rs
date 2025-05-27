use super::publisher::ExecutionReportPublisher;

use std::cmp::Ordering;
use std::cmp::min;
use std::collections::HashMap;

use priority_queue::PriorityQueue;
use slab::Slab;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleDecoder;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

const MAX_ORDERS: usize = 2_000_000;
const MAX_PRICE_LEVELS: usize = 1_000_000;

pub type ClOrdIdType = [u8; 16];
pub type PartyIdType = [u8; 16];
pub type SymbolType = [u8; 6];
pub type OrderBookKey = (PartyIdType, ClOrdIdType);
pub type OrderPool = Slab<Order>;
pub type OrderMap = HashMap<OrderBookKey, usize>;
pub type BidQueue = PriorityQueue<usize, BidPriority>;
pub type AskQueue = PriorityQueue<usize, AskPriority>;

pub trait OrderBookPriority: Ord + PartialOrd + Eq + PartialEq + std::fmt::Debug + Copy {
    fn price(&self) -> i64;
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BidPriority {
    pub price: i64,
    pub seq_num: u64,
}

impl Ord for BidPriority {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.price
            .cmp(&other.price)
            .then_with(|| other.seq_num.cmp(&self.seq_num))
    }
}

impl PartialOrd for BidPriority {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for BidPriority {
    #[inline(always)]
    fn price(&self) -> i64 {
        self.price
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AskPriority {
    pub price: i64,
    pub seq_num: u64,
}

impl Ord for AskPriority {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        // Price priority first (lower is better for asks), then time priority (earlier is better)
        other
            .price
            .cmp(&self.price)
            .then_with(|| other.seq_num.cmp(&self.seq_num))
    }
}

impl PartialOrd for AskPriority {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for AskPriority {
    #[inline(always)]
    fn price(&self) -> i64 {
        self.price
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
    pub leaves_qty: i64,       // 8 bytes - Remaining quantity to be filled
    pub price: i64,            // 8 bytes - Price for Limit orders
    pub cum_qty: i64,          // 8 bytes - Cumulative quantity filled
    total_notional: i64,       // 8 bytes - Total value of fills
    pub seq_num: u64,          // 8 bytes - Monotonic identifier for order
    order_qty: i64,            // 8 bytes - Original quantity of the order
    pub side: SideEnum,        // 4 bytes - Buy or Sell
    pub ord_type: OrdTypeEnum, // 4 bytes - Limit or Market
    // 64 bytes total for first cache line

    // Cold fields (rarely accessed during matching) - subsequent cache lines
    pub cl_ord_id: ClOrdIdType, // 16 bytes - Client Order ID
    pub party_id: PartyIdType,  // 16 bytes - Trading Party ID
    transact_time: u64,         // 8 bytes - Time of transaction from client
    pub symbol: SymbolType,     // 6 bytes - Instrument symbol
}

impl Order {
    #[inline(always)]
    pub fn is_fully_filled(&self) -> bool {
        self.leaves_qty == 0
    }

    #[inline(always)]
    pub fn can_trade_with(&self, other_party_id: &PartyIdType) -> bool {
        self.party_id != *other_party_id
    }

    #[inline(always)]
    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cum_qty += qty;
        self.leaves_qty -= qty;
        self.total_notional += qty * price;
    }

    #[inline(always)]
    pub fn avg_px(&self) -> i64 {
        if self.cum_qty > 0 {
            self.total_notional / self.cum_qty
        } else {
            0
        }
    }
}

pub struct Buy;
pub struct Sell;

trait SideSpecificContext {
    type Priority: OrderBookPriority;
    type OppositePriority: OrderBookPriority;

    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority);
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority;
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool;
}

impl SideSpecificContext for Buy {
    type Priority = BidPriority;
    type OppositePriority = AskPriority;

    #[inline(always)]
    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.bid_pq.push(order_idx, priority);
    }

    #[inline(always)]
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        BidPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price >= resting_price
    }
}

impl SideSpecificContext for Sell {
    type Priority = AskPriority;
    type OppositePriority = BidPriority;

    #[inline(always)]
    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.ask_pq.push(order_idx, priority);
    }

    #[inline(always)]
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        AskPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price <= resting_price
    }
}

pub struct OrderBook {
    bid_pq: BidQueue,
    ask_pq: AskQueue,
    order_pool: OrderPool,
    order_map: OrderMap,
    order_id_counter: u64,
    exec_id_counter: u64,
    publisher: ExecutionReportPublisher,
}

impl OrderBook {
    pub fn new(publisher: ExecutionReportPublisher) -> Self {
        Self {
            bid_pq: BidQueue::with_capacity(MAX_PRICE_LEVELS),
            ask_pq: AskQueue::with_capacity(MAX_PRICE_LEVELS),
            order_pool: OrderPool::with_capacity(MAX_ORDERS),
            order_map: OrderMap::with_capacity(MAX_ORDERS),
            order_id_counter: 0,
            exec_id_counter: 0,
            publisher,
        }
    }

    pub fn process_new_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let order = self.decode_order(header_decoder);

        if self.order_pool.len() >= MAX_ORDERS {
            // TODO: REJECT ORDER WITH TEXT "BOOK_CAPACITY_EXCEEDED"
            self.publish_reject(&order, OrdRejReasonEnum::Other);
            return;
        }

        if self
            .order_map
            .contains_key(&(order.party_id, order.cl_ord_id))
        {
            self.publish_reject(&order, OrdRejReasonEnum::DuplicateOrder);
            return;
        }

        self.publish_new_order(&order);
        self.route_order(order)
    }

    #[inline(always)]
    fn decode_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) -> Order {
        let order_decoder: NewOrderSingleDecoder<'_> =
            NewOrderSingleDecoder::default().header(header_decoder, 0);

        self.order_id_counter += 1;
        let order_qty_val = order_decoder.order_qty_decoder().mantissa();

        Order {
            cl_ord_id: order_decoder.cl_ord_id(),
            party_id: order_decoder.party_id(),
            symbol: order_decoder.symbol(),
            side: order_decoder.side(),
            transact_time: order_decoder.transact_time_decoder().time(),
            order_qty: order_qty_val,
            ord_type: order_decoder.ord_type(),
            price: order_decoder.price_decoder().mantissa(),
            seq_num: self.order_id_counter,
            leaves_qty: order_qty_val,
            cum_qty: 0,
            total_notional: 0,
        }
    }

    #[inline(always)]
    fn route_order(&mut self, order: Order) {
        match order.ord_type {
            OrdTypeEnum::Limit => self.process_limit_order(order),
            OrdTypeEnum::Market => {
                // TODO: Implement market order processing
            }
            _ => {
                self.publish_reject(&order, OrdRejReasonEnum::Other)
                // TODO: Log and continue
            }
        }
    }

    #[inline(always)]
    fn process_limit_order(&mut self, order: Order) {
        match order.side {
            SideEnum::Buy => self.match_and_add_order::<Buy>(order),
            SideEnum::Sell => self.match_and_add_order::<Sell>(order),
            _ => {
                self.publish_reject(&order, OrdRejReasonEnum::Other);
                // TODO: Log and continue
            }
        }
    }

    // pub fn cancel_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
    //     // TODO:
    //     // let order_decoder: NewOrderSingleDecoder<'_> =
    //     //     NewOrderSingleDecoder::default().header(header_decoder, 0);
    //     // let cl_ord_id: [u8; 16] = order_decoder.cl_ord_id();
    //     // let cl_ord_id_str = Uuid::from_bytes(cl_ord_id);
    //     return;
    // }

    // pub fn process_market_order(&mut self, order: Order) {
    //     // TODO:
    //     return;
    // }

    #[inline(always)]
    fn add_order<S: SideSpecificContext>(&mut self, order: Order) {
        let order_key: OrderBookKey = (order.party_id, order.cl_ord_id);
        let priority = S::create_priority(order.price, order.seq_num);
        let order_idx = self.order_pool.insert(order);
        self.order_map.insert(order_key, order_idx);
        S::push_to_queue(self, order_idx, priority);
    }

    #[inline(always)]
    fn match_and_add_order<S: SideSpecificContext>(&mut self, mut aggressor_order: Order) {
        while aggressor_order.leaves_qty > 0 {
            match S::peek_opposite_queue(self) {
                Some((resting_idx, resting_priority)) => {
                    let resting_price = resting_priority.price();
                    if S::can_aggressor_match(aggressor_order.price, resting_price) {
                        if !self.execute_match::<S>(
                            &mut aggressor_order,
                            resting_idx,
                            resting_price,
                        ) {
                            break; // Self-trade detected, stop processing
                        }
                    } else {
                        break; // No more matches possible at this price
                    }
                }
                None => break, // No orders on opposite side
            }
        }

        if aggressor_order.leaves_qty > 0 {
            self.add_order::<S>(aggressor_order);
        }
    }

    // Returns true if matching should continue, false if it should stop (e.g., self-trade)
    fn execute_match<S: SideSpecificContext>(
        &mut self,
        aggressor_order: &mut Order,
        resting_order_idx: usize,
        resting_order_price: i64,
    ) -> bool {
        let (trade_qty, resting_order_snapshot, should_remove_resting, resting_order_key) = {
            let resting_order = self
                .order_pool
                .get_mut(resting_order_idx)
                .expect("Resting order must exist in pool for matching");

            // Prevent self-trade
            if !aggressor_order.can_trade_with(&resting_order.party_id) {
                // TODO: REJECT WIH TEXT "SELF_TRADE_PREVENTION"
                self.publish_reject(aggressor_order, OrdRejReasonEnum::Other);
                aggressor_order.leaves_qty = 0;
                return false;
            }

            let trade_qty = min(aggressor_order.leaves_qty, resting_order.leaves_qty);
            let trade_px = resting_order_price;

            aggressor_order.fill(trade_qty, trade_px);
            resting_order.fill(trade_qty, trade_px);

            let resting_order_snapshot = *resting_order;
            let should_remove_resting = resting_order.is_fully_filled();
            let key = if should_remove_resting {
                Some((resting_order.party_id, resting_order.cl_ord_id))
            } else {
                None
            };

            (
                trade_qty,
                resting_order_snapshot,
                should_remove_resting,
                key,
            )
        };

        self.publish_trade(aggressor_order, trade_qty, resting_order_price);
        self.publish_trade(&resting_order_snapshot, trade_qty, resting_order_price);

        if should_remove_resting {
            self.remove_resting_order::<S>(resting_order_idx, resting_order_key);
        }

        true // Continue matching
    }

    #[inline(always)]
    fn remove_resting_order<S: SideSpecificContext>(
        &mut self,
        order_idx: usize,
        map_key: Option<OrderBookKey>,
    ) {
        S::pop_opposite_queue(self);
        self.order_pool.remove(order_idx);
        if let Some(key) = map_key {
            self.order_map.remove(&key);
        }
    }

    #[inline(always)]
    fn publish_new_order(&mut self, order: &Order) {
        self.exec_id_counter += 1;
        self.publisher
            .publish_new_order(order, self.exec_id_counter);
    }

    #[inline(always)]
    fn publish_trade(&mut self, order: &Order, qty: i64, px: i64) {
        self.exec_id_counter += 1;
        self.publisher
            .publish_trade(order, self.exec_id_counter, qty, px);
    }

    #[inline(always)]
    fn publish_cancel(&mut self, order: &Order) {
        self.exec_id_counter += 1;
        self.publisher.publish_cancel(order, self.exec_id_counter);
    }

    #[inline(always)]
    fn publish_reject(&mut self, order: &Order, reason: OrdRejReasonEnum) {
        self.exec_id_counter += 1;
        self.publisher
            .publish_reject(order, self.exec_id_counter, reason);
    }
}
