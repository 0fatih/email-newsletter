use std::net::TcpListener;

use email_newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("http://127.0.0.1:8080")?;
    run(listener)?.await
}
