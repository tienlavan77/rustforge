// Health module
// Module kiểm tra tình trạng hoạt động của backend.

pub mod dto;
pub mod handler;

use axum::{routing::get, Router};
use crate::app_state::AppState;

// Create health routes.
// Khởi tạo các route kiểm tra sức khỏe hệ thống.
pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/api/health", get(handler::health))
        .route("/api/health/live", get(handler::live))
        .route("/api/health/ready", get(handler::ready))
        .route("/api/debug/error", get(handler::error_test))
        .route("/api/debug/internal-error", get(handler::internal_error_test))              
}