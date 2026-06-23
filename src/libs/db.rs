use actix_request_reply_cache::{RedisCacheMiddleware, RedisCacheMiddlewareBuilder};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{sync::Arc, time::Duration};

use crate::config::APP_CONFIG;

pub async fn create_pool() -> PgPool {
    let databse_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        APP_CONFIG.db.user_name,
        APP_CONFIG.db.user_pwd,
        APP_CONFIG.db.host,
        APP_CONFIG.db.port,
        APP_CONFIG.db.db_name,
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

pub fn create_redis_cache() -> Arc<RedisCacheMiddleware> {
    Arc::new(
        RedisCacheMiddlewareBuilder::new(format!(
            "redis://{}:{}",
            APP_CONFIG.cache.host, APP_CONFIG.cache.port
        ))
        .ttl(APP_CONFIG.cache.ttl)
        .cache_prefix("actix-starter:")
        .cache_if(|ctx| ctx.method == "GET")
        .build(),
    )
}
