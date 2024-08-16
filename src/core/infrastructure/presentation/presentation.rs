use std::sync::Arc;
use color_eyre::eyre::{eyre, Result};
use color_eyre::Report;
use sea_orm::DatabaseConnection;
use crate::core::infrastructure::config::Config;
use crate::core::infrastructure::presentation::repository::Repository;
use super::{MySQL, PostgreSQL};

/// # Description
///     【基础设施】持久性连接组件实例
/// # Param
///     db DatabaseConnection: 数据库连接
///     repository Arc<Repository>: 数据仓库
pub struct Persistence {
    pub db: DatabaseConnection,
    pub repository: Arc<Repository>
}

impl Persistence {
    /// # Description
    ///     新建持久化连接
    /// # Param
    ///     config Arc<Config>: 配置
    /// # Return
    ///     Result<Persistence, Report>
    ///         - Persistence: 持久化连接实例
    ///         - Report: 错误报告
    pub async fn new(config: Arc<Config>) -> Result<Persistence, Report> {
        let db = Self::connect_database(config).await?;
        let repository = Arc::new(Repository::new(db.clone()).await);

        Ok(Self { db, repository })
    }

    /// # Description
    ///     数据库连接
    /// # Param
    ///     config Arc<Config>: 配置
    /// # Return
    ///     Result<DatabaseConnection, Report>
    ///         - DatabaseConnection: 数据库连接
    ///         - Report: 错误报告
    async fn connect_database(config: Arc<Config>) -> Result<DatabaseConnection, Report> {
        // 读取数据
        let persistence_config = &config.persistence;

        // 根据驱动类型连接数据库
        let db_task = match persistence_config.driver.as_str() {
            "mysql" => MySQL::connect(Arc::clone(&config)).await,
            "postgres" => PostgreSQL::connect(Arc::clone(&config)).await,
            _ => return Err(eyre!("+InfrastructureLayer [Database] Don't find this driver".to_string())),
        };

        // 返回 db
        Ok(db_task?)
    }
}