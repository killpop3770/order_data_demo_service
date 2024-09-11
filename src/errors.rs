use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use bb8_postgres::bb8::RunError;
use serde_json::json;
use thiserror::Error;

// Enum используется для типизации и автогенерации текста ошибки
#[derive(Error, Debug)]
pub enum OrderApiError {
    #[error("Error to create connection manager: {0}")]
    DBCreatingManagerError(tokio_postgres::Error),
    #[error("Error to create db pool: {0}")]
    DBCreatingPoolError(tokio_postgres::Error),
    #[error("Error getting connection from DB pool: {0}")]
    DBPoolError(RunError<tokio_postgres::Error>),
    #[error("Database query error: {0}")]
    DBQueryError(tokio_postgres::Error),
    #[error("Not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Serde error: {0}")]
    SerDeError(serde_json::Error),
}

// Реализация IntoResponse для enum для последующей возможности вернуть данные об ошибке пользователю
impl IntoResponse for OrderApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            OrderApiError::DBPoolError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database pool error: {}", error)
            ),
            OrderApiError::DBQueryError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database query error: {}", error)
            ),
            OrderApiError::NotFound => (
                StatusCode::NOT_FOUND,
                String::from("The requested resource was not found!"),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error!"),
            ),
        };
        (
            status,
            Json(json!({"message": message, "time": chrono::Utc::now().to_string()})),
        ).into_response()
    }
}
