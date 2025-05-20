use super::order::Order;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

#[derive(Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<i64, i64>,
    pub asks: BTreeMap<i64, i64>,
    pub sequence_num: String,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            sequence_num: "".to_string(),
        }
    }

    pub fn add_order(&mut self, order: &Order) -> i64 {
        let map = match order.side.as_str() {
            "buy" => &mut self.bids,
            "sell" => &mut self.asks,
            _ => panic!("Unknown side provided: {}", order.side),
        };
        let price = order.price.expect("Limit orders must have a price");
        let quantity = map.entry(price).or_insert(0);
        *quantity += order.size;
        *quantity
    }

    pub fn remove_order(&mut self, order: &Order) -> i64 {
        let map = match order.side.as_str() {
            "buy" => &mut self.bids,
            "sell" => &mut self.asks,
            _ => panic!("Unknown side provided: {}", order.side),
        };
        let price = order.price.expect("Limit orders must have a price");

        match map.entry(price) {
            Entry::Occupied(mut entry) => {
                let quantity = entry.get_mut();
                *quantity -= order.size;
                if *quantity == 0 {
                    entry.remove();
                    0
                } else {
                    *quantity
                }
            }
            Entry::Vacant(_) => {
                panic!(
                    "Assumption Violation: remove_order called for non-existent price level {} on side {}",
                    price, order.side
                );
            }
        }
    }
}
