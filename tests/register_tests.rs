use actix_web::{http::StatusCode, test, App};
use actix_starter::api::routes::public_routes;
use actix_starter::config::{AppData, AppMetrics};
use sqlx::postgres::PgPoolOptions;

fn test_app_data() -> actix_web::web::Data<AppData> {
    // dummy connection. never fire, act like a mock
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://postgres:postgres@127.0.0.1:1/postgres")
        .expect("failed to create lazy pg pool for tests");

    actix_web::web::Data::new(AppData {
        pg_pool: pool,
        metrics: AppMetrics::new(),
    })
}

#[actix_web::test]
async fn register_with_invalid_email_is_bad_request() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(public_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(serde_json::json!({
            "email": "not-an-email",
            "password": "Aa1!aaaa",
            "password_confirm": "Aa1!aaaa",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn register_with_weak_password_is_bad_request() {
    let app = test::init_service(
        App::new()
            .app_data(test_app_data())
            .configure(public_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/register")
        .set_json(serde_json::json!({
            "email": "user@example.com",
            "password": "alllowercase",
            "password_confirm": "alllowercase",
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}
