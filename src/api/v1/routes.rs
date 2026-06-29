use actix_web::web;

use crate::api::auth::login::create_session;
use crate::api::auth::register::create_register;
use crate::api::items::{
    create::create_item,
    fetch::{fetch_item, fetch_items},
    update::update_item,
};
use crate::api::monitoring::{external::metrics_prometheus, internal::health_check};

// Public routes
pub fn public_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(create_register));
    cfg.route("/login", web::post().to(create_session));
    cfg.route("/monitoring/health-check", web::get().to(health_check));
}

// Protected routes
pub fn private_routes(cfg: &mut web::ServiceConfig) {
    // toDo: amin routes
    cfg.route("/items/user/{uid}", web::get().to(fetch_items));
    cfg.route("/item/{iid}", web::get().to(fetch_item));
    cfg.route("/item", web::post().to(create_item));
    cfg.route("/item/{iid}", web::put().to(update_item));
    // monitoring. protected by its own token
    cfg.route("/monitoring/metrics", web::get().to(metrics_prometheus));
}
