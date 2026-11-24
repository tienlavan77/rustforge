use serde::Serialize;

use super::error_code::ErrorCode;

/// Chi tiết lỗi trả về cho client.
/// Error details returned to the client.
#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

/// Cấu trúc JSON chuẩn cho mọi lỗi API.
/// Standard JSON structure for all API errors.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl ErrorResponse {
    /// Tạo một error response thống nhất.
    /// Create a standardized error response.
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            success: false,
            error: ErrorDetail {
                code: code.as_str().to_string(),
                message,
            },
            request_id: None,
        }
    }
}