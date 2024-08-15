use std::sync::Arc;
use crate::core::domain::user::UserDomain;
use crate::core::infrastructure::InfrastructureLayer;

/// # Description
///     领域层，管理领域逻辑的核心组件
pub struct DomainLayer {
    pub user_domain: Arc<UserDomain>,
}

impl DomainLayer {
    /// # Description
    ///     初始化领域层，包括所有子领域的初始化
    /// # Param
    ///     infrastructure_layer Arc<InfrastructureLayer>: 基础设施层的实例，用于提供必要的依赖
    /// # Return
    ///     Self: 初始化后的领域层实例
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        // 初始化用户子领域，从基础设施层向领域层提供用户仓库
        let user_domain = Arc::new(UserDomain::new(infrastructure_layer.persistence.user_repository.clone()).await);

        Self {
            user_domain
        }
    }
}
