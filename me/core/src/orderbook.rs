use super::publisher::ExecutionReportPublisher;

use std::cmp::Ordering;
use std::cmp::min;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::process;

use priority_queue::PriorityQueue;
use slab::Slab;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleDecoder;
use sbe::ord_rej_reason_enum::OrdRejReasonEnum;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

pub type UuidType = [u8; 16];
pub type SymbolType = [u8; 6];
pub type OrderBookKey = (UuidType, UuidType);
pub type OrderPool = Slab<Order>;
pub type OrderMap = HashMap<OrderBookKey, usize>;
pub type BidQueue = PriorityQueue<usize, BidPriority>;
pub type AskQueue = PriorityQueue<usize, AskPriority>;

use tracing::{error, warn};

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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
    pub leaves_qty: i64,       // 8 bytes - Remaining quantity to be filled
    pub price: i64,            // 8 bytes - Price for Limit orders
    pub cum_qty: i64,          // 8 bytes - Cumulative quantity filled
    pub total_notional: i128,  // 16 bytes - Total value of fills
    pub seq_num: u64,          // 8 bytes - Monotonic identifier for order
    pub qty: i64,              // 8 bytes - Original quantity of the order
    pub side: SideEnum,        // 4 bytes - Buy or Sell
    pub ord_type: OrdTypeEnum, // 4 bytes - Limit or Market
    // 64 bytes total for first cache line

    // Cold fields (rarely accessed during matching) - subsequent cache lines
    pub cl_ord_id: UuidType, // 16 bytes - Client Order ID
    pub account: UuidType,   // 16 bytes - Account ID
    pub transact_time: u64,      // 8 bytes - Time of transaction from client
    pub symbol: SymbolType,  // 6 bytes - Instrument symbol
}

impl Order {
    #[inline(always)]
    pub fn is_fully_filled(&self) -> bool {
        self.leaves_qty == 0
    }

    #[inline(always)]
    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cum_qty += qty;
        self.leaves_qty -= qty;
        self.total_notional += i128::from(qty) * i128::from(price);
    }

    #[inline(always)]
    pub fn avg_px(&self) -> i64 {
        if self.cum_qty == 0 {
            return 0;
        }
        let avg = self.total_notional / i128::from(self.cum_qty);
        i64::try_from(avg).expect("avg_px: VWAP out of i64 range — invariant broken")
    }
}

pub struct Buy;
pub struct Sell;

trait SideSpecificContext {
    type Priority: Ord + Copy + Debug;
    type OppositePriority: Ord + Copy + Debug;

    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority);
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority;
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool;
}

impl SideSpecificContext for Buy {
    type Priority = BidPriority;
    type OppositePriority = AskPriority;

