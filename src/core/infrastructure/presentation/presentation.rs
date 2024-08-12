use std::error::Error;
use std::rc::Rc;
use color_eyre::eyre::Result;
use sea_orm::DatabaseConnection;
use crate::core::infrastructure::config::Config;
use super::{MySQL, PostgreSQL};

/// # Description
///     【基础设施】持久性连接组件实例
/// # Param
///     db DatabaseConnection: 数据库连接
pub struct Persistence {
    pub db: DatabaseConnection,
}

impl Persistence {
    /// # Description
    ///     新建持久化连接
    /// # Param
    ///     settings: 配置
    /// # Return
    ///     Result<Persistence, Box<dyn Error>>
    ///         - Persistence: 持久化连接实例
    ///         - Box<dyn Error>: 错误
    pub async fn new(config: Rc<Config>) -> Result<Persistence, Box<dyn Error>> {
        let db = Self::connect_database(config).await?;
        Ok(Self { db })
    }

    /// # Description
    ///     数据库连接
    /// # Param
    ///     settings: 配置
    /// # Return
    ///     Result<DatabaseConnection, Box<dyn Error + Send + Sync>>
    ///         - DatabaseConnection: 数据库连接
    ///         - Box<dyn Error + Send + Sync>: 错误
    async fn connect_database(config: Rc<Config>) -> Result<DatabaseConnection, Box<dyn Error>> {
        // 读取数据
        let persistence_config = &config.persistence;

        // 根据驱动类型连接数据库
        let db_task = match persistence_config.driver.as_str() {
            "mysql" => MySQL::connect(Rc::clone(&config)).await,
            "postgres" => PostgreSQL::connect(Rc::clone(&config)).await,
            _ => Err("Don't find this driver".into()),
        };

        // 返回 db
        Ok(db_task?)
    }
}