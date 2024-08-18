use std::sync::Arc;
use crate::core::domain::user::service::UserService;
use crate::core::infrastructure::InfrastructureLayer;

/// # Description
///     应用层，管理应用逻辑的核心组件
/// # Param
///     jwt_config Arc<JwtService>: Arc<JwtService> 的实例，用于提供必要的服务
pub struct ApplicationLayer {
    // pub jwt_service: Arc<JwtService>,
    // pub user_service: Arc<UserService>,
}

impl ApplicationLayer {
    /// # Description
    ///     初始化应用层，包括所有子应用初始化
    /// # Param
    ///     infrastructure_layer Arc<InfrastructureLayer>: 基础设施层的实例，用于提供必要的依赖
    /// # Return
    ///     Self: 初始化后的应用层实例
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        // 初始化 jwt 服务应用
        // let jwt_service = Arc::new(JwtService::new(infrastructure_layer.jwt.clone()));

        Self {
            // jwt_service,
            // user_service
        }
    }
}
