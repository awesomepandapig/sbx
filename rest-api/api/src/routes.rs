use super::AppState;
use super::errors::AppError;
use super::order::{Order, create_order_buffer};

use std::sync::Arc;
use std::time::SystemTime;

use axum::{Json, extract::State, http::StatusCode};

use sbe::{ord_type_enum::OrdTypeEnum, side_enum::SideEnum};

use serde::Deserialize;

use super::order::MESSAGE_SIZE;

use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateOrder {
    pub product_id: String,
    pub side: String,
    pub r#type: String,
    pub size: f64,
    pub price: Option<f64>,
}

#[derive(Debug)]
struct ParsedOrderInput {
    cl_ord_id: [u8; 16],
    account: [u8; 16],
    symbol: [u8; 6],
    side: SideEnum,
    ord_type: OrdTypeEnum,
    timestamp_ns: u64,
    qty_mantissa: i64,
    price_mantissa: i64,
}

fn parse_and_validate_order_payload(payload: &CreateOrder) -> Result<ParsedOrderInput, AppError> {
    let cl_ord_id = Uuid::new_v4().into_bytes(); // Uuid::into_bytes() returns [u8; 16] directly
    let account = Uuid::new_v4().into_bytes(); // Hardcoded for now, ensure it's [u8; 16]

    // --- Symbol Validation ---
    // TODO: VALIDATE SYMBOL (enhance this as needed)
    let mut symbol = [0u8; 6];
    let symbol_str = payload.product_id.to_uppercase();
    if symbol_str.is_empty() || symbol_str.len() > 6 {
        return Err(AppError::ValidationError(format!(
            "Product ID '{}' is invalid. It must be between 1 and 6 characters.",
            payload.product_id
        )));
    }
    symbol[..symbol_str.len()].copy_from_slice(symbol_str.as_bytes());

    // --- Side Validation ---
    let side = match payload.side.to_lowercase().as_str() {
        "buy" => SideEnum::Buy,
        "sell" => SideEnum::Sell,
        _ => return Err(AppError::InvalidSide),
    };

    // --- Order Type Validation ---
    let ord_type = match payload.r#type.to_lowercase().as_str() {
        "limit" => OrdTypeEnum::Limit,
        "market" => OrdTypeEnum::Market,
        _ => return Err(AppError::InvalidOrderType),
    };

    // --- Price Validation based on Order Type ---
    match ord_type {
        OrdTypeEnum::Limit => {
            if payload.price.is_none() {
                return Err(AppError::ValidationError(
                    "Price is required for limit orders.".to_string(),
                ));
            }
            if payload.price.unwrap_or(-1.0) <= 0.0 {
                return Err(AppError::ValidationError(
                    "Price must be positive for limit orders.".to_string(),
                ));
            }
        }
        OrdTypeEnum::Market => {
            if payload.price.is_some() {
                return Err(AppError::ValidationError(
                    "Price should not be provided for market orders.".to_string(),
                ));
            }
        }
        _ => {} // Should not happen if using defined enums
    }

    // --- Size (Quantity) Validation ---
    if payload.size <= 0.0 {
        return Err(AppError::ValidationError(
            "Size must be greater than 0.".to_string(),
        ));
    }
    let qty_mantissa = (payload.size * 100_000_000.0).round() as i64;
    if qty_mantissa <= 0 {
        // Double check after conversion
        return Err(AppError::ValidationError(
            "Calculated quantity (size) must be positive.".to_string(),
        ));
    }

    let price_mantissa = match payload.price {
        Some(p) => (p * 100_000_000.0).round() as i64,
        None => i64::MIN, // Sentinel for market orders or when price is not applicable
    };

    let timestamp_ns = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| AppError::InternalServerError(format!("Failed to get system time: {}", e)))?
        .as_nanos() as u64;

    Ok(ParsedOrderInput {
        cl_ord_id,
        account,
        symbol,
        side,
        ord_type,
        timestamp_ns,
        qty_mantissa,
        price_mantissa,
    })
}

pub async fn get_order() {}

pub async fn post_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrder>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    // TODO: ADD AUTH MIDDLEWARE
    // TODO: VALIDATE BODY SHAPE (i believe axum already does this for us with 422 error... want to control error messages ourselves tho)
    // TODO: VALIDATE FIX-PROTOCOL
    // TODO: VALIDATE PRICE AND QUANTITY ARE IN-BOUNDS
    // TODO: VALIDATE USER FUNDS

    // let account = b"AAAAAAAAAAAAAAAA"; // FOR NOW SINCE WE DON'T HAVE AUTH IMPLEMENTED WE WILL JUST USE A HARDCODED ID

    let parsed_input = parse_and_validate_order_payload(&payload)?;

    let order_buffer = create_order_buffer(
        &parsed_input.cl_ord_id,
        &parsed_input.account,
        &parsed_input.symbol,
        parsed_input.side,
        parsed_input.ord_type,
        parsed_input.timestamp_ns,
        parsed_input.qty_mantissa,
        parsed_input.price_mantissa,
    );

    state.buffer.put_bytes(0, &order_buffer);

    let result = state
        .publication
        .lock()
        .unwrap()
        .offer_part(state.buffer, 0, MESSAGE_SIZE as i32);

    match result {
        Ok(_code) => {}
        Err(err) => println!("Offer with error: {}", err),
    }

    let order = Order::from_buffer(
        parsed_input.cl_ord_id,
        parsed_input.symbol,
        parsed_input.side,
        parsed_input.ord_type,
        parsed_input.timestamp_ns,
        parsed_input.qty_mantissa,
        parsed_input.price_mantissa,
    );

    Ok((StatusCode::CREATED, Json(order)))
}
