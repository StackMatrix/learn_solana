use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::entity::prelude::*;
use async_trait::async_trait;
use std::sync::Arc;
use color_eyre::{Report, Result};
use crate::core::domain::wallet::entity::{ActiveModel as WalletActiveModel, Entity as WalletEntity, Model as WalletModel};
use crate::core::domain::wallet::repository::WalletRepositoryInterface;

/// # Description
///     钱包仓储实现
/// # Fields
///     db: Arc<DatabaseConnection>: 数据库连接
pub struct WalletRepository {
    db: Arc<DatabaseConnection>,
}

impl WalletRepository {
    /// # Description
    ///     创建新的钱包仓储实例
    /// # Param
    ///     db: DatabaseConnection: 数据库连接
    /// # Return
    ///     WalletRepository: 钱包仓储实例
    pub async fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl WalletRepositoryInterface for WalletRepository {
    /// # Description
    ///     根据钱包ID查找钱包
    /// # Param
    ///     id: u64: 钱包的唯一标识符
    /// # Return
    ///     Result<WalletModel, Report>: 钱包实体或错误信息
    async fn find_by_id(&self, id: u64) -> Result<Option<WalletModel>, Report> {
        // 使用SeaORM的查询功能查找钱包
        let result = WalletEntity::find()
            // 使用完全限定语法消除歧义
            .filter(<WalletEntity as EntityTrait>::Column::Id.eq(id))
            .one(self.db.as_ref())
            .await;

        Ok(result?)
    }

    /// # Description
    ///     保存钱包实体
    /// # Param
    ///     wallet: &WalletModel: 钱包实体
    /// # Return
    ///     Result<(), Report>: 保存结果
    async fn save(&self, wallet: WalletActiveModel) -> Result<(), Report> {
        // 显式地指定 ActiveModel 类型
        let wallet_model: WalletActiveModel = wallet.into();

        // 保存用户数据
        wallet_model.save(self.db.as_ref()).await?;

        Ok(())
    }
}
