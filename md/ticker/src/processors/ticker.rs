use std::string::ToString;

use crate::messages::{ExecutionReportMessage, SymbolType};
use crate::orderbook::OrderBook;

use chrono::{Datelike, Duration, TimeZone, Utc, DateTime, SecondsFormat};
use serde::Serialize;

use tracing::info;

#[derive(Serialize, Debug, Clone)]
pub struct Ticker {
    product_id: String,
    price: String,
    volume_24_h: String,
    low_24_h: String,
    high_24_h: String,
    open_24_h: String,
    low_52_w: String,
    high_52_w: String,
    price_percent_chg_24_h: String,
    best_bid: String,
    best_bid_quantity: String,
    best_ask: String,
    best_ask_quantity: String,
    timestamp: String,
}

#[derive(Debug, Clone)]
pub struct TickerState {
    product_id: String,
    start_24_hr_ts: u64,
    start_52_w_ts: u64,
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
            .timestamp_nanos_opt()
            .unwrap() as u64;

        let start_52_w_ts = Utc
            .with_ymd_and_hms(now.year(), 1, 1, 0, 0, 0)
            .unwrap()
            .timestamp_nanos_opt() 
            .unwrap() as u64;

        Self {
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
        }
    }

    fn check_and_reset_windows(&mut self, current_ts: u64) {
        let mut next_day_start_ts = self.start_24_hr_ts + Duration::days(1).num_nanoseconds().unwrap() as u64;

        while current_ts >= next_day_start_ts {
            self.start_24_hr_ts = next_day_start_ts;
            self.volume_24_h = 0;
            self.low_24_h = i64::MAX;
            self.high_24_h = i64::MIN;
            self.open_24_hr = self.last_price;
            next_day_start_ts += Duration::days(1).num_nanoseconds().unwrap() as u64;
        }


        // let mut year = Utc.timestamp_opt(self.start_52_w_ts, 0).unwrap().year();
        // let mut next_year_start_ts = Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap().timestamp();

        // while current_ts >= next_year_start_ts {
        //     info!("loopin");
        //     self.start_52_w_ts = next_year_start_ts;
        //     self.low_52_w = i64::MAX;
        //     self.high_52_w = i64::MIN;
        //     year += 1;
        //     next_year_start_ts = Utc.with_ymd_and_hms(year + 1, 1, 1, 0, 0, 0).unwrap().timestamp();
        // }
    }

    pub fn update_on_match(&mut self, report: &ExecutionReportMessage) {
        self.check_and_reset_windows(report.transact_time);

        self.last_price = report.price;
        self.volume_24_h += report.last_qty;

        self.low_24_h = self.low_24_h.min(report.price);
        self.high_24_h = self.high_24_h.max(report.price);
        if self.open_24_hr == 0 {
            self.open_24_hr = report.price;
        }

        self.low_52_w = self.low_52_w.min(report.price);
        self.high_52_w = self.high_52_w.max(report.price);
    }

    fn calculate_price_percent_change(&self) -> f64 {
        if self.open_24_hr != 0 && self.last_price != 0 {
            return ((self.last_price as f64 - self.open_24_hr as f64) / self.open_24_hr as f64)
                * 100.0;
        }
        0.0
    }

    pub fn create_ticker(&self, book: &OrderBook, timestamp: u64) -> Ticker {
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

        Ticker {
            product_id: self.product_id.clone(),
            price: format_decimal_with_exponent_neg8(&self.last_price),
            volume_24_h: format_decimal_with_exponent_neg8(&self.volume_24_h),
            low_24_h: format_decimal_with_exponent_neg8(&low_24h),
            high_24_h: format_decimal_with_exponent_neg8(&high_24h),
            open_24_h: format_decimal_with_exponent_neg8(&self.open_24_hr),
            low_52_w: format_decimal_with_exponent_neg8(&low_52w),
            high_52_w: format_decimal_with_exponent_neg8(&high_52w),
            price_percent_chg_24_h: self.calculate_price_percent_change().to_string(),
            best_bid: format_decimal_with_exponent_neg8(&best_bid),
            best_bid_quantity: format_decimal_with_exponent_neg8(&best_bid_quantity),
            best_ask: format_decimal_with_exponent_neg8(&best_ask),
            best_ask_quantity: format_decimal_with_exponent_neg8(&best_ask_quantity),
            timestamp: format_nanosecond_timestamp(&timestamp),
        }
    }
}

fn format_decimal_with_exponent_neg8(mantissa: &i64) -> String {
    const SCALE: i64 = 100_000_000;

    let integral = mantissa / SCALE;
    let fractional_part = (mantissa % SCALE).abs();

    if fractional_part == 0 {
        return integral.to_string();
    }

    let mut fractional_str = format!("{:08}", fractional_part);
    while fractional_str.ends_with('0') {
        fractional_str.pop();
    }

    if fractional_str.is_empty() {
        integral.to_string()
    } else {
        format!("{}.{}", integral, fractional_str)
    }
}

fn format_nanosecond_timestamp(timestamp: &u64) -> String {
    let secs = timestamp / 1_000_000_000;
    let nanos = (timestamp % 1_000_000_000) as u32;
    let datetime: DateTime<Utc> = Utc.timestamp_opt(secs.try_into().unwrap(), nanos).unwrap(); // TODO: DONT UNWRAP IN PROD
    datetime.to_rfc3339_opts(SecondsFormat::Nanos, true)
}

fn format_symbol(symbol_bytes: &[u8; 6]) -> String {
    String::from_utf8_lossy(symbol_bytes)
        .trim_end_matches(['\0', ' ']) // Trim both null characters and spaces.
        .to_string()
}