use actix_web::{
    Error, HttpResponse,
    body::{BoxBody, MessageBody, to_bytes},
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    middleware::Next,
    web,
};
use redis::{AsyncCommands, ExpireOption};

use crate::config::APP_CONFIG;

pub async fn cache_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let method = req.method().clone();
    if !matches!(
        &method,
        &Method::GET | &Method::PUT | &Method::PATCH | &Method::DELETE
    ) {
        return Ok(next.call(req).await?.map_into_boxed_body())
    }

    let redis_pool = req
        .app_data::<web::Data<redis::Client>>()
        .expect("Redis Client must be registered in app_data");
    let mut conn = redis_pool
        .get_multiplexed_async_connection()
        .await
        .expect("Redis Client failed to start");

    let path = req.path().to_owned();
    let query_str = req.query_string().to_owned();
    let cache_key = format!("{}:{}", &APP_CONFIG.cache.key_prefix, &path);

    if method == Method::GET {
        match conn
            .hget::<&str, &str, Option<String>>(&cache_key, &query_str)
            .await
        {
            Ok(Some(cached_json_res)) => {
                return Ok(req.into_response(
                    HttpResponse::Ok()
                        .content_type("application/json")
                        .body(cached_json_res),
                ))
            }
            _ => {}
        }
    }

    let res = next.call(req).await?;

    if res.status().is_success() {
        match method {
            Method::GET => {
                let (request, response) = res.into_parts();
                let res_body_bytes = to_bytes(response.into_body())
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err.into()))?;
                let res_cache_json = String::from_utf8_lossy(&res_body_bytes).into_owned();

                let _: () = conn
                    .hset(&cache_key, &query_str, &res_cache_json)
                    .await
                    .unwrap_or(());
                let _: () = conn
                    .hexpire(&cache_key, 3600, ExpireOption::NONE, &query_str)
                    .await
                    .unwrap_or(());

                return Ok(ServiceResponse::new(
                    request,
                    HttpResponse::Ok()
                        .content_type("application/json")
                        .body(res_body_bytes),
                ));
            }

            Method::PUT | Method::PATCH | Method::DELETE => {
                let _: () = conn.del(&cache_key).await.unwrap_or(());
            }
            _ => {}
        }
    }

    Ok(res.map_into_boxed_body())
}
