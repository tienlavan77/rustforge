
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use super::dto::HealthResponse;
use crate::{
    app_state::AppState,
    infrastructure::database::health::check_database,
    shared::error::AppError,
};

/// Kiểm tra tiến trình backend còn hoạt động hay không.
/// Check whether the backend process is still alive.
pub async fn live() -> Json<HealthResponse> {
    Json(HealthResponse::live())
}

/// Kiểm tra backend và MariaDB đã sẵn sàng phục vụ request hay chưa.
/// Check whether the backend and MariaDB are ready to serve requests.
pub async fn ready(
    State(state): State<AppState>,
) -> (StatusCode, Json<HealthResponse>) {
    match check_database(&state.db).await {
        Ok(()) => (
            StatusCode::OK,
            Json(HealthResponse::ready()),
        ),
        Err(error) => {
            // Ghi log lỗi database nhưng không tiết lộ chi tiết nội bộ cho client.
            // Log database errors without exposing internal details to clients.
            tracing::warn!(
                error = %error,
                "Database readiness check failed"
            );

            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthResponse::not_ready()),
            )
        }
    }
}

/// Endpoint tương thích ngược với health endpoint cũ.
/// Backward-compatible handler for the previous health endpoint.
pub async fn health() -> Json<HealthResponse> {
    live().await
}

/// Endpoint thử nghiệm xử lý lỗi 404.
/// Test endpoint for centralized 404 error handling.
pub async fn error_test() -> Result<Json<HealthResponse>, AppError> {
    Err(AppError::NotFound)
}

/// Tạo lỗi nội bộ để kiểm tra logging.
/// Generate an internal error to test server logging.
pub async fn internal_error_test() -> Result<Json<HealthResponse>, AppError> {
    Err(AppError::InternalServerError(
        "Test database connection failure".to_string(),
    ))
}

