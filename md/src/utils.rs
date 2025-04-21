use super::order::Order;

use redis::streams::StreamReadReply;
use redis::{Commands, Connection, RedisResult};

const CONSUMER_GROUP_NAME: &'static str = "market-data-service";
const CONSUMER_NAME: &'static str = "alice"; // TODO: REPLACE WITH POD_NAME

const REDIS_BLOCK_TIMEOUT_MS: usize = 5000;
const REDIS_READ_COUNT: usize = 1000;

pub fn read_from_streams(conn: &mut Connection, products: &Vec<String>) -> Vec<(String, Order)> {
    let stream_names: Vec<String> = products
        .iter()
        .map(|p| format!("instrument:events:{}", p))
        .collect();

    let results: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
    .arg("GROUP")
    .arg(CONSUMER_GROUP_NAME)
    .arg(CONSUMER_NAME)
    .arg("BLOCK")
    .arg(REDIS_BLOCK_TIMEOUT_MS)
    .arg("COUNT")
    .arg(REDIS_READ_COUNT)
    .arg("STREAMS")
    .arg(stream_names)
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
                Err(_) => todo!(),
            }
        }
    }
    return orders;
}

pub fn acknowledge(conn: &mut Connection, stream_name: &str, message_id: &str) {
    let _: RedisResult<i64> = conn.xack(stream_name, CONSUMER_GROUP_NAME, &[message_id]);
}
