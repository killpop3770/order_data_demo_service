use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use log::{error, info};
use serde_json::Value;
use tokio_postgres::NoTls;

use crate::data::OrderRequest;
use crate::errors::OrderApiError;

pub async fn create_pool(db_address: String) -> Result<Pool<PostgresConnectionManager<NoTls>>, OrderApiError> {
    let manager = PostgresConnectionManager::new_from_stringlike(db_address, NoTls)
        .map_err(OrderApiError::DBCreatingManagerError)?;
    let pool = Pool::builder().build(manager).await
        .map_err(OrderApiError::DBCreatingPoolError)?;
    Ok(pool)
}

pub async fn get_order(
    db_pool: &Pool<PostgresConnectionManager<NoTls>>,
    order_request: OrderRequest,
) -> Result<Value, OrderApiError> {
    let connection = db_pool
        .get()
        .await
        .map_err(OrderApiError::DBPoolError)?;
    let row = connection
        .query_opt("SELECT * FROM orders WHERE order_uid = $1", &[&order_request.order_uid])
        .await
        .map_err(OrderApiError::DBQueryError)?;
    if let Some(row) = row {
        let order_uid: String = row.get(1);
        let mut body: Value = row.get(2);
        if let Some(body_map) = body.as_object_mut() {
            body_map.insert("order_uid".to_string(), Value::String(order_uid));
        }
        info!("Get order from database!");
        Ok(body)
    } else {
        error!("Failed to get order from database, because it's not found!");
        Err(OrderApiError::NotFound)
    }
}

pub async fn create_order(
    db_pool: &Pool<PostgresConnectionManager<NoTls>>,
    order_uid: String,
    body: Value,
) -> Result<String, OrderApiError> {
    let connection = db_pool
        .get()
        .await
        .map_err(OrderApiError::DBPoolError)?;
    let query = "INSERT INTO orders (order_uid, body) VALUES ($1, $2) RETURNING *".to_string();
    let row = connection
        .query_one(query.as_str(), &[&order_uid, &body])
        .await
        .map_err(OrderApiError::DBQueryError)?;
    info!("Create order in database!");
    Ok(row.get(1))
}