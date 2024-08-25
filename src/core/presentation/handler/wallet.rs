use std::sync::Arc;
use axum::{Json, extract::State, response::IntoResponse};
use crate::core::presentation::{
    api::{
        request::{
            wallet::GenerationRequest,
        },
        response::Response,
        error::ErrorCode
    },
};
use crate::core::application::ApplicationLayer;

/// # Description
///     钱包任务处理
pub struct WalletHandle;

impl WalletHandle {
    /// # Description
    ///     钱包生成
    /// # Param
    ///     State(application_layer): State<Arc<ApplicationLayer>> - 应用层
    ///     Json(payload): Json<RegisterRequest> - 请求 payload
    /// # Return
    ///     impl IntoResponse: 路由
    pub async fn generation_wallet(
        State(application_layer): State<Arc<ApplicationLayer>>,
        Json(_payload): Json<GenerationRequest>,
    ) -> impl IntoResponse {
        // 根据结果返回响应
        match application_layer
            .wallet_application
            .generation_wallet()
            .await {
            Ok(_) => Response::<String>::success(Some("".to_string())),
            Err(e) => Response::<()>::failed(ErrorCode::DefaultError, e.to_string())
        }
    }


}