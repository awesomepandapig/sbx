use super::order::Order;

use redis::streams::StreamReadReply;
use redis::{Connection, Commands, RedisResult};

// TODO: CHANGE TO USE PRODUCT ID
// const STREAM_KEY_PREFIX: &'static str = "instrument";
// const EVENT_STREAM_SUFFIX: &'static str = "events";

// TODO: REPLACE WITH PRODUCTID
const STREAM_NAME: &'static str = "instrument:events:JSP";

const CONSUMER_GROUP_NAME: &'static str = "market-data-service";
const CONSUMER_NAME: &'static str = "alice"; // TODO: REPLACE WITH POD_NAME

const REDIS_BLOCK_TIMEOUT_MS: usize = 5000;
const REDIS_READ_COUNT: usize = 1000;

pub fn read_from_stream(conn: &mut Connection) -> Vec<(String, Order)> {
    let results: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
        .arg("GROUP")
        .arg(CONSUMER_GROUP_NAME)
        .arg(CONSUMER_NAME)
        .arg("BLOCK")
        .arg(REDIS_BLOCK_TIMEOUT_MS)
        .arg("COUNT")
        .arg(REDIS_READ_COUNT)
        .arg("STREAMS")
        .arg(STREAM_NAME)
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

pub fn acknowledge(conn: &mut Connection, message_id: &str) {
    let _result: RedisResult<i32> = conn.xack(STREAM_NAME, CONSUMER_GROUP_NAME, &[message_id]);
}
