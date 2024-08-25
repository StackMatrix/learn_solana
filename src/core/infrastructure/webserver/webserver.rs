
use std::{net::SocketAddr, sync::Arc};
use tracing::{info, Level};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use color_eyre::eyre::{Result, Report};
use tokio::sync::Notify;
use crate::core::infrastructure::{log::{CustomOnRequest, CustomOnResponse}, config::Config};
use crate::core::presentation::router::Router;


/// # Description
///     【WebServer】WebServer 服务
/// # Param
///     notify_shutdown: 关闭提醒
pub struct WebServer {
    pub notify_shutdown: Arc<Notify>,
    config: Arc<Config>
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

        Ok(Self{ notify_shutdown, config })
    }

    /// # Description
    ///     WebServer 连接，这个要放到应用层去调用启动，否则，路由模块不方便实例化
    /// # Param
    ///     config Arc<Config>: config 配置
    ///     notify_shutdown Arc<Notify>: Web 服务关闭
    /// # Return
    ///     Result<(), Report>
    ///         - (): None
    ///         - Report: 错误报告
    pub async fn axum_run(&self, router: Arc<Router>) -> Result<(), Report> {
        // 读取数据
        let app_config = &self.config.app;

        // 转换 IP 地址
        let ip_addr = app_config.ip_addr.parse()?;

        let addr = SocketAddr::new(ip_addr, app_config.port);

        // 在端口上监听任务
        let listener = tokio::net::TcpListener::bind(addr).await?;

        // 获取所有路由接口
        let app_router = router.v1_routes().await;
        info!("+InfrastructureLayer [WebServer] WebServer routes {:?}", app_router);

        // 为路由接口添加上日志记录
        let app_router_log = app_router.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::DEBUG))
                .on_request(CustomOnRequest::new().level(Level::INFO))
                .on_response(CustomOnResponse::new().level(Level::DEBUG))
        );
        info!("+InfrastructureLayer [WebServer] WebServer listener at http://{:?}", addr);

        // 克隆 Arc<Notify> 引用
        let notify_shutdown = Arc::clone(&self.notify_shutdown);

        // 启动 web 服务
        tokio::spawn(async move {
            axum::serve(listener, app_router_log)
                .with_graceful_shutdown(async move { notify_shutdown.notified().await })
                .await.map_err(|e| Report::msg(format!("+InfrastructureLayer [WebServer] WebServer run failed: {:?}" ,e.to_string())))
        });

        Ok(())
    }

    /// # Description
    ///     WebServer 关闭
    pub fn axum_shutdown(&self) {
        self.notify_shutdown.notify_one();
    }
}
