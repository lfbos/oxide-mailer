use actix_web::{App, HttpResponse, HttpServer, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
