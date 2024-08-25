use std::io;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::{Report, Result};
use sea_orm::IntoActiveModel;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_response::RpcVersionInfo;
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
use solana_transaction_status::{EncodedConfirmedBlock, UiTransactionEncoding};

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
    ///     client: &RpcClient - RPC 客户端实例
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息
    pub async fn get_cluster_info(client: &RpcClient) -> Result<RpcVersionInfo, Report> {
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

        Ok(version)
    }


    /// # Description
    ///     获取 Solana 的总供应量和流通供应量
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
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
    ///     client: &RpcClient - RPC 客户端实例
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
    ///     client: &RpcClient - RPC 客户端实例
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
    ///     client: &RpcClient - RPC 客户端实例
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


    /// # Description
    ///     获取 Solana 区块和交易数量
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
    ///     block_num: u64 - 区块数
    /// # Return
    ///     Result<EncodedConfirmedBlock, Report>: 成功时返回Ok(EncodedConfirmedBlock)，失败时返回错误信息。
    pub async fn get_block(client: &RpcClient, block_num: u64) -> Result<EncodedConfirmedBlock, Report>  {
        println!("Getting block number: {}", block_num);

        // 交易编码为 Base 64 (因为 Base 64 编码可以保存任何受支持大小的帐户信息)
        // 将支持的最大交易版本设置为0（否则，我们将遇到不受支持的版本的错误）
        let config = RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::Base64),
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        // 使用此配置对象通过调用来获取 client.get_block_with_config
        // 返回有关账本中已确认区块的身份和交易信息
        let block = client.get_block_with_config(block_num, config).await?;

        // 将返回结果的块数据
        let encoded_block: EncodedConfirmedBlock = block.into();

        Ok(encoded_block)
    }


    /// # Description
    ///     统计 Solana 用户交易量
    /// # Params
    ///     block: &EncodedConfirmedBlock - 确认的交易块
    /// # Return
    ///     Result<u64, Report>: 成功时返回Ok(u64)即用户交易数量，失败时返回错误信息。
    pub async fn count_user_transactions(block: &EncodedConfirmedBlock) -> Result<u64, Report> {
        let mut user_transactions_count: u64 = 0;

        // 开始循环遍历所有交易
        for transaction_status in &block.transactions {
            // 解码这个对象
            // 该对象包含一个 message 字段，该字段包含 instructions 和 static_account_keys。
            let transaction = transaction_status.transaction.decode().unwrap();
            // 此 static_account_keys 数组包含交易中使用的程序 ID（程序公钥）
            let account_keys = transaction.message.static_account_keys();

            let mut num_vote_instructions = 0;

            // 循环遍历交易指令
            for instruction in transaction.message.instructions() {
                // 要确定指令中使用了哪个程序，我们使用 instruction.program_id_index 索引来 account_keys 获取程序 ID
                let program_id_index = instruction.program_id_index;
                let program_id = account_keys[usize::from(program_id_index)];

                // 使用 solana_sdk 的投票 id 与投票程序 id 进行比较
                if program_id == solana_sdk::vote::program::id() {
                    num_vote_instructions += 1;
                    println!("Found vote instruction");
                } else {
                    println!("non-vote instruction");
                }
            }

            // 检查投票指令的数量是否与总指令数量相同。如果不是的话则是用户交易
            if num_vote_instructions == transaction.message.instructions().len() {
                println!("It's a vote transaction");
            } else {
                println!("it's a user transaction");
                user_transactions_count += 1;
            }
        }

        // 确定了投票交易总数，方法是从交易总数中减去用户交易总数
        // 使用 checked_sub 来防止溢出
        let vote_transactions_count = block
            .transactions
            .len()
            .checked_sub(user_transactions_count as usize)
            .expect("underflow");

        println!("solana total txs: {}", block.transactions.len());
        println!("solana user txs: {}", user_transactions_count);
        println!("solana vote txs: {}", vote_transactions_count);

        // 返回用户交易的数量
        Ok(user_transactions_count)
    }


    /// # Description
    ///     计算 Solana 网络的每秒交易量（TPS）
    /// # Params
    ///     oldest_timestamp: i64 - 最早的区块时间戳（秒）
    ///     newest_timestamp: i64 - 最新的区块时间戳（秒）
    ///     transactions_count: u64 - 在时间范围内的总交易数量
    /// # Return
    ///     Result<u64, Report>: 成功时返回Ok(u64)即用户交易数量，失败时返回错误信息。
    pub async fn calculate_tps(oldest_timestamp: i64, newest_timestamp: i64, transactions_count: u64) -> Result<f64, Report> {
        // 最新时间戳 - 最旧的时间戳 = 已经过去的秒数
        // 计算时间差（秒），使用 saturating_sub 以避免溢出
        let total_seconds_diff = newest_timestamp.saturating_sub(oldest_timestamp);

        // 交易计数 / 已经过去的秒数 = 每秒交易量（TPS）
        let mut transactions_per_second = transactions_count as f64 / total_seconds_diff as f64;

        // 如果结果为 NaN 或无穷大，将其设置为 0.0
        if transactions_per_second.is_nan() || transactions_per_second.is_infinite() {
            transactions_per_second = 0.0;
        }

        // 返回计算的 TPS 值
        Ok(transactions_per_second)
    }


    /// # Description
    ///     计算在指定时间范围内 Solana 网络的平均每秒交易量（TPS）
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
    ///     threshold_seconds: i64 - 计算TPS的时间窗口（秒）
    /// # Return
    ///     Result<(), Report>: 成功时返回 `Ok(())`，失败时返回错误信息
    pub async fn calculate_for_range(client: &RpcClient, threshold_seconds: i64) -> Result<(), Report> {
        // 记录计算开始时间
        let calculation_start = Utc::now();

        // 获取当前区块号（slot）
        let latest_block_number = client.get_slot().await?;

        // 获取当前区块的信息
        let mut current_block = Self::get_block(&client, latest_block_number).await?;

        // 从当前区块获取最新的时间戳
        let newest_timestamp = current_block.block_time.unwrap();

        // 计算时间戳阈值，确定最早应该查询的时间（最新时间 - 阈值秒数 = 时间戳阈值）
        let timestamp_threshold = newest_timestamp.checked_sub(threshold_seconds).unwrap();

        // 统计总交易量，用来跟踪用户交易的总数
        let mut total_transactions_count: u64 = 0;

        // 循环获取之前的区块信息，直到达到时间戳阈值
        let oldest_timestamp = loop {
            // 获取上一个区块数
            let prev_block_number = current_block.parent_slot;

            // 获取上一个区块的信息
            let prev_block = Self::get_block(client, prev_block_number).await?;

            // 统计当前区块的用户交易量
            let transactions_count = WalletApplication::count_user_transactions(&current_block).await?;

            // 格式化当前区块的时间并输出
            let naive_datetime = NaiveDateTime::from_timestamp(current_block.block_time.unwrap(), 0);
            let utc_dt: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
            println!("Block time: {}", utc_dt.format("%Y-%m-%d %H:%M:%S"));

            // 累加用户交易量
            total_transactions_count = total_transactions_count.checked_add(transactions_count).expect("overflow");

            // 检查是否已达到时间戳阈值，如果条件为真，将跳出循环并返回前一个块的时间戳
            let prev_block_timestamp = prev_block.block_time.unwrap();
            if prev_block_timestamp <= timestamp_threshold {
                break prev_block_timestamp;
            }

            // 检查是否已到达链中的第一个块，如果条件为真，将跳出循环并返回前一个块的时间戳
            if prev_block.block_height.unwrap() == 0 {
                break prev_block_timestamp;
            }

            // 更新当前区块为前一个区块，继续循环
            current_block = prev_block;
        };

        // 计算并输出每秒交易量（TPS）
        let transactions_per_second = Self::calculate_tps(oldest_timestamp, newest_timestamp, total_transactions_count).await?;

        // 记录计算结束时间并计算耗时
        let calculation_end = Utc::now();
        let duration = calculation_end.signed_duration_since(calculation_start).to_std()?;

        println!("calculation took: {} seconds", duration.as_secs());
        println!("total transactions per second over period: {}", transactions_per_second);

        Ok(())
    }
}