    #[inline(always)]
    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.bid_pq.push(order_idx, priority);
    }

    #[inline(always)]
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        BidPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool {
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
    fn peek_best_opposite(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    #[inline(always)]
    fn pop_best_opposite(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq.pop()
    }

    #[inline(always)]
    fn create_priority(price: i64, seq_num: u64) -> Self::Priority {
        AskPriority { price, seq_num }
    }

    #[inline(always)]
    fn can_cross(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price <= resting_price
    }
}

macro_rules! execute_trade {
    ($self:expr, $aggressor_order:expr, $resting_order:expr, $resting_idx:expr) => {{
        let trade_qty = min($aggressor_order.leaves_qty, $resting_order.leaves_qty);
        let trade_px = $resting_order.price;

        $aggressor_order.fill(trade_qty, trade_px);
        $resting_order.fill(trade_qty, trade_px);

        $self.match_id_counter += 1;
        $self.exec_id_counter += 1;
        $self.publisher.publish_trade(
            $aggressor_order,
            $self.exec_id_counter,
            $self.match_id_counter,
            trade_qty,
            trade_px,
        );

        $self.exec_id_counter += 1;
        $self.publisher.publish_trade(
            $resting_order,
            $self.exec_id_counter,
            $self.match_id_counter,
            trade_qty,
            trade_px,
        );
    }};
}

pub struct OrderBook {
    max_orders: usize,
    bid_pq: BidQueue,
    ask_pq: AskQueue,
    order_pool: OrderPool,
    order_map: OrderMap,
    order_id_counter: u64,
    exec_id_counter: u64,
    match_id_counter: u64,
    publisher: ExecutionReportPublisher,
}

impl OrderBook {
    pub fn new(publisher: ExecutionReportPublisher) -> Self {
        let max_order_str = env::var("MAX_ORDERS").unwrap_or_else(|e| {
            // TODO: ERROR HANDLER
            error!(target: "configuration", variable = "MAX_ORDERS", error = ?e, "Required environment variable for order book max orders not set. Exiting.");
            process::exit(1);
        });
        let max_orders = max_order_str.parse::<usize>().unwrap_or_else(|e| {
            // TODO: ERROR HANDLER
            error!(target: "configuration", variable = "MAX_ORDERS", value = %max_order_str, error = ?e, "Failed to parse MAX_ORDERS as usize. Exiting.");
            process::exit(1);
        });

        Self {
            max_orders,
            bid_pq: BidQueue::with_capacity(max_orders / 2),
            ask_pq: AskQueue::with_capacity(max_orders / 2),
            order_pool: OrderPool::with_capacity(max_orders),
            order_map: OrderMap::with_capacity(max_orders),
            order_id_counter: 0,
            exec_id_counter: 0,
            match_id_counter: 0,
            publisher,
        }
    }

    pub fn process_new_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let mut order = self.decode_order(header_decoder);

        if self.order_pool.len() >= self.max_orders {
            self.publish_reject(&order, OrdRejReasonEnum::Other);
            // TODO: ERROR HANDLER
            warn!(
                target: "matching_engine_capacity",
                current_book_size = self.order_pool.len(),
                reason = "Book capacity limit reached",
                "OPERATIONAL WARNING: Order book capacity limit reached (current size: {}). New orders are being rejected. Consider investigating load or if MAX_ORDERS (={}) needs adjustment.",
                self.order_pool.len(),
                self.max_orders
            );
            return;
        }

        if self
            .order_map
            .contains_key(&(order.account, order.cl_ord_id))
        {
            self.publish_reject(&order, OrdRejReasonEnum::DuplicateOrder);
            return;
        }

        self.publish_new_order(&order);
        self.route_by_type(&mut order);
    }

    #[inline(always)]
    fn decode_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) -> Order {
        let order_decoder: NewOrderSingleDecoder<'_> =
            NewOrderSingleDecoder::default().header(header_decoder, 0);

        self.order_id_counter += 1;
        let order_qty_val = order_decoder.order_qty_decoder().mantissa();

        Order {
            cl_ord_id: order_decoder.cl_ord_id(),
            account: order_decoder.account(),
            symbol: order_decoder.symbol(),
            side: order_decoder.side(),
            transact_time: order_decoder.transact_time_decoder().time(),
            qty: order_qty_val,
            ord_type: order_decoder.ord_type(),
            price: order_decoder.price_decoder().mantissa(),
            seq_num: self.order_id_counter,
            leaves_qty: order_qty_val,
            cum_qty: 0,
            total_notional: 0,
        }
    }

    #[inline(always)]
    fn route_by_type(&mut self, order: &mut Order) {
        match (order.ord_type, order.side) {
            (OrdTypeEnum::Limit, SideEnum::Buy) => self.handle_order_limit::<Buy>(order),
            (OrdTypeEnum::Limit, SideEnum::Sell) => self.handle_order_limit::<Sell>(order),
            (OrdTypeEnum::Market, SideEnum::Buy) => self.handle_order_market::<Buy>(order),
            (OrdTypeEnum::Market, SideEnum::Sell) => self.handle_order_market::<Sell>(order),

            (OrdTypeEnum::Limit | OrdTypeEnum::Market, SideEnum::NullVal) => {
                self.reject_invalid_field(order, "side");
            }

            (OrdTypeEnum::NullVal, _) => {
                self.reject_invalid_field(order, "ord_type");
            }
        }
    }

    #[inline(always)]
    fn reject_invalid_field(&mut self, order: &Order, field: &'static str) {
        self.publish_reject(order, OrdRejReasonEnum::Other);
        // TODO: ERROR HANDLER
        error!(
            target: "matching_engine_critical",
            order_id = ?order.cl_ord_id,
            order_details = ?order,
            "CRITICAL ERROR: Order received with NullVal for {field}. Order rejected. This may indicate message corruption, a gateway bug, or SBE schema mismatch.",
        );
    }

    #[inline(always)]
    fn handle_order_limit<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_qty > 0 {
            let Some((resting_idx, _)) = S::peek_best_opposite(self) else {
                break; // No orders on opposite side
            };

            let Some(resting_order) = self.order_pool.get_mut(resting_idx) else { 
                return // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
            };

            // Check prices cross
            if !S::can_cross(aggressor_order.price, resting_order.price) {
                break;
            }

            // Check for self-trading
            if aggressor_order.account == resting_order.account {
                self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                return;
            }

            execute_trade!(self, aggressor_order, resting_order, resting_idx);
            if resting_order.leaves_qty == 0 {
                S::pop_best_opposite(self);
                self.remove_filled_order(resting_idx);
            }
        }

        // Any remaining portion is added to the book
        if aggressor_order.leaves_qty > 0 {
            self.add_order::<S>(aggressor_order);
        }
    }

    #[inline(always)]
    fn handle_order_market<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
        while aggressor_order.leaves_qty > 0 {
            let Some((resting_idx, _)) = S::peek_best_opposite(self) else {
                // No orders on opposite side
                self.publish_cancel(aggressor_order);
                return;
            };

            let Some(resting_order) = self.order_pool.get_mut(resting_idx) else { 
                return // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
            };

            // Check for self-trading
            if aggressor_order.account == resting_order.account {
                self.publish_cancel(aggressor_order); // TODO: publish cancel with STP as reason
                return;
            }

            execute_trade!(self, aggressor_order, resting_order, resting_idx);
            if resting_order.leaves_qty == 0 {
                S::pop_best_opposite(self);
                self.remove_filled_order(resting_idx);
            }
        }
    }

    // #[inline(always)]
    // fn handle_order_fill_or_kill<S: SideSpecificContext>(&mut self, aggressor_order: &mut Order) {
    //     let mut remaining_qty = aggressor_order.leaves_qty;
    //     let mut fills_queue = Vec::new(); // TODO: instead of vec::new use an array bounded by leaves_qty

    //     while aggressor_order.leaves_qty > 0 {
    //         let (resting_idx, _) = match S::peek_best_opposite(self) {
    //             Some(idx) => idx,
    //             _ => {
    //                 // No orders on opposite side - kill
    //                 self.publish_cancel(&aggressor_order);
    //                 return;
    //             }
    //         };

    //         let resting_order = match self.order_pool.get_mut(resting_idx) {
    //             Some(order) => order,
    //             _ => return, // TODO: CREATE AN ERROR ENUM AND HANDLER FUNCTION  Log fatal error and exit the program
    //         };

    //         // Check prices cross
    //         if !S::can_cross(aggressor_order.price, resting_order.price) {
    //             break;
    //         }

    //         // Check for self-trading
    //         if aggressor_order.account == resting_order.account {
    //             self.publish_cancel(&aggressor_order); // TODO: publish cancel with STP as reason
    //             return;
    //         }

    //         remaining_qty -= min(remaining_qty, resting_order.leaves_qty);
    //         fills_queue.push((resting_order, resting_idx));

    //         // Temporarily pop orders off the book
    //         S::pop_best_opposite(self);
    //     }

    //     if remaining_qty == 0 {
    //         for (resting_order, resting_idx) in fills_queue {
    //             execute_trade!(self, aggressor_order, resting_order, resting_idx);
    //         }
    //     } else {
    //         // COLDER? than fills but idk how often FOK fills if we want to mark as cold... prob not
    //         // If FOK fails add removed orders back to the book
    //         for (resting_order, resting_idx) in fills_queue {
    //             let priority = S::create_priority(resting_order.price, resting_order.seq_num);
    //             S::push_to_queue(self, resting_idx, priority);
    //         }

    //         // kill the order
    //         self.publish_cancel(&aggressor_order);
    //         return;
    //     }
    // }

    #[inline(always)]
    fn remove_filled_order(&mut self, resting_idx: usize) {
        let resting_order = self.order_pool.remove(resting_idx);
        let order_key = (resting_order.account, resting_order.cl_ord_id);
        self.order_map.remove(&order_key);
    }

    #[inline(always)]
    fn add_order<S: SideSpecificContext>(&mut self, order: &Order) {
        let order_key: OrderBookKey = (order.account, order.cl_ord_id);
        let priority = S::create_priority(order.price, order.seq_num);
        let order_idx = self.order_pool.insert(*order);
        self.order_map.insert(order_key, order_idx);
        S::push_to_queue(self, order_idx, priority);
    }

    // pub fn cancel_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
    //     // TODO:
    //     // let order_decoder: NewOrderSingleDecoder<'_> =
    //     //     NewOrderSingleDecoder::default().header(header_decoder, 0);
    //     // let cl_ord_id: [u8; 16] = order_decoder.cl_ord_id();
    //     // let cl_ord_id_str = Uuid::from_bytes(cl_ord_id);
    //     return;
    // }

    #[inline(always)]
    fn publish_new_order(&mut self, order: &Order) {
        self.exec_id_counter += 1;
        self.publisher
            .publish_new_order(order, self.exec_id_counter);
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
