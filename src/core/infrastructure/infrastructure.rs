use std::sync::Arc;
use color_eyre::{ Report, Result };
use sea_orm_migration::MigratorTrait;
use tracing::info;
use crate::core::infrastructure::{
    config::Config,
    log::Log,
    presentation::{ Persistence, model::migration::MigratorHandle },
    webserver::WebServer,
    jwt::Jwt
};

/// # Description
///     基础设施层
/// # Fields
///     pub config: Arc<Config>, 配置实例
///     pub log: Arc<Log>, 日志实例
///     pub persistence: Arc<Persistence>, 持久化实例
///     pub webserver: Arc<WebServer>, WebServer 实例
///     pub jwt: Arc<Jwt>, Jwt 实例
pub struct InfrastructureLayer {
    pub config: Arc<Config>,
    pub log: Arc<Log>,
    pub persistence: Arc<Persistence>,
    pub webserver: Arc<WebServer>,
    pub jwt: Arc<Jwt>,
}

impl InfrastructureLayer {
    /// # Description
    ///     基础设施层实例化
    /// # Params
    ///     None
    /// # Return
    ///     Result<Self, Report>
    ///         - Self 基础设施层的实例
    ///         - Report 错误报告
    pub async fn new() -> Result<Self, Report> {
        // 加载配置
        let config = Arc::new(Config::new().await?);

        // 初始化日志
        let log = Arc::new(Log::new(Arc::clone(&config)).await?);
        info!("+InfrastructureLayer [Config] Instant config complete.");

        // 连接到数据库
        let persistence = Arc::new(Persistence::new(Arc::clone(&config)).await?);
        info!("+InfrastructureLayer [Persistence] Instant persistence complete.");

        // 迁移数据库
        let db = Arc::clone(&persistence);
        MigratorHandle::down(&db.db, None).await?;
        MigratorHandle::up(&db.db, None).await?;
        info!("+InfrastructureLayer [Persistence] Migrator persistence complete.");

        // JWT 实例
        let jwt = Arc::new(Jwt::new(Arc::clone(&config)));
        info!("+InfrastructureLayer [JWT] JWT persistence complete.");

        // 启动 Web 服务
        let webserver = Arc::new(WebServer::new(Arc::clone(&config)).await?);
        info!("+InfrastructureLayer [WebServer] Instant webserver complete.");

        Ok(Self {
            config: Arc::clone(&config),
            log: Arc::clone(&log),
            persistence: Arc::clone(&persistence),
            webserver: Arc::clone(&webserver),
            jwt: Arc::clone(&jwt),
        })
    }
}