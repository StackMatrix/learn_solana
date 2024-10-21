use serde::Deserialize;
use solana_program::pubkey::Pubkey;

#[cfg(test)]
mod tests {
    use crate::bootstrap::Bootstrap;
    use std::error::Error;
    use std::str::FromStr;
    use std::sync::Arc;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use color_eyre::Report;
    use serde::{Deserialize, Serialize};
    use solana_program::pubkey::Pubkey;
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_instruction;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::signature::{Keypair, Signer};
    use tracing::info;

    use crate::core::application::wallet::WalletApplication;
    use crate::core::domain::wallet::entity::Column::PubKey;
    use crate::core::domain::wallet::entity::WalletAddress;

    #[tokio::test]
    async fn test_user_registration_and_login() -> Result<(), Box<dyn Error>> {
        // 使用 bootstrap 初始化数据库连接和迁移
        let bootstrap = Bootstrap::run().await?;

        // 获取用户服务
        let user_application = bootstrap.application_layer.user_application.clone();

        // 测试用户注册
        let register_result = user_application.register_user("18160114162".into(), "password123".into()).await;
        assert!(register_result.is_ok(), "用户注册失败");

        // 测试用户登录
        let login_result = user_application.login_user("18160114162".into(), "password123".into()).await;
        info!("{}", format!("{:?}", login_result));
        assert!(login_result.is_ok(), "用户登录失败");

        // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
        let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

        // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
        tokio::signal::ctrl_c().await?;
        webserver.axum_shutdown();

        Ok(())
    }

    #[derive(Serialize, Deserialize)]
    struct Wallet {
        pub_key: String,
        priv_key: String,
    }

    #[tokio::test]
    async fn generate_wallet() -> Result<(), Box<dyn Error>> {
        println!("asd");
        // 1. 生成新的密钥对
        // let keypair = Keypair::new();
        //
        //
        // // 2. 获取公钥和私钥
        // let pub_key = keypair.pubkey().to_string();
        // // let priv_key = keypair.secret().as_bytes().to_base58(); // 将私钥编码为Base58字符串
        //
        // // 3. 将密钥保存到文件
        // let wallet = Wallet {
        //     pub_key: pub_key.to_string(),
        //     priv_key: base64::encode("priv_key"), // 使用Base64编码私钥
        // };

        // 保存为JSON文件
        // let mut file = File::create("solana_wallet.json")?;
        // file.write_all(&serde_json::to_vec(&wallet)?)?;

        // let from_pubkey = Pubkey::new_unique();
        // let nonce_pubkey = Pubkey::new_unique();
        // let authorized = nonce_pubkey;
        // let ixs = create_nonce_account(&from_pubkey, &nonce_pubkey, &authorized, 42);
        // assert_eq!(ixs.len(), 2);
        // let ix = &ixs[0];
        // assert_eq!(ix.program_id, system_program::id());
        // let pubkeys: Vec<_> = ix.accounts.iter().map(|am| am.pubkey).collect();
        //
        // println!("now pub key:{:?}", pubkeys);
        // assert!(pubkeys.contains(&from_pubkey));
        // assert!(pubkeys.contains(&nonce_pubkey));

        // // 使用 bootstrap 初始化数据库连接和迁移
        // let bootstrap = Bootstrap::run().await?;
        //
        // // 获取钱包服务
        // let wallet_service = bootstrap.application_layer.user_application.wallet_service.clone();
        //
        // // 生成钱包
        // let generate_result = wallet_service.generation_wallet().await;
        // info!("generate_result {:?}", generate_result);
        //
        // assert!(generate_result.is_ok(), "添加钱包金额失败");
        //
        // // 添加钱包金额
        // let deposit_result = wallet_service.deposit(1, 1.2).await;
        // info!("deposit_result {:?}", deposit_result);
        //
        // assert!(deposit_result.is_ok(), "添加钱包金额失败");
        //
        // // 钱包转账
        // let withdraw_result = wallet_service.withdraw(1, 1.1).await;
        // info!("withdraw_result {:?}", withdraw_result);
        //
        // assert!(withdraw_result.is_ok(), "钱包转账失败");
        //
        // // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
        // let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);
        //
        // // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
        // tokio::signal::ctrl_c().await?;
        // webserver.axum_shutdown();

        Ok(())
    }


