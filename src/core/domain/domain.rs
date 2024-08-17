use std::sync::Arc;
use tracing::info;
use crate::core::domain::user::UserDomain;
use crate::core::domain::wallet::WalletDomain;
use crate::core::infrastructure::InfrastructureLayer;

/// # Description
///     领域层，管理领域逻辑的核心组件
pub struct DomainLayer {
    pub wallet_domain: Arc<WalletDomain>,
    pub user_domain: Arc<UserDomain>,
}

impl DomainLayer {
    /// # Description
    ///     初始化领域层，包括所有子领域的初始化
    /// # Param
    ///     infrastructure_layer: Arc<InfrastructureLayer>: 基础设施层为领域层提供服务
    /// # Return
    ///     Self: 初始化后的领域层实例
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        // 初始化用户子领域
        let user_domain = Arc::new(UserDomain::new(infrastructure_layer.clone()).await);
        info!("+DomainLayer [UserDomain] Instant config complete.");

        // 初始化钱包子领域
        let wallet_domain = Arc::new(WalletDomain::new(infrastructure_layer).await);
        info!("+DomainLayer [WalletDomain] Instant config complete.");

        Self {
            user_domain,
            wallet_domain,
        }
    }
}
