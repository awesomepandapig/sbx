use super::order::Order;

use redis::streams::StreamReadReply;
use redis::{Commands, Connection, RedisResult};

const CONSUMER_GROUP_NAME: &str = "market-data-service";
const CONSUMER_NAME: &str = "alice"; // TODO: REPLACE WITH POD_NAME

const REDIS_BLOCK_TIMEOUT_MS: usize = 50;
const REDIS_READ_COUNT: usize = 5000;

pub fn read_from_stream(conn: &mut Connection, product_id: String) -> Vec<(String, Order)> {
    let stream_name = format!("instrument:events:{}", product_id);

    let results: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(CONSUMER_GROUP_NAME)
        .arg(CONSUMER_NAME)
        .arg("BLOCK")
        .arg(REDIS_BLOCK_TIMEOUT_MS)
        .arg("COUNT")
        .arg(REDIS_READ_COUNT)
        .arg("STREAMS")
        .arg(&stream_name)
        .arg(">")
        .query(conn);

    let mut orders: Vec<(String, Order)> = Vec::new();

    let reply = match results {
        Ok(reply) => reply,
        Err(err) => {
            eprintln!("Redis stream read failed: {}", err);
            return orders;
        }
    };

    for stream_key in reply.keys {
        for stream_id in stream_key.ids {
            let message_id = stream_id.id;
            match Order::from_redis_map(&stream_id.map) {
                Ok(order) => {
                    orders.push((message_id, order));
                }
                Err(err) => {
                    eprintln!("Failed to parse order from Redis map: {:?}", err);
                }
            }
        }
    }
    orders
}

pub fn acknowledge(conn: &mut Connection, stream_name: &str, message_ids: Vec<String>) {
    let _: RedisResult<i64> = conn.xack(stream_name, CONSUMER_GROUP_NAME, &message_ids);
}
