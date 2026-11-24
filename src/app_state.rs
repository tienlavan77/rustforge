use sqlx::MySqlPool;

/// Trạng thái dùng chung của toàn bộ ứng dụng.
/// Shared application state available to all application components.
#[derive(Clone)]
pub struct AppState {
    /// Connection pool dùng để truy cập MariaDB.
    /// Database connection pool used to access MariaDB.
    pub db: MySqlPool,
}

impl AppState {
    /// Tạo application state từ database connection pool.
    /// Create application state from an existing database connection pool.
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}
