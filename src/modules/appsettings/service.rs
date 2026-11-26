use sqlx::MySqlPool;

use crate::shared::error::AppError;

use super::{
    model::{AppSetting, NewAppSetting},
    repository::AppSettingRepository,
};

/// Service xử lý nghiệp vụ application settings.
/// Service layer for application settings business logic.
#[derive(Clone)]
pub struct AppSettingService {
    pool: MySqlPool,
}

impl AppSettingService {
   /// Tạo service từ database connection pool hiện có.
    /// Create the service using an existing database connection pool.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
    
    /// Chuyển lỗi SQLx thành lỗi ứng dụng.
/// Convert SQLx errors into application errors.
   fn map_database_error(error: sqlx::Error) -> AppError {
      match error {
         sqlx::Error::Database(database_error)
               if database_error.is_unique_violation() =>
         {
               AppError::Conflict(
                  "Application setting already exists".to_string(),
               )
         }
         sqlx::Error::RowNotFound => AppError::NotFound,
         error => {
               tracing::error!(
                  error = %error,
                  "Application settings database operation failed"
               );

               AppError::InternalServerError(
                  "Application settings database operation failed".to_string(),
               )
         }
      }
   }

    /// Tạo application setting mới.
    /// Create a new application setting.
    pub async fn create(
        &self,
        setting: &NewAppSetting,
    ) -> Result<AppSetting, AppError> {
        AppSettingRepository::create(&self.pool, setting)
            .await
            .map_err(Self::map_database_error)
    }

    /// Tìm application setting theo ID.
    /// Find an application setting by ID.
    pub async fn find_by_id(
        &self,
        id: u64,
    ) -> Result<AppSetting, AppError> {
        AppSettingRepository::find_by_id(&self.pool, id)
            .await
            .map_err(Self::map_database_error)?
            .ok_or(AppError::NotFound)
    }

    /// Tìm application setting theo key.
    /// Find an application setting by key.
    pub async fn find_by_key(
        &self,
        key: &str,
    ) -> Result<AppSetting, AppError> {
        AppSettingRepository::find_by_key(&self.pool, key)
            .await
            .map_err(Self::map_database_error)?
            .ok_or(AppError::NotFound)
    }

    /// Lấy toàn bộ application settings.
    /// Retrieve all application settings.
    pub async fn find_all(
        &self,
    ) -> Result<Vec<AppSetting>, AppError> {
        AppSettingRepository::find_all(&self.pool)
            .await
            .map_err(Self::map_database_error)
    }

    /// Lấy các application settings được phép công khai.
    /// Retrieve publicly accessible application settings.
    pub async fn find_public(
        &self,
    ) -> Result<Vec<AppSetting>, AppError> {
        AppSettingRepository::find_public(&self.pool)
            .await
            .map_err(Self::map_database_error)
    }

    /// Cập nhật application setting theo key.
    /// Update an application setting by key.
    pub async fn update(
        &self,
        key: &str,
        value: &serde_json::Value,
        setting_group: &str,
        description: Option<&str>,
        is_public: bool,
    ) -> Result<AppSetting, AppError> {
        AppSettingRepository::update(
            &self.pool,
            key,
            value,
            setting_group,
            description,
            is_public,
        )
        .await
        .map_err(Self::map_database_error)?
        .ok_or(AppError::NotFound)
    }

    /// Xóa application setting theo key.
    /// Delete an application setting by key.
    pub async fn delete(
        &self,
        key: &str,
    ) -> Result<(), AppError> {
        let deleted = AppSettingRepository::delete(&self.pool, key)
            .await
            .map_err(Self::map_database_error)?;

        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}