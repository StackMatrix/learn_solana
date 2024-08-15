mod bootstrap;
mod core;
mod test;

use std::process;
use std::sync::Arc;
use bootstrap::Bootstrap;
use color_eyre::{eyre::eyre, Result};
// use std::net::SocketAddr;
// use ::config::ConfigError;
// use axum::Router;
// use axum::routing::get;
// use tokio::task::JoinHandle;
// use solana_client::rpc_client::RpcClient;
// use address::Address;
// use wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    // 程序初始化
    // if let Err(err) = Bootstrap::run().await {
    //     Err(eyre!("Application error: {}", err))?;
    //
    //     process::exit(1);
    // };

    match Bootstrap::run().await {
        Ok(bootstrap) => {
            // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
            let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

            // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
            tokio::signal::ctrl_c().await?;
            webserver.axum_shutdown();

            Ok(())
        }
        Err(err) => Err(eyre!("Application error: {}", err))?
    }





    // let rpc_addr: String = Address::MainNet.into();
    // let rpc_client = RpcClient::new(rpc_addr);
    //
    // let mut wallet = Wallet::default();
    // wallet.pub_key = String::from("DdSkP7zTe3FDECZnrRiUguPu8ityrx1kBVoNZR4HA4nT");
    //
    // if let Err(err) = wallet.get_balance(&rpc_client) { eprintln!("{}", err); }
    //
    // let balance = wallet.balance_convert_sol();
    // println!("+[Wallet] {:?} Balance in SOL: {:?}", wallet.pub_key, balance);
}



