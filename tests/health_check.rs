use actix_web::{App, test};
use oxide_mailer::configure_routes;

#[actix_web::test]
async fn health_check_works() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}
