use opherast::create_router;
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;
use http_body_util::BodyExt;

#[tokio::test]
async fn root_path_returns_hello() {
    let app = create_router();
    let response = app.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_text = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_text, "Hello, Opherast!");
}