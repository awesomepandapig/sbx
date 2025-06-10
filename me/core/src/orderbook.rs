use crate::config::order_count_max;

use slab::Slab;
use std::collections::{BTreeSet, HashMap};

use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

pub type UuidType = u128;
pub type SymbolType = [u8; 6];
pub type OrderKey = (UuidType, UuidType);

#[derive(Debug, Clone, Copy)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
    pub prev_order_idx: Option<usize>,
    pub next_order_idx: Option<usize>,

    pub leaves_quantity: i64, // 8 bytes - Remaining quantity to be filled
    pub price: i64,           // 8 bytes - Price for Limit orders
    pub cumulative_quantity: i64, // 8 bytes - Cumulative quantity filled
    pub total_notional: i128, // 16 bytes - Total value of fills
    pub sequence_number: u64, // 8 bytes - Monotonic identifier for order
    pub quantity: i64,        // 8 bytes - Original quantity of the order
    pub side: SideEnum,       // 1 bytes - Buy or Sell
    pub r#type: OrdTypeEnum,  // 1 bytes - Limit or Market
    // 64 bytes total for first cache line

    // Cold fields (rarely accessed during matching) - subsequent cache lines
    pub client_order_id: UuidType, // 16 bytes - Client Order ID
    pub account: UuidType,         // 16 bytes - Account ID
    // pub transact_time: u64,        // 8 bytes - Time of transaction from client
    pub symbol: SymbolType, // 6 bytes - Instrument symbol
}

impl Order {
    pub fn key(&self) -> OrderKey {
        (self.account, self.client_order_id)
    }

    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cumulative_quantity += qty;
        self.leaves_quantity -= qty;
        self.total_notional += i128::from(qty) * i128::from(price);
    }

    pub fn avg_px(&self) -> i64 {
        if self.cumulative_quantity == 0 {
            return 0;
        }
        let avg = self.total_notional / i128::from(self.cumulative_quantity);
        i64::try_from(avg).expect("avg_px: VWAP out of i64 range — invariant broken") // TODO: NO EXPECTS
    }
}

struct Limit {
    head_order_idx: Option<usize>,
    tail_order_idx: Option<usize>,
}

impl Limit {
    fn new() -> Self {
        Self {
            head_order_idx: None,
            tail_order_idx: None,
        }
    }
}

pub struct OrderBook {
    pub pool: Slab<Order>,
    pub order_key_map: HashMap<OrderKey, usize>,
    pub bids_price_tree: BTreeSet<i64>,
    pub asks_price_tree: BTreeSet<i64>,
    bids_price_map: HashMap<i64, Limit>,
    asks_price_map: HashMap<i64, Limit>,
}

impl OrderBook {
    pub fn new() -> Self {
        let order_count_max = order_count_max() as usize;

        Self {
            pool: Slab::with_capacity(order_count_max),
            order_key_map: HashMap::new(),
            bids_price_tree: BTreeSet::new(),
            asks_price_tree: BTreeSet::new(),
            bids_price_map: HashMap::with_capacity(order_count_max / 2),
            asks_price_map: HashMap::with_capacity(order_count_max / 2),
        }
    }

    /// Adds a new buy order to the book.
    pub fn add_bid(&mut self, order: Order) {
        let order_key = order.key();
        if self.order_key_map.contains_key(&order_key) {
            panic!("Cannot add order: duplicate client_order_id"); // TODO: NO PANIC
        }

        let order_idx = self.pool.insert(order);
        self.order_key_map.insert(order_key, order_idx);

        if let Some(limit) = self.bids_price_map.get_mut(&order.price) {
            let tail_idx = limit
                .tail_order_idx
                .expect("Inconsistency: Existing limit cannot be empty"); // TODO: Handle error
            self.pool[tail_idx].next_order_idx = Some(order_idx);
            self.pool[order_idx].prev_order_idx = Some(tail_idx);
            limit.tail_order_idx = Some(order_idx);
        } else {
            let mut new_limit = Limit::new();
            new_limit.head_order_idx = Some(order_idx);
            new_limit.tail_order_idx = Some(order_idx);
            self.bids_price_map.insert(order.price, new_limit);
            self.bids_price_tree.insert(order.price);
        }
    }

