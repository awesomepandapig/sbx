mod order;
mod order_book;
mod ticker;
mod utils;

use order_book::OrderBook;
use utils::{acknowledge, read_from_stream};
use ticker::TickerBuilder;
use std::env;
use std::error::Error;
use chrono::Utc;
use redis::{Client, Commands, Connection, RedisResult};

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

use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct L2Data {
    side: String,
    event_time: i64,
    price_level: i64,
    new_quantity: i64
}

// TODO: REPLACE WITH PRODUCTID
const CHANNEL_NAME: &'static str = "marketdata:level2:JSP";

fn level2(conn: &mut Connection, side: &str, price_level: i64, new_quantity: i64) {
    let l2_data = L2Data {
        side: side.to_string(),
        event_time: Utc::now().timestamp(),
        price_level,
        new_quantity,
    };
    let json_payload = serde_json::to_string(&l2_data).expect("Failed to serialize l2_data");
    let result: RedisResult<String> = conn.publish(CHANNEL_NAME, json_payload);
    // // TODO: emit payload on level2 channel for product
    // println!("{:?}", payload);
    // println!(
    //     "side: {}, event_time: {}, price_level: {}, new_quantity: {}",
    //     side,
    //     Utc::now(),
    //     price_level,
    //     new_quantity
    // );
}

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Handle REDIS_TLS configuration
    let product_id: String = env::var("PRODUCT_ID")?;
    let redis_url: String = env::var("REDIS_URL")?;

    println!("Initializing matching engine for product: {}", product_id);
    println!("Connecting to Redis at: {}", redis_url);

    let client: Client = Client::open(redis_url)?;
    let mut conn: Connection = client.get_connection()?;

    let mut book: OrderBook = OrderBook::new();
    // startup (rebuild the book)

    let mut ticker: TickerBuilder = TickerBuilder::new();

    loop {
        let orders = read_from_stream(&mut conn);

        for (message_id, order) in orders {
            let mut price = 0;
            if order.r#type == "limit" {
                price = order.price.expect("Limit orders must have a price")
            }

            if order.action == "add" {
                // Modify the book
                let new_quantity = book.add_order(&order);
                level2(&mut conn, &order.side, price, new_quantity);
            }
            if order.action == "match" {
                // Modify the book
                let new_quantity = book.remove_order(&order);
                level2(&mut conn, &order.side, price, new_quantity);

                // Emit once per matched order pair (arbitrary side selection)
                if order.side == "buy" {
                    ticker.process_order(&order);
                    ticker.emit(&mut conn, &book);
                }
            }
            if order.action == "cancel" {
                // Modify the book
                let new_quantity = book.remove_order(&order);
                level2(&mut conn, &order.side, price, new_quantity);
            }
            // candles

            // Acknowledge the message as read
            acknowledge(&mut conn, &message_id);
        }
    }
}
