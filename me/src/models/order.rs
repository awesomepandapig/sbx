use redis::RedisResult;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub user_id: String,
    pub side: String,
    pub r#type: String,
    pub created_at: String,
    pub executed_value: i64,
    pub status: String,
    pub settled: bool,
    pub price: Option<i64>,
    pub cancel_after: Option<String>,
    pub size: i64,
}

impl Order {
    pub fn from_redis_map(
        map: &HashMap<String, redis::Value>,
    ) -> RedisResult<Self> {
        Ok(Self {
            id: redis::from_redis_value(map.get("id").unwrap())?,
            product_id: redis::from_redis_value(
                map.get("product_id").unwrap(),
            )?,
            user_id: redis::from_redis_value(map.get("user_id").unwrap())?,
            side: redis::from_redis_value(map.get("side").unwrap())?,
            r#type: redis::from_redis_value(map.get("type").unwrap())?,
            created_at: redis::from_redis_value(
                map.get("created_at").unwrap(),
            )?,
            executed_value: redis::from_redis_value(
                map.get("executed_value").unwrap(),
            )?,
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
        })
    }

    pub fn to_redis_tuples(&self) -> Vec<(&str, String)> {
        let mut fields = Vec::new();

        fields.push(("id", self.id.clone()));
        fields.push(("product_id", self.product_id.clone()));
        fields.push(("user_id", self.user_id.clone()));
        fields.push(("side", self.side.clone()));
        fields.push(("type", self.r#type.clone()));
        fields.push(("created_at", self.created_at.clone()));
        fields.push(("status", self.status.clone()));
        fields.push(("executed_value", self.executed_value.to_string()));
        fields.push(("settled", self.settled.to_string()));
        fields.push(("size", self.size.to_string()));

        // Option fields
        if let Some(price) = self.price {
            fields.push(("price", price.to_string()));
        }

        if let Some(cancel_after) = &self.cancel_after {
            fields.push(("cancel_after", cancel_after.clone()));
        }

        return fields;
    }
}
