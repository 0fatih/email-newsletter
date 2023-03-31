use std::net::TcpListener;

use email_newsletter::{configuration::get_configuration, startup::run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!(
        "http://127.0.0.1:{}",
        configuration.application_port
    ))?;
    run(listener, connection_pool)?.await
}
