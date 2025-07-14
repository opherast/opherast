use axum::{routing::get, Router, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Listening on https://{}", addr);
    serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Opherast!"
}
