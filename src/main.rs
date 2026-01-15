use std::net::TcpListener;
use news_letter_backend::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}",configuration.application_port);

    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    println!("server running on port {}", port);
    run(listener)?.await
}
