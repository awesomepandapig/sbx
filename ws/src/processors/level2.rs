use serde::Serialize;
use tracing::error;

#[derive(Serialize)]
struct L2Update {
    side: String,
    event_time: String,
    price_level: String,
    new_quantity: String,
    product_id: String,
}

pub fn emit_level2_update(
    side: String,
    event_time: String,
    price_level: String,
    new_quantity: String,
    product_id: String,
) {
    let l2_data = L2Update {
        side,
        event_time,
        price_level,
        new_quantity,
        product_id,
    };

    let update_message = match serde_json::to_string(&l2_data) {
        Ok(json_payload) => json_payload,
        Err(_) => {
            error!("Failed to serialize L2Update:");
            String::new()
        }
    };

    println!("{}", update_message);
}
