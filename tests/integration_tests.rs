use actix_web::{test, App};
use actix_starter::api::routes::{private_routes, public_routes};

#[actix_web::test]
async fn test_health_check_endpoint() {
    let app = test::init_service(
        App::new().configure(public_routes)
    ).await;

    let req = test::TestRequest::get().uri("/api/v1/monitoring/internl/health_check").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_protected_route_unauthorized() {
    let app = test::init_service(
        App::new().configure(private_routes)
    ).await;

    let req = test::TestRequest::get().uri("/api/v1/dashboard").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
}
