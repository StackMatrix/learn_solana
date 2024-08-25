use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse};
use tracing::info;
use crate::core::presentation::{
    api::{
        request::{
            user::{RegisterRequest, LoginRequest}
        },
        response::Response, error::ErrorCode
    }
};
use crate::core::application::ApplicationLayer;
use crate::core::infrastructure::jwt::TokenOutPut;

/// # Description
///     用户任务处理
pub struct UserHandle;

impl UserHandle {
    /// # Description
    ///     用户注册
    /// # Param
    ///     State(application_layer): State<Arc<ApplicationLayer>> - 应用层
    ///     Json(payload): Json<RegisterRequest> - 请求 payload
    /// # Return
    ///     impl IntoResponse: 路由
    pub async fn register(
        State(application_layer): State<Arc<ApplicationLayer>>,
        Json(payload): Json<RegisterRequest>,
    ) -> impl IntoResponse {
        // 根据结果返回响应
        match application_layer
            .user_application
            .register_user(payload.identifier, payload.password)
            .await {
                Ok(_) => Response::<String>::success(Some("".to_string())),
                Err(e) => Response::<()>::failed(ErrorCode::DefaultError, e.to_string())
        }
    }


    /// # Description
    ///     用户登陆
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn login(
        State(application_layer): State<Arc<ApplicationLayer>>,
        Json(payload): Json<LoginRequest>,
    ) -> impl IntoResponse {
        // 根据结果返回响应
        match application_layer
            .user_application
            .login_user(payload.identifier, payload.password)
            .await {
                Ok(value) => Response::<TokenOutPut>::success(Some(value)),
                Err(e) => Response::<()>::failed(ErrorCode::DefaultError, e.to_string())
            }
    }

    /// # Description
    ///     用户找回
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn retrieve() {
        info!("+ [Router] Retrieve handle");
    }
}