use std::sync::Arc;
use crate::core::domain::user::service::UserDomainService;

pub struct UserDomain {
    pub domain_service: Arc<UserDomainService>,
}

impl UserDomain {
    /// # Description
    ///     初始化用户领域
    /// # Param
    ///     infrastructure_layer: Arc<InfrastructureLayer>: 基础设施层为领域层提供服务
    /// # Return
    ///     Self: 初始化后的用户领域实例
    pub async fn new() -> Self {
        // 初始化用户服务并注入仓库
        let domain_service = Arc::new(UserDomainService::new().await);

        Self {
            domain_service: domain_service.clone()
        }
    }
}