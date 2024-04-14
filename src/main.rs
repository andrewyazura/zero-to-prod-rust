use std::net::TcpListener;

use zero2prod::get_http_server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1234").expect("Failed to bind to port 1234.");

    get_http_server(listener)?.await
}
