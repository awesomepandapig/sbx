use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use crate::messages::ExecutionReportMessage;
use sbe::side_enum::SideEnum;

#[derive(Clone, Debug)]
pub struct OrderBook {
    pub bids: BTreeMap<i64, i64>,
    pub asks: BTreeMap<i64, i64>,
    pub last_seen_id: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            last_seen_id: 0,
        }
    }

    pub fn add_order(&mut self, report: &ExecutionReportMessage) -> i64 {
        let map = match report.side {
            SideEnum::Buy => &mut self.bids,
            SideEnum::Sell => &mut self.asks,
            _ => panic!("Unknown side provided: {}", report.side),
        };
        let price = report.price;
        let quantity = map.entry(price).or_insert(0);
        *quantity += report.ord_qty;
        *quantity
    }

    pub fn fill_order(&mut self, report: &ExecutionReportMessage) -> i64 {
        let map = match report.side {
            SideEnum::Buy => &mut self.bids,
            SideEnum::Sell => &mut self.asks,
            _ => panic!("Unknown side provided: {}", report.side),
        };
        let price = report.price;

        match map.entry(price) {
            Entry::Occupied(mut entry) => {
                let quantity = entry.get_mut();
                *quantity -= report.last_qty;
                if *quantity == 0 {
                    entry.remove();
                    0
                } else {
                    *quantity
                }
            }
            Entry::Vacant(_) => {
                panic!(
                    "Assumption Violation: fill_order called for non-existent price level {} on side {}",
                    price, report.side
                );
            }
        }
    }

    pub fn remove_order(&mut self, report: &ExecutionReportMessage) -> i64 {
        let map = match report.side {
            SideEnum::Buy => &mut self.bids,
            SideEnum::Sell => &mut self.asks,
            _ => panic!("Unknown side provided: {}", report.side),
        };
        let price = report.price;

        match map.entry(price) {
            Entry::Occupied(mut entry) => {
                let quantity = entry.get_mut();
                *quantity -= report.ord_qty;
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
                    price, report.side
                );
            }
        }
    }

    pub fn get_best_bid(&self) -> Option<(&i64, &i64)> {
        self.bids.last_key_value()
    }

    pub fn get_best_ask(&self) -> Option<(&i64, &i64)> {
        self.asks.first_key_value()
    }
}
