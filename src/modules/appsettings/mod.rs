pub mod model;
pub mod repository;
pub mod service;
pub mod dto;
pub mod handler;

use axum::{   
   routing::{get, post}, 
   Router
};

use crate::app_state::AppState;

/// Khởi tạo các route quản lý application settings.
/// Initialize application settings routes.
pub fn routes() -> Router<AppState> {
    Router::<AppState>::new()
      .route("/api/settings", get(handler::list).post(handler::create))
      .route("/api/settings/{key}", get(handler::get_by_key).put(handler::update).delete(handler::delete))
}
