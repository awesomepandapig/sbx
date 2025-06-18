use serde::Serialize;

#[derive(Serialize)]
struct L2Update {
    side: String,
    event_time: String,
    price_level: String,
    new_quantity: String,
}

pub fn format_l2_update_json(
    side: String,
    event_time: String,
    price_level: String,
    new_quantity: String,
    product_id: String,
) -> Result<String, serde_json::Error> {
    let l2_data = L2Update {
        side,
        event_time,
        price_level,
        new_quantity,
    };

    serde_json::to_string(&l2_data)
}
