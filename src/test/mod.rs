#[cfg(test)]
mod tests {
    use crate::bootstrap::Bootstrap;
    use std::error::Error;
    use std::sync::Arc;
    use tracing::info;

    #[tokio::test]
    async fn test_user_registration_and_login() -> Result<(), Box<dyn Error>> {
        // 使用 bootstrap 初始化数据库连接和迁移
        let bootstrap = Bootstrap::run().await?;

        // 获取用户服务
        let user_service = bootstrap.domain_layer.user_domain.user_service.clone();

        // 测试用户注册
        let result = user_service.register_user("18160114162".into(), "password123".into()).await;
        assert!(result.is_ok(), "用户注册失败");

        // 测试用户登录
        let login_result = user_service.login_user("18160114162".into(), "password123".into()).await;
        info!("{}", format!("{:?}", login_result));
        assert!(login_result.is_ok(), "用户登录失败");

        // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
        let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

        // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
        tokio::signal::ctrl_c().await?;
        webserver.axum_shutdown();

        Ok(())
    }


    #[tokio::test]
    async fn test_wallet() -> Result<(), Box<dyn Error>> {
        // 使用 bootstrap 初始化数据库连接和迁移
        let bootstrap = Bootstrap::run().await?;

        // 获取钱包服务
        let wallet_service = bootstrap.domain_layer.wallet_domain.wallet_service.clone();

        // 生成钱包
        let generate_result = wallet_service.generation_wallet().await;
        info!("generate_result {:?}", generate_result);

        assert!(generate_result.is_ok(), "添加钱包金额失败");

        // 添加钱包金额
        // let deposit_result = wallet_service.deposit(1, 1.2).await;
        // info!("deposit_result {:?}", deposit_result);
        //
        // assert!(deposit_result.is_ok(), "添加钱包金额失败");
        //
        // // 钱包转账
        // let withdraw_result = wallet_service.withdraw(1, 1.1).await;
        // info!("withdraw_result {:?}", withdraw_result);

        // assert!(withdraw_result.is_ok(), "钱包转账失败");

        // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
        let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

        // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
        tokio::signal::ctrl_c().await?;
        webserver.axum_shutdown();

        Ok(())
    }
}