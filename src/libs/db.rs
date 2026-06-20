use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use std::time::Duration;

fn get_database_connection() -> String {
    format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DATABASE_USER").unwrap(),
        env::var("DATABASE_USER_PWD").unwrap(),
        env::var("DATABASE_HOST").unwrap(),
        env::var("DATABASE_PORT").unwrap(),
        env::var("DATABASE_NAME").unwrap(),
    )
}

pub async fn create_pool() -> PgPool {
    let database_url = get_database_connection();

    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await
        .expect("Failed to create unified database connection pool")
}