    #[tokio::test]
    async fn test_wallet() -> Result<(), Box<dyn Error>> {
        // 连接到 TestNet RPC 服务器
        let client = solana_client::nonblocking::rpc_client::RpcClient::new(WalletAddress::MainNet.into());

        // 获取 Solana 集群的当前版本和时间信息
        // let _rpc_version_info = WalletApplication::get_cluster_info(&client).await?;

        // 获取 Solana 的总供应量和流通供应量
        // WalletApplication::get_supply(&client).await?;

        // 生成密钥对
        // WalletApplication::generate_keypair(
        //     "./keypair.json",
        //     12,
        //     &None
        // ).await?;

        // 获取 Solana 钱包金额
        // WalletApplication::get_balance("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", &client).await?;
        // WalletApplication::get_balance("A4XtPLMQVfENt6RAHNKm9U35DahPNeBv6XNMeQMoyb9t", &client).await?;

        // 获取空投
        // WalletApplication::airdrop_sol("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", 5.0, &client).await?;

        // 获取 Solana 钱包金额
        // WalletApplication::get_balance("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", &client).await?;

        // // 将钱包转移到另一个地址
        // let key_pair_str = [244,125,253,242,251,53,43,101,136,224,141,68,211,20,196,111,143,188,58,148,211,36,225,231,53,228,138,138,81,235,31,156,97,41,233,203,45,106,202,88,204,238,82,197,140,186,19,195,19,62,229,68,161,28,89,238,193,180,238,217,250,52,35,159];
        // let key_pair = Keypair::from_bytes(&key_pair_str)?;
        // WalletApplication::transfer_sol(&client, &key_pair, "A4XtPLMQVfENt6RAHNKm9U35DahPNeBv6XNMeQMoyb9t", 0.5).await?;
        //
        // // 获取 Solana 钱包金额
        // WalletApplication::get_balance("A4XtPLMQVfENt6RAHNKm9U35DahPNeBv6XNMeQMoyb9t", &client).await?;

        // 获取最后一个区块数
        // let latest_block_number = client.get_slot().await?;
        // // 获取 Solana 区块和交易数量
        // let block =  WalletApplication::get_block(&client, latest_block_number).await?;
        // println!("Transactions count: {}", block.transactions.len());
        //
        // // 统计 Solana 用户交易量
        // let user_transactions_count =  WalletApplication::count_user_transactions(&block).await?;
        // println!("User Transactions count: {:?}", user_transactions_count);

        // 获取 Solana 最近 5 分钟的交易量
        // WalletApplication::calculate_for_range(&client, 2).await?;

        // 获取 usdt 余额
        // WalletApplication::get_token_balance(&client, "DgkvEfTwetYaqYRzn9K2rGg4m8FBMaaX2XqcurwBpR8J").await?;

        // 买家的 Keypair 对
        let key_pair_str = [172,247,177,102,165,45,246,213,67,21,171,139,163,67,167,119,185,88,3,48,65,30,85,113,18,35,191,77,74,28,156,162,187,162,67,250,251,105,143,127,136,142,123,231,183,135,48,248,136,31,214,187,17,240,242,244,48,130,79,245,5,110,230,0];
        let buyer_keypair = Keypair::from_bytes(&key_pair_str)?;

        // 来源账户的公钥
        let source_account_pubkey = Pubkey::from_str("5SsEs6LDDmwas8WLvPMgwNMkEagAGJ4monWEkKogKecu")?;

        // 目标账户的公钥
        let destination_account_pubkey = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL")?;

        // 兑换的 USDT 数量
        let buy_amount = 0.001;

        // 使用 SOL 购买 USDT
        // WalletApplication::buy_usdt_with_sol(&client, &buyer_keypair, &source_account_pubkey, &destination_account_pubkey, buy_amount).await?;

        // WalletApplication::swap_sol_to_usdt(&client, &buyer_keypair, &destination_account_pubkey, 0.00005).await?;


        Ok(())
    }

    /// 测试代币
    #[tokio::test]
    async fn test_wallet_token() -> Result<(), Box<dyn Error>> {
        // 连接到 TestNet RPC 服务器
        let client = solana_client::nonblocking::rpc_client::RpcClient::new(WalletAddress::MainNet.into());
        // let key_pair_str = [172,247,177,102,165,45,246,213,67,21,171,139,163,67,167,119,185,88,3,48,65,30,85,113,18,35,191,77,74,28,156,162,187,162,67,250,251,105,143,127,136,142,123,231,183,135,48,248,136,31,214,187,17,240,242,244,48,130,79,245,5,110,230,0];
        // let buyer_keypair = Keypair::from_bytes(&key_pair_str)?;
        //
        // // 来源账户的公钥
        // let source_account_pubkey = Pubkey::from_str("DdSkP7zTe3FDECZnrRiUguPu8ityrx1kBVoNZR4HA4nT")?;
        // // 目标账户的公钥
        // let destination_account_pubkey = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL")?;
        // // Raydium 的 program 地址
        // let pool_pubkey = Pubkey::from_str("CYbD9RaToYMtWKA7QZyoLahnHdWq553Vm62Lh6qWtuxq")?;

        // WalletApplication::swap_sol_to_usdt_raydium(
        //     &client, &buyer_keypair,
        //     &source_account_pubkey,
        //     &source_account_pubkey,
        //     &pool_pubkey,
        //     0.001
        // ).await?;

        WalletApplication::get_balance("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2", &client).await?;



        WalletApplication::get_market_price().await?;

        Ok(())
    }


