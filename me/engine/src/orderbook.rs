use std::cmp::Ordering;
use std::cmp::min;
use std::collections::HashMap;

use priority_queue::PriorityQueue;
use slab::Slab;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleDecoder;
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
    // fn seq_num(&self) -> u64;
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BidPriority {
    pub price: i64,
    pub seq_num: u64,
}

impl Ord for BidPriority {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.price
            .cmp(&other.price)
            .then_with(|| other.seq_num.cmp(&self.seq_num))
    }
}

impl PartialOrd for BidPriority {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for BidPriority {
    #[inline]
    fn price(&self) -> i64 {
        self.price
    }

    // #[inline]
    // fn seq_num(&self) -> u64 {
    //     self.seq_num
    // }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AskPriority {
    pub price: i64,
    pub seq_num: u64,
}

impl Ord for AskPriority {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // Price priority first (lower is better for asks), then time priority (earlier is better)
        other
            .price
            .cmp(&self.price)
            .then_with(|| other.seq_num.cmp(&self.seq_num))
    }
}

impl PartialOrd for AskPriority {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for AskPriority {
    #[inline]
    fn price(&self) -> i64 {
        self.price
    }

    // #[inline]
    // fn seq_num(&self) -> u64 {
    //     self.seq_num
    // }
}

#[repr(C)]
#[derive(Debug)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
    leaves_qty: i64,       // 8 bytes - Remaining quantity to be filled
    price: i64,            // 8 bytes - Price for Limit orders
    cum_qty: i64,          // 8 bytes - Cumulative quantity filled
    total_notional: i64,   // 8 bytes - Total value of fills
    seq_num: u64,          // 8 bytes - Monotonic identifier for order
    order_qty: i64,        // 8 bytes - Original quantity of the order
    transact_time: u64,    // 8 bytes - Time of transaction from client
    side: SideEnum,        // 4 bytes - Buy or Sell
    ord_type: OrdTypeEnum, // 4 bytes - Limit or Market
    // 64 bytes total for first cache line

    // Cold fields (rarely accessed during matching) - subsequent cache lines
    cl_ord_id: ClOrdIdType, // 16 bytes - Client Order ID
    party_id: PartyIdType,  // 16 bytes - Trading Party ID
    symbol: SymbolType,     // 6 bytes - Instrument symbol
                            // Padding will be added by compiler as needed
}

impl Order {
    #[inline]
    pub fn is_fully_filled(&self) -> bool {
        self.leaves_qty == 0
    }

    #[inline]
    pub fn can_trade_with(&self, other_party_id: &PartyIdType) -> bool {
        self.party_id != *other_party_id
    }

    #[inline]
    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cum_qty += qty;
        self.leaves_qty -= qty;
        self.total_notional += qty * price;
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

    #[inline]
    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.bid_pq.push(order_idx, priority);
    }

    #[inline]
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline]
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq.pop()
    }

    #[inline]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        BidPriority { price, seq_num }
    }

    #[inline]
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price >= resting_price
    }
}

impl SideSpecificContext for Sell {
    type Priority = AskPriority;
    type OppositePriority = BidPriority;

