//! Mirrors `org.alexdev.http.util.config.WebLoggingConfiguration`.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use tracing_subscriber::EnvFilter;

/// Mirrors `org.alexdev.http.util.config.WebLoggingConfiguration`.
pub struct WebLoggingConfiguration;

#[derive(Clone, PartialEq, Eq)]
struct AppenderSpec {
    file: Option<String>,
    target: String,
}

struct LoggerSpec {
    name: String,
    level: String,
    appenders: Vec<String>,
}

struct FanOutWriter<'a> {
    stdout: Option<&'a std::io::Stdout>,
    stderr: Option<&'a std::io::Stderr>,
    files: &'a [std::sync::Mutex<std::fs::File>],
}

impl std::io::Write for FanOutWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(out) = self.stdout {
            out.lock().write_all(buf)?;
        }
        if let Some(err) = self.stderr {
            err.lock().write_all(buf)?;
        }
        for file in self.files {
            file.lock()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "poisoned log file lock"))?
                .write_all(buf)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(out) = self.stdout {
            out.lock().flush()?;
        }
        if let Some(err) = self.stderr {
            err.lock().flush()?;
        }
        for file in self.files {
            file.lock()
                .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "poisoned log file lock"))?
                .flush()?;
        }
        Ok(())
    }
}

struct MultiWriter {
    stdout: Option<std::io::Stdout>,
    stderr: Option<std::io::Stderr>,
    files: Vec<std::sync::Mutex<std::fs::File>>,
}

impl MultiWriter {
    fn new(specs: &[AppenderSpec]) -> Self {
        let mut stdout: Option<std::io::Stdout> = None;
        let mut stderr: Option<std::io::Stderr> = None;
        let mut files = Vec::new();

        for spec in specs {
            match &spec.file {
                Some(path) => {
                    if let Some(parent) = Path::new(path).parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if let Ok(file) = fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                    {
                        files.push(std::sync::Mutex::new(file));
                    }
                }
                None => {
                    if spec.target.eq_ignore_ascii_case("System.err") {
                        stderr = Some(std::io::stderr());
                    } else {
                        stdout = Some(std::io::stdout());
                    }
                }
            }
        }

        Self {
            stdout,
            stderr,
            files,
        }
    }
}

impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for MultiWriter {
    type Writer = FanOutWriter<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        FanOutWriter {
            stdout: self.stdout.as_ref(),
            stderr: self.stderr.as_ref(),
            files: &self.files,
        }
    }
}

impl WebLoggingConfiguration {
    const DIST_LOGGING_CONFIG_PATH: &'static str = "config/log4j.web.properties";
    const LEGACY_LOGGING_CONFIG_PATH: &'static str = "log4j.web.properties";
    const LOG_DIRECTORY: &'static str = "logs";

    /// Mirrors `checkLoggingConfig()`.
    pub fn check_logging_config() -> std::io::Result<()> {
        let output = "log4j.rootLogger=INFO, stdout, SERVER_LOG\n\
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
            log4j.appender.ERROR_FILE.File=logs/error.web.log\n\
            log4j.appender.ERROR_FILE.ImmediateFlush=true\n\
            log4j.appender.ERROR_FILE.Threshold=debug\n\
            log4j.appender.ERROR_FILE.Append=true\n\
            log4j.appender.ERROR_FILE.layout=org.apache.log4j.PatternLayout\n\
            log4j.appender.ERROR_FILE.layout.conversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} - [%c] - %m%n\n\
            \n\
            # Define the file appender for server output\n\
            log4j.appender.SERVER_LOG=org.apache.log4j.FileAppender\n\
            log4j.appender.SERVER_LOG.File=logs/server.web.log\n\
            log4j.appender.SERVER_LOG.ImmediateFlush=true\n\
            log4j.appender.SERVER_LOG.Threshold=debug\n\
            log4j.appender.SERVER_LOG.Append=true\n\
            log4j.appender.SERVER_LOG.layout=org.apache.log4j.PatternLayout\n\
            log4j.appender.SERVER_LOG.layout.conversionPattern=%d{yyyy-MM-dd'T'HH:mm:ss.SSS} - [%c] - %m%n\n";

