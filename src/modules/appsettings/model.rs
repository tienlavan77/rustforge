use serde::{Deserialize, Serialize};

/// Đại diện cho một cấu hình ứng dụng được lưu trong MariaDB.
/// Represents an application setting stored in MariaDB.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AppSetting {
    /// ID duy nhất của cấu hình.
    /// Unique identifier of the setting.
    pub id: u64,

    /// Khóa định danh cấu hình.
    /// Unique setting key.
    pub setting_key: String,

    /// Giá trị cấu hình ở định dạng JSON.
    /// Setting value in JSON format.
    pub value: serde_json::Value,

    /// Nhóm cấu hình.
    /// Setting group.
    pub setting_group: String,

    /// Mô tả cấu hình.
    /// Setting description.
    pub description: Option<String>,

    /// Cho biết cấu hình có được phép công khai hay không.
    /// Indicates whether the setting can be publicly exposed.
    pub is_public: bool,
}

/// Dữ liệu đầu vào khi tạo cấu hình mới.
/// Input data required to create a new setting.
#[derive(Debug, Clone)]
pub struct NewAppSetting {
    /// Khóa định danh cấu hình.
    /// Unique setting key.
    pub setting_key: String,

    /// Giá trị cấu hình.
    /// Setting value.
    pub value: serde_json::Value,

    /// Nhóm cấu hình.
    /// Setting group.
    pub setting_group: String,

    /// Mô tả cấu hình.
    /// Setting description.
    pub description: Option<String>,

    /// Cho phép công khai cấu hình.
    /// Allow the setting to be publicly exposed.
    pub is_public: bool,
}