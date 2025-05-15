use super::order::Order;

use redis::streams::StreamReadReply;
use redis::RedisResult;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;

const CONSUMER_GROUP_NAME: &str = "snapshot-service";
const CONSUMER_NAME: &str = "alice"; // TODO: REPLACE WITH POD_NAME

const REDIS_BLOCK_TIMEOUT_MS: usize = 5000;
const REDIS_READ_COUNT: usize = 1000;

pub async fn read_from_stream(conn: &mut MultiplexedConnection, product_id: String) -> Vec<(String, Order)> {
    let stream_name = format!("instrument:events:{}", product_id);

    let result: RedisResult<StreamReadReply> = redis::cmd("XREADGROUP")
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
        .query_async(conn)
        .await;

    let mut orders: Vec<(String, Order)> = Vec::new();

    match result {
        Ok(reply) => {
            for stream_key in reply.keys {
                for stream_id in stream_key.ids {
                    let message_id = stream_id.id;
                    match Order::from_redis_map(&stream_id.map) {
                        Ok(order) => orders.push((message_id, order)),
                        Err(err) => eprintln!("Failed to parse order from Redis map: {:?}", err),
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Redis stream read failed: {}", err);
        }
    }

    orders
}

pub async fn acknowledge(conn: &mut MultiplexedConnection, stream_name: &str, message_id: &str) {
    let _: RedisResult<i64> = conn.xack(stream_name, CONSUMER_GROUP_NAME, &[message_id]).await;
}
