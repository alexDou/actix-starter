use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite},
    middleware::{Compress, Logger},
    web,
};
use actix_cors::Cors;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use time::Duration;

use actix_starter::api::routes::{private_routes, public_routes};
use actix_starter::config::{APP_CONFIG, AppData, AppMetrics};
use actix_starter::libs::{db, middleware::cache_middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let jwt_key = Key::from(APP_CONFIG.api.jwt_secret.as_bytes());

    let app_data = web::Data::new(AppData {
        pg_pool: db::create_pool().await?,
        metrics: AppMetrics::new(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data)
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), jwt_key.clone())
                    .cookie_name(APP_CONFIG.api.session_name.clone())
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::hours(APP_CONFIG.api.session_ttl_hrs)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(APP_CONFIG.api.cors_max_age),
            )
            .wrap(cache_middleware)
            .wrap(Compress::default())
            .service(
                web::scope("/api/v1")
                    .configure(public_routes)
                    .configure(private_routes),
            )
    })
    .bind((APP_CONFIG.api.host.clone(), APP_CONFIG.api.port))?
    .run()
    .await
}
