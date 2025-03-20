mod models;
use models::order::Order;
use priority_queue::PriorityQueue;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{Commands, Connection, RedisResult};
use std::cmp::Reverse;
use std::thread;

// Configuration variables
const BATCH_SIZE: usize = 100;

fn read_orders_from_stream(
    con: &mut Connection,
    stream_id: &mut String,
    product_id: &str,
    orders: &mut Vec<Order>,
) {
    let opts = StreamReadOptions::default().count(BATCH_SIZE);
    let stream_name = format!("{}:new", product_id);
    let results: RedisResult<StreamReadReply> =
        con.xread_options(&[stream_name], &[&stream_id], &opts);

    // If Redis read fails, log the error and return
    if results.is_err() {
        eprintln!("Error reading from Redis: {:?}", results.err().unwrap());
        return;
    }

    // TODO: make more efficient by removing
    let reply = results.unwrap();

    for stream in reply.keys.iter() {
        for entry in stream.ids.iter() {
            // Update the stream_id to the latest seen ID
            *stream_id = entry.id.clone();

            // Cast each stream entry to an Order
            if let Ok(order) = Order::from_redis_map(&entry.map) {
                println!("{:?}", order);
                orders.push(order);
            } else {
                eprintln!("Error parsing order");
            }
        }
    }
}

fn cancel_immediate(order: &Order) {
    // lookup order in buy/sell data structure based on order side
    // if the order EXISTS in the data structure
    // delete the order from the DS
    // create new message for success response
    // else
    // send CANCEL REJECT
}

fn cancel_after(order: &Order) {
    // TODO: Process cancel_after field
    // if order has cancel_after field set, send a message to the cancel fairy...
}

fn match_orders(
    bid_pq: &mut PriorityQueue<Order, i64>,
    ask_pq: &mut PriorityQueue<Order, Reverse<i64>>,
) -> Vec<Order> {
    let mut matches = Vec::<Order>::new();

    // TODO: Implement matching algorithm

    // Self-Trade Prevention Two orders from the same user are not allowed to match with one another.

    while let Some((order, price)) = bid_pq.pop() {
        matches.push(order);
    }

    // for each match update status, executed_value, and settled boolean (?)

    return matches;
}

fn matching_engine(product_id: &str) {
    let mut stream_id = String::from("0-0");
    let mut bid_pq: PriorityQueue<Order, i64> = PriorityQueue::new();
    let mut ask_pq: PriorityQueue<Order, Reverse<i64>> = PriorityQueue::new();

    // Create redis connection
    let client = redis::Client::open("redis://127.0.0.1/0").unwrap();
    let mut con = client.get_connection().unwrap();

    let mut i = 0; // TODO: REMOVE
    while i < 2 {
        // Read new orders from stream
        let mut orders = Vec::new();
        read_orders_from_stream(
            &mut con,
            &mut stream_id,
            &product_id,
            &mut orders,
        );
        for order in orders {
            // Enqueue limit orders into the order book queues
            if order.r#type == "limit" {
                if order.side == "buy" {
                    bid_pq.push(order.clone(), order.price.unwrap());
                } else {
                    ask_pq.push(order.clone(), Reverse(order.price.unwrap()));
                }
            }
            // Handle IOC (market orders are not placed in book)

            // if order type is cancel
            // cancel_immediate();

            // if order cancel_after is set:
            // cancel_after(&order);

            // Run matching algorithm
            let matches = match_orders(&mut bid_pq, &mut ask_pq);

            // Send each match to redis output stream
            for matched_order in matches {
                // Add to Redis stream
                let redis_tuples = matched_order.to_redis_tuples();
                let stream_name = format!("{}:matches", product_id);
                let _result: RedisResult<()> =
                    con.xadd(&[stream_name], "*", &redis_tuples);
            }
        }

        i = i + 1; // TODO: REMOVE
    }
}

fn main() {
    // TODO: for each product in product_ids spawn a new thread
    // Spawn worker threads with dedicated streams
    let worker1 = { thread::spawn(move || matching_engine("GOLDEN_DRAGON")) };
    let worker2 = { thread::spawn(move || matching_engine("VANGUARD")) };

    worker1.join().unwrap();
    worker2.join().unwrap();
}