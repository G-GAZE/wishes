//! 日志初始化
//! 
//! 同时输出到控制台 (stderr) 和日志文件.
//! 日志文件按天轮转, 保留最近 `MAX_LOG_FILES` 个文件.
//! 
//! 默认级别:
//! - dev  构建: `wishes=debug`
//! - release 构建: `wishes=info`
//! 
//! 环境变量 `RUST_LOG` 可覆盖默认级别.

use std::path::Path;

use anyhow::{Context, Result};
use tracing_appender::{non_blocking::WorkerGuard, rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// 日志文件保留数量.
const MAX_LOG_FILES: usize = 7;


/// 日志系统守卫.
/// 
/// 内部持有 `WorkerGuard`, 必须在进程存续期间保持存活, 
/// 否则非阻塞写入器的后台线程会被提前关闭, 最后几条日志可能丢失.
pub struct LogGuard {
    _guard: WorkerGuard,
}

/// 初始化日志系统.
/// 
/// # 参数
/// - `log_dir`: 日志文件输出目录, 若不存在则创建.
/// 
/// # 返回
/// 返回 `LogGuard`, 调用方需持有至进程结束.
pub fn init(log_dir: &Path) -> Result<LogGuard> {
    std::fs::create_dir_all(log_dir)
        .with_context(|| format!("创建日志目录失败: {}", log_dir.display()))?;

    // 按天轮转, 自动清理超过 MAX_LOG_FILES 的旧文件
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("wishes")
        .filename_suffix("log")
        .max_log_files(MAX_LOG_FILES)
        .build(log_dir)
        .with_context(|| "创建日志文件 appender 失败")?;

    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // 过滤规则: 优先 RUST_LOG, 否则使用构建类型对应的默认值
    let default_filter = if cfg!(debug_assertions) {
        "wishes=debug"
    } else {
        "wishes=info"
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(default_filter));

    // 控制台层, 开发时便于观察
    let console_layer = fmt::layer()
        .with_target(false)
        .with_ansi(cfg!(debug_assertions));
    
    // 文件层, 带模块路径
    let file_layer = fmt::layer()
        .with_target(true)
        .with_ansi(false)
        .with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    Ok(LogGuard { _guard: guard })
}