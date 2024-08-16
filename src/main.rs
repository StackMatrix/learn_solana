mod bootstrap;
mod core;
mod test;

use std::sync::Arc;
use bootstrap::Bootstrap;
use color_eyre::{eyre::eyre, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 程序初始化
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
}
