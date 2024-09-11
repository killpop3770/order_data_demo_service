use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;
use log::info;
use moka::future::Cache;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{AppState, db};
use crate::errors::OrderApiError;
use crate::responses::OrderApiResponse;
use crate::data::{Order, OrderRequest};

// Хэндлер заглушка для отсутствующих роутов приложения
pub async fn not_found() -> OrderApiError {
    return OrderApiError::NotFound;
}

// Хэндлер для возвращения заказа в json формате
// Если данные о заказе есть в кэше, то они будут возращены из кэша
// Иначе данные берутся из БД и дополнительно сохраняются в кэш
pub async fn get_order(
    State(state): State<Arc<AppState>>,
    Query(order_request): Query<OrderRequest>,
) -> Result<OrderApiResponse, OrderApiError> {
    info!("Order request: {:?}", order_request);
    if let Some(order) = read_from_cache(&state.cache.clone(), order_request.order_uid.clone()).await {
        return Ok(OrderApiResponse::DataFromCache(order));
    } else {
        let result = db::get_order(&state.db_pool, order_request.order_uid)
            .await.map_err(|_err| OrderApiError::InternalServerError)?;
        let order: Order = serde_json::from_value(result.clone()).map_err(|err| OrderApiError::SerDeError(err))?;
        let order_uid = order.order_uid.clone();
        let body = serde_json::to_value(&order).map_err(|err| OrderApiError::SerDeError(err))?;
        save_in_cache(&state.cache.clone(), &state.keys.clone(), order_uid, body).await;
        info!("Get order!");
        Ok(OrderApiResponse::Data(result))
    }
}

// Функция чтения заказа из кэша
async fn read_from_cache(cache: &Cache<String, Value>, order_uid: String) -> Option<Value> {
    if let Some(mut order) = cache.get(&order_uid).await {
        info!("Get order from cache!");
        if let Some(body_map) = order.as_object_mut() {
            body_map.insert("order_uid".to_string(), Value::String(order_uid));
        }
        Some(order)
    } else {
        info!("Can not find order in cash!");
        None
    }
}

// Хэндлер для создания заказа в БД и дополнительного сохранения в кэше
pub async fn create_order(
    State(state): State<Arc<AppState>>,
    Json(order): Json<Order>,
) -> Result<OrderApiResponse, OrderApiError> {
    let order_uid = order.order_uid.clone();
    let body = serde_json::to_value(&order).map_err(|err| OrderApiError::SerDeError(err))?;
    let result = db::create_order(&state.db_pool.clone(), order_uid.clone(), body.clone())
        .await.map_err(|_err| OrderApiError::InternalServerError)?;
    save_in_cache(&state.cache.clone(), &state.keys.clone(), order_uid, body).await;
    info!("Create order!");
    Ok(OrderApiResponse::Created(result))
}

// Функция сохранения заказа в кэш
async fn save_in_cache(cache: &Cache<String, Value>, keys: &Mutex<Vec<String>>, order_uid: String, body: Value) {
    if cache.get(&order_uid).await.is_none() {
        cache.insert(order_uid.clone(), body).await;
        let mut keys = keys.lock().await;
        keys.push(order_uid);
        info!("Order saved in cash!");
    };
}