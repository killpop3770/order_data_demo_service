use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::Value;

pub enum OrderApiResponse {
    Data(Value),
    DataFromCache(Value),
    Created(String),
}

impl IntoResponse for OrderApiResponse {
    fn into_response(self) -> Response {
        match self {
            OrderApiResponse::Data(data) => (StatusCode::OK, Json(data)).into_response(),
            OrderApiResponse::Created(task_id) => (StatusCode::CREATED, Json(task_id)).into_response(),
            OrderApiResponse::DataFromCache(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}