//! main.rs
use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    run(address)?.await
}
