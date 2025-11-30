use oxide_mailer::{configuration::get_configuration, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // We have removed the hard-coded `8080` port number - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener: TcpListener = TcpListener::bind(address)?;
    run(listener, connection_pool).await?.await
}
