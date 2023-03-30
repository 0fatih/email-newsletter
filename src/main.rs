use std::net::TcpListener;

use email_newsletter::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let listener = TcpListener::bind(format!(
        "http://127.0.0.1:{}",
        configuration.application_port
    ))?;
    run(listener)?.await
}
