use super::order::Order;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

pub struct OrderBook {
    bids: BTreeMap<i64, i64>,
    asks: BTreeMap<i64, i64>,
}

impl OrderBook {
    pub fn new() -> Self {
        return Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        };
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
        // TODO: update book snapshot in redis
        return *quantity;
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
                    return 0;
                } else {
                    return *quantity;
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

    pub fn get_best_bid(&self) -> Option<(&i64, &i64)> {
        return self.bids.last_key_value(); // maximum key
    }

    pub fn get_best_ask(&self) -> Option<(&i64, &i64)> {
        return self.asks.first_key_value(); // minimum key
    }
}
