use axum::serve;
use opherast::create_router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = create_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Listening on https://{}", addr);
    serve(listener, app).await.unwrap();
}
