mod data;
mod db;
mod handlers;
mod responses;
mod errors;

use std::sync::Arc;
use tokio::sync::Mutex;
use axum::Router;
use axum::routing::{get, post};
use dotenv::dotenv;
use std::env;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use env_logger::Builder;
use log::info;
use tokio_postgres::NoTls;
use moka::future::{Cache, CacheBuilder};
use serde_json::Value;
use crate::handlers::{create_order, get_order, not_found};

//===================================================================
#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("Can not create config file from .env!");

    dotenv().ok();
    let log_level = env::var("APP_LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    Builder::new()
        .parse_filters(&log_level)
        .init();

    info!("Database url: {}", config.database_url);
    let db_pool = db::create_pool(config.database_url)
        .await
        .expect("Can not create db pool!");
    let app_state = AppState {
        db_pool: Arc::new(db_pool),
        cache: Arc::new(CacheBuilder::new(100).build()),
        keys: Arc::new(Mutex::new(Vec::with_capacity(100))),
    };
    let app = Router::new()
        .route("/", get(get_order))
        .route("/", post(create_order))
        .with_state(app_state.into())
        .fallback(not_found);

    let service_url = format!("{}:{}", config.service_address, config.service_port);
    info!("Service url: {}", service_url);

    let listener = tokio::net::TcpListener::bind(service_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//===================================================================
#[derive(Clone)]
struct AppState {
    db_pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
    cache: Arc<Cache<String, Value>>,
    keys: Arc<Mutex<Vec<String>>>,
}
//===================================================================

pub struct Config {
    pub database_url: String,
    pub service_address: String,
    pub service_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Config, env::VarError> {
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            service_address: env::var("SERVICE_ADDRESS")?,
            service_port: env::var("SERVICE_PORT")?.parse().unwrap_or(8000),
        })
    }
}