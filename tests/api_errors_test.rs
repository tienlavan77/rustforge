
use axum::{
    body::{to_bytes, Body},
    http::{HeaderMap, Request, StatusCode},
};
use forgerust::{
    app_state::AppState,
    server::router::create_router,
};
use serde_json::Value;
use tower::ServiceExt;
use uuid::Uuid;
use sqlx::mysql::MySqlPoolOptions;
/// Tạo application state dành cho integration test.
/// Create an application state for integration tests.
fn create_test_state() -> AppState {
    let pool = MySqlPoolOptions::new()
        .connect_lazy("mysql://test:test@127.0.0.1:3306/forgerust")
        .expect("Failed to create lazy MySQL pool");

    AppState::new(pool)
}
/// Gửi HTTP request vào Router và đọc JSON response.
/// Send an HTTP request to the Router and read its JSON response.
async fn request_json(path: &str) -> (StatusCode, HeaderMap, Value) {
    let app = create_router(create_test_state());

    let request = Request::builder()
        .uri(path)
        .body(Body::empty())
        .expect("Failed to build HTTP request");

    let response = app
        .oneshot(request)
        .await
        .expect("HTTP request failed");

    let status = response.status();
    let headers = response.headers().clone();

    let body = to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("Failed to read response body");

    let json: Value =
        serde_json::from_slice(&body).expect("Response body is not valid JSON");

    (status, headers, json)
}

/// Kiểm tra Request ID trong response header có phải UUID hợp lệ.
/// Verify that the Request ID in the response header is a valid UUID.
fn get_request_id(headers: &HeaderMap) -> String {
    let request_id = headers
        .get("x-request-id")
        .expect("Missing X-Request-ID header")
        .to_str()
        .expect("Invalid X-Request-ID header")
        .to_string();

    assert!(
        Uuid::parse_str(&request_id).is_ok(),
        "X-Request-ID is not a valid UUID"
    );

    request_id
}

#[tokio::test]
async fn test_health_endpoint_returns_200() {
    // Kiểm tra Health API trả về HTTP 200.
    // Verify that the Health API returns HTTP 200.

    let (status, headers, _body) = request_json("/api/health").await;

    assert_eq!(status, StatusCode::OK);

    assert!(headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("application/json")));

    get_request_id(&headers);
}

#[tokio::test]
async fn test_not_found_returns_standard_error_response() {
    // Kiểm tra lỗi 404 có đúng cấu trúc JSON chuẩn.
    // Verify that a 404 error uses the standard JSON structure.

    let (status, headers, body) = request_json("/api/debug/error").await;

    assert_eq!(status, StatusCode::NOT_FOUND);

    let header_request_id = get_request_id(&headers);

    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "NOT_FOUND");
    assert_eq!(body["error"]["message"], "Resource not found");

    assert_eq!(
        body["request_id"].as_str(),
        Some(header_request_id.as_str())
    );
}

#[tokio::test]
async fn test_internal_error_hides_sensitive_information() {
    // Kiểm tra lỗi 500 không tiết lộ thông tin nội bộ.
    // Verify that a 500 error does not expose internal information.

    let (status, headers, body) =
        request_json("/api/debug/internal-error").await;

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

    let header_request_id = get_request_id(&headers);

    assert_eq!(body["success"], false);
    assert_eq!(body["error"]["code"], "INTERNAL_SERVER_ERROR");

    assert_eq!(
        body["error"]["message"],
        "An internal server error occurred"
    );

    assert_eq!(
        body["request_id"].as_str(),
        Some(header_request_id.as_str())
    );

    let response_text = body.to_string();

    assert!(
        !response_text.contains("Test database connection failure"),
        "Internal error details must not be exposed to clients"
    );
}
