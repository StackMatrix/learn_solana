use color_eyre::{Report, Result};
use tracing::info;
use super::entity::{ActiveModel as WalletActiveModel, Model as WalletModel};

/// # Description
///     钱包服务
/// # Fields
///     repository: Arc<dyn WalletRepositoryInterface>: 钱包仓储接口的引用
pub struct WalletDomainService {
    // repository: Arc<dyn WalletRepositoryInterface>,
}

impl WalletDomainService {
    /// # Description
    ///     创建新的钱包服务
    /// # Param
    ///     infrastructure_layer: Arc<InfrastructureLayer>: 基础设施层为领域层提供服务
    /// # Return
    ///     WalletService: 钱包服务实例
    pub fn new() -> Self {
        // let repository = infrastructure_layer.persistence.repository.wallet_repository.clone();
        // Self { repository }

        Self {}
    }


    /// # Description
    ///     生成新钱包
    /// # Param
    ///     None
    /// # Return
    ///     WalletService: 钱包服务实例
    pub fn generation_wallet(&self, user_id: i32, pub_key: String, privy_key: String) -> WalletActiveModel {
        WalletModel::new(
            user_id,
            pub_key,
            privy_key,
        )
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
    pub fn deposit(&self, mut wallet: WalletModel, amount: f64) -> Result<(), Report> {
        wallet.update_balance(amount).map_err(|_e| {Report::msg("更新金额失败")})?;

        Ok(())
    }

    // /// # Description
    // ///     从钱包中取出金额
    // /// # Param
    // ///     id: u64: 钱包的唯一标识符
    // ///     amount: f64: 要取出的金额
    // /// # Return
    // ///     Result<(), String>: 处理结果
    // pub async fn withdraw(&self, id: u64, amount: f64) -> Result<()> {
    //     self.deposit(id, -amount).await?;
    //
    //     Ok(())
    // }
}
