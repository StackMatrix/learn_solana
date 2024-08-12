use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::rc::Rc;
use axum::handler::Handler;
use tracing::{info, Level, Span};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, OnResponse, TraceLayer};
use color_eyre::eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use crate::core::infrastructure::log::{CustomOnRequest, CustomOnResponse};
use crate::core::interfaces::router::Router;
use crate::core::infrastructure::config::Config;


/// # Description
///     【WebServer】WebServer 服务
pub struct WebServer;

impl WebServer {
    /// # Description
    ///     新建 WebServer 服务
    /// # Param
    ///     settings: 配置
    /// # Return
    ///     Result<Persistence, Box<dyn Error>>
    ///         - (): None
    ///         - Box<dyn Error>: 错误
    pub async fn new(config: Rc<Config>) -> Result<(), Box<dyn Error>> {
        Self::axum_run(config).await?;
        Ok(())
    }


    /// # Description
    ///     WebServer 连接
    /// # Param
    ///     None
    /// # Return
    ///     Result<(), Box<dyn Error + Send + Sync>>
    ///         - (): None
    ///         - Box<dyn Error + Send + Sync>: 错误
    async fn axum_run(config: Rc<Config>) -> Result<(), Box<dyn Error>> {
        // 读取数据
        let app_config = &config.app;

        // 转换 IP 地址
        let ip_addr: IpAddr = app_config.ip_addr.parse().map_err(|e| format!("- [WebServer] Invalid IP address: {}", e))?;

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
        axum::serve(listener, router_log).await?;

        Ok(())
    }
}
