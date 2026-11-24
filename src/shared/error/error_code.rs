/// Mã lỗi chuẩn hóa của toàn bộ ứng dụng.
/// Standardized error codes used throughout the application.
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    InternalServerError,
}

impl ErrorCode {
    /// Trả về chuỗi mã lỗi tương ứng.
    /// Return the string representation of the error code.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::InternalServerError => "INTERNAL_SERVER_ERROR",
        }
    }
}