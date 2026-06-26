use actix_web::{
    Error, HttpResponse,
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web,
};
use redis::{AsyncCommands, RedisError};

use crate::config::APP_CONFIG;

pub async fn cache_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, RedisError> {
    if !matches!(req.method(), Method::GET | Method::PUT | Method::PATCH | Method::DELETE) {
        return next.call(req).map_err(|_| RedisError).await;
    }

    let redis_pool = req
        .app_data::<web::Data<redis::Client>>()
        .expect("Redis Client must be registered in app_data");

    let mut conn = redis_pool
        .get_multiplexed_async_connection()
        .await
        .map_err(RedisError)?;

    let pathname = req.uri().path_and_query().clone();
    let method = req.method().clone();
    let cache_key = format!(
        "{}:{}",
        APP_CONFIG.cache.key_prefix,
        hex::encode(md5::compute(format!("{}:{}", method, pathname).as_bytes()))
    );

    if method == actix_web::http::Method::GET {
        if let Ok(Some(cached_json)) = conn
            .hget::<_, _, Option<String>>(&cache_key, hash_field)
            .await
        {
            let response = HttpResponse::Ok()
                .content_type("application/json")
                .body(cached_json);

            return Ok(req.into_response(response).map_into_boxed_body());
        }
    }

    let res = next.call(req).await?;

    if (method == actix_web::http::Method::PUT
        || method == actix_web::http::Method::PATCH
        || method == actix_web::http::Method::DELETE)
        && res.status().is_success()
    {
        let cache_key = format!("cache:{}", path);

        let _: () = conn.del(cache_key).await.unwrap_or(());
    }

    Ok(res)
}
