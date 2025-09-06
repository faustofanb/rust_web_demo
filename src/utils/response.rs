use axum::response::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

pub fn success_response<T>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success(data))
}

pub fn success_response_with_message<T>(data: T, message: String) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success_with_message(data, message))
}

pub fn error_response<T>(message: String) -> Json<ApiResponse<T>> {
    Json(ApiResponse::error(message))
}
