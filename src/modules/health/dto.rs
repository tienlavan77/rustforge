
use serde::Serialize;

/// Phản hồi chuẩn dùng chung cho các health endpoints.
/// Standard response shared by all health endpoints.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Trạng thái tổng thể của ứng dụng.
    /// Overall application health status.
    pub status: &'static str,

    /// Tên ứng dụng.
    /// Application name.
    pub application: String,

    /// Phiên bản ứng dụng.
    /// Application version.
    pub version: String,

    /// Trạng thái database, chỉ có trong readiness response.
    /// Database health status, present only in readiness responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<DatabaseHealth>,
}

/// Trạng thái sức khỏe của database.
/// Database health status.
#[derive(Debug, Serialize)]
pub struct DatabaseHealth {
    /// Trạng thái kết nối database.
    /// Database connection status.
    pub status: &'static str,
}

impl HealthResponse {
    /// Tạo phản hồi liveness khi backend đang hoạt động.
    /// Create a liveness response when the backend is running.
    pub fn live() -> Self {
        Self {
            status: "ok",
            application: "ForgeRust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database: None,
        }
    }

    /// Tạo phản hồi readiness khi database sẵn sàng.
    /// Create a readiness response when the database is available.
    pub fn ready() -> Self {
        Self {
            status: "ready",
            application: "ForgeRust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database: Some(DatabaseHealth {
                status: "connected",
            }),
        }
    }

    /// Tạo phản hồi readiness khi database không sẵn sàng.
    /// Create a readiness response when the database is unavailable.
    pub fn not_ready() -> Self {
        Self {
            status: "not_ready",
            application: "ForgeRust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            database: Some(DatabaseHealth {
                status: "unavailable",
            }),
        }
    }
}
