// HTTP router configuration
// Cấu hình các HTTP route của ứng dụng.

use axum::{   
    middleware,
    // routing::get,
    Router,
};

use crate::{
    app_state::AppState,
    modules::{health, appsettings},
    server::middleware::request_id::request_id_middleware,
};

/// Tạo HTTP router và gắn application state.
/// Create the HTTP router and attach shared application state.
pub fn create_router(state: AppState) -> Router {
    Router::<AppState>::new()
        // Health endpoint.
        // Endpoint kiểm tra trạng thái ứng dụng.
        .merge(health::routes())
        .merge(appsettings::routes())

        // Apply request ID middleware.
        // Áp dụng middleware quản lý Request ID.
        .layer(middleware::from_fn(request_id_middleware))

        // Attach shared application state.
        // Gắn trạng thái dùng chung của ứng dụng.
        .with_state(state)
}