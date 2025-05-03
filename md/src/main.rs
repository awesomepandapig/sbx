mod order;
mod order_book;
mod ticker;
// mod candle;
mod utils;

use order_book::OrderBook;
use ticker::TickerService;
// use candle::CandleService;
use utils::{acknowledge, read_from_streams};

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::{Duration, Instant};

use redis::{Client, Commands, Connection, RedisResult};
use serde::Serialize;
use serde_json;

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
    side: String,
    event_time: i64,
    price_level: i64,
    new_quantity: i64,
}

fn level2(
    conn: &mut Connection,
    product_id: &str,
    side: &str,
    created_at: i64,
    price_level: i64,
    new_quantity: i64,
) {
    let channel_name = format!("marketdata:level2:{}", product_id);
    let l2_data = L2Data {
        side: side.to_string(),
        event_time: created_at,
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
    let redis_url: String = env::var("REDIS_URL")?;
    println!("Connecting to Redis at: {}", redis_url);

    let client: Client = Client::open(redis_url)?;
    let mut conn: Connection = client.get_connection()?;

    // TODO: Fetch this dynamically from Redis/DB/Config file
    let products = vec!["JSP".to_string()];

    let mut order_books: HashMap<String, OrderBook> = HashMap::new();
    let mut ticker_service = TickerService::new(products.clone());
    // let mut candle_service = CandleService::new(products.clone());

    // Initialize state for each product
    for product_id in &products {
        order_books.insert(product_id.clone(), OrderBook::new());
    }
    println!("Initialized state for products: {:?}", products);

    // TODO: startup (rebuild the book)

    let batch_interval = Duration::from_secs(5);
    let mut last_batch_emit = Instant::now();

    loop {
        let orders = read_from_streams(&mut conn, &products);

        if orders.is_empty() {
            if last_batch_emit.elapsed() >= batch_interval {
                ticker_service.emit_batch(&mut conn, &order_books);
                last_batch_emit = Instant::now();
            }
            continue;
        }

        for (message_id, order) in orders {
            let product_id = order.product_id.clone();
            let stream_name = format!("instrument:events:{}", product_id);

            // Get mutable access to the order book for this product
            let book = match order_books.get_mut(&product_id) {
                Some(b) => b,
                None => {
                    eprintln!(
                        "[{}] Received event for unknown or inactive product. Skipping.",
                        product_id
                    );
                    acknowledge(&mut conn, &stream_name, &message_id); // Acknowledge to prevent reprocessing
                    continue; // Skip this order
                }
            };

            let price = order.price.unwrap_or(0); // TODO: Default to 0 if no price? Or handle error?
            let mut quantity_change = 0;

            match order.action.as_str() {
                "create" => {
                    if order.r#type == "limit" {
                        quantity_change = book.add_order(&order);
                        level2(
                            &mut conn,
                            &product_id,
                            &order.side,
                            order.created_at,
                            price,
                            quantity_change,
                        );
                    } else {
                        // TODO: Handle market order additions if necessary (e.g., logging)
                    }
                }
                "match" => {
                    // A match reduces liquidity on the book side specified in the order
                    quantity_change = book.remove_order(&order);
                    level2(
                        &mut conn,
                        &product_id,
                        &order.side,
                        order.created_at,
                        price,
                        quantity_change,
                    );

                    // Process one order per matched order pair (arbitrary side selection)
                    if order.side == "buy" {
                        ticker_service.process_match(&order);
                        // candle_service.process_match(&order);
                    }
                    ticker_service.emit_individual(&mut conn, &product_id, book);
                }
                "cancel" => {
                    // Cancellation removes liquidity
                    quantity_change = book.remove_order(&order);
                    level2(
                        &mut conn,
                        &product_id,
                        &order.side,
                        order.created_at,
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

            acknowledge(&mut conn, &stream_name, &message_id);
        }

        if last_batch_emit.elapsed() >= batch_interval {
            ticker_service.emit_batch(&mut conn, &order_books);
            last_batch_emit = Instant::now();
        }
    }
}
