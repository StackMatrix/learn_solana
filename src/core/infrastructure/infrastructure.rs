use std::sync::Arc;
use color_eyre::Report;
use sea_orm_migration::MigratorTrait;
use tracing::info;
use crate::core::infrastructure::{config::Config, log::Log, presentation::model::migration::MigratorHandle, presentation::Persistence, webserver, webserver::WebServer};

pub struct InfrastructureLayer {
    pub config: Arc<Config>,
    pub log: Arc<Log>,
    pub persistence: Arc<Persistence>,
    pub webserver: Arc<WebServer>,
}

impl InfrastructureLayer {
    pub async fn new() -> color_eyre::Result<Self, Report> {
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

        // 启动 Web 服务
        let webserver = Arc::new(WebServer::new(Arc::clone(&config)).await?);

        info!("+InfrastructureLayer [WebServer] Instant webserver complete.");

        Ok(Self {
            config: Arc::clone(&config),
            log: Arc::clone(&log),
            persistence: Arc::clone(&persistence),
            webserver: Arc::clone(&webserver)
        })
    }
}