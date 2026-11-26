use sqlx::MySqlPool;

use crate::modules::appsettings::service::AppSettingService;

/// Trạng thái dùng chung của toàn bộ ứng dụng.
/// Shared application state available to all application components.
#[derive(Clone)]
pub struct AppState {
    /// Connection pool dùng để truy cập MariaDB.
    /// Database connection pool used to access MariaDB.
    pub db: MySqlPool,

    /// Service quản lý application settings.
    /// Service for managing application settings.
    pub app_settings: AppSettingService,
}

impl AppState {
    /// Tạo application state từ database connection pool.
    /// Create application state from an existing database connection pool.
    pub fn new(db: MySqlPool) -> Self {
        let app_settings = AppSettingService::new(db.clone());

        Self {
            db,
            app_settings,
        }
    }
}