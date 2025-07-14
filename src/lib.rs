use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, Opherast!"
}