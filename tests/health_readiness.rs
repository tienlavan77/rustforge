
use std::time::Duration;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::mysql::MySqlPoolOptions;

use forgerust::{
    app_state::AppState,
    modules::health::handler,
};

/// Kiểm tra readiness trả HTTP 200 khi MariaDB hoạt động.
/// Verify readiness returns HTTP 200 when MariaDB is available.
#[tokio::test]
async fn readiness_returns_200_when_database_is_available() {
    // Nạp cấu hình database từ file .env.
    // Load database configuration from the .env file.
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be configured in .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to MariaDB for integration test");

    let state = AppState::new(pool);

    let (status, Json(response)) =
        handler::ready(State(state)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(response.status, "ready");
    assert_eq!(
        response.database.as_ref().unwrap().status,
        "connected"
    );
}

/// Kiểm tra readiness trả HTTP 503 khi database không khả dụng.
/// Verify readiness returns HTTP 503 when the database is unavailable.
#[tokio::test]
async fn readiness_returns_503_when_database_is_unavailable() {
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy("mysql://test:test@127.0.0.1:1/forgerust")
        .expect("Failed to create lazy MySQL pool");

    let state = AppState::new(pool);

    let (status, Json(response)) =
        handler::ready(State(state)).await;

    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.status, "not_ready");
    assert_eq!(
        response.database.as_ref().unwrap().status,
        "unavailable"
    );
}

/// Kiểm tra response lỗi readiness không tiết lộ thông tin database.
/// Verify readiness error responses do not expose database details.
#[tokio::test]
async fn readiness_does_not_expose_database_connection_details() {
    let pool = MySqlPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy("mysql://test:test@127.0.0.1:1/forgerust")
        .expect("Failed to create lazy MySQL pool");

    let state = AppState::new(pool);

    let (_, Json(response)) =
        handler::ready(State(state)).await;

    let response_json = serde_json::to_string(&response)
        .expect("Failed to serialize readiness response");

    assert!(!response_json.contains("mysql://"));
    assert!(!response_json.contains("127.0.0.1"));
    assert!(!response_json.contains("connection refused"));
}

/// Kiểm tra liveness vẫn hoạt động khi database không khả dụng.
/// Verify liveness remains operational when the database is unavailable.
#[tokio::test]
async fn liveness_returns_200_without_database_connection() {
    let Json(response) = handler::live().await;

    assert_eq!(response.status, "ok");
    assert!(response.database.is_none());
}
