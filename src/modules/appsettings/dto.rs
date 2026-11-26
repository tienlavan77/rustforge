use serde::{Deserialize, Serialize};

use super::model::AppSetting;

/// Request tạo application setting mới.
/// Request payload for creating a new application setting.
#[derive(Debug, Deserialize)]
pub struct CreateAppSettingRequest {
    /// Khóa định danh cấu hình.
    /// Unique setting key.
    pub setting_key: String,

    /// Giá trị cấu hình JSON.
    /// JSON setting value.
    pub value: serde_json::Value,

    /// Nhóm cấu hình.
    /// Setting group.
    #[serde(default = "default_setting_group")]
    pub setting_group: String,

    /// Mô tả cấu hình.
    /// Setting description.
    pub description: Option<String>,

    /// Cho phép công khai cấu hình.
    /// Whether the setting can be publicly exposed.
    #[serde(default)]
    pub is_public: bool,
}

/// Request cập nhật application setting.
/// Request payload for updating an application setting.
#[derive(Debug, Deserialize)]
pub struct UpdateAppSettingRequest {
    /// Giá trị cấu hình JSON.
    /// JSON setting value.
    pub value: serde_json::Value,

    /// Nhóm cấu hình.
    /// Setting group.
    pub setting_group: String,

    /// Mô tả cấu hình.
    /// Setting description.
    pub description: Option<String>,

    /// Cho phép công khai cấu hình.
    /// Whether the setting can be publicly exposed.
    pub is_public: bool,
}

/// Response đại diện cho một application setting.
/// Response representation of an application setting.
#[derive(Debug, Serialize)]
pub struct AppSettingResponse {
    /// ID cấu hình.
    /// Setting identifier.
    pub id: u64,

    /// Khóa định danh cấu hình.
    /// Unique setting key.
    pub setting_key: String,

    /// Giá trị cấu hình JSON.
    /// JSON setting value.
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

impl From<AppSetting> for AppSettingResponse {
    /// Chuyển database model thành API response.
    /// Convert a database model into an API response.
    fn from(setting: AppSetting) -> Self {
        Self {
            id: setting.id,
            setting_key: setting.setting_key,
            value: setting.value,
            setting_group: setting.setting_group,
            description: setting.description,
            is_public: setting.is_public,
        }
    }
}

/// Response chứa danh sách application settings.
/// Response containing a list of application settings.
#[derive(Debug, Serialize)]
pub struct AppSettingListResponse {
    /// Danh sách cấu hình.
    /// List of settings.
    pub items: Vec<AppSettingResponse>,

    /// Tổng số cấu hình trong danh sách.
    /// Total number of settings in the list.
    pub total: usize,
}

/// Giá trị nhóm mặc định khi tạo cấu hình.
/// Default setting group used when creating a setting.
fn default_setting_group() -> String {
    "general".to_string()
}