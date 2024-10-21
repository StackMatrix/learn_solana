// mod bootstrap;
// mod core;
// mod test;
//
// use std::sync::Arc;
// use bootstrap::Bootstrap;
// use color_eyre::{eyre::eyre, Result};
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     // 程序初始化
//     match Bootstrap::run().await {
//         Ok(bootstrap) => {
//             // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
//             let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);
//
//             // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
//             tokio::signal::ctrl_c().await?;
//             webserver.axum_shutdown();
//
//             Ok(())
//         }
//         Err(err) => Err(eyre!("Application error: {}", err))?
//     }
// }

use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_sdk::program_pack::Pack;
use spl_token::{instruction as token_instruction, state::Account as TokenAccount};
use raydium_library;

#[tokio::main]
async fn main() {
    // Solana 主网或测试网的 RPC URL
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=9f42f30d-04bb-40ac-84f9-1b9b7b1b5601";
    let client = RpcClient::new(rpc_url);


    // 发起者钱包的私钥
    let key_pair_str = [172,247,177,102,165,45,246,213,67,21,171,139,163,67,167,119,185,88,3,48,65,30,85,113,18,35,191,77,74,28,156,162,187,162,67,250,251,105,143,127,136,142,123,231,183,135,48,248,136,31,214,187,17,240,242,244,48,130,79,245,5,110,230,0];
    let sender = Keypair::from_bytes(&key_pair_str).unwrap();

    // 收款人钱包地址
    // let recipient_pubkey = Pubkey::from_str("RECIPIENT_PUBLIC_KEY").unwrap();2DX9kzzsGSxu7JphYnF7g1Vq4VRLbs6pHHwkAuBNPVPc

    // SPL 代币账户地址
    let token_program_id = Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap();

    // SPL 代币账户地址 (发件人)
    let sender_token_account = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL").unwrap();

    // SPL 代币账户地址 (收件人)
    let recipient_token_account = Pubkey::from_str("C3G6UdF3ujSr2pk2QXf1ZWRe9ANJvesd8GXDSknpc7FL").unwrap();

    // 转账金额
    let amount = 1_000_000; // 表示 1 个代币 (假设代币有 6 位小数)

    // 获取最近的区块哈希，供交易使用
    let blockhash = client.get_latest_blockhash().unwrap();




    println!("Transaction signature: {}", signature);
}