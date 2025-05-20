use super::models::order::Order;
use priority_queue::PriorityQueue;
use redis::streams::StreamReadReply;
use redis::{Client, Commands, Connection, RedisResult, from_redis_value};
use slab::Slab;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

const STREAM_KEY_PREFIX: &str = "instrument";
const ORDER_STREAM_SUFFIX: &str = "orders";
const EVENT_STREAM_SUFFIX: &str = "events";

const CONSUMER_GROUP_NAME: &str = "matching-engine-service";
const CONSUMER_NAME: &str = "alice"; // TODO: REPLACE WITH POD_NAME

const ORDER_SIDE_BUY: &str = "buy";
const ORDER_SIDE_SELL: &str = "sell";

const ORDER_ACTION_CREATE: &str = "create";
const ORDER_ACTION_CANCEL: &str = "cancel";

const ORDER_TYPE_LIMIT: &str = "limit";
// const ORDER_TYPE_MARKET: &str = "market";

const ORDER_STATUS_DONE: &str = "done";
// const ORDER_STATUS_CANCELLED: &str = "cancelled";

const REDIS_BLOCK_TIMEOUT_MS: usize = 5000;
const REDIS_READ_COUNT: usize = 1000;

#[derive(Eq, PartialEq)]
struct BidPriority {
    price: i64,
    created_at: i64,
    sequence_num: u64,
}

#[derive(Eq, PartialEq)]
struct AskPriority {
    price: i64,
    created_at: i64,
    sequence_num: u64,
}

impl Ord for BidPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher price is better
        self.price
            .cmp(&other.price)
            .then_with(|| other.created_at.cmp(&self.created_at))
            .then_with(|| other.sequence_num.cmp(&self.sequence_num))
    }
}

impl PartialOrd for BidPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AskPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lower price is better
        other
            .price
            .cmp(&self.price)
            .then_with(|| other.created_at.cmp(&self.created_at))
            .then_with(|| other.sequence_num.cmp(&self.sequence_num))
    }
}

impl PartialOrd for AskPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type BidQueue = PriorityQueue<usize, BidPriority>;
type AskQueue = PriorityQueue<usize, AskPriority>;

type OrderId = String;
type OrderPool = Slab<Order>;
type OrderMap = HashMap<OrderId, usize>;

pub struct MatchingEngine {
    redis_connection: Connection,
    bid_pq: BidQueue,
    ask_pq: AskQueue,
    order_pool: OrderPool,
    order_map: OrderMap,
    order_stream: String,
    event_stream: String,
    sequence_num: u64,
}

impl MatchingEngine {
    pub fn new(
        product_id: String,
        redis_url: String,
    ) -> Result<Self, Box<dyn Error>> {
        let client: Client = Client::open(redis_url)?;
        let redis_connection: Connection = client.get_connection()?;

        let order_stream = format!(
            "{}:{}:{}",
            STREAM_KEY_PREFIX, ORDER_STREAM_SUFFIX, product_id
        );
        let event_stream = format!(
            "{}:{}:{}",
            STREAM_KEY_PREFIX, EVENT_STREAM_SUFFIX, product_id
        );

        Ok(Self {
            redis_connection,
            bid_pq: PriorityQueue::new(),
            ask_pq: PriorityQueue::new(),
            order_pool: OrderPool::with_capacity(10_000),
            order_map: OrderMap::with_capacity(10_000),
            order_stream,
            event_stream,
            sequence_num: 0,
        })
    }

    #[inline]
    fn get_next_sequence_num(&mut self) -> u64 {
        let sequence_num = self.sequence_num;
        self.sequence_num = self.sequence_num.wrapping_add(1);
        sequence_num
    }

