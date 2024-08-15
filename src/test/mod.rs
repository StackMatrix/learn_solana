#[cfg(test)]
mod tests {
    use crate::bootstrap::Bootstrap;
    use std::error::Error;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_user_registration_and_login() -> Result<(), Box<dyn Error>> {
        // 使用 bootstrap 初始化数据库连接和迁移
        let bootstrap = Bootstrap::run().await?;

        // 获取用户服务
        let mut user_service = bootstrap.domain_layer.user_domain.user_service.clone();

        // 测试用户注册
        let result = user_service.register_user(
            "test_user".into(), "test_user@example.com".into(), "password123".into()
        ).await;

        assert!(result.is_ok(), "用户注册失败");

        // 测试用户登录
        let login_result = user_service.login_user(
            "test_user".into(), "password123".into()
        ).await;

        assert!(login_result.is_ok(), "用户登录失败");

        // 在 axum_shutdown 调用之前，确保 infrastructure_layer 被正确引用
        let webserver = Arc::clone(&bootstrap.infrastructure_layer.webserver);

        // 等待 Ctrl+C 信号，并在接收到信号后关闭 Web 服务器
        tokio::signal::ctrl_c().await?;
        webserver.axum_shutdown();

        Ok(())
    }
}