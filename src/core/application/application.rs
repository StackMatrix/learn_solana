use std::sync::Arc;
use crate::core::application::user::UserApplication;
use crate::core::domain::DomainLayer;
use crate::core::infrastructure::InfrastructureLayer;

/// # Description
///     应用层，管理应用逻辑的核心组件
/// # Fields
///     jwt_config Arc<JwtService>: Arc<JwtService> 的实例，用于提供必要的服务
pub struct ApplicationLayer {
    // pub jwt_service: Arc<JwtService>,
    pub user_application: Arc<UserApplication>,
}

impl ApplicationLayer {
    /// # Description
    ///     初始化应用层，包括所有子应用初始化
    /// # Param
    ///     infrastructure_layer Arc<InfrastructureLayer>: 基础设施层的实例，用于提供必要的依赖
    /// # Return
    ///     Self: 初始化后的应用层实例
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>, domain_layer: Arc<DomainLayer>) -> Self {
        // 初始化 jwt 服务应用
        // let jwt_service = Arc::new(JwtService::new(infrastructure_layer.jwt.clone()));

        // 初始化 user 应用
        let user_application = Arc::new(UserApplication::new(infrastructure_layer.clone(), domain_layer.clone()));

        Self {
            // jwt_service,
            user_application
        }
    }
}
