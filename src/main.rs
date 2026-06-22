use actix_cors::Cors;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite},
    middleware::{Compress, Logger},
    web,
};
use dotenvy::dotenv;
use time::Duration;

use actix_starter::api::routes::{private_routes, public_routes};
use actix_starter::config::get_api_config;
use actix_starter::libs::db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let api_config = get_api_config().unwrap();

    let db_pool = db::create_pool().await;
    let jwt_key = Key::from(api_config.jwt_secret.as_bytes());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), jwt_key.clone())
                    .cookie_name(api_config.session_name.clone())
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(Duration::hours(api_config.session_ttl_hrs)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(api_config.cors_max_age),
            )
            .wrap(Compress::default())
            .service(
                web::scope("/api/v1")
                    .configure(public_routes)
                    .configure(private_routes),
            )
    })
    .bind((api_config.host, api_config.port))?
    .run()
    .await
}
