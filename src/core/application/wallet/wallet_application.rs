use std::error::Error;
use std::{io, thread};
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::{Report, Result};
use sea_orm::IntoActiveModel;
use solana_client::nonblocking::rpc_client::RpcClient;
use tracing::error;
use crate::core::domain::DomainLayer;
use crate::core::domain::wallet::repository::WalletRepositoryInterface;
use crate::core::infrastructure::InfrastructureLayer;
use solana_program;
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction::create_nonce_account;
use solana_program::{system_instruction, system_program, sysvar};
use solana_program::clock::Clock;
use solana_program::native_token::{lamports_to_sol, sol_to_lamports};
use solana_sdk::account::from_account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{keypair_from_seed, write_keypair_file, Keypair, Signer};
use solana_sdk::transaction::Transaction;
use crate::core::domain::wallet::entity::WalletAddress;

pub struct WalletApplication {
    domain_layer: Arc<DomainLayer>,
    infrastructure_layer: Arc<InfrastructureLayer>
}

impl WalletApplication {
    /// # Description
    ///     创建新的钱包应用服务实例
    /// # Params
    ///     infrastructure_layer: Arc<InfrastructureLayer> - 基础设施层的引用，用于获取共享服务如 JWT
    ///     domain_layer: Arc<DomainLayer> - 领域层的引用，用于获取用户领域服务
    /// # Return
    ///     Self: 返回一个新的 `WalletApplication` 实例
    pub fn new(infrastructure_layer: Arc<InfrastructureLayer>, domain_layer: Arc<DomainLayer>) -> Self {
        Self { domain_layer, infrastructure_layer }
    }


