use std::error::Error;
use std::rc::Rc;
use color_eyre::eyre::Result;
use sea_orm_migration::MigratorTrait;
use crate::core::infrastructure::config::Config;
use crate::core::infrastructure::{log::Log, presentation::Persistence, webserver::WebServer};
use crate::core::infrastructure::presentation::model::migration::MigratorHandle;

/// # Description
///     【app】引导组件实例化
/// # Param
///     log Log: 日志实例
///     persistence Persistence: 持久化实例
pub struct Bootstrap {
    log: Log,
    persistence: Persistence,
}

impl Bootstrap {
    /// # Description
    ///     初始化依赖
    /// # Parama
    ///     None
    /// # Return
    ///     Result<(), Box<dyn Error>>
    ///         - (): None
    ///         - Box<dyn Error + Send + Sync>: 错误
    pub async fn run() -> Result<Self, Box<dyn Error>> {
        // 加载配置
        let config = Config::new().map_err(|e| format!("- [Config] Init err: {:?}", e))?;

        // 配置引用
        let config_arc = Rc::new(config);

        // 初始化日志
        let log = Log::new(Rc::clone(&config_arc)).await?;

        // 连接到数据库
        let persistence = Persistence::new(Rc::clone(&config_arc)).await?;

        // 迁移数据库
        let persistence_rc = Rc::new(&persistence);
        MigratorHandle::down(&persistence_rc.db, None).await?;
        MigratorHandle::up(&persistence_rc.db, None).await?;

        // 启动 Web 服务
        WebServer::new(Rc::clone(&config_arc)).await?;

        Ok(Self { log, persistence })
    }
}