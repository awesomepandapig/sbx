mod order;
mod order_book;
mod ticker;
// mod candle;
mod utils;

use order_book::OrderBook;
use ticker::TickerService;
// use candle::CandleService;
use utils::{acknowledge, read_from_stream};

use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

use chrono::{SecondsFormat, Utc};
use redis::{Client, Commands, Connection, RedisResult};
use serde::Serialize;

// pub fn startup() {
//     // Read the entire stream until the last acknowledged message to rebuild the book
//     // for order in orders
//         if(order.type == 'new') {
//             orderbook.add(&order)
//         }
//         if(order.type == 'match') {
//             // reduce order size by the match size
//             // if the size on book is 0
//                 // remove order

//     // Use XPENDING to read any unacknowledge messages
//     let missed_orders = ;

//     // For each of these we read do the normal emissions
//     // candles
//     // ticker
//     // level2
//     // updates
// }

#[derive(Serialize)]
struct L2Data {
    message_id: String,
    side: String,
    event_time: String,
    price_level: i64,
    new_quantity: i64,
}

fn level2(
    conn: &mut Connection,
    message_id: &str,
    product_id: &str,
    side: &str,
    price_level: i64,
    new_quantity: i64,
) {
    let channel_name = format!("marketdata:level2:{}", product_id);
    let l2_data = L2Data {
        message_id: message_id.to_string(),
        side: side.to_string(),
        event_time: Utc::now().to_rfc3339_opts(SecondsFormat::Nanos, true),
        price_level,
        new_quantity,
    };

    match serde_json::to_string(&l2_data) {
        Ok(json_payload) => {
            let result: RedisResult<i64> = conn.publish(&channel_name, json_payload);
            if let Err(e) = result {
                eprintln!(
                    "[{}] Failed to publish L2 update to Redis channel {}: {}",
                    product_id, channel_name, e
                );
            }
        }
        Err(e) => {
            eprintln!("[{}] Failed to serialize L2Data: {}", product_id, e);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Handle REDIS_TLS configuration
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL environment variable not set");
    let product_id: String =
        env::var("PRODUCT_ID").expect("PRODUCT_ID environment variable not set");

    println!("Connecting to Redis at: {}", redis_url);
    let client: Client = Client::open(redis_url)?;
    let mut conn: Connection = client.get_connection()?;

    // Initialize state for each product
    let mut book = OrderBook::new();
    let mut ticker_service = TickerService::new(product_id.clone());

    // let mut candle_service = CandleService::new(products.clone());

    // TODO: startup (rebuild the book)

    let batch_interval = Duration::from_secs(5);
    let mut last_batch_emit = Instant::now();

    loop {
        let orders = read_from_stream(&mut conn, product_id.clone());

        if orders.is_empty() {
            if last_batch_emit.elapsed() >= batch_interval {
                ticker_service.emit_batch(&mut conn, &book);
                last_batch_emit = Instant::now();
            }
            continue;
        }

        let mut ack_ids = Vec::new();

        for (message_id, order) in orders {
            let price = order.price.unwrap_or(0);

            match order.action.as_str() {
                "create" => {
                    if order.r#type == "limit" {
                        let quantity_change = book.add_order(&order);
                        level2(
                            &mut conn,
                            &message_id,
                            &product_id,
                            &order.side,
                            price,
                            quantity_change,
                        );
                    }
                }
                "match" => {
                    // A match reduces liquidity on the book side specified in the order
                    let quantity_change = book.remove_order(&order);
                    level2(
                        &mut conn,
                        &message_id,
                        &product_id,
                        &order.side,
                        price,
                        quantity_change,
                    );

                    // Process one order per matched order pair (arbitrary side selection)
                    if order.side == "buy" {
                        ticker_service.process_match(&order);
                        // candle_service.process_match(&order);
                    }
                    ticker_service.emit_individual(&mut conn, &book);
                }
                "cancel" => {
                    // Cancellation removes liquidity
                    let quantity_change = book.remove_order(&order);
                    level2(
                        &mut conn,
                        &message_id,
                        &product_id,
                        &order.side,
                        price,
                        quantity_change,
                    );
                }
                "cancel_reject" => {}
                _ => {
                    eprintln!(
                        "[{}] Unknown order action: '{}'. Skipping.",
                        product_id, order.action
                    );
                }
            }

            // Emit candle every second
            // candle_service.emit_individual(&mut conn, &product_id);

            ack_ids.push(message_id);
        }

        let stream_name = format!("instrument:events:{}", &product_id);
        if !ack_ids.is_empty() {
            acknowledge(&mut conn, &stream_name, ack_ids);
        }

        if last_batch_emit.elapsed() >= batch_interval {
            ticker_service.emit_batch(&mut conn, &book);
            last_batch_emit = Instant::now();
        }
    }
}
