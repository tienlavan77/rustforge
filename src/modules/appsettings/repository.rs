use sqlx::MySqlPool;

use super::model::{AppSetting, NewAppSetting};

/// Các thao tác truy cập dữ liệu application settings.
/// Database access operations for application settings.
pub struct AppSettingRepository;

impl AppSettingRepository {
    /// Tạo cấu hình mới và trả về bản ghi vừa tạo.
    /// Create a new setting and return the inserted record.
    pub async fn create(
        pool: &MySqlPool,
        setting: &NewAppSetting,
    ) -> Result<AppSetting, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT INTO app_settings (
                setting_key,
                `value`,
                setting_group,
                description,
                is_public
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&setting.setting_key)
        .bind(&setting.value)
        .bind(&setting.setting_group)
        .bind(&setting.description)
        .bind(setting.is_public)
        .execute(pool)
        .await?;

        let id = result.last_insert_id();

        Self::find_by_id(pool, id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    /// Tìm cấu hình theo ID.
    /// Find a setting by its ID.
    pub async fn find_by_id(
        pool: &MySqlPool,
        id: u64,
    ) -> Result<Option<AppSetting>, sqlx::Error> {
        sqlx::query_as::<_, AppSetting>(
            r#"
            SELECT
                id,
                setting_key,
                `value`,
                setting_group,
                description,
                is_public
            FROM app_settings
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// Tìm cấu hình theo khóa định danh.
    /// Find a setting by its unique key.
    pub async fn find_by_key(
        pool: &MySqlPool,
        key: &str,
    ) -> Result<Option<AppSetting>, sqlx::Error> {
        sqlx::query_as::<_, AppSetting>(
            r#"
            SELECT
                id,
                setting_key,
                `value`,
                setting_group,
                description,
                is_public
            FROM app_settings
            WHERE setting_key = ?
            "#,
        )
        .bind(key)
        .fetch_optional(pool)
        .await
    }

    /// Lấy toàn bộ cấu hình theo thứ tự nhóm và khóa.
    /// Retrieve all settings ordered by group and key.
    pub async fn find_all(
        pool: &MySqlPool,
    ) -> Result<Vec<AppSetting>, sqlx::Error> {
        sqlx::query_as::<_, AppSetting>(
            r#"
            SELECT
                id,
                setting_key,
                `value`,
                setting_group,
                description,
                is_public
            FROM app_settings
            ORDER BY setting_group ASC, setting_key ASC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Lấy các cấu hình được phép công khai.
    /// Retrieve settings allowed to be publicly exposed.
    pub async fn find_public(
        pool: &MySqlPool,
    ) -> Result<Vec<AppSetting>, sqlx::Error> {
        sqlx::query_as::<_, AppSetting>(
            r#"
            SELECT
                id,
                setting_key,
                `value`,
                setting_group,
                description,
                is_public
            FROM app_settings
            WHERE is_public = TRUE
            ORDER BY setting_group ASC, setting_key ASC
            "#,
        )
        .fetch_all(pool)
        .await
    }

    /// Cập nhật thông tin cấu hình, không thay đổi setting_key.
    /// Update setting fields without changing its setting_key.
    pub async fn update(
        pool: &MySqlPool,
        key: &str,
        value: &serde_json::Value,
        setting_group: &str,
        description: Option<&str>,
        is_public: bool,
    ) -> Result<Option<AppSetting>, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE app_settings
            SET
                `value` = ?,
                setting_group = ?,
                description = ?,
                is_public = ?
            WHERE setting_key = ?
            "#,
        )
        .bind(value)
        .bind(setting_group)
        .bind(description)
        .bind(is_public)
        .bind(key)
        .execute(pool)
        .await?;

        Self::find_by_key(pool, key).await
    }

    /// Xóa cấu hình theo khóa định danh.
    /// Delete a setting by its unique key.
    pub async fn delete(
        pool: &MySqlPool,
        key: &str,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM app_settings WHERE setting_key = ?",
        )
        .bind(key)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}