    /// Adds a new sell order to the book.
    pub fn add_ask(&mut self, order: Order) {
        let order_key = order.key();
        if self.order_key_map.contains_key(&order_key) {
            panic!("Cannot add order: duplicate client_order_id"); // TODO: NO PANIC
        }

        let order_idx = self.pool.insert(order);
        self.order_key_map.insert(order_key, order_idx);

        if let Some(limit) = self.asks_price_map.get_mut(&order.price) {
            let tail_idx = limit
                .tail_order_idx
                .expect("Inconsistency: Existing limit cannot be empty"); // TODO: Handle error
            self.pool[tail_idx].next_order_idx = Some(order_idx);
            self.pool[order_idx].prev_order_idx = Some(tail_idx);
            limit.tail_order_idx = Some(order_idx);
        } else {
            let mut new_limit = Limit::new();
            new_limit.head_order_idx = Some(order_idx);
            new_limit.tail_order_idx = Some(order_idx);
            self.asks_price_map.insert(order.price, new_limit);
            self.asks_price_tree.insert(order.price);
        }
    }

    pub fn remove(&mut self, order_key: OrderKey) -> Order {
        // Remove the order key from the order_key_map to get its stable index
        let order_idx = self
            .order_key_map
            .remove(&order_key)
            .expect("Cannot cancel order: order_key not found in order_id_map"); // TODO: Handle error

        // Remove the order from the memory pool
        let order = self.pool.remove(order_idx);

        // Get appropriate structs based on side
        let (price_map, tree) = if order.side == SideEnum::Buy {
            (&mut self.bids_price_map, &mut self.bids_price_tree)
        } else {
            (&mut self.asks_price_map, &mut self.asks_price_tree)
        };

        let limit = price_map
            .get_mut(&order.price)
            .expect("Data inconsistency: price level not found for a valid order"); // TODO: Handle error

        // Unlink the order from the doubly-linked list.
        match (order.prev_order_idx, order.next_order_idx) {
            (Some(prev), Some(next)) => {
                // Middle of the list
                self.pool[prev].next_order_idx = Some(next);
                self.pool[next].prev_order_idx = Some(prev);
            }
            (Some(prev), None) => {
                // Tail of the list
                self.pool[prev].next_order_idx = None;
                limit.tail_order_idx = Some(prev);
            }
            (None, Some(next)) => {
                // Head of the list
                self.pool[next].prev_order_idx = None;
                limit.head_order_idx = Some(next);
            }
            (None, None) => {
                // Only order at this price level
                // This is the O(log N) path.
                price_map.remove(&order.price);
                tree.remove(&order.price);
            }
        };

        order
    }

    /// Returns a mutable reference to the best bid order, if one exists.
    /// This allows for in-place modification of the order.
    /// # Time Complexity: O(log N)
    pub fn best_bid(&mut self) -> Option<&mut Order> {
        if let Some(&best_price) = self.bids_price_tree.last() {
            let head_idx = self
                .bids_price_map
                .get(&best_price)
                .expect("Inconsistency: sorted_prices has a price that price_map does not") // TODO: Handle error
                .head_order_idx
                .expect("Inconsistency: Price level exists but has no orders"); // TODO: Handle error

            self.pool.get_mut(head_idx)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the best ask/offer order, if one exists.
    /// This allows for in-place modification of the order.
    /// # Time Complexity: O(log N)
    pub fn best_ask(&mut self) -> Option<&mut Order> {
        if let Some(&best_price) = self.asks_price_tree.last() {
            let head_idx = self
                .asks_price_map
                .get(&best_price)
                .expect("Inconsistency: sorted_prices has a price that price_map does not") // TODO: Handle error
                .head_order_idx
                .expect("Inconsistency: Price level exists but has no orders"); // TODO: Handle error

            self.pool.get_mut(head_idx)
        } else {
            None
        }
    }
}
