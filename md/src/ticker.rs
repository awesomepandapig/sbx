use super::order::Order;
use super::order_book::OrderBook;
use chrono::{Datelike, Duration, TimeZone, Utc};
use redis::{Commands, Connection, RedisResult};
use serde::Serialize;
use serde_json;

// TODO: REPLACE WITH PRODUCTID
const CHANNEL_NAME: &'static str = "marketdata:ticker:JSP";

#[derive(Serialize)]
struct Ticker {
    product_id: String,
    price: i64,
    volume_24_h: i64,
    low_24_h: i64,
    high_24_h: i64,
    low_52_w: i64,
    high_52_w: i64,
    price_percent_chg_24_h: f64,
    best_bid: i64,
    best_bid_quantity: i64,
    best_ask: i64,
    best_ask_quantity: i64,
}

pub struct TickerBuilder {
    start_24_hr: i64,
    start_52_w: i64,
    volume_24_h: i64,
    low_24_h: i64,
    high_24_h: i64,
    open_24_hr: i64,
    low_52_w: i64,
    high_52_w: i64,
    price: i64,
    price_percent_chg_24_h: f64,
    batch_start: i64,
    batch_updates: Vec<Ticker>,
}

impl TickerBuilder {
    pub fn new() -> Self {
        let now = Utc::now();

        let start_24_hr = Utc
            .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
            .unwrap()
            .timestamp();

        let start_52_w = Utc
            .with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp();

        return Self {
            start_24_hr,
            start_52_w,
            volume_24_h: 0,
            low_24_h: i64::MAX,
            high_24_h: i64::MIN,
            open_24_hr: 0,
            low_52_w: i64::MAX,
            high_52_w: i64::MIN,
            price: 0,
            price_percent_chg_24_h: 0.0,
            batch_start: now.timestamp(),
            batch_updates: Vec::new(),
        };
    }

    fn reset_24_h_values(&mut self) {
        let next_day =
            (Utc.timestamp_opt(self.start_24_hr, 0).unwrap() + Duration::days(1)).timestamp();
        self.start_24_hr = next_day;
        self.volume_24_h = 0;
        self.low_24_h = i64::MAX;
        self.high_24_h = i64::MIN;
        self.open_24_hr = 0;
        self.price_percent_chg_24_h = 0.0;
    }

    fn reset_52_w_values(&mut self) {
        let curr_year = Utc.timestamp_opt(self.start_52_w, 0).unwrap().year();
        let next_year = Utc
            .with_ymd_and_hms(curr_year + 1, 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp();
        self.start_52_w = next_year;
        self.low_52_w = i64::MAX;
        self.high_52_w = i64::MIN;
    }

    pub fn process_order(&mut self, order: &Order) {
        let mut next_day =
            (Utc.timestamp_opt(self.start_24_hr, 0).unwrap() + Duration::days(1)).timestamp();
        let mut next_year = Utc
            .with_ymd_and_hms(
                Utc.timestamp_opt(self.start_52_w, 0).unwrap().year() + 1,
                1,
                1,
                0,
                0,
                0,
            )
            .unwrap()
            .timestamp();

        while order.created_at >= next_day {
            self.reset_24_h_values();
            next_day =
                (Utc.timestamp_opt(self.start_24_hr, 0).unwrap() + Duration::days(1)).timestamp();
        }
        while order.created_at >= next_year {
            self.reset_52_w_values();
            next_year = Utc
                .with_ymd_and_hms(
                    Utc.timestamp_opt(self.start_52_w, 0).unwrap().year() + 1,
                    1,
                    1,
                    0,
                    0,
                    0,
                )
                .unwrap()
                .timestamp();
        }
        let price = order.price.expect("Limit orders must have a price");
        self.update_values(price, order.size);
    }

    fn update_values(&mut self, price: i64, size: i64) {
        self.price = price;
        self.volume_24_h += size;

        if price < self.low_24_h {
            self.low_24_h = price;
        }
        if price > self.high_24_h {
            self.high_24_h = price;
        }
        if self.open_24_hr == 0 {
            self.open_24_hr = price;
        }

        if price < self.low_52_w {
            self.low_52_w = price;
        }
        if price > self.high_52_w {
            self.high_52_w = price;
        }

        if self.open_24_hr != 0 {
            self.price_percent_chg_24_h =
                ((price as f64 - self.open_24_hr as f64) / self.open_24_hr as f64) * 100.0;
        }
    }

    pub fn emit(&mut self, conn: &mut Connection, book: &OrderBook) {
        let (best_bid, best_bid_quantity) = book
            .get_best_bid()
            .map(|(p, q)| (*p, *q)) // deref the &i64s
            .unwrap_or((0, 0));

        let (best_ask, best_ask_quantity) =
            book.get_best_ask().map(|(p, q)| (*p, *q)).unwrap_or((0, 0));

        let ticker = Ticker {
            product_id: "JSP".to_string(), // TODO: fill in real product_id
            price: self.price,
            volume_24_h: self.volume_24_h,
            low_24_h: self.low_24_h,
            high_24_h: self.high_24_h,
            low_52_w: self.low_52_w,
            high_52_w: self.high_52_w,
            price_percent_chg_24_h: self.price_percent_chg_24_h,
            best_bid,
            best_bid_quantity,
            best_ask,
            best_ask_quantity,
        };

        let json_payload = serde_json::to_string(&ticker).expect("Failed to serialize ticker");
        let result: RedisResult<String> = conn.publish(CHANNEL_NAME, json_payload);

        if let Err(e) = result {
            eprintln!("Error emitting");
        }

        // self.batch_updates.push(payload);
        // if Utc::now > self.batch_start + 5 {
        //     // TODO: publish snapshotted batch of updates on ticker_batch channel
        //     self.batch_start += 5;
        //     self.batch_updates.clear();
        // }
    }
}
