use std::env;
use diesel_async::AsyncMysqlConnection;
use diesel_async::pooled_connection::{deadpool, AsyncDieselConnectionManager};
use diesel_async::pooled_connection::deadpool::{Object, Pool};
use dotenvy::dotenv;
use once_cell::sync::Lazy;

pub struct Database { pool: Pool<AsyncMysqlConnection> }

impl Database {
    pub async fn new() -> Self {
        Database { pool: DB_POOL.clone() }
    }

    pub async fn get_connection(&self) -> Result<Object<AsyncMysqlConnection>, deadpool::PoolError> {
        self.pool.get().await
    }
}

static DB_POOL: Lazy<Pool<AsyncMysqlConnection>> = Lazy::new(|| {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(database_url);

    Pool::builder(config)
        .build()
        .expect("Failed to create database connection pool")
});