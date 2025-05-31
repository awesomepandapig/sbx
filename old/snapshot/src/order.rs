use redis::RedisResult;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub user_id: String,
    pub side: String,
    pub r#type: String,
    pub created_at: i64,
    pub executed_value: i64,
    pub status: String,
    pub settled: bool,
    pub price: Option<i64>,
    pub cancel_after: Option<String>,
    pub size: i64,
    pub action: String,
}

impl Order {
    pub fn from_redis_map(map: &HashMap<String, redis::Value>) -> RedisResult<Self> {
        Ok(Self {
            id: redis::from_redis_value(map.get("id").unwrap())?,
            product_id: redis::from_redis_value(map.get("product_id").unwrap())?,
            user_id: redis::from_redis_value(map.get("user_id").unwrap())?,
            side: redis::from_redis_value(map.get("side").unwrap())?,
            r#type: redis::from_redis_value(map.get("type").unwrap())?,
            created_at: redis::from_redis_value(map.get("created_at").unwrap())?,
            executed_value: redis::from_redis_value(map.get("executed_value").unwrap())?,
            status: redis::from_redis_value(map.get("status").unwrap())?,
            settled: {
                redis::from_redis_value::<String>(map.get("settled").unwrap())
                    .unwrap()
                    .parse::<bool>()
                    .unwrap()
            },
            price: {
                map.get("price")
                    .and_then(|v| redis::from_redis_value::<i64>(v).ok())
            },
            cancel_after: {
                map.get("cancel_after")
                    .and_then(|v| redis::from_redis_value::<String>(v).ok())
            },
            size: redis::from_redis_value(map.get("size").unwrap())?,
            action: redis::from_redis_value(map.get("action").unwrap())?,
        })
    }
}
