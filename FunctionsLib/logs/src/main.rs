use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

/// Logger 封装
pub struct Logger;

impl Logger {
    /// 初始化日志系统
    pub fn init() -> Result<(), SetLoggerError> {
        // 设置日志文件路径
        let file_path = "app.log";
        // 设置日志级别，默认是 Info 级别
        let level = LevelFilter::Debug;
        // 配置日志文件输出，日志格式：年月日时分秒 - 文件名 - 行数 - 日志消息
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{d(%Y-%m-%d %H:%M:%S)} - {f}:{L} - {l} - {m}\n",
            ))) // 设置日志格式为：日期(年-月-日 时:分:秒) - 文件名:行号 - 日志级别 - 日志消息
            .build(file_path) // 设置日志文件路径
            .unwrap();

        // 配置日志记录器
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile))) // 只添加文件输出
            .build(
                Root::builder()
                    .appender("logfile") // 将输出目标指定为文件
                    .build(level), // 设置日志级别
            )
            .unwrap();

        // 初始化日志配置
        log4rs::init_config(config)?;
        Ok(())
    }

    /// 记录错误日志
    pub fn log_error(message: &str) {
        log::error!("{}", message);
    }

    /// 记录警告日志
    pub fn log_warn(message: &str) {
        log::warn!("{}", message);
    }

    /// 记录信息日志
    pub fn log_info(message: &str) {
        log::info!("{}", message);
    }

    /// 记录调试日志
    pub fn log_debug(message: &str) {
        log::debug!("{}", message);
    }

    /// 记录追踪日志
    pub fn log_trace(message: &str) {
        log::trace!("{}", message);
    }
}




fn main() {
    // 初始化日志，直接配置日志文件和日志级别
    Logger::init().unwrap();
    
    // 输出日志
    Logger::log_info("这是一个信息级别的日志");
    Logger::log_warn("这是一个警告级别的日志");
    Logger::log_error("这是一个错误级别的日志");
    Logger::log_debug("这是一个调试级别的日志");
    Logger::log_trace("这是一个追踪级别的日志");
}
