use std::sync::Arc;
use std::time::Duration;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{error, info};
use crate::core::infrastructure::config::Config;

/// # Description
///     PostgreSQL 连接
pub struct PostgreSQL;

impl PostgreSQL {
    /// # Description
    ///     PostgreSQL 连接
    /// # Param
    ///     config Arc<Config>: config 配置
    /// # Return
    ///     Result<DatabaseConnection, Report>
    ///         - DatabaseConnection: 数据库连接
    ///         - Report: 错误报告
    pub async fn connect(config: Arc<Config>) -> Result<DatabaseConnection, Report>{
        // 读取数据
        let persistence_config = &config.persistence;

        if let Some(config) = &persistence_config.postgres {
            info!("+ [Database] Connecting to Postgres at {}:{}", config.host, config.port);

            // 构建连接字符串
            let connection_string = format!(
                "postgres://{}:{}@{}:{}/{}",
                config.username, config.password, config.host, config.port, config.database
            );

            // 配置数据库连接
            let mut opt = ConnectOptions::new(&connection_string);
            opt.max_connections(100)
                .min_connections(5)
                .connect_timeout(Duration::from_secs(8))
                .acquire_timeout(Duration::from_secs(8))
                .idle_timeout(Duration::from_secs(8))
                .max_lifetime(Duration::from_secs(8))
                .sqlx_logging(false)  // 禁用 SQLx 的日志
                .sqlx_logging_level(log::LevelFilter::Info)
                .set_schema_search_path("my_schema");

            // 连接到数据库
            match Database::connect(&connection_string).await {
                Ok(connection) => {
                    info!("+ [Database] Successfully connected to MySQL");
                    Ok(connection)
                }
                Err(e) => {
                    error!("- [Database] Failed to connect to MySQL: {}", e);
                    Err(eyre!(e))
                }
            }
        } else {
            error!("- [Database] PostgreSQL configuration is missing");
            Err(eyre!("PostgreSQL configuration is missing"))
        }
    }
}

