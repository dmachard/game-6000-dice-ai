use dice6000::api::Status;
use dice6000::api::create_router;

use axum::body::to_bytes;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::util::ServiceExt;

#[tokio::test]
async fn test_status_endpoint() {
    let app = create_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/status")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: Status = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(
        body,
        Status {
            status: "ok".to_string()
        }
    );
}
