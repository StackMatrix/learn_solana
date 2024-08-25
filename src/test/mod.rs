#[cfg(test)]
mod tests {
    use crate::bootstrap::Bootstrap;
    use std::error::Error;
    use std::io::Write;
    use std::sync::Arc;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{Deserialize, Serialize};
    use tracing::info;

    use crate::core::application::wallet::WalletApplication;
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
        let client = solana_client::nonblocking::rpc_client::RpcClient::new(WalletAddress::TestNet.into());

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
        // WalletApplication::airdrop_sol("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", 1.0, &client).await?;

        // 获取 Solana 钱包金额
        // WalletApplication::get_balance("7YHcfnrRbdAATVC3PXqNQ4ejzHSDwMqQauzSHrtF36CW", &client).await?;
        //
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
        WalletApplication::calculate_for_range(&client, 2).await?;

        Ok(())
    }



}
