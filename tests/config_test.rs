
use forgerust::config::{
    DatabaseSettings,
    Environment,
    Settings,
};


/// Kiểm tra khả năng parse các môi trường hợp lệ.
/// Verify parsing of supported environments.
#[test]
fn test_environment_parsing() {
    assert_eq!(
        Environment::parse("development").unwrap(),
        Environment::Development
    );

    assert_eq!(
        Environment::parse("production").unwrap(),
        Environment::Production
    );

    assert_eq!(
        Environment::parse("test").unwrap(),
        Environment::Test
    );
}

/// Kiểm tra alias của các môi trường.
/// Verify environment aliases.
#[test]
fn test_environment_aliases() {
    assert_eq!(
        Environment::parse("dev").unwrap(),
        Environment::Development
    );

    assert_eq!(
        Environment::parse("prod").unwrap(),
        Environment::Production
    );
}

/// Kiểm tra từ chối môi trường không hợp lệ.
/// Verify rejection of unsupported environments.
#[test]
fn test_invalid_environment_is_rejected() {
    assert!(Environment::parse("staging").is_err());
}

/// Kiểm tra cấu hình hợp lệ.
/// Verify valid application settings.
#[test]
fn test_valid_settings() {
    let settings = valid_settings();

    assert!(settings.validate().is_ok());
}

/// Kiểm tra từ chối host rỗng.
/// Verify that an empty host is rejected.
#[test]
fn test_empty_host_is_rejected() {
    let mut settings = valid_settings();
    settings.host = " ".to_string();
    assert!(settings.validate().is_err());
}

/// Kiểm tra từ chối port bằng 0.
/// Verify that port zero is rejected.
#[test]
fn test_zero_port_is_rejected() {
    let mut settings = valid_settings();
    settings.port = 0;

    assert!(settings.validate().is_err());
}

/// Kiểm tra từ chối log level không hợp lệ.
/// Verify that an invalid log filter is rejected.
#[test]
fn test_invalid_log_level_is_rejected() {
    let mut settings = valid_settings();
    settings.log_level = "invalid".to_string();
    assert!(settings.validate().is_err());
}

/// Tạo cấu hình hợp lệ dùng chung cho các bài test.
/// Create valid shared settings for tests.
fn valid_settings() -> Settings {
    Settings {
        environment: Environment::Development,
        host: "0.0.0.0".to_string(),
        port: 3300,
        log_level: "debug".to_string(),
        database: DatabaseSettings {
            url: "mysql://user:password@127.0.0.1:3306/forgerust".to_string(),
            max_connections: 10,
            acquire_timeout_secs: 5,
        },
    }
}

/// Kiểm tra từ chối database URL không hợp lệ.
/// Verify that an invalid database URL is rejected.
#[test]
fn test_invalid_database_url_is_rejected() {
    let mut settings = valid_settings();
    settings.database.url = "postgres://localhost/test".to_string();

    assert!(settings.validate().is_err());
}

/// Kiểm tra từ chối connection pool có kích thước bằng 0.
/// Verify that a zero-sized connection pool is rejected.
#[test]
fn test_zero_database_connections_is_rejected() {
    let mut settings = valid_settings();
    settings.database.max_connections = 0;

    assert!(settings.validate().is_err());
}

/// Kiểm tra từ chối thời gian chờ database bằng 0.
/// Verify that a zero database acquire timeout is rejected.
#[test]
fn test_zero_database_timeout_is_rejected() {
    let mut settings = valid_settings();
    settings.database.acquire_timeout_secs = 0;

    assert!(settings.validate().is_err());
}
