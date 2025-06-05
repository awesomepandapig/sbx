use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

use priority_queue::PriorityQueue;
use slab::Slab;

use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

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

#[derive(Debug, Clone, Copy)]
pub struct Order {
    // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
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
    #[inline(always)]
    pub fn is_fully_filled(&self) -> bool {
        self.leaves_quantity == 0
    }

    #[inline(always)]
    pub fn fill(&mut self, qty: i64, price: i64) {
        self.cumulative_quantity += qty;
        self.leaves_quantity -= qty;
        self.total_notional += i128::from(qty) * i128::from(price);
    }

    #[inline(always)]
    pub fn avg_px(&self) -> i64 {
        if self.cumulative_quantity == 0 {
            return 0;
        }
        let avg = self.total_notional / i128::from(self.cumulative_quantity);
        i64::try_from(avg).expect("avg_px: VWAP out of i64 range — invariant broken") // TODO: NO EXPECTS
    }
}

#[derive(Debug)]
pub enum CapacityExceededError {
    Slab,
    Queue,
    Map,
}

pub struct BoundedSlab<T> {
    max_len: u64,
    inner: Slab<T>,
}

impl<T> BoundedSlab<T> {
    pub fn with_capacity(max_len: u64) -> Self {
        Self {
            max_len,
            inner: Slab::with_capacity(max_len as usize),
        }
    }

    pub fn try_insert(&mut self, val: T) -> Result<usize, CapacityExceededError> {
        if self.inner.len() as u64 >= self.max_len {
            Err(CapacityExceededError::Slab)
        } else {
            Ok(self.inner.insert(val))
        }
    }

    pub fn remove(&mut self, key: usize) -> T {
        self.inner.remove(key)
    }

    pub fn is_full(&self) -> bool {
        self.inner.len() as u64 >= self.max_len
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        self.inner.get_mut(key)
    }
}

pub struct BoundedPriorityQueue<I, P> {
    max_len: u64,
    inner: PriorityQueue<I, P>,
}

impl<I, P> BoundedPriorityQueue<I, P>
where
    P: Ord,
    I: Hash + Eq,
{
    pub fn with_capacity(max_len: u64) -> Self {
        Self {
            max_len,
            inner: PriorityQueue::with_capacity(max_len as usize),
        }
    }

    pub fn try_insert(&mut self, item: I, priority: P) -> Result<(), CapacityExceededError> {
        if self.inner.len() as u64 >= self.max_len {
            Err(CapacityExceededError::Queue)
        } else {
            self.inner.push(item, priority);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<(I, P)> {
        self.inner.pop()
    }

    pub fn remove<Q>(&mut self, item: &Q) -> Option<(I, P)>
    where
        I: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.remove(item)
    }

    pub fn is_full(&self) -> bool {
        self.inner.len() as u64 >= self.max_len
    }

    pub fn peek(&self) -> Option<(&I, &P)> {
        self.inner.peek()
    }
}

pub struct BoundedHashMap<K, V> {
    max_len: u64,
    inner: HashMap<K, V>,
}

impl<K, V> BoundedHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn with_capacity(max_len: u64) -> Self {
        Self {
            max_len,
            inner: HashMap::with_capacity(max_len as usize),
        }
    }

    pub fn try_insert(&mut self, key: K, value: V) -> Result<(), CapacityExceededError> {
        if self.inner.len() as u64 >= self.max_len {
            Err(CapacityExceededError::Map)
        } else {
            self.inner.insert(key, value);
            Ok(())
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CancelRequest {
    pub original_client_order_id: UuidType, // 16 bytes - Original Client Order ID
    pub client_order_id: UuidType,          // 16 bytes - Client Order ID
    pub account: UuidType,                  // 16 bytes - Account ID
    // pub transact_time: u64,                 // 8 bytes - Time of transaction from client
    // pub symbol: SymbolType,                 // 6 bytes - Instrument symbol
    pub side: SideEnum, // 1 bytes - Buy or Sell
}

pub type UuidType = [u8; 16];
pub type SymbolType = [u8; 6];
pub type OrderBookKey = (UuidType, UuidType);
pub type OrderPool = BoundedSlab<Order>;
pub type OrderMap = BoundedHashMap<OrderBookKey, usize>;
pub type BidQueue = BoundedPriorityQueue<usize, BidPriority>;
pub type AskQueue = BoundedPriorityQueue<usize, AskPriority>;