    /// 创建账号
    #[tokio::test]
    async fn test_create_account() -> Result<(), Box<dyn Error>> {
        // 连接到 devNet RPC 服务器
        let rpc_url = String::from(WalletAddress::CustomPpc(String::from_str("https://devnet.helius-rpc.com/?api-key=9f42f30d-04bb-40ac-84f9-1b9b7b1b5601")?));
        let rpc_client = RpcClient::new(rpc_url);

        // Generate fee payer and new account key pairs
        let fee_payer = Keypair::new();
        let new_account = Keypair::new();
        println!("fee_payer: {:?}", fee_payer);
        println!("new_account: {:?}", new_account);

        // Request an airdrop for the fee payer and wait for the transaction to be confirmed
        WalletApplication::airdrop_sol("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", 5.0, &rpc_client).await?;

        // WalletApplication::create_account(&rpc_client, &fee_payer, &new_account).await?;

        Ok(())
    }
}


#[cfg(test)]
mod wallet_test {
    use std::error::Error;
    use std::str::FromStr;
    use solana_program::native_token::{lamports_to_sol, sol_to_lamports};
    use solana_program::pubkey::Pubkey;
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;

    use crate::core::application::wallet::WalletApplication;
    use crate::core::domain::wallet::entity::WalletAddress;

    #[tokio::test]
    async fn get_swap_token_amount() -> Result<(), Box<dyn Error>> {
        let client = solana_client::nonblocking::rpc_client::RpcClient::new(WalletAddress::MainNet.into());
        let key_pair_str = [172,247,177,102,165,45,246,213,67,21,171,139,163,67,167,119,185,88,3,48,65,30,85,113,18,35,191,77,74,28,156,162,187,162,67,250,251,105,143,127,136,142,123,231,183,135,48,248,136,31,214,187,17,240,242,244,48,130,79,245,5,110,230,0];
        let owner = Keypair::from_bytes(&key_pair_str)?;

        // 输入代币地址和交易细节
        let input_mint = "So11111111111111111111111111111111111111112";  // SOL 的 mint 地址
        let output_mint = "nosXBVoaCTtYdLvKY6Csb4AC8JCdQKKAaWYtx2ZMoo7,EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";  // USDC 的 mint 地址
        let amount = 1000000;  // 输入的数量（lamports） 0.001
        let slippage = 0.5;  // 允许的滑点（百分比）
        let tx_version = "v0";  // 交易版本
        // SOL 和 USDT 的 mint 地址
        let sol_mint_pubkey = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();
        let usdt_mint_pubkey = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL").unwrap();

        let recipient_usdt_account = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL").unwrap();

        // 获取 Swap 报价
        // let swap_quote = WalletApplication::get_token_price(output_mint).await;
        //
        // WalletApplication::transfer_usdt(
        //     &client, &owner,
        //     &recipient_usdt_account, // 来源账户（您的 USDT 代币账户）
        //     &recipient_usdt_account, // 目标账户（接收 USDT 的账户）
        //     100,         // 转账数量（USDT的最小单位）
        // ).await?;

        // 执行交易
        let _ = WalletApplication::perform_swap(&client, &owner, &sol_mint_pubkey, &usdt_mint_pubkey, 0.5, 10.0, &recipient_usdt_account).await?;

        // 获取账户信息
        let account = WalletApplication::get_account_info(&client,&Pubkey::from_str("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2")?).await?;
        println!("account: {:?}", account);

        // WalletApplication::get_price().await?;

        // 获取 baseVault (SOL) 和 quoteVault (USDC) 的公钥
        // let base_vault = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[72..104])?);
        // let quote_vault = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[104..136])?);


        // let base_vault = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[72..104])?);
        // let quote_vault = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[104..136])?);
        // let base_mint = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[136..168])?);
        // let quote_mint = Pubkey::new_from_array(<[u8; 32]>::try_from(&account.data[168..200])?);
        //
        // println!("base_vault: {:?}", base_vault);
        // println!("quote_vault: {:?}", quote_vault);
        // // 获取 SOL 和 USDC 代币余额
        // let sol_balance = WalletApplication::get_account_info(&client, &base_vault).await?;
        // let usdc_balance = WalletApplication::get_account_info(&client, &quote_vault).await?;

        Ok(())
    }
}
