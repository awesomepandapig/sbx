use sbe::WriteBuf;
use sbe::message_header_codec::ENCODED_LENGTH;
use sbe::new_order_single_codec::{NewOrderSingleEncoder, SBE_BLOCK_LENGTH};
use sbe::ord_type_enum::OrdTypeEnum;
use sbe::side_enum::SideEnum;
use uuid::Uuid;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use serde::Serialize;

pub const MESSAGE_SIZE: usize = SBE_BLOCK_LENGTH as usize + ENCODED_LENGTH;

#[derive(Serialize, Debug)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub side: String,
    pub r#type: String,
    pub created_at: String,
    pub executed_value: f64,
    pub status: String,
    pub settled: bool,
    pub price: Option<f64>,
    pub cancel_after: Option<String>,
    pub size: f64,
}

pub fn format_timestamp_ns(timestamp_ns: u64) -> String {
    let secs = (timestamp_ns / 1_000_000_000) as i64;
    let nanos = (timestamp_ns % 1_000_000_000) as u32;

    let naive_dt = NaiveDateTime::from_timestamp_opt(secs, nanos)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()); // Default to epoch if out of range

    let datetime: DateTime<Utc> = Utc.from_utc_datetime(&naive_dt);
    datetime.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
}

impl Order {
    pub fn from_buffer(
        cl_ord_id: [u8; 16],
        symbol: [u8; 6],
        side: SideEnum,
        ord_type: OrdTypeEnum,
        timestamp_ns: u64,
        qty_mantissa: i64,
        price_mantissa: i64,
    ) -> Self {
        Order {
            id: Uuid::from_bytes(cl_ord_id).to_string(),
            product_id: String::from_utf8_lossy(&symbol)
                .trim_end_matches('\0')
                .to_string(),
            side: match side {
                SideEnum::Buy => "buy".to_string(),
                SideEnum::Sell => "sell".to_string(),
                _ => "unknown".to_string(),
            },
            r#type: match ord_type {
                OrdTypeEnum::Limit => "limit".to_string(),
                OrdTypeEnum::Market => "market".to_string(),
                _ => "unknown".to_string(),
            },
            created_at: format_timestamp_ns(timestamp_ns),
            executed_value: 0.0,
            status: "open".to_string(),
            settled: false,
            price: if price_mantissa != i64::MIN {
                Some(price_mantissa as f64 / 100_000_000.0)
            } else {
                None
            },
            cancel_after: None,
            size: qty_mantissa as f64 / 100_000_000.0,
        }
    }
}

pub fn create_order_buffer(
    cl_ord_id: &[u8; 16],
    account: &[u8; 16],
    symbol: &[u8; 6],
    side: SideEnum,
    ord_type: OrdTypeEnum,
    timestamp_ns: u64,
    qty_mantissa: i64,
    price_mantissa: i64,
) -> [u8; MESSAGE_SIZE] {
    let mut buffer = [0u8; MESSAGE_SIZE];
    let write_buf = WriteBuf::new(&mut buffer[..]);

    let mut order_encoder = NewOrderSingleEncoder::default().wrap(write_buf, ENCODED_LENGTH);

    let mut header_composite_encoder = order_encoder.header(0);
    order_encoder = header_composite_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after SBE header encoding");

    order_encoder.cl_ord_id(cl_ord_id);
    order_encoder.account(account);
    order_encoder.symbol(symbol);
    order_encoder.side(side);

    let mut transact_time_encoder = order_encoder.transact_time_encoder();
    transact_time_encoder.time(timestamp_ns);
    order_encoder = transact_time_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after transact_time encoding");

    order_encoder.ord_type(ord_type);

    let mut order_qty_encoder = order_encoder.order_qty_encoder();
    order_qty_encoder.mantissa(qty_mantissa);
    order_encoder = order_qty_encoder
        .parent()
        .expect("Failed to retrieve parent encoder after order_qty encoding");

    let mut price_encoder_composite = order_encoder.price_encoder();
    price_encoder_composite.mantissa(price_mantissa);
    price_encoder_composite
        .parent()
        .expect("Failed to retrieve parent encoder after price encoding");

    buffer
}