    /// # Description
    ///     生成钱包
    /// # Params
    ///     None
    /// # Return
    ///     Result<(), Report>: 成功返回 Ok() ，失败返回错误信息
    pub async fn generation_wallet(&self) -> Result<(), Report> {
        // 生成公钥
        let from_pubkey = Pubkey::new_unique();
        let nonce_pubkey = Pubkey::new_unique();
        let authorized = nonce_pubkey;
        let ixs = create_nonce_account(&from_pubkey, &nonce_pubkey, &authorized, 42);
        assert_eq!(ixs.len(), 2);
        let ix = &ixs[0];
        assert_eq!(ix.program_id, system_program::id());
        let pubkeys: Vec<_> = ix.accounts.iter().map(|am| am.pubkey).collect();
        assert!(pubkeys.contains(&from_pubkey));
        assert!(pubkeys.contains(&nonce_pubkey));

        // 调用领域服务生成钱包实体
        let new_wallet = self.domain_layer.wallet_domain.domain_service.generation_wallet(
            1,
            from_pubkey.to_string(),
            from_pubkey.to_string(),
        );

        // 调用仓库接口生成钱包实体
        match self.infrastructure_layer.persistence.repository.wallet_repository.save(new_wallet).await {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                error!("钱包生成失败: {:?}", e);
                Err(Report::msg(format!("钱包生成失败: {}", e)))
            },
        }
    }

    /// # Description
    ///     为钱包添加金额
    /// # Param
    ///     id: u64: 钱包的唯一标识符
    ///     amount: f64: 要添加的金额
    /// # Return
    ///     Result<(), String>: 处理结果
    pub async fn deposit(&self, id: u64, amount: f64) -> Result<(), Report> {
        let wallet = self.infrastructure_layer.persistence.repository.wallet_repository.find_by_id(id).await?;

        match wallet {
            None => {}
            Some(wallet) => {
                self.domain_layer.wallet_domain.domain_service.deposit(wallet.clone(), amount)?;
                // 将 WalletModel 转换为 WalletActiveModel
                let wallet_model =  wallet.clone().into_active_model();
                self.infrastructure_layer.persistence.repository.wallet_repository.save(wallet_model).await?;
            }
        }

        Ok(())
    }


    /// # Description
    ///     获取 Solana 集群的当前版本和时间信息
    /// # Params
    ///     client: RpcClient - RPC 客户端
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息
    pub async fn get_cluster_info(client: &RpcClient) -> Result<(), Report> {
        // 获取服务器正在运行的 Solana 版本
        let version = client.get_version().await?;

        // 获取集群日期，从 Clock 程序中获取数据
        // Clock 是系统账户用于存储Solana集群的时间信息，包括UNIX时间戳和slot信息
        // CommitmentConfig::finalized 用于确保获取的数据是已经被最终确认的
        let result = client
            .get_account_with_commitment(&sysvar::clock::id(), CommitmentConfig::finalized())
            .await?;

        // 将结果数据反序列化为 sysvar 或系统帐户，获得 UNIX 时间戳并获取 slot
        // Slot 是Solana区块链中的时间度量单位，用于表示区块生成的时间点
        let (slot, timestamp) = match result.value {
            Some(clock_account) => {
                let clock: Clock = from_account(&clock_account).unwrap();
                (result.context.slot, clock.unix_timestamp)
            }
            None => {
                panic!("Unexpected None");
            }
        };

        // 转换可读时间戳
        let datetime = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap(),
            Utc,
        );

        // 打印当前服务器正在运行的 Solana 集群版本
        println!("Cluster version: {}", version.solana_core);

        // 打印当前块的槽位和时间
        println!(
            "Block: {}, Time: {}",
            slot,
            datetime.format("%Y-%m-%d %H:%M:%S")
        );

        Ok(())
    }


    /// # Description
    ///     获取 Solana 的总供应量和流通供应量
    /// # Params
    ///     client: RpcClient - RPC 客户端
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn get_supply(client: &RpcClient) -> Result<(), Report> {
        let supply_response = client.supply().await?;
        let supply = supply_response.value;

        println!(
            "Total supply: {} SOL\nCirculating: {} SOL\nNon-Circulating: {} SOL",
            lamports_to_sol(supply.total),
            lamports_to_sol(supply.circulating),
            lamports_to_sol(supply.non_circulating)
        );

        Ok(())
    }

    /// # Description
    ///     生成 Solana 密钥对
    /// # Params
    ///     output_path: &str - 保存文件名
    ///     mnemonic_word_count: usize - 助记词个数
    ///     passphrase: &Option<String> - 密码
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn generate_keypair(output_path: &str, mnemonic_word_count: usize, passphrase: &Option<String>) -> Result<(), Report> {
        // 助记词个数类型
        let mnemonic_type = MnemonicType::for_word_count(mnemonic_word_count).map_err(|e|{Report::msg(e.to_string())})?;

        // 除英语外，还支持其他语言，我们可以将其作为 KeyGen 命令的一个选项。但为了简单起见，我们在这里硬编码了英语。
        let mnemonic = Mnemonic::new(mnemonic_type, Language::English);

        // 检查是否有密码,根据密码是否为空生成种子
        let seed = match passphrase {
            Some(phrase) => Seed::new(&mnemonic, phrase),
            None => Seed::new(&mnemonic, ""),
        };

        // 调用 keypair_from_seed 并必须将种子作为字节数组传入以获取 Keypair 对象
        let keypair = keypair_from_seed(seed.as_bytes()).map_err(|e|{Report::msg(e.to_string())})?;

        // 将密钥对写入指定的文件，用于 write_keypair_file 存储密钥对以供以后使用
        write_keypair_file(&keypair, output_path).map_err(|e|{Report::msg(e.to_string())})?;

        println!("Mnemonic: {:?}", mnemonic);
        println!("Public key: {}", &keypair.pubkey());

        Ok(())
    }


    /// # Description
    ///     获取 Solana 钱包金额
    /// # Params
    ///     address: &str - 钱包地址
    ///     client: &RpcClient - RPC 客户端
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn get_balance(address: &str, client: &RpcClient) -> Result<(), Report> {
        // 将地址转为 Pubkey 对象
        let pub_key = Pubkey::from_str(address)?;
        // 获取余额
        let balance = client.get_balance(&pub_key).await?;

        println!("Balance for {}: {}", address, lamports_to_sol(balance));

        Ok(())
    }


    /// # Description
    ///     为 Solana 钱包请求空投，只能在测试和开发网络中进行
    /// # Params
    ///     address: &str - 要空投的钱包地址
    ///     sol: f64 - Solana 金额
    ///     client: &RpcClient - RPC 客户端
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn airdrop_sol(address: &str, sol: f64, client: &RpcClient) -> Result<(), Report>  {
        // 将 Sol 值转换为 Lamports
        let lamports = sol_to_lamports(sol);

        // 将地址转为 Pubkey 对象
        let pub_key = Pubkey::from_str(address)?;

        // 为该钱包请求空投，这将发送请求但不会等待确认
        let signature = client.request_airdrop(&pub_key, lamports).await?;

        // 等待请求空投操作
        let wait_milis = Duration::from_millis(100);
        print!("Waiting to confirm");
        io::stdout().flush()?;

        // 检查交易是否成功
        loop {
            if let Ok(confirmed) = client.confirm_transaction(&signature).await {
                if confirmed {
                    println!("\nAirdrop to {}: {}", address, confirmed);
                    break;
                }
            }
            print!(".");
            io::stdout().flush()?;
            tokio::time::sleep(wait_milis).await;
        }

        Ok(())
    }


    /// # Description
    ///     将 Solana 钱包的资金转移到另一个地址
    /// # Params
    ///     client: &RpcClient - RPC 客户端
    ///     keypair: &Keypair - 密钥对
    ///     to_key: &str - 要转移的地址
    ///     sol_amount: f64 - 要转移的金额
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn transfer_sol(client: &RpcClient, keypair: &Keypair, to_key: &str, sol_amount: f64) -> Result<(), Report> {
        // 将地址转为 Pubkey 对象
        let to_pubkey = Pubkey::from_str(to_key)?;

        // 将 Sol 值转换为 Lamports
        let lamports = sol_to_lamports(sol_amount);

        // 通过系统程序的创建转账指令进行交易
        let transfer_instruction = system_instruction::transfer(&keypair.pubkey(), &to_pubkey, lamports);

        // 获取最新的区块哈希
        let latest_blockhash = client.get_latest_blockhash().await?;

        // 签署交易需要密钥对和最新的区块哈希
        // 注：由于转账交易会产生相关成本，故必须交易行为进行买单
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&keypair.pubkey()),
            &[keypair],
            latest_blockhash,
        );

        // 等待请求获取最后一个区块链 hash 操作
        let wait_milis = Duration::from_millis(100);
        print!("Waiting to confirm");
        io::stdout().flush()?;

        // 发送交易
        match client.send_transaction(&transaction).await {
            Ok(signature) => loop {
                // 等待交易被确认
                if let Ok(confirmed) = client.confirm_transaction(&signature).await {
                    if confirmed {
                        println!("\nTransfer of sol was confirmed");
                        break;
                    }
                }
                print!(".");
                io::stdout().flush()?;
                tokio::time::sleep(wait_milis).await;
            },
            Err(e) => {
                println!("Error transferring sol: {}", e);
            }
        }

        Ok(())
    }
}