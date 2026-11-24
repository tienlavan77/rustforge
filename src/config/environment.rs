
use std::fmt;

/// Các môi trường mà ForgeRust hỗ trợ.
/// Application environments supported by ForgeRust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Environment {
    /// Chuyển chuỗi cấu hình thành Environment.
    /// Convert a configuration string into an Environment.
    pub fn parse(value: &str) -> Result<Self, String> {
        match value.trim().to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            _ => Err(format!(
                "Invalid APP_ENV value: '{}'. Expected development, production, or test.",
                value
            )),
        }
    }

    /// Trả về tên chuẩn của môi trường.
    /// Return the canonical environment name.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
            Self::Test => "test",
        }
    }
}

impl Default for Environment {
    /// Mặc định chạy ở môi trường Development.
    /// Use Development as the default environment.
    fn default() -> Self {
        Self::Development
    }
}

impl fmt::Display for Environment {
    /// Hiển thị tên môi trường dưới dạng chuỗi.
    /// Display the environment as a string.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}
