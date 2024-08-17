use std::sync::Arc;
use crate::core::domain::wallet::service::WalletService;
use crate::core::infrastructure::InfrastructureLayer;

pub struct WalletDomain {
    pub wallet_service: Arc<WalletService>,
}

impl WalletDomain {
    /// # Description
    ///     初始化钱包领域
    /// # Param
    ///     infrastructure_layer: Arc<InfrastructureLayer>: 基础设施层为领域层提供服务
    /// # Return
    ///     Self: 初始化后的钱包领域实例
    pub async fn new(infrastructure_layer: Arc<InfrastructureLayer>) -> Self {
        // 初始化钱包服务并注入仓库
        let wallet_service = Arc::new(WalletService::new(infrastructure_layer));

        Self {
            wallet_service
        }
    }
}