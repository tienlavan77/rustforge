
use axum::{
   extract::{State, Path},
   http::StatusCode,
   Json,
};

use super::{
   dto::{
      AppSettingListResponse,
      AppSettingResponse,
      CreateAppSettingRequest,
      UpdateAppSettingRequest,
   },
   model::NewAppSetting,
};

use crate::{
   app_state::AppState,
   shared::error::AppError,
};


/// Lấy danh sách application settings được phép công khai.
/// Retrieve the list of application settings allowed for public access.
pub async fn list(
   State(state): State<AppState>,
) -> Result<Json<AppSettingListResponse>, AppError> {
   let settings = state.app_settings.find_public().await?;

   let items: Vec<AppSettingResponse> =
      settings.into_iter().map(AppSettingResponse::from).collect();

   let total = items.len();

   Ok(Json(AppSettingListResponse {
      items,
      total,
   }))
}

/// Lấy application setting công khai theo key.
/// Retrieve a public application setting by its key.
pub async fn get_by_key(
   State(state): State<AppState>,
   Path(key): Path<String>,
) -> Result<Json<AppSettingResponse>, AppError> {
   let setting = state.app_settings.find_by_key(&key).await?;

   if !setting.is_public {
      return Err(AppError::NotFound);
   }

   Ok(Json(AppSettingResponse::from(setting)))
}

/// Tạo application setting mới.
/// Create a new application setting.
pub async fn create(
   State(state): State<AppState>,
   Json(request): Json<CreateAppSettingRequest>,
) -> Result<(StatusCode, Json<AppSettingResponse>), AppError> {
   if request.setting_key.trim().is_empty() {
      return Err(AppError::BadRequest(
         "Setting key cannot be empty".to_string(),
      ));
   }

   let setting = NewAppSetting {
      setting_key: request.setting_key,
      value: request.value,
      setting_group: request.setting_group,
      description: request.description,
      is_public: request.is_public,
   };

   let created = state.app_settings.create(&setting).await?;

   Ok((
      StatusCode::CREATED,
      Json(AppSettingResponse::from(created)),
   ))
}

/// Cập nhật application setting theo key.
/// Update an application setting by its key.
pub async fn update(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(request): Json<UpdateAppSettingRequest>,
) -> Result<Json<AppSettingResponse>, AppError> {
    if key.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Setting key cannot be empty".to_string(),
        ));
    }

    let setting = state
        .app_settings
        .update(
            &key,
            &request.value,
            &request.setting_group,
            request.description.as_deref(),
            request.is_public,
        )
        .await?;

    Ok(Json(AppSettingResponse::from(setting)))
}

/// Xóa application setting theo key.
/// Delete an application setting by its key.
pub async fn delete(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, AppError> {
    if key.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Setting key cannot be empty".to_string(),
        ));
    }

    state.app_settings.delete(&key).await?;

    Ok(StatusCode::NO_CONTENT)
}
