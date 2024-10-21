use std::{
    io::{self, Write},
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use std::collections::BTreeMap;
use std::error::Error;
use bincode::deserialize; // 导入 bincode 反序列化函数
use axum::http;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use chrono::{DateTime, NaiveDateTime, Utc};
use color_eyre::{Report, Result};
use reqwest::StatusCode;
use sea_orm::IntoActiveModel;
use serde::{Deserialize, Serialize};
use serum_dex::state::MarketState;
use tracing::error;
use spl_token::instruction as token_instruction;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_client::rpc_response::RpcVersionInfo;
use spl_token::instruction::{initialize_account, initialize_mint, transfer as spl_transfer};
use solana_transaction_status::{
    EncodedConfirmedBlock,
    UiTransactionEncoding
};
use spl_associated_token_account::{
    create_associated_token_account,
    get_associated_token_address,
    processor::process_instruction
};
use solana_sdk::{
    account::from_account,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
    signature::{keypair_from_seed, write_keypair_file, Signer, Keypair}
};
use solana_program::{
    system_instruction::{create_nonce_account, transfer},
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar,
    program::invoke,
    account_info::AccountInfo,
    native_token::{lamports_to_sol, sol_to_lamports},
    clock::Clock,
    system_instruction
};
use solana_program::program_pack::Pack;
use solana_program::pubkey::ParsePubkeyError;
use solana_sdk::account::Account;
use solana_sdk::transaction::VersionedTransaction;
use spl_token_swap::instruction::{SwapInstruction, Swap, swap};
use spl_token_swap::solana_program::pubkey;
use crate::core::domain::DomainLayer;
use crate::core::domain::wallet::repository::WalletRepositoryInterface;
use crate::core::infrastructure::InfrastructureLayer;


pub struct WalletApplication {
    domain_layer: Arc<DomainLayer>,
    infrastructure_layer: Arc<InfrastructureLayer>
}

/// Solana 钱包基础功能
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
    ///     获取 Solana 金额
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
    ///     获取账号信息
    /// # Params
    ///     address: &str - 钱包地址
    ///     client: &RpcClient - RPC 客户端实例
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn get_account_info(client: &RpcClient, pub_key: &Pubkey) -> Result<Account, Report> {
        // 获取账号信息
        let account = client.get_account(&pub_key).await?;

        Ok(account)
    }

    /// Make a call to the raydium api endpoint to retrieve all liquidity pools.
    pub async fn get_all_liquidity_pools() -> Result<(), Report> {
        let response = reqwest::get("https://api.raydium.io/v2/sdk/liquidity/mainnet.json")
            .await?
            .json()
            .await?;

        println!("get_all_liquidity_pools： {:?}", response);

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
    pub async fn airdrop_sol(address: &str, sol: f64, client: &solana_client::rpc_client::RpcClient) -> Result<(), Report>  {
        // 将 Sol 值转换为 Lamports
        let lamports = sol_to_lamports(sol);

        // 将地址转为 Pubkey 对象
        let pub_key = Pubkey::from_str(address)?;

        // 为该钱包请求空投，这将发送请求但不会等待确认
        let signature = client.request_airdrop(&pub_key, lamports)?;

        // 等待请求空投操作
        let wait_milis = Duration::from_millis(100);
        print!("Waiting to confirm");
        io::stdout().flush()?;

        // 检查交易是否成功
        loop {
            if let Ok(confirmed) = client.confirm_transaction(&signature) {
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


    /// 创建账户
    pub async fn create_account(client: &solana_client::rpc_client::RpcClient, fee_payer: &Keypair, new_account: &Keypair, ) -> Result<(), Report> {
        // Specify account data length
        let space = 0;
        // Get minimum balance required to make an account with specified data length rent exempt
        let rent_exemption_amount = client
            .get_minimum_balance_for_rent_exemption(space)?;

        // Create instruction to create an account
        let create_account_ix = system_instruction::create_account(
            &fee_payer.pubkey(),
            &new_account.pubkey(),
            rent_exemption_amount,
            space as u64,
            &fee_payer.pubkey(),
        );

        // Get recent blockhash
        let recent_blockhash = client.get_latest_blockhash()?;
        // Create transaction to create an account
        let create_account_tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
            &[create_account_ix],
            Some(&fee_payer.pubkey()),
            &[&fee_payer, &new_account],
            recent_blockhash,
        );

        // Submit a transaction to create an account and wait for confirmation
        let create_account_tx_signature = client
            .send_and_confirm_transaction(&create_account_tx)?;

        // Print transaction signature and account address
        println!("Transaction signature: {create_account_tx_signature}");
        println!("New account {} created successfully", new_account.pubkey());

        Ok(())
    }

}



/// Solana 代币功能
impl WalletApplication {
    /// # Description
    ///     获取 USDT 的市场价格
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
    ///     buyer_keypair: &Keypair - 买家的密钥对
    ///     source_account_pubkey: &Pubkey - SOL 来源账户的公钥
    ///     usdt_mint_pubkey: &Pubkey - USDT 的 Mint 公钥
    ///     usdt_account_pubkey: &Pubkey - 目标 USDT 账户的公钥
    ///     pool_swap_pubkey: &Pubkey - 交换池的公钥
    ///     pool_authority_pubkey: &Pubkey - 交换池的授权公钥
    ///     pool_token_a_account_pubkey: &Pubkey - 交换池中 SOL 的代币账户公钥
    ///     pool_token_b_account_pubkey: &Pubkey - 交换池中 USDT 的代币账户公钥
    ///     sol_amount: u64 - 兑换的 SOL 数量（以 lamports 为单位）
    /// # Return
    ///     Result<(), Report>: 成功时返回 Ok()，失败时返回错误信息。
    pub async fn get_market_price() -> Result<()>  {
        let url = "https://min-api.cryptocompare.com/data/pricemulti?fsyms=SOL,USD&tsyms=USD,SOL";
        let response = reqwest::get(url).await?; // 发送 GET 请求

        let parsed = response.json::<serde_json::Value>().await?;

        // 提取 SOL/USD 的汇率
        if let Some(sol_usd_rate) = parsed["SOL"]["USD"].as_f64() {
            println!("1 SOL can be exchanged for {} USD", sol_usd_rate);
        } else {
            println!("Could not find the exchange rate for SOL/USD");
        }

        Ok(())
    }

    pub async fn get_all_token_price() -> Result<()> {
        let price_info_result: serde_json::Value = reqwest::get("https://api.raydium.io/v2/main/price")
            .await?
            .json()
            .await?;

        println!("{:?}", price_info_result);

        Ok(())
    }

    pub async fn get_token_price(mints: &str) -> Result<()> {
        // let url = format!("https://api-v3.raydium.io/mint/price?mints={}", mints);
        // let price_info_result: serde_json::Value = reqwest::get(url)
        //     .await?
        //     .json()
        //     .await?;
        //
        // println!("{:#?}", price_info_result["data"]);

        let client = reqwest::Client::new();
        let swap_url = format!(
            "https://api.raydium.io/v2/sdk/swap?inputMint={}&outputMint={}&amount={}&slippage={}",
            "So11111111111111111111111111111111111111112",  // SOL Mint
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",  // USDT Mint
            0.1, // 兑换的 SOL 数量
            1 // 允许的滑点
        );
        let response = client.get(&swap_url).send().await?;
        let swap_response: SwapResponse = serde_json::from_str(&response.text().await?)?;

        Ok(())
    }


    pub async fn transfer_usdt(
        connection: &RpcClient,
        buyer_keypair: &Keypair,
        source_usdt_account: &Pubkey,   // 来源账户（您的 USDT 代币账户）
        recipient_usdt_account: &Pubkey, // 目标账户（接收 USDT 的账户）
        usdt_amount: u64,              // 转账数量（USDT的最小单位）
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 获取最新的区块哈希
        let blockhash = connection.get_latest_blockhash().await?;

        // 创建 USDT 转账指令
        let transfer_instruction = token_instruction::transfer(
            &spl_token::id(),
            source_usdt_account,          // 您的 USDT 代币账户
            recipient_usdt_account,       // 接收 USDT 的代币账户
            &buyer_keypair.pubkey(),      // 授权签名者
            &[],                          // 其他签名者（如果有）
            usdt_amount,                  // 转账数量，按最小单位
        )?;

        // 创建交易并签名
        let mut tx = Transaction::new_with_payer(
            &[transfer_instruction],
            Some(&buyer_keypair.pubkey()),
        );
        tx.sign(&[buyer_keypair], blockhash);

        // 发送并确认代币转账交易
        let transfer_signature = connection.send_and_confirm_transaction(&tx).await?;
        println!("USDT transfer confirmed: {}", transfer_signature);

        Ok(())
    }

    /// # Description
    ///     通过 solxtence API 进行代币交换
    /// # Params
    ///     rpc_url: &str - RPC 节点的 URL
    ///     buyer_keypair: &Keypair - 买家的密钥对
    ///     sol_mint_pubkey: &Pubkey - SOL 的 Mint 公钥
    ///     usdt_mint_pubkey: &Pubkey - USDT 的 Mint 公钥
    ///     sol_amount: f64 - 兑换的 SOL 数量
    ///     slip: f64 - 允许的滑点
    ///     recipient_usdt_account: USDT 代币账户
    /// # Return
    ///     Result<(), Box<dyn std::error::Error>> - 成功时返回 Ok()，失败时返回错误信息。
    pub async fn perform_swap(
        connection: &RpcClient,
        buyer_keypair: &Keypair,
        sol_mint_pubkey: &Pubkey,
        usdt_mint_pubkey: &Pubkey,
        sol_amount: f64,
        slip: f64,
        recipient_usdt_account: &Pubkey,
    ) -> Result<(), Box<dyn Error>> {
        // 定义交易参数
        let params = vec![
            ("from", sol_mint_pubkey.to_string()),
            ("to", usdt_mint_pubkey.to_string()),
            ("amount", sol_amount.to_string()), // 输入 SOL 的数量
            ("slip", slip.to_string()),         // 滑点
            ("payer", buyer_keypair.pubkey().to_string()), // 付款方地址
            ("fee", "0.00009".to_string()),     // 优先费用
            ("txType", "v0".to_string()),       // 交易版本
        ];

        // 使用 Reqwest 发出 GET 请求获取 swap 交易信息
        let client = reqwest::Client::new();
        let swap_url = format!(
            "https://swap.solxtence.com/swap?{}",
            serde_urlencoded::to_string(&params)?
        );
        let response_data = client.get(&swap_url).send().await?.text().await?;
        println!("response_data. {:#?}", response_data);

        // 解析响应
        let swap_response: SwapResponse = serde_json::from_str(&*response_data)?;
        println!("response. {:#?}", swap_response);

        // 获取最新的区块哈希
        let blockhash = connection.get_latest_blockhash().await?;

        // 定义 transaction 为 VersionedTransaction 类型
        let mut transaction: VersionedTransaction;
        // 反序列化交易并签署
        if swap_response.transaction.tx_type == "v0" {
            // `v0` 交易使用 `VersionedTransaction`
            let serialized_tx = &swap_response.transaction.serialized_tx;

            // 解码并反序列化
            let decoded_tx = base64::decode(serialized_tx)?;  // 将 Base64 解码后的字节存储为 Vec<u8>
            let mut versioned_transaction: VersionedTransaction = bincode::deserialize(decoded_tx.as_slice())?;  // 使用 bincode 反序列化

            // 设置 recent_blockhash
            versioned_transaction.message.set_recent_blockhash(blockhash);

            // 签署交易
            // 使用 `sign` 签署交易, 传递一个包含买家 keypair 的数组
            let signature = buyer_keypair.try_sign_message(&versioned_transaction.message.serialize()).unwrap();
            versioned_transaction.signatures = vec![signature];

            // 发送并确认交易
            let signature_result = connection.send_and_confirm_transaction(&versioned_transaction).await?;
            println!("Transaction confirmed: {}", signature_result);

            // 向 USDT 账户转账
            // let transfer_instruction = token_instruction::transfer(
            //     &spl_token::id(),
            //     &usdt_mint_pubkey,                      // 来源账户（USDT 池的代币账户）
            //     recipient_usdt_account,                 // 目标账户（你的 USDT 代币账户）
            //     &buyer_keypair.pubkey(),                // 授权签名者
            //     &[],                      // 任何其他签名者
            //     sol_amount as u64,                      // 转账数量
            // )?;
            //
            // // 创建交易
            // let mut tx = Transaction::new_with_payer(
            //     &[transfer_instruction],
            //     Some(&buyer_keypair.pubkey()),
            // );
            // tx.sign(&[buyer_keypair], blockhash);
            //
            // // 发送并确认代币转账交易
            // let transfer_signature = connection.send_and_confirm_transaction(&tx).await?;
            // println!("Transfer transaction confirmed: {}", transfer_signature);
        }

        Ok(())
    }

    pub async fn swap_sol_to_usdt_raydium(
        client: &RpcClient,
        buyer_keypair: &Keypair,
        pool_pubkey: &Pubkey, // Raydium 池的公钥
        source_account_pubkey: &Pubkey, // 用户 SOL 账户
        destination_account_pubkey: &Pubkey, // 用户 USDT 账户
        sol_amount: f64, // 交换的 SOL 数量
    ) -> Result<()> {
        // 1. 将 SOL 转换为 lamports
        let lamports = sol_to_lamports(sol_amount);

        // 2. 获取最新的区块哈希
        let latest_blockhash = client.get_latest_blockhash().await?;

        // 获取报价
        let response = reqwest::get("https://api-v3.raydium.io/compute")
            .await?
            .json()
            .await?;

        // 3. 构建 Raydium Swap 指令
        // 注意：这只是一个示例，你需要根据 Raydium 的智能合约结构构建实际的交换指令。
        let swap_instruction = Instruction::new_with_bincode(
            *pool_pubkey,  // 使用 Raydium 交换池的程序 ID
            &lamports, // 指令数据
            vec![
                AccountMeta::new(*source_account_pubkey, true), // 用户的 SOL 账户
                AccountMeta::new(*destination_account_pubkey, false), // 用户的 USDT 账户
                // AccountMeta::new_readonly(*pool_pubkey, false), // Raydium 池的账户
                // 其他必要的账户，比如流动性池中的 tokenA, tokenB 账户等
            ],
        );

        // 4. 创建并签署交易
        let mut transaction = Transaction::new_with_payer(
            &[swap_instruction],
            Some(&buyer_keypair.pubkey()),
        );
        println!("Transaction: {:?}", transaction);


        transaction.sign(&[buyer_keypair], latest_blockhash);

        // 5. 发送交易并等待确认
        let signature = client.send_and_confirm_transaction(&transaction).await?;

        println!("signature: {:#?}", signature);

        Ok(())
    }


    /// # Description
    ///     使用 SOL 通过 spl_token_swap 程序交换 USDT
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
    ///     buyer_keypair: &Keypair - 买家的密钥对
    ///     source_account_pubkey: &Pubkey - SOL 来源账户的公钥
    ///     usdt_mint_pubkey: &Pubkey - USDT 的 Mint 公钥
    ///     usdt_account_pubkey: &Pubkey - 目标 USDT 账户的公钥
    ///     pool_swap_pubkey: &Pubkey - 交换池的公钥
    ///     pool_authority_pubkey: &Pubkey - 交换池的授权公钥
    ///     pool_token_a_account_pubkey: &Pubkey - 交换池中 SOL 的代币账户公钥
    ///     pool_token_b_account_pubkey: &Pubkey - 交换池中 USDT 的代币账户公钥
    ///     sol_amount: u64 - 兑换的 SOL 数量（以 lamports 为单位）
    /// # Return
    ///     Result<(), Report>: 成功时返回 Ok()，失败时返回错误信息。
    pub async fn swap_sol_to_usdt(
        client: &RpcClient,
        buyer_keypair: &Keypair,
        source_account_pubkey: &pubkey::Pubkey,
        usdt_mint_pubkey: &pubkey::Pubkey,
        usdt_account_pubkey: &pubkey::Pubkey,
        pool_swap_pubkey: &pubkey::Pubkey,
        pool_authority_pubkey: &pubkey::Pubkey,
        pool_token_a_account_pubkey: &pubkey::Pubkey,
        pool_token_b_account_pubkey: &pubkey::Pubkey,
        pool_mint_pubkey: &pubkey::Pubkey,
        pool_fee_pubkey: &pubkey::Pubkey,
        sol_amount: u64,
    ) -> Result<()> {
        // // 1. 获取最新的区块哈希
        // let latest_blockhash = client.get_latest_blockhash()?;
        //
        // // 2. 创建 Swap 指令
        // let swap_instruction = swap(
        //     &spl_token_swap::id(), // SPL Token Swap 程序 ID
        //     &spl_token::id(), // SPL Token 程序 ID
        //     pool_swap_pubkey, // 交换池的公钥
        //     pool_authority_pubkey, // 交换池的授权公钥
        //     &buyer_keypair.pubkey(), // 用户转账授权公钥
        //     source_account_pubkey, // 用户 SOL 账户的公钥
        //     pool_token_a_account_pubkey, // 交换池中 SOL 的代币账户公钥
        //     pool_token_b_account_pubkey, // 交换池中 USDT 的代币账户公钥
        //     usdt_account_pubkey, // 用户的 USDT 账户的公钥
        //     pool_mint_pubkey, // 交换池的 Mint 账户公钥
        //     pool_fee_pubkey, // 费用账户的公钥
        //     None, // 没有主机费用账户
        //     Swap {
        //         amount_in: sol_amount, // 用户输入的 SOL 数量
        //         minimum_amount_out: 1, // 最小接收的 USDT 数量，防止滑点
        //     },
        // )?;
        //
        // // 3. 创建并签署交易
        // let mut transaction = Transaction::new_with_payer(
        //     &[swap_instruction],
        //     Some(&buyer_keypair.pubkey()),
        // );
        // transaction.sign(&[buyer_keypair], latest_blockhash);
        //
        // // 4. 发送交易并等待确认
        // let signature = client.send_and_confirm_transaction(&transaction)?;

        Ok(())
    }


    /// # Description
    ///     使用 SOL 购买 USDT
    /// # Params
    ///     client: &RpcClient - RPC 客户端实例
    ///     buyer_keypair: &Keypair - 买家的密钥对
    ///     source_account_pubkey: &Pubkey - 来源账户的公钥
    ///     buyer_usdt_account_pubkey: &Pubkey - 目标账户的公钥
    ///     sol_amount: f64 - 兑换的 USDT 数量
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn buy_usdt_with_sol(client: &RpcClient, buyer_keypair: &Keypair, source_account_pubkey: &Pubkey, destination_account_pubkey: &Pubkey, sol_amount: f64, ) -> Result<(), Report> {
        // 1. 将 SOL 转换为 lamports
        let lamports = sol_to_lamports(sol_amount);

        // 2. 获取最新的区块哈希
        let latest_blockhash = client.get_latest_blockhash().await?;

        // 3. 生成交易指令，将SOL发送到交换池
        let swap_instruction = transfer(
            &buyer_keypair.pubkey(),
            &Pubkey::from_str("CYbD9RaToYMtWKA7QZyoLahnHdWq553Vm62Lh6qWtuxq")?, // Raydium 的 program 地址
            lamports,
        );

        // 4. 创建并签署交易
        let mut transaction = Transaction::new_with_payer(
            &[swap_instruction],
            Some(&buyer_keypair.pubkey()),
        );
        transaction.try_sign(&[buyer_keypair], latest_blockhash)?;

        // 5. 发送交易并等待确认
        let signature = client.send_transaction(&transaction).await?;
        client.confirm_transaction(&signature).await?;

        // 6. 生成USDT转账指令
        let usdt_transfer_instruction = spl_transfer(
            &spl_token::id(), // SPL Token 程序ID
            &source_account_pubkey, // 来源账户公钥 (USDT的Mint公钥)
            &destination_account_pubkey, // 目标账户公钥 (买家的USDT账户)
            &buyer_keypair.pubkey(), // 授权者公钥 (买家账户的所有者)
            &[], // 任何授权者的公钥（如果有）
            lamports, // 转账数量，单位是最小单位的数量（可能是6个小数位)
        )?;

        // 7. 再次创建交易并签名
        let mut transaction = Transaction::new_with_payer(
            &[usdt_transfer_instruction],
            Some(&buyer_keypair.pubkey()),
        );
        transaction.try_sign(&[buyer_keypair], latest_blockhash)?;

        // 8. 发送交易并等待确认
        let signature = client.send_transaction(&transaction).await?;
        client.confirm_transaction(&signature).await?;

        Ok(())
    }

    /// # Description
    ///     通过 spl_token_swap 在 Solana 区块链上交换两种代币。
    /// # Params
    ///     program_id: &Pubkey - spl_token_swap 程序的公钥
    ///     token_program_id: &Pubkey - SPL 代币程序的公钥
    ///     swap_pubkey: &Pubkey - 代币交换池的公钥
    ///     authority_pubkey: &Pubkey - 授权交换操作的公钥
    ///     user_transfer_authority_pubkey: &Pubkey - 用户的转账授权公钥
    ///     source_pubkey: &Pubkey - 用户持有的源代币账户的公钥
    ///     swap_source_pubkey: &Pubkey - 交换池中源代币账户的公钥
    ///     swap_destination_pubkey: &Pubkey - 交换池中目标代币账户的公钥
    ///     destination_pubkey: &Pubkey - 用户希望接收目标代币的账户的公钥
    ///     pool_mint_pubkey: &Pubkey - 交换池中用于生成交易费用的代币账户的公钥
    ///     pool_fee_pubkey: &Pubkey - 交易费用接收账户的公钥
    ///     host_fee_pubkey: Option<&Pubkey> - 可选的主机费用账户的公钥
    ///     amount_in: u64 - 用户希望交换的源代币数量
    ///     minimum_amount_out: u64 - 用户希望最少接收的目标代币数量，防止滑点过大
    /// # Return
    ///     Result<(), ProgramError>: 成功时返回 Ok()，失败时返回 ProgramError。
    pub async fn swap_tokens(
        program_id: &Pubkey,
        token_program_id: &Pubkey,
        swap_pubkey: &Pubkey,
        authority_pubkey: &Pubkey,
        user_transfer_authority_pubkey: &Pubkey,
        source_pubkey: &Pubkey,
        swap_source_pubkey: &Pubkey,
        swap_destination_pubkey: &Pubkey,
        destination_pubkey: &Pubkey,
        pool_mint_pubkey: &Pubkey,
        pool_fee_pubkey: &Pubkey,
        host_fee_pubkey: Option<&Pubkey>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<(), Report> {
        // 创建 Swap 指令
        let swap_instruction = SwapInstruction::Swap(Swap {
            amount_in,
            minimum_amount_out,
        });

        // 打包指令数据
        let data = swap_instruction.pack();

        // 构建交易涉及的账户列表
        let mut accounts = vec![
            AccountMeta::new_readonly(*swap_pubkey, false),
            AccountMeta::new_readonly(*authority_pubkey, false),
            AccountMeta::new_readonly(*user_transfer_authority_pubkey, true),
            AccountMeta::new(*source_pubkey, false),
            AccountMeta::new(*swap_source_pubkey, false),
            AccountMeta::new(*swap_destination_pubkey, false),
            AccountMeta::new(*destination_pubkey, false),
            AccountMeta::new(*pool_mint_pubkey, false),
            AccountMeta::new(*pool_fee_pubkey, false),
            AccountMeta::new_readonly(*token_program_id, false),
        ];

        // 如果提供了 host_fee_pubkey，则将其加入账户列表
        if let Some(host_fee_pubkey) = host_fee_pubkey {
            accounts.push(AccountMeta::new(*host_fee_pubkey, false));
        }

        // 构建 Instruction 对象
        let instruction = Instruction {
            program_id: *program_id,
            accounts,
            data,
        };

        // 执行交换操作
        // invoke(&instruction, &[
        //     AccountInfo::new(&swap_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), program_id, false, 0),
        //     AccountInfo::new(&*authority_pubkey, false, false, &mut 0, program_id,),
        //     AccountInfo::new_readonly(*user_transfer_authority_pubkey, false, program_id),
        //     AccountInfo::new(&source_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        //     AccountInfo::new(&swap_source_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        //     AccountInfo::new(&swap_destination_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        //     AccountInfo::new(&destination_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        //     AccountInfo::new(&pool_mint_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        //     AccountInfo::new(&pool_fee_pubkey, false, false, &mut 0, Pubkey::new_unique().as_mut(), token_program_id, false, 0),
        // ])?;

        Ok(())
    }
    // async fn swap_sol_to_usdt(
    //     client: &RpcClient,
    //     user_keypair: &Keypair,
    //     sol_amount: f64,
    // ) -> Result<()> {
    //     // Raydium 交易池（资金账户，该账户必须是系统账户）
    //     let mint = Pubkey::from_str("CYbD9RaToYMtWKA7QZyoLahnHdWq553Vm62Lh6qWtuxq")?;
    //     // 买家的 Keypair 对
    //     let key_pair_str = [172,247,177,102,165,45,246,213,67,21,171,139,163,67,167,119,185,88,3,48,65,30,85,113,18,35,191,77,74,28,156,162,187,162,67,250,251,105,143,127,136,142,123,231,183,135,48,248,136,31,214,187,17,240,242,244,48,130,79,245,5,110,230,0];
    //     let authority_pubkey = Keypair::from_bytes(&key_pair_str)?;
    //     // 来源账户的公钥
    //     let source_account_pubkey = Pubkey::from_str("5SsEs6LDDmwas8WLvPMgwNMkEagAGJ4monWEkKogKecu")?;
    //     // 目标账户的公钥
    //     let destination = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL")?;
    //     // 兑换的 USDT 数量
    //     let buy_amount = 0.001;
    //     // 获取最新的区块哈希
    //     let latest_blockhash = client.get_latest_blockhash().await?;
    //
    //     // 新增指令
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[Instruction::new_with_bytes(
    //             raydium_program_id,
    //             &buy_amount.to_le_bytes(),
    //             vec![
    //                 AccountMeta::new(source_account_pubkey.pubkey(), false),
    //                 AccountMeta::new_readonly(mint.pubkey(), false),
    //                 AccountMeta::new(destination.pubkey(), false),
    //                 AccountMeta::new_readonly(authority_pubkey, false),
    //                 AccountMeta::new_readonly(spl_token::id(), false),
    //             ],
    //         )],
    //         Some(&payer.pubkey()),
    //         &[&payer],
    //         latest_blockhash,
    //     );
    //
    //     // 将 SOL 转换为 lamports
    //     let lamports = sol_to_lamports(sol_amount);
    //
    //     // 构建 Raydium 交换指令（这是一个伪代码，需要根据 Raydium 合约文档进一步调整）
    //     let swap_instruction = Instruction {
    //         program_id: raydium_program_id,
    //         accounts: vec![
    //             AccountMeta {
    //                 pubkey: user_keypair.pubkey(),
    //                 is_signer: true,
    //                 is_writable: true,
    //             }
    //         ],
    //         data: vec![],  // 填写 Raydium 合约要求的数据
    //     };
    //
    //     // 创建并签署交易
    //     let mut transaction = Transaction::new_with_payer(
    //         &[swap_instruction],
    //         Some(&user_keypair.pubkey()),
    //     );
    //     transaction.try_sign(&[user_keypair], latest_blockhash)?;
    //
    //     // 发送交易并等待确认
    //     let signature = client.send_and_confirm_transaction(&transaction).await?;
    //     println!("Successfully swapped {} SOL to USDT. Transaction signature: {:?}", sol_amount, signature);
    //
    //     Ok(())
    // }


    /// # Description
    ///     创建并返回 USDT 账户的公钥
    /// # Params
    ///     client: &RpcClient - RPC客户端实例
    ///     owner: &Keypair - 所有者的密钥对
    /// # Return
    ///     Result<Pubkey, Report>: 成功时返回 USDT 账户的公钥，失败时返回错误信息。
    pub async fn create_usdt_account(client: &RpcClient, owner: &Keypair, ) -> Result<Pubkey, Report> {
        // 1. 定义USDT的Mint地址
        let usdt_mint_pubkey = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB")?;

        // 2. 获取或创建与所有者公钥相关联的USDT代币账户公钥
        let usdt_account_pubkey = get_associated_token_address(
            &owner.pubkey(),
            &usdt_mint_pubkey
        );

        // 3. 检查USDT账户是否已经存在
        if client.get_account(&usdt_account_pubkey).await.is_err() {
            // 4. 如果账户不存在，创建关联代币账户
            let create_account_instruction = create_associated_token_account(
                &owner.pubkey(), // 账户的所有者
                &owner.pubkey(), // 账户的资助者
                &usdt_mint_pubkey, // USDT的Mint地址
            );

            // 5. 获取最新的区块哈希
            let latest_blockhash = client.get_latest_blockhash().await?;

            // 6. 创建并签署交易
            let mut transaction = Transaction::new_with_payer(
                &[create_account_instruction],
                Some(&owner.pubkey()),
            );
            transaction.try_sign(&[owner], latest_blockhash)?;

            // 7. 发送交易并等待确认
            let signature = client.send_transaction(&transaction).await?;
            client.confirm_transaction(&signature).await?;
        }

        println!("USDT账户的公钥: {:?}", usdt_account_pubkey);

        // 8. 返回USDT账户公钥
        Ok(usdt_account_pubkey)
    }

    /// # Description
    ///     获取代币账号的余额
    /// # Params
    ///     client: &RpcClient - RPC客户端实例
    ///     token_account: &str - 代币账号
    /// # Return
    ///     Result<(), Report>: 成功时返回Ok()，失败时返回错误信息。
    pub async fn get_token_balance(client: &RpcClient, pub_key: &Pubkey) -> Result<(), Report> {
        let balance = client.get_token_account_balance(&pub_key).await?;

        println!("USDT账户余额: {} USDT", balance.amount);
        // balance.amount.parse::<u64>()?

        Ok(())
    }
}


/// SwapResponse - Swap API 的响应格式
#[derive(Deserialize, Debug)]
struct SwapResponse {
    #[serde(rename = "transaction")]
    transaction: TransactionData,  // 使用 Option 来允许字段缺失
    #[serde(rename = "swapDetails")]
    swap_details: SwapDetails,
    #[serde(rename = "tokenInfo")]
    token_info: TokenInfo,
}

#[derive(Deserialize, Debug)]
struct TransactionData {
    #[serde(rename = "serializedTx")]
    serialized_tx: String,
    #[serde(rename = "txType")]
    tx_type: String,
    #[serde(rename = "executionTime")]
    execution_time: f64,
}

#[derive(Deserialize, Debug)]
struct SwapDetails {
    inputAmount: f64,
    outputAmount: f64,
    minimumOutputAmount: f64,
    priceData: PriceData,
    feeInfo: FeeInfo,
}

#[derive(Deserialize, Debug)]
struct PriceData {
    spotPrice: f64,
    effectivePrice: f64,
    priceImpactPercentage: f64,
}

#[derive(Deserialize, Debug)]
struct FeeInfo {
    swapFee: u64,
    platformFeeAmount: f64,
    platformFeeFormatted: f64,
}

#[derive(Deserialize, Debug)]
struct TokenInfo {
    sourceToken: TokenData,
    destinationToken: TokenData,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    address: String,
    decimalPlaces: u64,
}

use std::fmt::Display;


#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Clone)]
pub struct RaydiumPair {
    pub name: String,
    #[serde(rename = "ammId")]
    pub amm_id: String,
    #[serde(rename = "lpMint")]
    pub lp_mint: String,
    #[serde(rename = "baseMint")]
    pub base_mint: String,
    #[serde(rename = "quoteMint")]
    pub quote_mint: String,
    pub market: String,
    pub liquidity: Option<f64>,
    pub volume24h: Option<f64>,
    #[serde(rename = "volume24hQuote")]
    pub volume24h_quote: Option<f64>,
    pub fee24h: Option<f64>,
    #[serde(rename = "fee24hQuote")]
    pub fee24h_quote: Option<f64>,
    pub volume7d: Option<f64>,
    #[serde(rename = "volume7dQuote")]
    pub volume7d_quote: Option<f64>,
    pub fee7d: Option<f64>,
    #[serde(rename = "fee7dQuote")]
    pub fee7d_quote: Option<f64>,
    pub volume30d: Option<f64>,
    #[serde(rename = "volume30dQuote")]
    pub volume30d_quote: Option<f64>,
    pub fee30d: Option<f64>,
    #[serde(rename = "fee30dQuote")]
    pub fee30d_quote: Option<f64>,
    pub price: Option<f64>,
    #[serde(rename = "lpPrice")]
    pub lp_price: Option<f64>,
    #[serde(rename = "tokenAmountCoin")]
    pub token_amount_coin: Option<f64>,
    #[serde(rename = "tokenAmountPc")]
    pub token_amount_pc: Option<f64>,
    #[serde(rename = "tokenAmountLp")]
    pub token_amount_lp: Option<f64>,
    pub apr24h: Option<f64>,
    pub apr7d: Option<f64>,
    pub apr30d: Option<f64>,
}

impl Display for RaydiumPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name: {}", self.name)?;
        // writeln!(f, "amm_id: {}", self.amm_id)?;
        // writeln!(f, "lp_mint: {}", self.lp_mint)?;
        // writeln!(f, "base_mint: {}", self.base_mint)?;
        // writeln!(f, "quote_mint: {}", self.quote_mint)?;
        // writeln!(f, "market: {}", self.market)?;
        writeln!(f, "liquidity: {:?}", self.liquidity)?;
        writeln!(f, "volume24h: {:?}", self.volume24h)?;
        // writeln!(f, "volume24h_quote: {:?}", self.volume24h_quote)?;
        writeln!(f, "fee24h: {:?}", self.fee24h)?;
        // writeln!(f, "fee24h_quote: {:?}", self.fee24h_quote)?;
        writeln!(f, "volume7d: {:?}", self.volume7d)?;
        // writeln!(f, "volume7d_quote: {:?}", self.volume7d_quote)?;
        writeln!(f, "fee7d: {:?}", self.fee7d)?;
        // writeln!(f, "fee7d_quote: {:?}", self.fee7d_quote)?;
        writeln!(f, "volume30d: {:?}", self.volume30d)?;
        // writeln!(f, "volume30d_quote: {:?}", self.volume30d_quote)?;
        writeln!(f, "fee30d: {:?}", self.fee30d)?;
        // writeln!(f, "fee30d_quote: {:?}", self.fee30d_quote)?;
        writeln!(f, "price: {:?}", self.price)?;
        writeln!(f, "lp_price: {:?}", self.lp_price)?;
        // writeln!(f, "token_amount_coin: {:?}", self.token_amount_coin)?;
        // writeln!(f, "token_amount_pc: {:?}", self.token_amount_pc)?;
        // writeln!(f, "token_amount_lp: {:?}", self.token_amount_lp)?;
        writeln!(f, "apr24h: {:?}", self.apr24h)?;
        writeln!(f, "apr7d: {:?}", self.apr7d)?;
        writeln!(f, "apr30d: {:?}", self.apr30d)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaydiumPairs {
    pub pairs: Vec<RaydiumPair>,
}

impl Display for RaydiumPairs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pair in &self.pairs {
            writeln!(f, "{}", pair)?;
        }
        Ok(())
    }
}

impl RaydiumPairs {
    pub fn new() -> Self {
        RaydiumPairs { pairs: vec![] }
    }
    pub fn from_vec(pairs: Vec<RaydiumPair>) -> Self {
        RaydiumPairs { pairs }
    }
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
}

#[test]
fn test_raydiumpairs() {
    // read fixed file
    let current_dir = std::env::current_dir().unwrap();
    println!("current_dir: {:?}", current_dir);
    let read_file_path = current_dir.join("storage/raydium/raydium.json");
    println!("read_file_path: {:?}", read_file_path);
    let content = std::fs::read_to_string(read_file_path).unwrap();

    let tokens: Vec<RaydiumPair> = serde_json::from_str(&content).expect("JSON was not well-formatted");
    println!("token: {:#?}", tokens);
    assert_eq!(tokens.len(), 1);
}
