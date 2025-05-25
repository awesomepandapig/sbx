use std::cmp::Ordering;
use std::cmp::min;
use std::collections::HashMap;

use chrono::Utc;
use priority_queue::PriorityQueue;
use slab::Slab;

use sbe::ReadBuf;
use sbe::message_header_codec::MessageHeaderDecoder;
use sbe::new_order_single_codec::NewOrderSingleDecoder;
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

pub trait OrderBookPriority:
    Ord + PartialOrd + Eq + PartialEq + std::fmt::Debug + Clone + Copy
{
    fn price(&self) -> i64;
    fn entry_time_ns(&self) -> u64;
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct BidPriority {
    pub price: i64,
    pub entry_time_ns: u64,
}

impl Ord for BidPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price
            .cmp(&other.price)
            .then_with(|| other.entry_time_ns.cmp(&self.entry_time_ns))
    }
}

impl PartialOrd for BidPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for BidPriority {
    fn price(&self) -> i64 {
        self.price
    }
    fn entry_time_ns(&self) -> u64 {
        self.entry_time_ns
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct AskPriority {
    pub price: i64,
    pub entry_time_ns: u64,
}

impl Ord for AskPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .price
            .cmp(&self.price)
            .then_with(|| other.entry_time_ns.cmp(&self.entry_time_ns))
    }
}

impl PartialOrd for AskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl OrderBookPriority for AskPriority {
    fn price(&self) -> i64 {
        self.price
    }
    fn entry_time_ns(&self) -> u64 {
        self.entry_time_ns
    }
}

pub type OrderPool = Slab<Order>;
pub type OrderMap = HashMap<[u8; 16], usize>;
pub type BidQueue = PriorityQueue<usize, BidPriority>;
pub type AskQueue = PriorityQueue<usize, AskPriority>;

#[derive(Debug, Clone)]
pub struct Order {
    cl_ord_id: [u8; 16],   // Client Order ID
    party_id: [u8; 16],    // Trading Party ID (e.g., client, firm)
    symbol: [u8; 6],       // Instrument symbol
    side: SideEnum,        // Buy or Sell
    transact_time: u64,    // Time of transaction from client
    order_qty: i64,        // Original quantity of the order
    ord_type: OrdTypeEnum, // Limit or Market
    price: i64,            // Price for Limit orders (ignored for Market)
    entry_time_ns: u64,    // Timestamp when order entered the book (ns)
    cum_qty: i64,          // Cumulative quantity filled
    leaves_qty: i64,       // Remaining quantity to be filled
    avg_px: i64,           // Average execution price for this order (0 if no fills)
}

pub struct Buy;
pub struct Sell;

trait SideSpecificContext {
    type Priority: OrderBookPriority;
    type OppositePriority: OrderBookPriority;

    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority);
    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)>;
    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)>;

    fn create_priority(price: i64, entry_time_ns: u64) -> Self::Priority;
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool;
}

impl SideSpecificContext for Buy {
    type Priority = BidPriority;
    type OppositePriority = AskPriority;

    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.bid_pq.push(order_idx, priority);
    }

    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.ask_pq.pop()
    }

    fn create_priority(price: i64, entry_time_ns: u64) -> Self::Priority {
        BidPriority {
            price,
            entry_time_ns,
        }
    }

    /// Buy aggressor matches if its price is >= resting ask price
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price >= resting_price
    }
}

impl SideSpecificContext for Sell {
    type Priority = AskPriority;
    type OppositePriority = BidPriority;

    fn push_to_queue(book: &mut OrderBook, order_idx: usize, priority: Self::Priority) {
        book.ask_pq.push(order_idx, priority);
    }

    fn peek_opposite_queue(book: &OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq
            .peek()
            .map(|(&idx, &priority_val)| (idx, priority_val))
    }

    fn pop_opposite_queue(book: &mut OrderBook) -> Option<(usize, Self::OppositePriority)> {
        book.bid_pq.pop()
    }

    fn create_priority(price: i64, entry_time_ns: u64) -> Self::Priority {
        AskPriority {
            price,
            entry_time_ns,
        }
    }

    /// Sell aggressor matches if its price is <= resting bid price
    fn can_aggressor_match(aggressor_price: i64, resting_price: i64) -> bool {
        aggressor_price <= resting_price
    }
}

pub struct OrderBook {
    bid_pq: BidQueue,
    ask_pq: AskQueue,
    order_pool: OrderPool,
    order_map: OrderMap,
}

impl OrderBook {
    pub fn new() -> Self {
        const MAX_ORDERS: usize = 2_000_000; // For the OrderPool
        const MAX_PRICE_LEVELS: usize = 500_000; // For bid/ask queues

        Self {
            bid_pq: BidQueue::with_capacity(MAX_PRICE_LEVELS),
            ask_pq: AskQueue::with_capacity(MAX_PRICE_LEVELS),
            order_pool: OrderPool::with_capacity(MAX_ORDERS),
            order_map: OrderMap::with_capacity(MAX_ORDERS),
        }
    }

