use std::sync::Arc;
use axum::http::StatusCode;
use axum::{async_trait, Json, extract::{Extension, State}, debug_handler};
use axum::response::IntoResponse;
use serde_json::json;
use tracing::info;
use tracing_subscriber::fmt::format;
use crate::core::application::ApplicationLayer;
use crate::core::domain::DomainLayer;
use crate::core::domain::user::UserDomain;
use crate::core::infrastructure::InfrastructureLayer;
use crate::core::presentation::{api::{user::RegisterRequest, response::Response}, router::Router};
use crate::core::presentation::api::error::ErrorCode;

/// # Description
///     用户任务处理
pub struct UserHandle;

impl UserHandle {
    /// # Description
    ///     用户注册
    /// # Param
    ///     State(application_layer): State<Arc<ApplicationLayer>> - 应用层
    /// # Return
    ///     impl IntoResponse: 路由
    pub async fn register(
        State(application_layer): State<Arc<ApplicationLayer>>,
        Json(payload): Json<RegisterRequest>,
    ) -> impl IntoResponse {
        let result = application_layer
            .user_application
            .register_user(payload.identifier, payload.password).await;

        info!("Presentation + [Router] Register handle");



        // 根据结果返回响应
        // match result {
        //     Ok(_) => (StatusCode::OK, Response::success(None)),
        //     Err(e) => (StatusCode::BAD_REQUEST, Response::failed(ErrorCode::DefaultError, "!23".to_string()))
        // }

        // match result {
        //     Ok(_) => json!({"status": "ok"}),
        //     Err(_) => json!({"status": "err"}),
        // }

        Response::<String>::success(Some("".to_string()))
        // Response::<()>::failed(ErrorCode::DefaultError, "".to_string())
    }


    /// # Description
    ///     用户登陆
    /// # Param
    ///     None
    /// # Return
    ///     Router: 路由
    pub async fn login(
        State(application_layer): State<Arc<ApplicationLayer>>,
        Json(payload): Json<RegisterRequest>,
    ) {
        info!("+ [Router] Login handle");
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