        let log_directory = Path::new(Self::LOG_DIRECTORY);

        if !log_directory.exists() {
            fs::create_dir_all(log_directory)?;
        }

        let logging_config = Self::resolve_logging_config_file();
        let parent_directory = logging_config.parent();

        if let Some(parent) = parent_directory {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if !logging_config.exists() {
            let mut writer = fs::File::create(&logging_config)?;
            writer.write_all(output.as_bytes())?;
            writer.flush()?;
        }

        Self::init_tracing(&logging_config);
        Ok(())
    }

    /// Mirrors `resolveLoggingConfigFile()`.
    fn resolve_logging_config_file() -> PathBuf {
        let dist_logging_config = Path::new(Self::DIST_LOGGING_CONFIG_PATH);

        if dist_logging_config.exists() || Path::new("config").is_dir() {
            dist_logging_config.to_path_buf()
        } else {
            Path::new(Self::LEGACY_LOGGING_CONFIG_PATH).to_path_buf()
        }
    }

    fn parse_properties(path: &Path) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                let line = line.trim();

                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    properties.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        properties
    }

    fn appender_spec(properties: &HashMap<String, String>, name: &str) -> AppenderSpec {
        let appender_type = format!("log4j.appender.{name}");
        let is_file = properties
            .get(&appender_type)
            .map(|value| value.contains("FileAppender"))
            .unwrap_or(false);

        let target_key = format!("log4j.appender.{name}.Target");
        let file_key = format!("log4j.appender.{name}.File");

        AppenderSpec {
            file: if is_file {
                properties.get(&file_key).cloned()
            } else {
                None
            },
            target: properties
                .get(&target_key)
                .cloned()
                .unwrap_or_else(|| "System.out".to_string()),
        }
    }

    /// Mirrors `PropertyConfigurator.configure(path)` — parses the log4j
    /// properties (the root level, the per-logger levels, the console/file
    /// appender targets) and installs the `tracing` subscriber on them.
    fn init_tracing(config_path: &Path) {
        let properties = Self::parse_properties(config_path);

        let (root_level, root_appenders) = match properties.get("log4j.rootLogger") {
            Some(spec) => {
                let mut parts = spec.split(',').map(|part| part.trim());
                let level = parts.next().unwrap_or("info").to_lowercase();
                (
                    level,
                    parts
                        .filter(|part| !part.is_empty())
                        .map(|part| part.to_string())
                        .collect(),
                )
            }
            None => ("info".to_string(), vec!["stdout".to_string()]),
        };

        let mut named_loggers: Vec<LoggerSpec> = Vec::new();

        for (key, value) in properties.iter() {
            let Some(logger_name) = key.strip_prefix("log4j.logger.") else {
                continue;
            };

            let mut parts = value.split(',').map(|part| part.trim());
            let level = parts.next().unwrap_or("info").to_lowercase();

            named_loggers.push(LoggerSpec {
                name: logger_name.to_string(),
                level,
                appenders: parts
                    .filter(|part| !part.is_empty())
                    .map(|part| part.to_string())
                    .collect(),
            });
        }

        let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            let mut directive = root_level.clone();
            for logger in &named_loggers {
                directive.push_str(&format!(", {}={}", logger.name, logger.level));
            }
            EnvFilter::new(directive)
        });

        let mut specs: Vec<AppenderSpec> = root_appenders
            .iter()
            .map(|name| Self::appender_spec(&properties, name))
            .collect();

        for logger in &named_loggers {
            for name in &logger.appenders {
                let spec = Self::appender_spec(&properties, name);
                if !specs.contains(&spec) {
                    specs.push(spec);
                }
            }
        }

        let _ = tracing_subscriber::fmt()
            .with_writer(MultiWriter::new(&specs))
            .with_env_filter(filter)
            .try_init();
    }
}
