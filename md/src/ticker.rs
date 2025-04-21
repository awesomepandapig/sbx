use super::order::Order;
use super::order_book::OrderBook;
use chrono::{Datelike, Duration, TimeZone, Utc};
use redis::{Commands, Connection, RedisResult};
use serde::Serialize;
use serde_json;
use std::collections::HashMap;
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
                // Set open on first trade of the day
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

    fn create_ticker_snapshot(&self, book: &OrderBook, timestamp: i64) -> Ticker {
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
    states: HashMap<String, TickerState>,
    // Batch channel can be constant or configurable
    batch_channel_name: String,
}

impl TickerService {
    pub fn new(products: Vec<String>) -> Self {
        let mut states = HashMap::new();
        for product_id in products {
            states.insert(product_id.clone(), TickerState::new(product_id));
        }
        return Self {
            states,
            batch_channel_name: "marketdata:ticker_batch:all".to_string(), // Global batch channel
        };
    }

    pub fn process_match(&mut self, order: &Order) {
        if let Some(state) = self.states.get_mut(&order.product_id) {
            state.update_on_match(order);
        } else {
            // Should not happen if main loop checks product validity
            eprintln!(
                "[TickerService] Received match for unknown product: {}",
                order.product_id
            );
        }
    }

    pub fn emit_individual(&mut self, conn: &mut Connection, product_id: &str, book: &OrderBook) {
        if let Some(state) = self.states.get_mut(product_id) {
            let now = Utc::now().timestamp();
            // Optional: Ensure windows are up-to-date before emitting
            // state.check_and_reset_windows(now);
            let ticker = state.create_ticker_snapshot(book, now);
            let channel_name = format!("marketdata:ticker:{}", product_id);

            match serde_json::to_string(&ticker) {
                Ok(json_payload) => {
                    let result: RedisResult<i64> = conn.publish(channel_name, json_payload);
                    if let Err(e) = result {
                        eprintln!("Failed to publish Ticker update to Redis");
                    }
                }
                Err(e) => {
                    eprintln!("[{}] Failed to serialize TickerData: {}", product_id, e);
                }
            }
        }
    }

    // TODO: CHANGE TO EMIT BATCH PER PRODUCT
    // marketdata:ticker_batch:JSP
    pub fn emit_batch(&mut self, conn: &mut Connection, order_books: &HashMap<String, OrderBook>) {
        let now = Utc::now().timestamp();
        let mut batch_payloads: Vec<Ticker> = Vec::with_capacity(self.states.len());

        for (product_id, state) in self.states.iter_mut() {
            // Ensure state's time windows are current before creating snapshot
            state.check_and_reset_windows(now);

            // Get the corresponding order book
            if let Some(book) = order_books.get(product_id) {
                let ticker = state.create_ticker_snapshot(book, now);
                batch_payloads.push(ticker);
            } else {
                eprintln!("[TickerService] No order book found for product '{}' during batch emit. Skipping.", product_id);
                // TODO: Optionally create a default/empty ticker?
            }
        }

        if batch_payloads.is_empty() {
            return;
        }

        match serde_json::to_string(&batch_payloads) {
            Ok(json_payload) => {
                let result: RedisResult<i64> =
                    conn.publish(self.batch_channel_name.clone(), json_payload);
                if let Err(e) = result {
                    eprintln!("Failed to publish TickerBatch update to Redis");
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize TickerBatchData");
            }
        }
    }
}
