use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

/// # Description
///     用户任务处理
pub struct UserHandle;

impl UserHandle {
    /// # Description
    ///     用户注册
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn register() -> impl IntoResponse {
        info!("+ [Router] Register handle");
        (StatusCode::OK, "Register successful")
    }

    /// # Description
    ///     用户登陆
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn login() -> impl IntoResponse {
        info!("+ [Router] Login handle");
        (StatusCode::OK, "Login successful")
    }

    /// # Description
    ///     用户找回
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn retrieve() -> impl IntoResponse {
        info!("+ [Router] Retrieve handle");
        (StatusCode::OK, "Retrieve successful")
    }
}