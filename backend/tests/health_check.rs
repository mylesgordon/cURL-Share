mod common;
use reqwest::StatusCode;

#[tokio::test]
async fn health_check_returns_200() {
    let app = common::spawn_test_app().await;
    let response = app.health_check().await;
    assert_eq!(response.status(), StatusCode::OK);
}