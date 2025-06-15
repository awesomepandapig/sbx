// src/errors.rs (or a similar module)
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidSide,
    InvalidOrderType,
    ValidationError(String),
    JsonDeserializationError(String), // For custom messages from JsonRejection
    InternalServerError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_details) = match self {
            AppError::InvalidSide => (
                StatusCode::BAD_REQUEST,
                "Invalid side provided.".to_string(),
                Some("Allowed values are 'buy' or 'sell'.".to_string()),
            ),
            AppError::InvalidOrderType => (
                StatusCode::BAD_REQUEST,
                "Invalid order type provided.".to_string(),
                Some("Allowed values are 'limit' or 'market'.".to_string()),
            ),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg, None),
            AppError::JsonDeserializationError(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg, None)
            }
            AppError::InternalServerError(msg) => {
                // It's good practice to log internal errors on the server
                eprintln!("Internal Server Error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error occurred.".to_string(),
                    None, // Avoid exposing internal details to the client
                )
            }
        };

        let body = Json(json!(ErrorResponse {
            message: error_message,
            details: error_details,
        }));

        (status, body).into_response()
    }
}

// This allows `?` to be used on `Json` extractors if the handler returns `Result<_, AppError>`
impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        // You can customize the message further based on the type of JsonRejection
        // For example, distinguishing between JsonSyntaxError and JsonDataError
        AppError::JsonDeserializationError(format!(
            "Request payload deserialization error: {}",
            rejection.body_text()
        ))
    }
}
