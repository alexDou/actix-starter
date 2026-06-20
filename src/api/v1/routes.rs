use actix_web::web;

use crate::api::auth::login::create_session;
use crate::api::auth::register::create_register;
use crate::api::dashboard::fetch::fetch_user_items;
use crate::api::monitoring::internal::health_check;

// Public routes
pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(create_register));
    cfg.route("/login", web::post().to(create_session));
    cfg.route("/health_check", web::get().to(health_check));
}

// Protected routes
pub fn private_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/dashboard", web::get().to(fetch_user_items));
}
