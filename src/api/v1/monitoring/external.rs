use actix_web::{HttpRequest, HttpResponse, Responder, http::header::AUTHORIZATION, web};
use prometheus_client::{
    encoding::text::encode,
};
use std::{time::Duration};
use tokio::time::timeout;

use crate::config::{APP_CONFIG, AppData, DependencyLabels};
use crate::libs::errors::AppError;

pub async fn metrics_prometheus(req: HttpRequest, app_data: web::Data<AppData>) -> Result<impl Responder, AppError> {
    let req_token = match req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|raw| raw.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
    {
        Some(token) => token,
        None => {
            return Err(AppError::Unauthorized);
        }
    };

    if String::from(req_token) != APP_CONFIG.api.metrics_token {
        return Err(AppError::Unauthorized);
    }

    let probe_timeout = Duration::from_millis(APP_CONFIG.api.metrics_ttl);

    // Ping postgres
    let is_pg_up = match timeout(
        probe_timeout,
        sqlx::query("SELECT 1").execute(&app_data.pg_pool),
    )
    .await
    {
        Ok(Ok(_)) => 1,
        _ => 0,
    };
    app_data
        .metrics
        .dependency_health
        .get_or_create(&DependencyLabels {
            dependency: "postgres".to_owned(),
        })
        .set(is_pg_up);

    // Ping redis
    let redis_client = redis::Client::open(format!(
        "redis://{}:{}",
        APP_CONFIG.cache.host, APP_CONFIG.cache.port
    )).map_err(|_| AppError::InternalServerError )?;
    let redis_ping = async {
        match redis_client
            .clone()
            .get_multiplexed_async_connection()
            .await
        {
            Ok(mut conn) => match redis::cmd("PING").exec_async(&mut conn).await {
                Ok(_) => 1,
                Err(_) => 0,
            },
            Err(_) => 0,
        }
    };
    let is_redis_up = match timeout(probe_timeout, redis_ping).await {
        Ok(status) => status,
        Err(_) => 0,
    };

    app_data
        .metrics
        .dependency_health
        .get_or_create(&DependencyLabels {
            dependency: "redis".to_owned(),
        })
        .set(is_redis_up);

    let mut buffer = String::new();
    if let Err(e) = encode(&mut buffer, &app_data.metrics.registry) {
        log::error!("Failed to encode prometheus metrics: {}", e);
        return Err(AppError::InternalServerError);
    }

    Ok(HttpResponse::Ok()
        .content_type("application/openmetrics-text; version=1.0.0; charset=utf-8")
        .body(buffer))
}
