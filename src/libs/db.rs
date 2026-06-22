use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::config::get_db_config;

pub async fn create_pool() -> PgPool {
    let db_config = get_db_config().unwrap();
    let databse_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user_name,
        db_config.user_pwd,
        db_config.host,
        db_config.port,
        db_config.db_name,
    );

    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&databse_url)
        .await
        .expect("Failed to create unified database connection pool")
}
