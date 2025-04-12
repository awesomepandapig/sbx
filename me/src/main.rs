mod models;
use models::order::Order;
use priority_queue::PriorityQueue;
use redis::streams::{StreamReadOptions, StreamReadReply};
use redis::{Commands, Connection, RedisResult};
use std::cmp::Reverse;
use std::thread;

fn read_orders_from_stream(
    con: &mut Connection,
    product_id: &str,
    orders: &mut Vec<Order>,
    message_ids: &mut Vec<String>,
) {
    let stream_name: String = format!("product:{}:new", product_id);
    let group_name = "matchers";
    let consumer_name = format!("matcher:{}", product_id);

    let results: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(&group_name)
        .arg(&consumer_name)
        .arg("BLOCK")
        .arg(50)
        .arg("COUNT")
        .arg(1000)
        .arg("STREAMS")
        .arg(&stream_name)
        .arg(">")
        .query(con);

    // If Redis read fails, log the error and return
    if results.is_err() {
        eprintln!("Error reading from Redis: {:?}", results.err().unwrap());
        return;
    }

    let reply = match results {
        Ok(reply) => reply,
        Err(err) => {
            eprintln!("Redis stream read failed: {}", err);
            return;
        }
    };

    for stream in reply.keys.iter() {
        for entry in stream.ids.iter() {
            // Collect Redis stream message ID
            message_ids.push(entry.id.clone());

            // Cast each stream entry to an Order
            match Order::from_redis_map(&entry.map) {
                Ok(order) => orders.push(order),
                Err(_) => {
                    eprintln!("Error parsing order");
                    // Don't acknowledge
                    message_ids.pop();
                }
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
    let mut bid_pq: PriorityQueue<Order, i64> = PriorityQueue::new();
    let mut ask_pq: PriorityQueue<Order, Reverse<i64>> = PriorityQueue::new();

    // Create redis connection
    let client: redis::Client =
        redis::Client::open("redis://localhost/0").unwrap();
    let mut con: Connection = client.get_connection().unwrap();

    loop {
        let mut orders: Vec<Order> = Vec::new();
        let mut message_ids: Vec<String> = Vec::new();

        read_orders_from_stream(
            &mut con,
            &product_id,
            &mut orders,
            &mut message_ids,
        );

        for (order, message_id) in orders.into_iter().zip(message_ids.iter()) {
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
                println!("{:?}", matched_order);
                let redis_tuples: Vec<(&str, String)> =
                    matched_order.to_redis_tuples();
                let stream_name: String = format!("product:{}:match", product_id);
                let _: RedisResult<()> =
                    con.xadd(&[stream_name], "*", &redis_tuples);
                // Delete the message
                let stream_name: String = format!("product:{}:new", product_id);
                let _: RedisResult<()> = con.xdel(&[stream_name], &[message_id]);
            }

            // Acknowledge the message
            let stream_name: String = format!("product:{}:new", product_id);
            let group_name = "matchers";
            let stream_name_str = stream_name.as_str();
            let _: RedisResult<()> = con.xack(&[stream_name_str], &[group_name], &[message_id]);
        }
    }
}

fn main() {
    // Create redis connection
    let client: redis::Client =
        redis::Client::open("redis://localhost/0").unwrap();
    let mut con: Connection = client.get_connection().unwrap();

    let products: Vec<String> = con.smembers("product").unwrap();
    let mut worker_threads: Vec<thread::JoinHandle<()>> = vec![];

    for product in products {
        let product_id: String = product.clone();
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            matching_engine(&product_id);
        });
        worker_threads.push(handle);
    }

    for worker in worker_threads {
        let _ = worker.join();
    }
}
// TODO: XPENDING CHECK AT BEGGINING OF MATCHING ENGINE