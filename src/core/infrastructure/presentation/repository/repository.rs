use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::core::infrastructure::presentation::repository::user::user_repository::UserRepository;
use crate::core::infrastructure::presentation::repository::wallet::wallet_repository::WalletRepository;

/// # Description
///     【基础设施】持久性连接组件实例的向上层暴露的数据仓库
/// # Param
///     user_repository Arc<UserRepository>: 用户仓库
///     wallet_repository Arc<WalletRepository>: 钱包仓库
pub struct Repository {
    pub user_repository: Arc<UserRepository>,
    pub wallet_repository: Arc<WalletRepository>
}

impl Repository {
    pub async fn new(db: DatabaseConnection) -> Self {
        let user_repository = Arc::new(UserRepository::new(db.clone()).await);
        let wallet_repository = Arc::new(WalletRepository::new(db.clone()).await);

        Self {
            user_repository,
            wallet_repository
        }
    }
}