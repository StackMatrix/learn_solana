use std::sync::Arc;
use crate::core::domain::wallet::repository::WalletRepositoryInterface;
use crate::core::domain::wallet::service::WalletService;

pub struct WalletDomain {
    pub wallet_service: Arc<WalletService>,
}

impl WalletDomain {
    /// # Description
    ///     初始化钱包领域
    /// # Param
    ///     user_repository Arc<WalletRepositoryInterface>: 仓库接口实例
    /// # Return
    ///     Self: 初始化后的钱包领域实例
    pub async fn new(user_repository: Arc<dyn WalletRepositoryInterface>) -> Self {
        // 初始化钱包服务并注入仓库
        let wallet_service = Arc::new(WalletService::new(user_repository));

        Self {
            wallet_service
        }
    }
}