    #[inline]
    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.ask_pq.push(order_idx, priority);
    }

    #[inline]
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline]
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq.pop()
    }

    #[inline]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        AskPriority { price, seq_num }
    }

    #[inline]
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
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bid_pq: BidQueue::with_capacity(MAX_PRICE_LEVELS),
            ask_pq: AskQueue::with_capacity(MAX_PRICE_LEVELS),
            order_pool: OrderPool::with_capacity(MAX_ORDERS),
            order_map: OrderMap::with_capacity(MAX_ORDERS),
            order_id_counter: 0,
            exec_id_counter: 0,
        }
    }

    pub fn create_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        if self.order_pool.len() >= MAX_ORDERS {
            // TODO: REJECT ORDERS IF BOOK SIZE IS TOO LARGE
            println!("Book has exceeded capacity");
            return;
        }

        let order = self.decode_order(header_decoder);

        match order.ord_type {
            OrdTypeEnum::Market => {
                // TODO: Implement market order processing
                // self.process_market_order(order);
            }
            OrdTypeEnum::Limit => match order.side {
                SideEnum::Buy => {
                    self.process_limit_order::<Buy>(order);
                }
                SideEnum::Sell => {
                    self.process_limit_order::<Sell>(order);
                }
                _ => {
                    panic!("Unknown side for limit order: {:?}", order.side);
                }
            },
            _ => {
                panic!("Unsupported OrdType: {:?}", order.ord_type);
            }
        }
    }

    fn decode_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) -> Order {
        let order_decoder: NewOrderSingleDecoder<'_> =
            NewOrderSingleDecoder::default().header(header_decoder, 0);

        self.order_id_counter += 1;
        let order_seq_num = self.order_id_counter;
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
            seq_num: order_seq_num,
            leaves_qty: order_qty_val,
            cum_qty: 0,
            total_notional: 0,
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

    #[inline]
    fn add_order<S: SideSpecificContext>(&mut self, order: Order) {
        let map_key: OrderBookKey = (order.party_id, order.cl_ord_id);
        let priority = S::create_priority(order.price, order.seq_num);
        let order_idx = self.order_pool.insert(order);
        self.order_map.insert(map_key, order_idx);
        S::push_to_queue(self, order_idx, priority);
    }

    fn process_limit_order<S: SideSpecificContext>(&mut self, mut aggressor_order: Order) {
        while aggressor_order.leaves_qty > 0 {
            match S::peek_opposite_queue(self) {
                Some((resting_idx, resting_priority)) => {
                    let resting_price = resting_priority.price();
                    if S::can_aggressor_match(aggressor_order.price, resting_price) {
                        if !self.process_match::<S>(
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
    fn process_match<S: SideSpecificContext>(
        &mut self,
        aggressor_order: &mut Order,
        resting_order_idx: usize,
        resting_order_price: i64,
    ) -> bool {
        let (should_remove_resting, resting_map_key) = {
            let resting_order = self
                .order_pool
                .get_mut(resting_order_idx)
                .expect("Resting order must exist in pool for matching");

            // Prevent self-trade
            if !aggressor_order.can_trade_with(&resting_order.party_id) {
                // TODO: Cancel the aggressor
                aggressor_order.cum_qty = aggressor_order.order_qty;
                aggressor_order.leaves_qty = 0;
                return false;
            }

            let trade_qty = min(aggressor_order.leaves_qty, resting_order.leaves_qty);
            let execution_price = resting_order_price;

            aggressor_order.fill(trade_qty, execution_price);
            resting_order.fill(trade_qty, execution_price);

            self.exec_id_counter += 1;

            // TODO: Replace with proper execution report publishing via SBE/FIX over Aeron
            println!(
                "Trade #{}: Aggressor {} {:?} vs Resting {} {:?}, Qty: {}, Price: {}",
                self.exec_id_counter,
                String::from_utf8_lossy(&aggressor_order.cl_ord_id),
                aggressor_order.side,
                String::from_utf8_lossy(&resting_order.cl_ord_id),
                resting_order.side,
                trade_qty,
                execution_price
            );

            if resting_order.is_fully_filled() {
                let key = (resting_order.party_id, resting_order.cl_ord_id);
                (true, Some(key))
            } else {
                (false, None)
            }
        };

        if should_remove_resting {
            self.remove_resting_order::<S>(resting_order_idx, resting_map_key);
        }

        true // Continue matching
    }

    #[inline]
    fn remove_resting_order<S: SideSpecificContext>(
        &mut self,
        order_idx: usize,
        map_key: Option<OrderBookKey>,
    ) {
        S::pop_opposite_queue(self).expect("Resting order should be poppable from its queue");
        self.order_pool.remove(order_idx);
        if let Some(key) = map_key {
            self.order_map.remove(&key);
        }
    }

    // #[inline]
    // fn publish_trade(
    //     &self,
    //     exec_id: u64,
    //     aggressor_order: &Order,
    //     resting_order: &Order,
    //     qty: i64,
    //     price: i64,
    // ) {
    //     return;
    // }

    // fn publish_reject() {
    //     return;
    // }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}