    pub fn create_order(&mut self, header_decoder: MessageHeaderDecoder<ReadBuf<'_>>) {
        let order_decoder: NewOrderSingleDecoder<'_> =
            NewOrderSingleDecoder::default().header(header_decoder, 0);

        let order_qty_val = order_decoder.order_qty_decoder().mantissa();
        let order = Order {
            cl_ord_id: order_decoder.cl_ord_id(),
            party_id: order_decoder.party_id(),
            symbol: order_decoder.symbol(),
            side: order_decoder.side(),
            transact_time: order_decoder.transact_time_decoder().time(),
            order_qty: order_qty_val,
            ord_type: order_decoder.ord_type(),
            price: order_decoder.price_decoder().mantissa(),
            entry_time_ns: Utc::now().timestamp_nanos_opt().unwrap() as u64,
            leaves_qty: order_qty_val,
            cum_qty: 0,
            avg_px: 0,
        };

        match order.ord_type {
            OrdTypeEnum::Market => {
                // TODO: self.process_market_order(order);
            }
            OrdTypeEnum::Limit => match order.side {
                SideEnum::Buy => self.process_limit_order::<Buy>(order),
                SideEnum::Sell => self.process_limit_order::<Sell>(order),
                _ => {
                    panic!("Unknown side for limit order: {:?}", order.side);
                }
            },
            _ => {
                panic!("Unsupported OrdType: {:?}", order.ord_type);
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

    fn add_order<S: SideSpecificContext>(&mut self, order: Order) {
        let cl_ord_id = order.cl_ord_id;
        let priority = S::create_priority(order.price, order.entry_time_ns);
        let order_idx = self.order_pool.insert(order);
        self.order_map.insert(cl_ord_id, order_idx);
        S::push_to_queue(self, order_idx, priority);
    }

    fn process_limit_order<S: SideSpecificContext>(&mut self, mut aggressor_order: Order) {
        while aggressor_order.leaves_qty > 0 {
            if let Some((resting_idx, resting_priority_ref)) = S::peek_opposite_queue(self) {
                let resting_price = resting_priority_ref.price();
                if S::can_aggressor_match(aggressor_order.price, resting_price) {
                    self.process_match::<S>(&mut aggressor_order, resting_idx, resting_price);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        if aggressor_order.leaves_qty > 0 {
            self.add_order::<S>(aggressor_order);
        }
    }

    fn process_match<S: SideSpecificContext>(
        &mut self,
        aggressor_order: &mut Order,
        resting_order_idx: usize,
        resting_order_price: i64,
    ) {
        let (remove_resting_order, resting_cl_ord_id_for_map_removal) = {
            let resting_order = self
                .order_pool
                .get_mut(resting_order_idx)
                .expect("Resting order must exist in pool for matching");

            // Prevent self-trade
            if aggressor_order.party_id == resting_order.party_id {
                // "Cancel" the aggressor by marking it as fully filled (leaves_qty = 0)
                aggressor_order.cum_qty = aggressor_order.order_qty;
                aggressor_order.leaves_qty = 0;
                // TODO: Depending on rules, may need to cancel resting too, or specific self-trade handling (e.g. STPC, STPO)
                // self.cancel_order(header_decoder);
                return;
            }

            let trade_qty = min(aggressor_order.leaves_qty, resting_order.leaves_qty);
            let execution_price = resting_order_price;

            // CAN WE DE-DUPE THIS WITH A GENERIC FUNCTION? OR JUST A REGULAR FUNCTION EVEN?
            if aggressor_order.cum_qty + trade_qty > 0 {
                // Avoid division by zero if initial cum_qty is 0
                aggressor_order.avg_px = (aggressor_order.avg_px * aggressor_order.cum_qty
                    + execution_price * trade_qty)
                    / (aggressor_order.cum_qty + trade_qty);
            } else {
                aggressor_order.avg_px = execution_price; // First fill
            }
            if resting_order.cum_qty + trade_qty > 0 {
                resting_order.avg_px = (resting_order.avg_px * resting_order.cum_qty
                    + execution_price * trade_qty)
                    / (resting_order.cum_qty + trade_qty);
            } else {
                resting_order.avg_px = execution_price;
            }
            // ------------------

            aggressor_order.cum_qty += trade_qty;
            aggressor_order.leaves_qty -= trade_qty;
            resting_order.cum_qty += trade_qty;
            resting_order.leaves_qty -= trade_qty;

            println!(
                "Trade: Aggressor ClOrdID {:?} ({}), Resting ClOrdID {:?} ({}), Qty: {}, Price: {}",
                String::from_utf8_lossy(&aggressor_order.cl_ord_id),
                aggressor_order.side,
                String::from_utf8_lossy(&resting_order.cl_ord_id),
                resting_order.side,
                trade_qty,
                execution_price
            );
            // TODO: Publish execution report (SBE/FIX message) via Aeron or other transport

            if resting_order.leaves_qty == 0 {
                (true, Some(resting_order.cl_ord_id)) // Signal to remove resting order
            } else {
                (false, None) // Resting order still has quantity, leave it in the book
            }
        };

        // Remove resting order if fully filled
        if remove_resting_order {
            S::pop_opposite_queue(self)
                .expect("Resting order (marked as filled) should be poppable from its queue");
            self.order_pool.remove(resting_order_idx);
            if let Some(cl_ord_id) = resting_cl_ord_id_for_map_removal {
                self.order_map.remove(&cl_ord_id);
            }
        }
    }
}
