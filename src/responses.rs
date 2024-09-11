use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::Value;

// Enum используется для определения различных типов ответов, с указанием на источник данных
pub enum OrderApiResponse {
    Data(Value),
    DataFromCache(Value),
    Created(String),
}



// Реализация IntoResponse для enum для последующей возможности вернуть успешный ответ пользователю
impl IntoResponse for OrderApiResponse {
    fn into_response(self) -> Response {
        match self {
            OrderApiResponse::Data(data) => (StatusCode::OK, Json(data)).into_response(),
            OrderApiResponse::Created(task_id) => (StatusCode::CREATED, Json(task_id)).into_response(),
            OrderApiResponse::DataFromCache(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}