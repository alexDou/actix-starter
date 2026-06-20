use actix_cors::Cors;
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::{
    App, HttpServer,
    cookie::{Key, SameSite},
    middleware::{Compress, Logger},
    web,
};
use dotenvy::dotenv;
use std::{env, io};
use time::Duration;

use actix_starter::libs::db;
use actix_starter::api::routes::{private_routes, public_routes};
use actix_starter::domain::auth::lib::common::get_jwt_secret;

const DEFAULT_APP_PORT: &str = "80";
const SESSION_NAME: &str = "a_session";
const SESSION_TTL_HRS: i64 = 18;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db_pool = db::create_pool().await;
    let jwt_key = Key::from(get_jwt_secret().as_bytes());

    let app_host = env::var("SERVER_HOST").expect("App host must be set");
    let app_port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| DEFAULT_APP_PORT.to_owned())
        .parse::<u16>()
        .expect("Invalid port");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), jwt_key.clone())
                    .cookie_name(SESSION_NAME.to_owned())
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Strict)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::hours(SESSION_TTL_HRS)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .service(
                web::scope("/api/v1")
                    .configure(public_routes)
                    .configure(private_routes),
            )
    })
    .bind((app_host, app_port))?
    .run()
    .await
}