    /// Reads new orders from the Redis stream using XREADGROUP.
    fn read_orders_from_stream(&mut self) -> (Vec<usize>, Vec<String>) {
        let results: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(CONSUMER_GROUP_NAME)
            .arg(CONSUMER_NAME)
            .arg("BLOCK")
            .arg(REDIS_BLOCK_TIMEOUT_MS)
            .arg("COUNT")
            .arg(REDIS_READ_COUNT)
            .arg("STREAMS")
            .arg(&self.order_stream)
            .arg(">") // Read only new messages
            .query(&mut self.redis_connection);

        let mut order_entries: Vec<usize> = Vec::new();
        let mut message_ids: Vec<String> = Vec::new();

        let reply = match results {
            Ok(reply) => reply,
            Err(err) => {
                eprintln!("Redis stream read failed: {}", err);
                return (order_entries, message_ids);
            }
        };

        for stream_key in reply.keys {
            for stream_id in stream_key.ids {
                let message_id = stream_id.id.clone();
                let map: &HashMap<String, redis::Value> = &stream_id.map;

                message_ids.push(message_id.clone());

                let action_val: &redis::Value =
                    map.get("action").expect("action missing");
                let action: String = from_redis_value::<String>(action_val)
                    .expect("action value not convertible to String");

                match action.as_str() {
                    ORDER_ACTION_CANCEL => {
                        let order_id_val: &redis::Value =
                            map.get("id").expect("order_id missing for cancel");
                        let order_id: String = from_redis_value::<String>(
                            order_id_val,
                        )
                        .expect("order_id value not convertible to String");

                        self.cancel_immediate(&order_id);
                    }
                    ORDER_ACTION_CREATE => {
                        match Order::from_redis_map(&stream_id.map) {
                            Ok(order) => {
                                let order_index = self.order_pool.insert(order);
                                order_entries.push(order_index);
                            }
                            Err(e) => {
                                eprintln!(
                                    "ERR: CREATE parse failed {}: {}",
                                    message_id, e
                                );
                            }
                        }
                    }
                    unknown_action => {
                        eprintln!(
                            "ERR: Unknown action '{}' {}",
                            unknown_action, message_id
                        );
                    }
                }
            }
        }
        (order_entries, message_ids)
    }

    /// Adds a limit order to the appropriate priority queue and the order map.
    fn add_limit_order(&mut self, order_index: usize) {
        let order = &self.order_pool[order_index];
        let order_id = order.id.clone();

        let price: i64 = order.price.expect("Limit orders must have a price");
        let created_at = order.created_at;

        if order.side == ORDER_SIDE_BUY {
            let priority: BidPriority = BidPriority {
                price,
                created_at,
                sequence_num: self.get_next_sequence_num(),
            };
            self.bid_pq.push(order_index, priority);
        } else {
            let priority: AskPriority = AskPriority {
                price,
                created_at,
                sequence_num: self.get_next_sequence_num(),
            };
            self.ask_pq.push(order_index, priority);
        }

        self.order_map.insert(order_id, order_index);
    }

    /// Attempts to match orders at the top of the book.
    fn match_orders(&mut self, output: &mut Vec<Order>) {
        while let (Some((&bid_index, _)), Some((&ask_index, _))) =
            (self.bid_pq.peek(), self.ask_pq.peek())
        {
            // Get the orders from the pool
            let bid_order: &Order = self
                .order_pool
                .get(bid_index)
                .expect("Order should exist in pool");
            let ask_order: &Order = self
                .order_pool
                .get(ask_index)
                .expect("Order should exist in pool");

            if bid_order.price < ask_order.price {
                break;
            }

            // --- Match Found ---

            // TODO: Prevent self-trade
            // if bid_order.user_id == ask_order.user_id {
            //     // What to do? Cancel one? Which one? The aggressor? Or the resting?
            //     // For now, we just break the matching loop iteration.
            //     eprintln!("Self-trade detected between {} and {}", best_bid_id, best_ask_id);
            //     // Need a strategy here - potentially remove one order based on rules
            //     // Skipping match for now. How to proceed? Let's just break for simplicity, though incorrect.
            //     break; // Or continue? Needs clear rules.
            // }

            // Pop orders from queues *before* removing from map
            let (bid_index, _) =
                self.bid_pq.pop().expect("Known to exist from peek");
            let (ask_index, _) =
                self.ask_pq.pop().expect("Known to exist from peek");

            // Take ownership of the orders from the pool
            let mut bid = self.order_pool.remove(bid_index);
            let mut ask = self.order_pool.remove(ask_index);

            // Remove indices from map
            self.order_map.remove(&bid.id);
            self.order_map.remove(&ask.id);

            // Update order execution details
            let execution_price = bid.price.unwrap();
            bid.status = ORDER_STATUS_DONE.to_string();
            ask.status = ORDER_STATUS_DONE.to_string();
            bid.executed_value = execution_price;
            ask.executed_value = execution_price;
            bid.settled = true;
            ask.settled = true;

            // TODO: Add cross-reference to matched orders
            // bid.matched_order_id = Some(ask.id.clone());
            // ask.matched_order_id = Some(bid.id.clone());

            println!(
                "Match: Bid {} Ask {} @ Price {}",
                bid.id, ask.id, execution_price
            );

            // Store *copies* of the matched orders for emitting
            output.push(bid);
            output.push(ask);
        }
    }

