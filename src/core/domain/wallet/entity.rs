use chrono::{DateTime, Utc};
use color_eyre::Report;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DerivePrimaryKey, DeriveRelation, EnumIter, PrimaryKeyTrait};
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

/// # Description
///     该枚举代表钱包地址。
/// # Param
///     DevNet: https://api.devnet.solana.com 开发网
///     TestNet: https://api.testnet.solana.com 测试网
///     MainNet: http://api.mainnet-beta.solana.com 主网
///     CustomPpc: 自定义 RPC
pub enum Address {
    DevNet,
    TestNet,
    MainNet,
    CustomPpc
}

impl From<Address> for String {
    fn from(platform: Address) -> Self {
        match platform {
            Address::DevNet => "https://api.devnet.solana.com".into(),
            Address::TestNet => "https://api.testnet.solana.com".into(),
            Address::MainNet => "http://api.mainnet-beta.solana.com".into(),
            Address::CustomPpc => platform.into(),
        }
    }
}

/// # Description
///     该结构体代表钱包实体，并映射到数据库中的 `wallet` 表。
/// # Param
///     id: 主键，自动递增
///     user_id: 钱包拥有者，关联 user 表
///     pub_key: 钱包公钥
///     privy_key: 钱包助记词
///     balance: 钱包余额
///     disable: 钱包禁用状态
///     created_at: 创建时间
///     updated_at: 更新时间
///     deleted_at: 删除时间（软删除）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wallet")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i8,
    pub pub_key: Option<String>,
    pub privy_key: Option<String>,
    pub balance: f64,
    pub disable: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Model {
    /// # Description
    ///     创建新的钱包实体
    /// # Param
    ///     None
    /// # Return
    ///     Result<(), Report>
    pub fn new() -> Result<(), Report>{

        Ok(())
    }

    /// # Description
    ///     更新钱包余额
    /// # Param
    ///     amount: f64: 变动的金额
    /// # Return
    ///     Result<(), String>: 更新结果
    pub fn update_balance(&mut self, amount: f64) -> Result<(), String> {
        if self.balance + amount < 0.0 {
            Err("余额不足".into())
        } else {
            self.balance += amount;
            Ok(())
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

