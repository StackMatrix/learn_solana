use std::sync::Arc;
use crate::core::domain::wallet::service::WalletDomainService;

pub struct WalletDomain {
    pub domain_service: Arc<WalletDomainService>,
}

impl WalletDomain {
    /// # Description
    ///     初始化钱包领域
    /// # Param
    ///     infrastructure_layer: Arc<InfrastructureLayer>: 基础设施层为领域层提供服务
    /// # Return
    ///     Self: 初始化后的钱包领域实例
    pub async fn new() -> Self {
        // 初始化钱包服务并注入仓库
        let domain_service = Arc::new(WalletDomainService::new());

        Self {
            domain_service
        }
    }
}