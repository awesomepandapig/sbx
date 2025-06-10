// use std::collections::HashMap;
// use std::hash::Hash;

// use slab::Slab;

// use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;

// #[derive(Debug, Clone, Copy)]
// pub struct Order {
//     // Hot fields (accessed frequently during matching) - first cache line (64 bytes)
//     pub leaves_quantity: i64, // 8 bytes - Remaining quantity to be filled
//     pub price: i64,           // 8 bytes - Price for Limit orders
//     pub cumulative_quantity: i64, // 8 bytes - Cumulative quantity filled
//     pub total_notional: i128, // 16 bytes - Total value of fills
//     pub sequence_number: u64, // 8 bytes - Monotonic identifier for order
//     pub quantity: i64,        // 8 bytes - Original quantity of the order
//     pub side: SideEnum,       // 1 bytes - Buy or Sell
//     pub r#type: OrdTypeEnum,  // 1 bytes - Limit or Market
//     // 64 bytes total for first cache line

//     // Cold fields (rarely accessed during matching) - subsequent cache lines
//     pub client_order_id: UuidType, // 16 bytes - Client Order ID
//     pub account: UuidType,         // 16 bytes - Account ID
//     // pub transact_time: u64,        // 8 bytes - Time of transaction from client
//     pub symbol: SymbolType, // 6 bytes - Instrument symbol
// }

// impl Order {
//     pub fn fill(&mut self, qty: i64, price: i64) {
//         self.cumulative_quantity += qty;
//         self.leaves_quantity -= qty;
//         self.total_notional += i128::from(qty) * i128::from(price);
//     }

//     pub fn avg_px(&self) -> i64 {
//         if self.cumulative_quantity == 0 {
//             return 0;
//         }
//         let avg = self.total_notional / i128::from(self.cumulative_quantity);
//         i64::try_from(avg).expect("avg_px: VWAP out of i64 range — invariant broken") // TODO: NO EXPECTS
//     }
// }

// #[derive(Debug)]
// pub enum CapacityExceededError {
//     Slab,
//     Map,
// }

// pub struct BoundedSlab<T> {
//     max_len: u64,
//     inner: Slab<T>,
// }

// impl<T> BoundedSlab<T> {
//     pub fn with_capacity(max_len: u64) -> Self {
//         Self {
//             max_len,
//             inner: Slab::with_capacity(max_len as usize),
//         }
//     }

//     pub fn try_insert(&mut self, val: T) -> Result<usize, CapacityExceededError> {
//         if self.inner.len() as u64 >= self.max_len {
//             Err(CapacityExceededError::Slab)
//         } else {
//             Ok(self.inner.insert(val))
//         }
//     }

//     pub fn remove(&mut self, key: usize) -> T {
//         self.inner.remove(key)
//     }

//     pub fn is_full(&self) -> bool {
//         self.inner.len() as u64 >= self.max_len
//     }

//     pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
//         self.inner.get_mut(key)
//     }
// }

// pub struct BoundedHashMap<K, V> {
//     max_len: u64,
//     inner: HashMap<K, V>,
// }

// impl<K, V> BoundedHashMap<K, V>
// where
//     K: Hash + Eq,
// {
//     pub fn with_capacity(max_len: u64) -> Self {
//         Self {
//             max_len,
//             inner: HashMap::with_capacity(max_len as usize),
//         }
//     }

//     pub fn try_insert(&mut self, key: K, value: V) -> Result<(), CapacityExceededError> {
//         if self.inner.len() as u64 >= self.max_len {
//             Err(CapacityExceededError::Map)
//         } else {
//             self.inner.insert(key, value);
//             Ok(())
//         }
//     }

//     pub fn remove(&mut self, key: &K) -> Option<V> {
//         self.inner.remove(key)
//     }

//     pub fn contains_key(&self, key: &K) -> bool {
//         self.inner.contains_key(key)
//     }

//     pub fn get(&self, key: &K) -> Option<&V> {
//         self.inner.get(key)
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct CancelRequest {
    pub original_client_order_id: UuidType, // 16 bytes - Original Client Order ID
    pub client_order_id: UuidType,          // 16 bytes - Client Order ID
    pub account: UuidType,                  // 16 bytes - Account ID
    // pub transact_time: u64,                 // 8 bytes - Time of transaction from client
    // pub symbol: SymbolType,                 // 6 bytes - Instrument symbol
    pub side: SideEnum, // 1 bytes - Buy or Sell
}

pub type UuidType = u128;
// pub type SymbolType = [u8; 6];
// pub type OrderKey = (UuidType, UuidType);
// pub type OrderPool = BoundedSlab<Order>;
// pub type OrderMap = BoundedHashMap<OrderKey, usize>;
