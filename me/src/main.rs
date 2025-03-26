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
    let opts: StreamReadOptions =
        StreamReadOptions::default().count(BATCH_SIZE);
    let stream_name: String = format!("{}:new", product_id);
    let results: RedisResult<StreamReadReply> =
        con.xread_options(&[stream_name], &[&stream_id], &opts);

    // If Redis read fails, log the error and return
    if results.is_err() {
        eprintln!("Error reading from Redis: {:?}", results.err().unwrap());
        return;
    }

    let reply: StreamReadReply = results.unwrap();
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

fn cancel_immediate(
    order: &Order,
    bid_pq: &mut PriorityQueue<Order, i64>,
    ask_pq: &mut PriorityQueue<Order, Reverse<i64>>,
) {
    // lookup order in buy/sell data structure based on order.side
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
    let mut matches: Vec<Order> = Vec::new();

    while let (Some((highest_bid, _)), Some((lowest_ask, _))) =
        (bid_pq.peek(), ask_pq.peek())
    {
        if highest_bid.price < lowest_ask.price {
            // No match possible
            break;
        }

        // TODO: Prevent self-trade
        // if bid_order.user_id == ask_order.user_id {
        //     //cancel_immediate(ask_order, bid_pq, ask_pq);
        //     continue;
        // }

        // Match found, pop orders from queues
        let mut bid: Order = bid_pq.pop().unwrap().0;
        let mut ask: Order = ask_pq.pop().unwrap().0;

        // Update order execution details
        bid.status = "done".to_string();
        ask.status = "done".to_string();
        bid.executed_value = bid.price.unwrap();
        ask.executed_value = bid.price.unwrap();
        bid.settled = true;
        ask.settled = true;

        // TODO: Add cross-reference to matched orders
        //bid.matched_order_id = Some(ask.id);
        //ask.matched_order_id = Some(bid.id);

        // Store matched orders
        matches.push(bid.clone());
        matches.push(ask.clone());
    }
    return matches;
}

fn matching_engine(product_id: &str) {
    let mut stream_id: String = String::from("0-0");
    let mut bid_pq: PriorityQueue<Order, i64> = PriorityQueue::new();
    let mut ask_pq: PriorityQueue<Order, Reverse<i64>> = PriorityQueue::new();

    // Create redis connection
    let client: redis::Client =
        redis::Client::open("redis://localhost/0").unwrap();
    let mut con: Connection = client.get_connection().unwrap();

    // let mut i = 0; // TODO: REMOVE
    // while i < 2 {
    loop {
        // Read new orders from stream
        let mut orders: Vec<Order> = Vec::new();
        read_orders_from_stream(
            &mut con,
            &mut stream_id,
            &product_id,
            &mut orders,
        );
        for order in orders {
            // TODO: we should spawn 3 seperate orders for a sized limit order
            // Enqueue limit orders into the order book queues
            if order.r#type == "limit" {
                if order.side == "buy" {
                    bid_pq.push(
                        order.clone(),
                        (order.price.unwrap() + order.created_at),
                    );
                } else {
                    ask_pq.push(
                        order.clone(),
                        Reverse(order.price.unwrap() + order.created_at),
                    );
                }
            }

            // Handle IOC (market orders are not placed in book)

            // if order type is cancel
            // cancel_immediate();

            // if order cancel_after is set:
            // cancel_after(&order);

            // Run matching algorithm
            let matches: Vec<Order> = match_orders(&mut bid_pq, &mut ask_pq);
            for matched_order in matches {
                // Add to Redis stream
                let redis_tuples: Vec<(&str, String)> =
                    matched_order.to_redis_tuples();
                let stream_name: String = format!("{}:matches", product_id);
                let _result: RedisResult<()> =
                    con.xadd(&[stream_name], "*", &redis_tuples);
            }
        }

        // i = i + 1; // TODO: REMOVE
    }
}

fn main() {
    // TODO: for each product in product_ids spawn a new thread
    // Spawn worker threads with dedicated streams
    let worker1: thread::JoinHandle<()> =
        { thread::spawn(move || matching_engine("DRG")) };
    let worker2: thread::JoinHandle<()> =
        { thread::spawn(move || matching_engine("FRY")) };
    let worker3: thread::JoinHandle<()> =
        { thread::spawn(move || matching_engine("JSP")) };

    worker1.join().unwrap();
    worker2.join().unwrap();
}
/*



Popping Orders from Redis Streams: Redis streams (XREADGROUP) don't inherently support "popping" in the way a queue does, but you can use XACK to acknowledge processing, and XDEL to delete once safely processed.

Crash Resilience: If the matching engine crashes mid-processing, you can use XPENDING to check unacknowledged messages and retry them safely.

Idempotency: Ensure orders have unique UUIDs and store processed orders in a separate Redis set (or an append-only log) to reject duplicates.
*/
