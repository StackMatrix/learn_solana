mod bootstrap;
mod core;

use std::process;
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
    if let Err(err) = Bootstrap::run().await {
        Err(eyre!("Application error: {}", err))?;

        process::exit(1);
    };

    Ok(())



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



