use anyhow::Result;
use serde::{Deserialize, Deserializer};
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::Targets, fmt, prelude::*, Layer};
use tracing_subscriber::{EnvFilter, Registry};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
enum LogFormat {
    #[default]
    Full,
    Compact,
    Pretty,
    Json,
}

fn default_as_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
struct FileLogger {
    filename: String,
    path: PathBuf,
    #[serde(flatten)]
    options: SharedOptions,
    modules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct SharedOptions {
    #[serde(default = "default_as_true")]
    enabled: bool,
    #[serde(default)]
    format: LogFormat,
    #[serde(default)]
    line_number: bool,
    #[serde(default)]
    file: bool,
    #[serde(default)]
    thread_name: bool,
    #[serde(default)]
    thread_id: bool,
}

#[derive(Debug, Deserialize)]
struct StreamLogger {
    #[serde(default)]
    color: bool,
    #[serde(flatten)]
    options: SharedOptions,
    modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LogConfig {
    global: GlobalLogger,
    stream_logger: StreamLogger,
    file_logger: Option<Vec<FileLogger>>,
}

#[derive(Debug, Deserialize)]
struct GlobalLogger {
    #[serde(deserialize_with = "deserialize_loglevel")]
    #[serde(default = "default_loglevel")]
    log_level: Level,
    #[serde(flatten)]
    options: SharedOptions,
}

const fn default_loglevel() -> Level {
    Level::INFO
}

fn deserialize_loglevel<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    let raw: String = String::deserialize(deserializer)?;
    let level = match raw.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    Ok(level)
}
pub struct DynamicLogger {
    config: LogConfig,
    layers: RefCell<Vec<Box<dyn Layer<Registry> + Send + Sync>>>,
    guards: RefCell<Vec<WorkerGuard>>,
}

impl DynamicLogger {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let config = toml::from_str(&read_to_string(path.as_ref())?)?;
        Ok(Self {
            config,
            layers: RefCell::new(Vec::new()),
            guards: RefCell::new(Vec::new()),
        })
    }

    fn init_stdout(&self) -> Result<()> {
        let options = self.config.stream_logger.options.clone();
        if !self.config.stream_logger.options.enabled {
            return Ok(());
        }

        match Targets::from_str(&self.config.stream_logger.modules.join(",")) {
            Ok(targets) => {
                let stream_layer = fmt::Layer::new()
                    .with_writer(std::io::stdout)
                    .with_file(options.file)
                    .with_line_number(options.line_number)
                    .with_thread_names(options.thread_name)
                    .with_thread_ids(options.thread_id);
                let format = &options.format.clone();
                let layer = match format {
                    LogFormat::Full => stream_layer
                        .with_ansi(self.config.stream_logger.color)
                        .with_filter(targets)
                        .boxed(),
                    LogFormat::Compact => stream_layer
                        .with_ansi(self.config.stream_logger.color)
                        .compact()
                        .with_filter(targets)
                        .boxed(),
                    LogFormat::Pretty => stream_layer.pretty().with_filter(targets).boxed(),
                    LogFormat::Json => stream_layer.json().with_filter(targets).boxed(),
                };

                self.layers.borrow_mut().push(layer);
                Ok(())
            }
            Err(msg) => {
                eprintln!(
                "Error parsing file targets. stdout logging failed to initialize, config has errors: {:#?}",
                &self.config.stream_logger);
                Err(anyhow::anyhow!(msg))
            }
        }
    }

    fn register_filelogger_target(&self, entry: &FileLogger) {
        let log_dir = &entry.path;
        let dirs = create_dir_all(log_dir);
        if dirs.is_ok() && log_dir.exists() {
            let appender = tracing_appender::rolling::never(log_dir, &entry.filename);
            let (file_writer, guard) = tracing_appender::non_blocking(appender);
            self.guards.borrow_mut().push(guard);

            let options = entry.options.clone();
            match Targets::from_str(&entry.modules.join(",")) {
                Ok(file_targets) => {
                    let file_layer = fmt::Layer::new()
                        .with_writer(file_writer)
                        .with_ansi(false)
                        .with_file(options.file)
                        .with_line_number(options.line_number)
                        .with_thread_names(options.thread_name)
                        .with_thread_ids(options.thread_id);
                    let layer = match options.format {
                        LogFormat::Full => file_layer.with_filter(file_targets).boxed(),
                        LogFormat::Compact => {
                            file_layer.compact().with_filter(file_targets).boxed()
                        }
                        LogFormat::Pretty => file_layer.pretty().with_filter(file_targets).boxed(),
                        LogFormat::Json => file_layer.json().with_filter(file_targets).boxed(),
                    };

                    self.layers.borrow_mut().push(layer);
                }
                Err(_) => {
                    eprintln!("Error parsing file targets. failed to initialize file logging for {entry:#?}");
                }
            }
        }
    }

    fn init_filelogger(&self) {
        if let Some(file_logger_table) = &self.config.file_logger {
            file_logger_table
                .iter()
                .filter(|file| file.options.enabled)
                .for_each(|entry| {
                    self.register_filelogger_target(entry);
                });
        }
    }

    pub fn init(&self) -> Result<()> {
        let envfilter =
            EnvFilter::from_default_env().add_directive(self.config.global.log_level.into());
        self.init_stdout()?;
        self.init_filelogger();
        let options = self.config.global.options.clone();
        let layer = fmt::layer()
            .with_file(options.file)
            .with_line_number(options.line_number)
            .with_thread_names(options.thread_name)
            .with_thread_ids(options.thread_id);

        let env_layer = match self.config.global.options.format {
            LogFormat::Full => layer.with_filter(envfilter).boxed(),
            LogFormat::Compact => layer.compact().with_filter(envfilter).boxed(),
            LogFormat::Pretty => layer.pretty().with_filter(envfilter).boxed(),
            LogFormat::Json => layer.json().with_filter(envfilter).boxed(),
        };
        self.layers.borrow_mut().push(env_layer);
        tracing_subscriber::registry()
            .with(self.layers.take())
            .init();
        Ok(())
    }
}
