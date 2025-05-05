use super::order::Order;
use super::order_book::OrderBook;
use chrono::{Datelike, Duration, TimeZone, Utc};
use redis::{Commands, Connection, RedisResult};
use serde::Serialize;
use serde_json;
// use std::time::{Duration as StdDuration, Instant}; // For batch timing

#[derive(Serialize, Debug, Clone)]
pub struct Ticker {
    product_id: String,
    price: i64,
    volume_24_h: i64,
    low_24_h: i64,
    high_24_h: i64,
    open_24_h: i64,
    low_52_w: i64,
    high_52_w: i64,
    price_percent_chg_24_h: f64,
    best_bid: i64,
    best_bid_quantity: i64,
    best_ask: i64,
    best_ask_quantity: i64,
    timestamp: i64,
}

#[derive(Debug, Clone)]
struct TickerState {
    product_id: String,
    start_24_hr_ts: i64,
    start_52_w_ts: i64,
    volume_24_h: i64,
    low_24_h: i64,
    high_24_h: i64,
    open_24_hr: i64,
    low_52_w: i64,
    high_52_w: i64,
    last_price: i64,
}

impl TickerState {
    pub fn new(product_id: String) -> Self {
        let now = Utc::now();

        let start_24_hr_ts = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
            .unwrap()
            .timestamp();

        let start_52_w_ts = Utc
            .with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        return Self {
            product_id,
            start_24_hr_ts,
            start_52_w_ts,
            volume_24_h: 0,
            low_24_h: i64::MAX,
            high_24_h: i64::MIN,
            open_24_hr: 0,
            low_52_w: i64::MAX,
            high_52_w: i64::MIN,
            last_price: 0,
        };
    }

    fn reset_24_h_values(&mut self) {
        let current_day_start_dt = Utc.timestamp_opt(self.start_24_hr_ts, 0).unwrap();
        let next_day_start_dt = current_day_start_dt + Duration::days(1);
        self.start_24_hr_ts = next_day_start_dt.timestamp();
        self.volume_24_h = 0;
        self.low_24_h = i64::MAX;
        self.high_24_h = i64::MIN;
        self.open_24_hr = self.last_price; // Carry over last price as new open
    }

    fn reset_52_w_values(&mut self) {
        let current_year_start_dt = Utc.timestamp_opt(self.start_52_w_ts, 0).unwrap();
        let next_year_start_dt = Utc
            .with_ymd_and_hms(current_year_start_dt.year() + 1, 1, 1, 0, 0, 0)
            .unwrap();
        self.start_52_w_ts = next_year_start_dt.timestamp();
        self.low_52_w = i64::MAX;
        self.high_52_w = i64::MIN;
    }

    fn check_and_reset_windows(&mut self, current_ts: i64) {
        let next_day_start_ts = Utc
            .timestamp_opt(self.start_24_hr_ts, 0)
            .unwrap()
            .timestamp()
            + Duration::days(1).num_seconds();
        while current_ts >= next_day_start_ts {
            self.reset_24_h_values();
        }

        let next_year_start_ts = Utc
            .with_ymd_and_hms(
                Utc.timestamp_opt(self.start_52_w_ts, 0).unwrap().year() + 1,
                1,
                1,
                0,
                0,
                0,
            )
            .unwrap()
            .timestamp();
        while current_ts >= next_year_start_ts {
            self.reset_52_w_values();
        }
    }

    fn update_on_match(&mut self, order: &Order) {
        if let Some(price) = order.price {
            let size = order.size;
            let timestamp = order.created_at;

            self.check_and_reset_windows(timestamp);

            self.last_price = price;
            self.volume_24_h += size;

            self.low_24_h = self.low_24_h.min(price);
            self.high_24_h = self.high_24_h.max(price);
            if self.open_24_hr == 0 {
                self.open_24_hr = price;
            }

            self.low_52_w = self.low_52_w.min(price);
            self.high_52_w = self.high_52_w.max(price);
        }
    }

    fn calculate_price_percent_change(&self) -> f64 {
        if self.open_24_hr != 0 && self.last_price != 0 {
            return ((self.last_price as f64 - self.open_24_hr as f64) / self.open_24_hr as f64)
                * 100.0;
        }
        return 0.0;
    }

    fn create_ticker(&self, book: &OrderBook, timestamp: i64) -> Ticker {
        let (best_bid, best_bid_quantity) = book
            .get_best_bid()
            .map(|(p, q)| (*p, *q))
            .unwrap_or_default();
        let (best_ask, best_ask_quantity) = book
            .get_best_ask()
            .map(|(p, q)| (*p, *q))
            .unwrap_or_default();

        // Handle cases where min/max haven't been updated yet
        let low_24h = if self.low_24_h == i64::MAX {
            self.last_price
        } else {
            self.low_24_h
        };
        let high_24h = if self.high_24_h == i64::MIN {
            self.last_price
        } else {
            self.high_24_h
        };
        let low_52w = if self.low_52_w == i64::MAX {
            self.last_price
        } else {
            self.low_52_w
        };
        let high_52w = if self.high_52_w == i64::MIN {
            self.last_price
        } else {
            self.high_52_w
        };

        return Ticker {
            product_id: self.product_id.clone(),
            price: self.last_price,
            volume_24_h: self.volume_24_h,
            low_24_h: low_24h,
            high_24_h: high_24h,
            open_24_h: self.open_24_hr,
            low_52_w: low_52w,
            high_52_w: high_52w,
            price_percent_chg_24_h: self.calculate_price_percent_change(),
            best_bid,
            best_bid_quantity,
            best_ask,
            best_ask_quantity,
            timestamp,
        };
    }
}

pub struct TickerService {
    product_id: String,
    state: TickerState,
    batch_channel_name: String,
}

impl TickerService {
    pub fn new(product_id: String) -> Self {
        let state = TickerState::new(product_id.clone());
        let batch_channel_name = format!("marketdata:ticker_batch:{}", product_id);
        return Self {
            product_id,
            state,
            batch_channel_name,
        };
    }

    // TODO: This error checking may be unnecessary
    pub fn process_match(&mut self, order: &Order) {
        if order.product_id == self.product_id {
            self.state.update_on_match(order);
        } else {
            eprintln!(
                "[TickerService] Ignored order with mismatched product_id: {}",
                order.product_id
            );
        }
    }

    pub fn emit_individual(&mut self, conn: &mut Connection, book: &OrderBook) {
        let now = Utc::now().timestamp();
        let ticker = self.state.create_ticker(book, now);
        let channel_name = format!("marketdata:ticker:{}", self.product_id);

        match serde_json::to_string(&ticker) {
            Ok(json_payload) => {
                let result: RedisResult<i64> = conn.publish(channel_name, json_payload);
                if let Err(e) = result {
                    eprintln!("Failed to publish individual ticker: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Serialization error for individual ticker: {}", e);
            }
        }
    }

    
    pub fn emit_batch(&mut self, conn: &mut Connection, book: &OrderBook) {
        let now = Utc::now().timestamp();
        self.state.check_and_reset_windows(now);

        let ticker = self.state.create_ticker(book, now);
        match serde_json::to_string(&vec![ticker]) {
            Ok(json_payload) => {
                let result: RedisResult<i64> =
                    conn.publish(&self.batch_channel_name, json_payload);
                if let Err(e) = result {
                    eprintln!("Failed to publish ticker batch: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Serialization error for ticker batch: {}", e);
            }
        }
    }
}
