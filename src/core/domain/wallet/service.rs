use std::sync::Arc;
use color_eyre::{Report, Result};
use tracing::{error, info};
use crate::core::domain::wallet::repository::WalletRepositoryInterface;
// use std::net::SocketAddr;
use chrono::Utc;
use sea_orm::{ActiveValue, IntoActiveModel, NotSet};
// use solana_client::rpc_client::RpcClient;
use super::entity::Address;
use super::entity::{ActiveModel as WalletActiveModel, Model as WalletModel};

/// # Description
///     钱包服务
/// # Fields
///     repository: Arc<dyn WalletRepositoryInterface>: 钱包仓储接口的引用
pub struct WalletService {
    repository: Arc<dyn WalletRepositoryInterface>,
}

impl WalletService {
    /// # Description
    ///     创建新的钱包服务
    /// # Param
    ///     repository: Arc<dyn WalletRepositoryInterface>: 钱包仓储接口的引用
    /// # Return
    ///     WalletService: 钱包服务实例
    pub fn new(repository: Arc<dyn WalletRepositoryInterface>) -> Self {
        Self { repository }
    }


    /// # Description
    ///     生成新钱包
    /// # Param
    ///     None
    /// # Return
    ///     WalletService: 钱包服务实例
    pub async fn generation_wallet(&self) -> Result<(), String> {
        // let rpc_addr: String = Address::MainNet.into();
        // let rpc_client = RpcClient::new(rpc_addr);

        let get_now_utc = Utc::now();

        let new_wallet = WalletActiveModel {
            id: NotSet,
            user_id: ActiveValue::set(102),  // i8 类型
            pub_key: ActiveValue::set(Some("公钥".to_owned())),  // Option<String> 类型
            privy_key: ActiveValue::set(Some("私钥".to_owned())),  // Option<String> 类型
            balance: ActiveValue::set(1.2),  // f64 类型
            disable: ActiveValue::set(false),  // bool 类型
            created_at: ActiveValue::set(get_now_utc),  // DateTimeUtc 类型
            updated_at: ActiveValue::set(get_now_utc),  // DateTimeUtc 类型
            deleted_at: Default::default(),  // Option<DateTime<Utc>> 类型
        };

        // info!("选择网络: {}", format!("选择网络: {:?}", rpc_addr));
        info!("钱包生成成功: {}", format!("钱包生成成功: {:?}", new_wallet));

        match self.repository.save(new_wallet).await {
            Ok(_) => {
                // info!("钱包生成成功: {}", format!("选择网络: {:?}", rpc_addr.clone()));
                Ok(())
            }
            Err(e) => {
                error!("钱包生成失败: {:?}", e);
                Err(format!("钱包生成失败: {}", e))
            },
        }
    }


    /// # Description
    ///     查询钱包金额
    /// # Param
    ///     pub_key: u64: 钱包公钥
    /// # Return
    ///     Result<(), String>: 处理结果
    pub fn query_wallet_amount() -> Result<()> {
        info!("查询钱包金额");

        // let mut wallet = Wallet::default();
        // wallet.pub_key = String::from("DdSkP7zTe3FDECZnrRiUguPu8ityrx1kBVoNZR4HA4nT");
        //
        // if let Err(err) = wallet.get_balance(&rpc_client) { eprintln!("{}", err); }
        //
        // let balance = wallet.balance_convert_sol();
        // println!("+[Wallet] {:?} Balance in SOL: {:?}", wallet.pub_key, balance);
        Ok(())
    }


    /// # Description
    ///     为钱包添加金额
    /// # Param
    ///     id: u64: 钱包的唯一标识符
    ///     amount: f64: 要添加的金额
    /// # Return
    ///     Result<(), String>: 处理结果
    pub async fn deposit(&self, id: u64, amount: f64) -> Result<(), Report> {
        let mut wallet: Option<WalletModel> = self.repository.find_by_id(id).await?;

        info!("wallet: {:?}", wallet);

        // wallet.unwrap().update_balance(amount).map_err(|e| Report::msg(e))?;

        // 将 WalletModel 转换为 WalletActiveModel
        let wallet_model =  wallet.unwrap().into_active_model();

        self.repository.save(wallet_model).await?;

        Ok(())
    }

    /// # Description
    ///     从钱包中取出金额
    /// # Param
    ///     id: u64: 钱包的唯一标识符
    ///     amount: f64: 要取出的金额
    /// # Return
    ///     Result<(), String>: 处理结果
    pub async fn withdraw(&self, id: u64, amount: f64) -> Result<()> {
        self.deposit(id, -amount).await?;

        Ok(())
    }
}
