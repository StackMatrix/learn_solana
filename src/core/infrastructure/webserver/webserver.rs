
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, Level};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use color_eyre::eyre::{Result, Report};
use tokio::sync::Notify;
use crate::core::infrastructure::log::{CustomOnRequest, CustomOnResponse};
use crate::core::interfaces::router::Router;
use crate::core::infrastructure::config::Config;


/// # Description
///     【WebServer】WebServer 服务
/// # Param
///     notify_shutdown: 关闭提醒
pub struct WebServer {
    notify_shutdown: Arc<Notify>,
}

impl WebServer {
    /// # Description
    ///     新建 WebServer 服务
    /// # Param
    ///     config Arc<Config>: config 配置
    /// # Return
    ///     Result<Persistence, Report>
    ///         - WebServer: Web 服务
    ///         - Report: 错误报告
    pub async fn new(config: Arc<Config>) -> Result<Self, Report> {
        let notify_shutdown = Arc::new(Notify::new());
        let server_notify_shutdown = Arc::clone(&notify_shutdown);

        tokio::spawn(Self::axum_run(config, server_notify_shutdown));

        Ok(Self{ notify_shutdown })
    }


    /// # Description
    ///     WebServer 连接
    /// # Param
    ///     config Arc<Config>: config 配置
    ///     notify_shutdown Arc<Notify>: Web 服务关闭
    /// # Return
    ///     Result<(), Report>
    ///         - (): None
    ///         - Report: 错误报告
    async fn axum_run(config: Arc<Config>, notify_shutdown: Arc<Notify>) -> Result<(), Report> {
        // 读取数据
        let app_config = &config.app;

        // 转换 IP 地址
        let ip_addr = app_config.ip_addr.parse()?;

        let addr = SocketAddr::new(ip_addr, app_config.port);

        // 在端口上监听任务
        let listener = tokio::net::TcpListener::bind(addr).await?;

        // 获取路由接口
        let router = Router::new().await?;

        // 为路由接口添加上日志记录
        let router_log = router.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_request(CustomOnRequest::new().level(Level::INFO))
                .on_response(CustomOnResponse::new().level(Level::DEBUG))
        );

        info!("+ [WebServer] WebServer listener at http://{:?}", addr);

        // 启动 web 服务
        axum::serve(listener, router_log)
            .with_graceful_shutdown(async move { notify_shutdown.notified().await })
            .await?;

        Ok(())
    }

    /// # Description
    ///     WebServer 关闭
    pub fn axum_shutdown(&self) {
        self.notify_shutdown.notify_one();
    }
}