    /// Emits matched orders to the match stream.
    fn emit_matches(&mut self, matched_orders: &Vec<Order>) {
        let mut emit_buf: Vec<(&str, String)> = Vec::with_capacity(10);

        let mut pipe = redis::pipe();
        for order in matched_orders {
            emit_buf.clear();
            emit_buf.extend(order.to_redis_tuples());
            emit_buf.push(("action", "match".to_string()));

            pipe.cmd("XADD")
                .arg(&self.event_stream)
                .arg("*")
                .arg(&emit_buf);
        }

        if let Err(e) = pipe.query::<()>(&mut self.redis_connection) {
            eprintln!("Error emitting match batch: {}", e);
        }
    }

    // Batch acknowledge messages
    fn acknowledge_messages(&mut self, message_ids: &[String]) {
        let result: RedisResult<i32> = self.redis_connection.xack(
            &self.order_stream,
            CONSUMER_GROUP_NAME,
            message_ids,
        );
        if let Err(e) = result {
            eprintln!("Error acknowledging messages: {}", e);
            // TODO: Error handling - retry? Log?
        }
    }

    /// Processes a single incoming order.
    fn process_order(&mut self, order_index: usize) {
        let order = &self.order_pool[order_index];
        match order.r#type.as_str() {
            ORDER_TYPE_LIMIT => {
                // Emit Audit Log "ADDED" event
                let mut redis_tuples: Vec<(&str, String)> =
                    order.to_redis_tuples();
                redis_tuples.push(("action", "create".to_string()));

                let _result: RedisResult<String> = self.redis_connection.xadd(
                    &self.event_stream, // Target stream name
                    "*",                // Auto-generate message ID
                    &redis_tuples,      // Key-value pairs
                );
                self.add_limit_order(order_index);
            }
            // ORDER_TYPE_MARKET => {
            // TODO: Implement IOC / Market order logic (immediate matching attempt without booking)
            // }
            _ => {} // TODO: if order.cancel_after is set:
                    // self.schedule_cancellation(&order);
        }
    }

    fn emit_cancel_reject(&mut self, order_id: &str) {
        let redis_tuples = vec![
            ("action", "cancel_reject".to_string()),
            ("id", order_id.to_string()),
        ];
        let _result: RedisResult<String> =
            self.redis_connection
                .xadd(&self.event_stream, "*", &redis_tuples);
    }

    fn cancel_immediate(&mut self, order_id: &str) {
        let order_index = match self.order_map.remove(order_id) {
            Some(index) => index,
            None => {
                self.emit_cancel_reject(order_id);
                return;
            }
        };

        let order: &Order = match self.order_pool.get(order_index) {
            Some(order) => order,
            None => {
                self.emit_cancel_reject(order_id);
                return;
            }
        };

        let removed_from_pq = match order.side.as_str() {
            ORDER_SIDE_BUY => self.bid_pq.remove(&order_index).is_some(),
            ORDER_SIDE_SELL => self.ask_pq.remove(&order_index).is_some(),
            _ => false,
        };

        if !removed_from_pq {
            self.emit_cancel_reject(order_id);
            return;
        }

        let mut redis_tuples: Vec<(&str, String)> = order.to_redis_tuples();
        redis_tuples.push(("action", "cancel".to_string()));

        let _result: RedisResult<String> =
            self.redis_connection
                .xadd(&self.event_stream, "*", &redis_tuples);
    }

    #[allow(dead_code)] // Keep function signature
    fn schedule_cancellation(&mut self, order: &Order) {
        // TODO: Process cancel_after field
        // This would likely involve sending the order ID and cancel time
        // to a separate system/timer mechanism, or storing it locally
        // with a scheduled check.
        // let cancel_timestamp = 0;
        // if order.cancel_after == 'min'
        // cancel_timestamp = order.created_at + one minute
        // else if order.cancel_after == 'hour'
        // cancel_timestamp = order.created_at + one hour

        // send the order to the cancel fairy
        println!(
            "TODO: Schedule cancellation for order {} (cancel_after)",
            order.id
        );
    }

    // --- Main Loop ---
    pub fn run(&mut self) {
        // TODO: XPENDING CHECK AT BEGINNING OF MATCHING ENGINE
        // self.process_pending_messages()?; // Add a method to handle messages not acked from previous runs

        let mut matches: Vec<Order> = Vec::with_capacity(REDIS_READ_COUNT * 2);

        loop {
            let (created_order_indices, handled_message_ids): (
                Vec<usize>,
                Vec<String>,
            ) = self.read_orders_from_stream();

            if handled_message_ids.is_empty() {
                continue;
            }

            for order_index in created_order_indices {
                self.process_order(order_index);
            }

            matches.clear();
            self.match_orders(&mut matches);
            if !matches.is_empty() {
                self.emit_matches(&matches);
            }

            self.acknowledge_messages(&handled_message_ids);
        }
    }
}
