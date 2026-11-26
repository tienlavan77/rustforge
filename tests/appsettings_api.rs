use axum::{
    body::{to_bytes, Body},
    http::{Method, Request, Response, StatusCode},
    Router,
};
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tower::ServiceExt;

use forgerust::{
    app_state::AppState,
    infrastructure::database::migration::run_migrations,
    server::router::create_router,
};

/// Gửi HTTP request đến application router.
/// Send an HTTP request to the application router.
async fn send_request(
    app: &Router,
    method: Method,
    uri: &str,
    payload: Option<Value>,
) -> Response<Body> {
    let mut builder = Request::builder().method(method).uri(uri);

    let body = match payload {
        Some(value) => {
            builder = builder.header("Content-Type", "application/json");
            Body::from(value.to_string())
        }
        None => Body::empty(),
    };

    let request = builder.body(body).unwrap();

    app.clone().oneshot(request).await.unwrap()
}

/// Đọc JSON từ HTTP response.
/// Read JSON from an HTTP response.
async fn response_json(response: Response<Body>) -> Value {
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();

    serde_json::from_slice(&bytes).unwrap()
}

/// Kiểm thử toàn bộ CRUD API của application settings.
/// Test the complete application settings CRUD API.
#[tokio::test]
async fn appsettings_api_crud_operations() {
    dotenvy::dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be configured");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to MariaDB");

    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    let app = create_router(AppState::new(pool.clone()));

    let setting_key = format!("test.db009.api.{}", std::process::id());

    // Dọn dữ liệu còn sót lại từ lần chạy trước.
    // Clean up leftover data from a previous run.
    sqlx::query("DELETE FROM app_settings WHERE setting_key = ?")
        .bind(&setting_key)
        .execute(&pool)
        .await
        .unwrap();

    // 1. CREATE: Tạo application setting.
    // 1. CREATE: Create an application setting.
    let response = send_request(
        &app,
        Method::POST,
        "/api/settings",
        Some(json!({
            "setting_key": setting_key,
            "value": "initial value",
            "setting_group": "general",
            "description": "API integration test",
            "is_public": true
        })),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let created = response_json(response).await;

    assert_eq!(created["setting_key"], setting_key);
    assert_eq!(created["value"], "initial value");
    assert_eq!(created["is_public"], true);

    // 2. DUPLICATE: Không cho phép tạo key trùng.
    // 2. DUPLICATE: Reject duplicate setting keys.
    let response = send_request(
        &app,
        Method::POST,
        "/api/settings",
        Some(json!({
            "setting_key": setting_key,
            "value": "duplicate",
            "is_public": true
        })),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CONFLICT);

    // 3. VALIDATION: Không cho phép key rỗng.
    // 3. VALIDATION: Reject an empty setting key.
    let response = send_request(
        &app,
        Method::POST,
        "/api/settings",
        Some(json!({
            "setting_key": "",
            "value": "invalid"
        })),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // 4. GET BY KEY: Đọc setting công khai.
    // 4. GET BY KEY: Retrieve a public setting.
    let uri = format!("/api/settings/{setting_key}");

    let response = send_request(&app, Method::GET, &uri, None).await;

    assert_eq!(response.status(), StatusCode::OK);

    let retrieved = response_json(response).await;

    assert_eq!(retrieved["setting_key"], setting_key);
    assert_eq!(retrieved["value"], "initial value");

    // 5. GET LIST: Kiểm tra setting xuất hiện trong danh sách công khai.
    // 5. GET LIST: Verify the setting appears in the public list.
    let response =
        send_request(&app, Method::GET, "/api/settings", None).await;

    assert_eq!(response.status(), StatusCode::OK);

    let list = response_json(response).await;

    assert!(
        list["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["setting_key"] == setting_key)
    );

    // 6. UPDATE: Cập nhật setting.
    // 6. UPDATE: Update the setting.
    let response = send_request(
        &app,
        Method::PUT,
        &uri,
        Some(json!({
            "value": "updated value",
            "setting_group": "testing",
            "description": "Updated integration test",
            "is_public": false
        })),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);

    let updated = response_json(response).await;

    assert_eq!(updated["setting_key"], setting_key);
    assert_eq!(updated["value"], "updated value");
    assert_eq!(updated["setting_group"], "testing");
    assert_eq!(updated["is_public"], false);

    // 7. PRIVATE: Không cho phép đọc setting riêng tư.
    // 7. PRIVATE: Prevent access to private settings.
    let response = send_request(&app, Method::GET, &uri, None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 8. UPDATE NOT FOUND: Không cập nhật key không tồn tại.
    // 8. UPDATE NOT FOUND: Reject updates for missing keys.
    let missing_uri = format!("{uri}.missing");

    let response = send_request(
        &app,
        Method::PUT,
        &missing_uri,
        Some(json!({
            "value": "missing",
            "setting_group": "testing",
            "description": null,
            "is_public": false
        })),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 9. DELETE: Xóa setting.
    // 9. DELETE: Delete the setting.
    let response =
        send_request(&app, Method::DELETE, &uri, None).await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 10. GET AFTER DELETE: Xác nhận setting đã bị xóa.
    // 10. GET AFTER DELETE: Verify the setting has been deleted.
    let response = send_request(&app, Method::GET, &uri, None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // 11. DELETE NOT FOUND: Xóa lại key không tồn tại.
    // 11. DELETE NOT FOUND: Reject deletion of a missing key.
    let response =
        send_request(&app, Method::DELETE, &uri, None).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    // Dọn dữ liệu test để không ảnh hưởng các lần chạy tiếp theo.
    // Clean up test data to avoid affecting subsequent runs.
    sqlx::query("DELETE FROM app_settings WHERE setting_key = ?")
        .bind(&setting_key)
        .execute(&pool)
        .await
        .unwrap();
}