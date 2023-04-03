use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use email_newsletter::{configuration::get_configuration, startup::run};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

    run(listener, connection_pool)?.await?;

    Ok(())
}
