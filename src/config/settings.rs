
use std::{env, fmt};

use crate::config::Environment;

/// Cấu hình kết nối cơ sở dữ liệu.
/// Database connection configuration.
#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub acquire_timeout_secs: u64,
}

/// Cấu hình tổng thể của ứng dụng.
/// Application-wide configuration.
#[derive(Debug, Clone)]
pub struct Settings {
    pub environment: Environment,
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub database: DatabaseSettings,
}

/// Lỗi phát sinh trong quá trình đọc hoặc kiểm tra cấu hình.
/// Errors raised while loading or validating configuration.
#[derive(Debug, Clone)]
pub struct SettingsError {
    pub message: String,
}

impl SettingsError {
    /// Tạo một lỗi cấu hình mới.
    /// Create a new configuration error.
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for SettingsError {
    /// Chuyển lỗi cấu hình thành chuỗi để hiển thị.
    /// Format the configuration error as a displayable string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SettingsError {}

impl Settings {
    /// Đọc cấu hình từ biến môi trường và file .env.
    /// Load configuration from environment variables and the .env file.
    pub fn from_env() -> Result<Self, SettingsError> {
        let _ = dotenvy::dotenv();

        let environment_value = env::var("APP_ENV")
            .unwrap_or_else(|_| "development".to_string());

        let environment = Environment::parse(&environment_value)
            .map_err(SettingsError::new)?;

        let host = env::var("APP_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "3300".to_string())
            .parse::<u16>()
            .map_err(|_| {
                SettingsError::new(
                    "APP_PORT must be a valid port number.".to_string(),
                )
            })?;

        let default_log_level = match environment {
            Environment::Development => "debug",
            Environment::Production => "info",
            Environment::Test => "warn",
        };

        let log_level = env::var("LOG_LEVEL")
            .unwrap_or_else(|_| default_log_level.to_string());

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| {
                SettingsError::new(
                    "DATABASE_URL is required.".to_string(),
                )
            })?;

        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .map_err(|_| {
                SettingsError::new(
                    "DB_MAX_CONNECTIONS must be a positive integer.".to_string(),
                )
            })?;

        let acquire_timeout_secs = env::var("DB_ACQUIRE_TIMEOUT_SECS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .map_err(|_| {
                SettingsError::new(
                    "DB_ACQUIRE_TIMEOUT_SECS must be a positive integer.".to_string(),
                )
            })?;

        let settings = Self {
            environment,
            host,
            port,
            log_level,
            database: DatabaseSettings {
                url: database_url,
                max_connections,
                acquire_timeout_secs,
            },
        };

        settings.validate()?;

        Ok(settings)
    }

    /// Kiểm tra tính hợp lệ của toàn bộ cấu hình ứng dụng.
    /// Validate all application configuration settings.
    /// Kiểm tra tính hợp lệ của toàn bộ cấu hình ứng dụng.
/// Validate all application settings.
pub fn validate(&self) -> Result<(), SettingsError> {
    // Validate application host.
    // Kiểm tra địa chỉ host không được để trống.
    if self.host.trim().is_empty() {
        return Err(SettingsError::new(
            "APP_HOST must not be empty.".to_string(),
        ));
    }

    // Validate application port.
    // Kiểm tra port phải lớn hơn 0.
    if self.port == 0 {
        return Err(SettingsError::new(
            "APP_PORT must be greater than 0.".to_string(),
        ));
    }

    // Validate log level.
    // Chỉ chấp nhận các mức log được hỗ trợ.
    let valid_log_levels = ["trace", "debug", "info", "warn", "error", "off"];

    let normalized_log_level = self.log_level.trim().to_lowercase();

    if !valid_log_levels.contains(&normalized_log_level.as_str()) {
        return Err(SettingsError::new(
            "LOG_LEVEL must be one of: trace, debug, info, warn, error, off."
                .to_string(),
        ));
    }

    // Validate database URL.
    // Kiểm tra URL phải sử dụng MySQL/MariaDB protocol.
    if self.database.url.trim().is_empty()
        || !self.database.url.starts_with("mysql://")
    {
        return Err(SettingsError::new(
            "DATABASE_URL must be a valid mysql:// connection URL.".to_string(),
        ));
    }

    // Validate database connection pool size.
    // Số lượng kết nối tối đa phải lớn hơn 0.
    if self.database.max_connections == 0 {
        return Err(SettingsError::new(
            "DB_MAX_CONNECTIONS must be greater than 0.".to_string(),
        ));
    }

    // Validate database acquire timeout.
    // Thời gian chờ kết nối database phải lớn hơn 0.
    if self.database.acquire_timeout_secs == 0 {
        return Err(SettingsError::new(
            "DB_ACQUIRE_TIMEOUT_SECS must be greater than 0.".to_string(),
        ));
    }

    Ok(())
}
}
