use axum::async_trait;
use color_eyre::{Report, Result};
use crate::core::domain::wallet::entity::{ActiveModel as WalletActiveModel, Model as WalletModel};

/// # Description
///     钱包仓储接口
#[async_trait]
pub trait WalletRepositoryInterface {
    async fn find_by_id(&self, id: u64) -> Result<Option<WalletModel>, Report>;
    async fn save(&self, wallet: WalletActiveModel) -> Result<(), Report>;
}
