// use super::order::Order;
// use serde::Serialize;
// use serde_json;

// // use chrono::Utc;
// use redis::{Commands, Connection, RedisResult};

// const CANDLE_INTERVAL_SECONDS: i64 = 300;

// #[derive(Serialize)]
// struct Candle {
//     start: i64,
//     open: i64,
//     high: i64,
//     low: i64,
//     close: i64,
//     volume: i64,
//     product_id: String
// }

// pub struct CandleBuilder {
//     product_id: String,
//     channel: String,
//     curr_candle: Option<Candle>
// }

// impl CandleBuilder {
//     pub fn new(product_id: String) -> Self {
//         let channel = format!("{}:{}:{}", "marketdata", "candles", product_id);

//         // TODO: Load cached_start from Redis
//         // let now = Utc::now().timestamp();
//         // let cached_start = now;

//         return Self {
//             product_id,
//             channel,
//             curr_candle: None,
//         }
//     }

//     fn recover(&mut self, missed_orders: Vec<Order>) {
//         for order in missed_orders {
//             self.process_order(&order);
//         }
//     }

//     pub fn process_order(&mut self, order: &Order) {
//         let price = order.price.expect("Limit orders must have a price");
//         let volume = order.size;
//         let timestamp = order.created_at;
//         let interval_start_ts = timestamp - (timestamp % CANDLE_INTERVAL_SECONDS);

//         match &mut self.curr_candle {
//             None => {
//                 self.curr_candle = Some(Candle {
//                     start: interval_start_ts,
//                     open: price,
//                     high: price,
//                     low: price,
//                     close: price, // First trade sets all OHLC
//                     volume: volume,
//                     product_id: self.product_id.clone(),
//                 });
//             }
//             Some(candle) => {
//                 if interval_start_ts > candle.start {
//                      // Start a new candle based on this trade.
//                     *candle = Candle {
//                           start: interval_start_ts,
//                           open: price,
//                           high: price,
//                           low: price,
//                           close: price,
//                           volume: volume,
//                           product_id: self.product_id.clone(),
//                     };
//                 }
//                 else if interval_start_ts == candle.start {
//                     candle.high = candle.high.max(price);
//                     candle.low = candle.low.min(price);
//                     candle.close = price; // Close always tracks the latest price
//                     candle.volume += volume;
//                 }
//             }
//         }
//     }

//     pub fn emit(&mut self, conn: &mut Connection) {
//         if let Some(candle) = &self.curr_candle {
//             let json_payload = serde_json::to_string(&candle).expect("Failed to serialize candle");
//             let result: RedisResult<String> = conn.publish(self.channel.clone(), json_payload);
//             if let Err(e) = result {
//                 eprintln!("Error emitting");
//             }
//         }
//     }
// }
