use std::env;

use forgerust::{
    modules::appsettings::{
        model::NewAppSetting,
        repository::AppSettingRepository,
        service::AppSettingService,
    },
    shared::error::AppError,
};
use sqlx::mysql::MySqlPoolOptions;

/// Kiểm thử toàn bộ thao tác application settings trên MariaDB.
/// Test all application settings operations against MariaDB.
#[tokio::test]
async fn appsettings_repository_crud_operations() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be configured in .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to MariaDB");

    // Đảm bảo migration đã được áp dụng.
    // Ensure database migrations have been applied.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let key = format!("test.db008.{}", std::process::id());

    // Dọn dữ liệu còn sót lại từ lần chạy trước.
    // Clean up leftover data from a previous run.
    let _ = AppSettingRepository::delete(&pool, &key).await;

    let service = AppSettingService::new(pool.clone());

    let setting = NewAppSetting {
        setting_key: key.clone(),
        value: serde_json::json!({
            "enabled": true,
            "max_items": 20
        }),
        setting_group: "testing".to_string(),
        description: Some("DB-008 integration test".to_string()),
        is_public: true,
    };

    // Kiểm tra tạo cấu hình.
    // Verify setting creation.
    let created = AppSettingRepository::create(&pool, &setting)
        .await
        .expect("Failed to create setting");

    assert!(created.id > 0);
    assert_eq!(created.setting_key, key);

    // Kiểm tra tìm theo ID.
    // Verify lookup by ID.
    let by_id = AppSettingRepository::find_by_id(&pool, created.id)
        .await
        .expect("Failed to find setting by ID")
        .expect("Setting not found by ID");

    assert_eq!(by_id.setting_key, key);

    // Kiểm tra tìm theo key.
    // Verify lookup by key.
    let by_key = AppSettingRepository::find_by_key(&pool, &key)
        .await
        .expect("Failed to find setting by key")
        .expect("Setting not found by key");

    assert_eq!(by_key.id, created.id);

    // Kiểm tra lấy toàn bộ cấu hình.
    // Verify retrieving all settings.
    let all = AppSettingRepository::find_all(&pool)
        .await
        .expect("Failed to retrieve settings");

    assert!(all.iter().any(|item| item.setting_key == key));

    // Kiểm tra lấy cấu hình công khai.
    // Verify retrieving public settings.
    let public = AppSettingRepository::find_public(&pool)
        .await
        .expect("Failed to retrieve public settings");

    assert!(public.iter().any(|item| item.setting_key == key));

    // Kiểm tra cập nhật cấu hình.
    // Verify setting update.
    let updated = AppSettingRepository::update(
        &pool,
        &key,
        &serde_json::json!({
            "enabled": false,
            "max_items": 50
        }),
        "testing_updated",
        Some("Updated DB-008 test"),
        false,
    )
    .await
    .expect("Failed to update setting")
    .expect("Setting not found during update");

    assert_eq!(updated.setting_key, key);
    assert_eq!(updated.setting_group, "testing_updated");
    assert_eq!(updated.value["enabled"], false);
    assert!(!updated.is_public);

    // Kiểm tra cấu hình đã bị loại khỏi danh sách public.
    // Verify the setting is no longer publicly accessible.
    let public_after_update = AppSettingRepository::find_public(&pool)
        .await
        .expect("Failed to retrieve public settings");

    assert!(
        !public_after_update
            .iter()
            .any(|item| item.setting_key == key)
    );

    // Kiểm tra Service chuyển lỗi duplicate key thành Conflict.
    // Verify the service maps duplicate keys to Conflict.
    let duplicate = service.create(&setting).await;

    assert!(matches!(duplicate, Err(AppError::Conflict(_))));

    // Kiểm tra Service trả về NotFound khi không tồn tại.
    // Verify the service returns NotFound for missing settings.
    let missing_key = format!("{key}.missing");
    let missing = service.find_by_key(&missing_key).await;

    assert!(matches!(missing, Err(AppError::NotFound)));

    // Kiểm tra xóa cấu hình.
    // Verify setting deletion.
    let deleted = AppSettingRepository::delete(&pool, &key)
        .await
        .expect("Failed to delete setting");

    assert!(deleted);

    // Kiểm tra cấu hình không còn tồn tại.
    // Verify the setting no longer exists.
    let after_delete = AppSettingRepository::find_by_key(&pool, &key)
        .await
        .expect("Failed to query deleted setting");

    assert!(after_delete.is_none());

    // Kiểm tra xóa lại trả về false.
    // Verify deleting the same setting again returns false.
    let deleted_again = AppSettingRepository::delete(&pool, &key)
        .await
        .expect("Failed to delete setting again");

    assert!(!deleted_again);

    pool.close().await;
}