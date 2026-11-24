use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::{
    error_code::ErrorCode,
    error_response::ErrorResponse,
};

/// Các loại lỗi dùng chung trong ứng dụng.
/// Shared application error types.
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict(String),

    // Thông tin chi tiết chỉ được lưu ở server.
    // Internal details are only logged on the server.
    InternalServerError(String),
}

impl AppError {
   /// Chuyển loại lỗi thành HTTP status code.
   /// Convert an application error into an HTTP status code.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Xác định mã lỗi tương ứng.
    /// Determine the corresponding error code.
    fn error_code(&self) -> ErrorCode {
        match self {
            Self::BadRequest(_) => ErrorCode::BadRequest,
            Self::Unauthorized => ErrorCode::Unauthorized,
            Self::Forbidden => ErrorCode::Forbidden,
            Self::NotFound => ErrorCode::NotFound,
            Self::Conflict(_) => ErrorCode::Conflict,
            Self::InternalServerError(_) => {
                ErrorCode::InternalServerError
            }
        }
    }

    /// Tạo thông báo an toàn để trả về client.
    /// Generate a safe message to return to the client.
    fn public_message(&self) -> String {
        match self {
            Self::BadRequest(message) => message.clone(),

            Self::Unauthorized => {
                "Authentication is required".to_string()
            }

            Self::Forbidden => {
                "Access is forbidden".to_string()
            }

            Self::NotFound => {
                "Resource not found".to_string()
            }

            Self::Conflict(message) => message.clone(),

            Self::InternalServerError(_) => {
                "An internal server error occurred".to_string()
            }
        }
    }
}

/// Chuyển AppError thành HTTP response.
/// Convert AppError into an HTTP response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Ghi chi tiết lỗi nội bộ vào server log.
        // Log internal error details on the server.
        if let Self::InternalServerError(ref context) = self {
            tracing::error!(
                error_context = %context,
                "Internal server error"
            );
        }

        let status = self.status_code();

        let body = ErrorResponse::new(
            self.error_code(),
            self.public_message(),
        );

        (status, Json(body)).into_response()
    }
}

/// Chuyển lỗi I/O thành lỗi nội bộ của ứng dụng.
/// Convert I/O errors into application internal errors.
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::InternalServerError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Kiểm tra ánh xạ HTTP status code của các lỗi.
    /// Test HTTP status code mapping for application errors.
    #[test]
    fn test_error_status_mapping() {
        assert_eq!(
            AppError::BadRequest("Invalid input".to_string())
                .status_code(),
            StatusCode::BAD_REQUEST
        );

        assert_eq!(
            AppError::Unauthorized.status_code(),
            StatusCode::UNAUTHORIZED
        );

        assert_eq!(
            AppError::Forbidden.status_code(),
            StatusCode::FORBIDDEN
        );

        assert_eq!(
            AppError::NotFound.status_code(),
            StatusCode::NOT_FOUND
        );

        assert_eq!(
            AppError::Conflict("Conflict".to_string())
                .status_code(),
            StatusCode::CONFLICT
        );

        assert_eq!(
            AppError::InternalServerError("Database failed".to_string())
                .status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    /// Đảm bảo lỗi nội bộ không bị tiết lộ cho client.
    /// Ensure internal error details are never exposed to clients.
    #[test]
    fn test_internal_error_message_is_hidden() {
        let error = AppError::InternalServerError(
            "Database password is incorrect".to_string()
        );

        assert_eq!(
            error.public_message(),
            "An internal server error occurred"
        );
    }
}