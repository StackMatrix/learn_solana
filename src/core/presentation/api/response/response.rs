use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::core::presentation::api::error::ErrorCode;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub code: u32,
    pub data: Option<T>,
    pub message: String,
}

impl<T> Response<T> {
    /// Creates a successful response wrapped in `Json<Value>`
    pub fn success(data: Option<T>) -> Json<Value>
    where
        T: Serialize,
    {
        Json(json!(Response {
            code: 200,
            data,
            message: "ok".to_string(),
        }))
    }

    /// Creates a failed response wrapped in `Json<Value>`, without requiring `T: Serialize`
    pub fn failed(code: ErrorCode, message: String) -> Json<Value> {
        Json(json!(Response {
            code: ErrorCode::code(&code),
            data: None::<Value>,  // 使用这个方法避免出现T被能被序列化的错误
            message,
        }))
    }
}
