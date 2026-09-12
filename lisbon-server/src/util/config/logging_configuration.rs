//! Mirrors `net.h4bbo.lisbon.util.config.LoggingConfiguration`.
//!
//! Java configures log4j from a properties file. Here we still create the
//! default properties file (for parity with the deployed layout) and then
//! initialise a `tracing` subscriber.

use std::fs;
use std::path::{Path, PathBuf};

pub struct LoggingConfiguration;

const DIST_LOGGING_CONFIG_PATH: &str = "config/log4j.properties";
const LEGACY_LOGGING_CONFIG_PATH: &str = "log4j.properties";
const LOG_DIRECTORY: &str = "logs";

const DEFAULT_LOGGING_OUTPUT: &str = "log4j.rootLogger=INFO, stdout, SERVER_LOG\n\
log4j.appender.stdout.threshold=info\n\
log4j.appender.stdout=org.apache.log4j.ConsoleAppender\n\
log4j.appender.stdout.Target=System.out\n\
log4j.appender.stdout.layout=org.apache.log4j.PatternLayout\n\
log4j.appender.stdout.layout.ConversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} %-5p [%c] - %m%n\n\
\n\
# Create new logger information for error\n\
log4j.logger.ErrorLogger=ERROR, error, ERROR_FILE\n\
log4j.additivity.ErrorLogger=false\n\
\n\
# Set settings for the error logger\n\
log4j.appender.error=org.apache.log4j.ConsoleAppender\n\
log4j.appender.error.Target=System.err\n\
log4j.appender.error.layout=org.apache.log4j.PatternLayout\n\
log4j.appender.error.layout.ConversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} %-5p [%c] - %m%n\n\
\n\
# Define the file appender for errors\n\
log4j.appender.ERROR_FILE=org.apache.log4j.FileAppender\n\
log4j.appender.ERROR_FILE.File=logs/error.log\n\
log4j.appender.ERROR_FILE.ImmediateFlush=true\n\
log4j.appender.ERROR_FILE.Threshold=debug\n\
log4j.appender.ERROR_FILE.Append=true\n\
log4j.appender.ERROR_FILE.layout=org.apache.log4j.PatternLayout\n\
log4j.appender.ERROR_FILE.layout.conversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} - [%c] - %m%n\n\
\n\
# Define the file appender for server output\n\
log4j.appender.SERVER_LOG=org.apache.log4j.FileAppender\n\
log4j.appender.SERVER_LOG.File=logs/server.log\n\
log4j.appender.SERVER_LOG.ImmediateFlush=true\n\
log4j.appender.SERVER_LOG.Threshold=debug\n\
log4j.appender.SERVER_LOG.Append=true\n\
log4j.appender.SERVER_LOG.layout=org.apache.log4j.PatternLayout\n\
log4j.appender.SERVER_LOG.layout.conversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} - [%c] - %m%n\n";

impl LoggingConfiguration {
    /// Mirrors `checkLoggingConfig`.
    pub fn check_logging_config() -> std::io::Result<()> {
        fs::create_dir_all(LOG_DIRECTORY)?;

        let logging_config = Self::resolve_logging_config_file();
        if let Some(parent) = logging_config.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if !logging_config.exists() {
            fs::write(&logging_config, DEFAULT_LOGGING_OUTPUT)?;
        }

        // Mirrors `PropertyConfigurator.configure(path)`.
        Self::init_tracing();

        Ok(())
    }

    /// Mirrors `resolveLoggingConfigFile`.
    fn resolve_logging_config_file() -> PathBuf {
        let dist = Path::new(DIST_LOGGING_CONFIG_PATH);
        if dist.exists() || Path::new("config").is_dir() {
            dist.to_path_buf()
        } else {
            PathBuf::from(LEGACY_LOGGING_CONFIG_PATH)
        }
    }

    /// Initialises the `tracing` subscriber, honouring `RUST_LOG`.
    fn init_tracing() {
        let filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
    }
}
