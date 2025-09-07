use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Failed to serialize or deserialize: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Concurrency conflict: Aggregate version mismatch")]
    ConcurrencyConflict,

    #[error("Aggregate not found: {0}")]
    AggregateNotFound(String),

    #[error("Business rule violation: {0}")]
    DomainError(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Authorization failed: {0}")]
    AuthorizationError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("An unknown error occurred")]
    Unknown,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ValidationError(errors) => {
                let formatted_errors: Vec<String> = errors
                    .field_errors()
                    .iter()
                    .flat_map(|(field, errors)| {
                        errors.iter().map(move |error| {
                            format!("{}: {}", field, error.message.as_ref().unwrap_or(&error.code))
                        })
                    })
                    .collect();
                (StatusCode::BAD_REQUEST, formatted_errors.join(", "))
            }
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::ConcurrencyConflict => {
                (StatusCode::CONFLICT, "Concurrency conflict: Resource was modified by another request".to_string())
            }
            AppError::AggregateNotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::DomainError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed".to_string())
            }
            AppError::SerializationError(_) => {
                (StatusCode::BAD_REQUEST, "Invalid data format".to_string())
            }
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Unknown => {
                (StatusCode::INTERNAL_SERVER_ERROR, "An unknown error occurred".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
