use serde::Deserialize;
use color_eyre::eyre::Result;
use color_eyre::Report;
use config::{Config as Conf, File};

/// # Description
///     #[derive(Debug, Deserialize)]
///     Config 配置
/// # Fields
///     pub app: AppConfig, APP 配置
///     pub log: LogConfig, 日志配置
///     pub persistence: PersistenceConfig, 持久化连接配置
///     pub jwt: JwtConfig, JWT 配置
///     pub redis: RedisConfig, Redis 配置
///     pub storage: StorageConfig, 存储配置
///     pub queue: QueueConfig, 消息队列配置
///     pub smtp: SmtpConfig, SMTP 邮件配置
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub app: AppConfig,
    pub log: LogConfig,
    pub persistence: PersistenceConfig,
    pub jwt: JwtConfig,
    pub redis: RedisConfig,
    pub storage: StorageConfig,
    pub queue: QueueConfig,
    pub smtp: SmtpConfig,
}

/// # Description
///     #[derive(Debug, Deserialize)]
///     App 配置
/// # Fields
///     pub env: String, 项目环境调试级别
///     pub port: u16, 程序运行端口
///     pub app_name: String, 程序名称
///     pub ip_addr: String, 程序运行 IP 地址
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub env: String,
    pub port: u16,
    pub app_name: String,
    pub ip_addr: String,
}

/// # Description
///     #[derive(Debug, Deserialize)]
///     Log 配置
/// # Fields
///     pub level: String, 日志级别
///     pub root_dir: String, 日志目录
///     pub filename: String,
///     pub max_backups: u32,
///     pub max_size: u32,
///     pub max_age: u32,
///     pub compress: bool,
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LogConfig {
    pub level: String,
    pub root_dir: String,
    pub filename: String,
    pub max_backups: u32,
    pub max_size: u32,
    pub max_age: u32,
    pub compress: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PersistenceConfig {
    pub driver: String,
    pub db_level: String,
    pub mysql: Option<MySQLConfig>,
    pub postgres: Option<PostgresSQLConfig>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub charset: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub log_mode: String,
    pub enable_file_log_writer: bool,
    pub log_filename: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PostgresSQLConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: String,
    pub charset: String,
    pub max_idle_conns: u32,
    pub max_open_conns: u32,
    pub log_mode: String,
    pub enable_file_log_writer: bool,
    pub log_filename: String,
}

/// # Description
///     #[derive(Debug, Deserialize)]
///     JWT 配置
/// # Fields
///     secret: String, JWT 密钥
///     jwt_ttl: u64, JWT 的存活时间（秒）
///     jwt_blacklist_grace_period: u64, 黑名单宽限期（秒）
///     refresh_grace_period: u64, 刷新令牌宽限期（秒）
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct JwtConfig {
    pub secret: String,
    pub jwt_ttl: u64,
    pub jwt_blacklist_grace_period: u64,
    pub refresh_grace_period: u64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u32,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct StorageConfig {
    pub default: String,
    pub disks: DisksConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DisksConfig {
    pub local: LocalDiskConfig,
    pub ali_oss: OssConfig,
    pub qi_niu: QiniuConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LocalDiskConfig {
    pub root_dir: String,
    pub app_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct OssConfig {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub bucket: String,
    pub endpoint: String,
    pub is_ssl: bool,
    pub is_private: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QiniuConfig {
    pub access_key: String,
    pub secret_key: String,
    pub bucket: String,
    pub domain: String,
    pub is_ssl: bool,
    pub is_private: bool,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QueueConfig {
    pub rabbitmq: RabbitmqConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RabbitmqConfig {
    pub uri: String,
    pub exchange_name: String,
    pub delay_exchange_name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SmtpConfig {
    pub net_ease_163: SmtpDetails,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SmtpDetails {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: String,
}

impl Config {
    /// # Description
    ///     解析程序配置文件
    /// # Return
    ///     Result<Self, Report>
    ///         - Config: 程序配置
    ///         - Report: 错误报告
    pub async fn new() -> Result<Config, Report> {
        let builder = Conf::builder()
            .set_default("default", "1")?
            .add_source(File::with_name("conf/app.yaml"))
            .add_source(File::with_name("conf/jwt.yaml"))
            .add_source(File::with_name("conf/log.yaml"))
            .add_source(File::with_name("conf/persistence.yaml"))
            .add_source(File::with_name("conf/queue.yaml"))
            .add_source(File::with_name("conf/redis.yaml"))
            .add_source(File::with_name("conf/smtp.yaml"))
            .add_source(File::with_name("conf/storage.yaml"))
            .set_override("override", "1")?;

        let config = builder.build()?;
        config.try_deserialize().map_err(|e| e.into())
    }
}
