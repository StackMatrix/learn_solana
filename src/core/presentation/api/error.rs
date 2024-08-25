#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorCode {
    DefaultError,        // 默认错误
    ValidateError,       // 验证错误
    TokenError,          // Token失效
    Forbidden,           // 无权限
    NotFound,            // 数据不存在
    TooManyRequests,     // 请求过于频繁
    UserNotFound,        // 用户不存在
    ServerError,         // 服务器错误
}

impl ErrorCode {
    pub fn code(&self) -> u32 {
        match self {
            ErrorCode::DefaultError => 40000,
            ErrorCode::ValidateError => 42200,
            ErrorCode::TokenError => 40100,
            ErrorCode::Forbidden => 40300,
            ErrorCode::NotFound => 40400,
            ErrorCode::TooManyRequests => 42900,
            ErrorCode::UserNotFound => 40401,
            ErrorCode::ServerError => 50000,
        }
    }


    pub fn description(&self) -> &str {
        match self {
            ErrorCode::DefaultError => "Default error",
            ErrorCode::ValidateError => "Validation error",
            ErrorCode::TokenError => "Token expired",
            ErrorCode::Forbidden => "Permission denied",
            ErrorCode::NotFound => "Data not found",
            ErrorCode::TooManyRequests => "Too many requests",
            ErrorCode::UserNotFound => "User not found",
            ErrorCode::ServerError => "Internal server error",
        }
    }
}