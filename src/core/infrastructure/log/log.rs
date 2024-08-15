use std::sync::Arc;
use color_eyre::eyre::Result;
use color_eyre::Report;
use time::{UtcOffset, macros::format_description};
use tklog::{LEVEL, Format, LOG};
use tracing::Level;
use tracing_appender::{rolling::{RollingFileAppender, Rotation}, non_blocking::WorkerGuard};
use tracing_subscriber::{fmt::{format::FmtSpan, time::OffsetTime}, layer::SubscriberExt, fmt, Layer, EnvFilter, Registry};
use crate::core::infrastructure::config::Config;

/// # Description
///     【基础设施】日志组件实例
/// # Param
///     log_guard WorkerGuard: 日志守护
///     sql_guard WorkerGuard: 日志守护
pub struct Log {
    log_guard: WorkerGuard,
    sql_guard: WorkerGuard,
}

impl Log {
    /// # Description
    ///     初始化日志
    /// # Param
    ///     settings Arc<Config>: config 配置
    /// # Return
    ///     Result<Log, Report>
    ///         - Log: 日志实例化
    ///         - Report: 错误报告
    pub async fn new(config: Arc<Config>) -> Result<Self, Report> {
        let (log_guard, sql_guard) = Self::tracing_log(Arc::clone(&config)).await?;
        Ok(Self{ log_guard, sql_guard })
    }

    /// # Description
    ///     tracing 日志方法
    /// # Param
    ///     settings Arc<Config>: config 配置
    /// # Return
    ///     Result<(WorkerGuard, WorkerGuard), Report>
    ///         - (WorkerGuard, WorkerGuard): 日志守护
    ///         - Report: 错误报告
    async fn tracing_log(config: Arc<Config>) -> Result<(WorkerGuard, WorkerGuard), Report> {
        // 读取数据
        let log_config = &config.log;

        // 创建默认日志文件 appender
        let file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            &log_config.root_dir,
            &log_config.filename,
        );

        // 创建 SQL 日志文件 appender
        let sql_appender = RollingFileAppender::new(
            Rotation::DAILY,
            &log_config.root_dir,
            "sql"
        );

        // 设置文件日志的非阻塞写入
        let (non_blocking, log_guard) = tracing_appender::non_blocking(file_appender);

        // 设置 SQL 日志的非阻塞写入
        let (sql_non_blocking, sql_guard) = tracing_appender::non_blocking(sql_appender);

        // 设置日志级别并格式化时间
        let local_time = OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
        );

        // 配置命令行日志输出样式
        let console_layer = fmt::layer()
            .with_target(false) // 输出日志目标
            .with_level(true) // 输出日志级别
            .with_file(true) // 输出文件名
            .with_line_number(false) // 输出文件和行号
            .with_timer(local_time.clone()) // 输出时间
            .with_writer(std::io::stderr) // 输出到控制台
            .with_ansi(true); // 启用 ANSI 彩色输出

        // 配置 sql 文件日志输出样式
        let sql_log_layer = fmt::layer()
            .json() // 输出为 json 格式
            .with_ansi(false) // 关闭 ANSI 彩色输出
            .with_writer(sql_non_blocking)
            .with_timer(local_time.clone()) // 输出时间
            .with_ansi(false) // 关闭 ANSI 彩色输出
            .with_target(true); // 输出日志目标

        // 从配置文件中获取日志级别
        let file_filter = match log_config.level.as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::INFO, // 默认日志级别
        };

        // 配置普通文件日志输出样式，并排除 sea_orm 和 sqlx_core 日志
        let file_layer = fmt::layer()
            .json() // 输出为 json 格式
            .with_writer(non_blocking)
            .with_target(true) // 输出日志目标
            .with_level(true) // 输出日志级别
            .with_file(true) // 输出文件名
            .with_line_number(true) // 输出文件和行号
            .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT) // 记录 span 事件
            .with_timer(local_time) // 输出时间
            .with_ansi(false); // 关闭 ANSI 彩色输出

        // 初始化 tracing_subscriber
        let subscriber = Registry::default()
            .with(console_layer.with_filter(
                EnvFilter::new("learn_solana=info,warn,error")
                    .add_directive("sea_orm_migration=off".parse()?)
            ))
            .with(sql_log_layer.with_filter(EnvFilter::new("sea_orm=info,error,warn")))
            .with(file_layer.with_filter(
                EnvFilter::new(format!("{}", file_filter))
                    .add_directive("sea_orm=off".parse()?)
                    .add_directive("sqlx=off".parse()?)
            ));
        tracing::subscriber::set_global_default(subscriber)?;

        // 安裝 color-eyre 的 panic 处理句柄
        color_eyre::install()?;

        Ok((log_guard, sql_guard))
    }


    /// # Description
    ///     tklog 日志方法
    /// # Param
    ///     settings Arc<Mutex<Config>>: config 配置
    /// # Return
    ///     Result<(), Report>
    ///         - (): None
    ///         - Report: 错误报告
    async fn tklog_log(config: Arc<Config>) -> Result<(), Report> {
        // 读取数据
        let log_config = &config.log;

        // 从配置文件中获取日志级别
        let log_level = match log_config.level.as_str() {
            "trace" => LEVEL::Trace,
            "debug" => LEVEL::Debug,
            "info" => LEVEL::Info,
            "warn" => LEVEL::Warn,
            "error" => LEVEL::Error,
            "fatal" => LEVEL::Fatal,
            "off" => LEVEL::Off,
            _ => LEVEL::Debug, // 默认日志级别
        };

        let _log = LOG.set_console(true)  // 设置控制台日志
            .set_level(log_level)  // 日志级别，默认Debug
            .set_format(Format::LevelFlag | Format::Time | Format::ShortFileName)  // 结构化日志，定义输出的日志信息
            .set_cutmode_by_size(&log_config.filename, 1<<20, 10, true)  // 日志文件切割模式为文件大小，每1M文件切分一次，保留10个备份日志文件，并压缩备份日志
            .set_formatter("{level}{time} {file}:{message}");  // 自定义日志输出格式。默认：{level}{time} {file}:{message}

        Ok(())
    }
}

