use anyhow::Result;
use serde::Deserialize;
use std::cell::RefCell;
use std::fmt::Debug;
use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::Registry;
use tracing_subscriber::{filter::Targets, fmt, prelude::*, Layer};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum LogFormat {
    Full,
    Compact,
    Pretty,
    Json,
}

#[derive(Debug, Deserialize)]
pub struct FileLogger {
    pub enabled: bool,
    pub filename: String,
    pub path: PathBuf,
    pub modules: Vec<String>,
    pub format: LogFormat,
}

#[derive(Debug, Deserialize)]
pub struct StreamLogger {
    pub enabled: bool,
    pub color: bool,
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub stream_logger: StreamLogger,
    pub file_logger: Option<Vec<FileLogger>>,
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
        match Targets::from_str(&self.config.stream_logger.modules.join(",")) {
            Ok(targets) => {
                let stream_layer = fmt::Layer::new()
                    .with_writer(std::io::stdout)
                    .with_ansi(self.config.stream_logger.color)
                    .with_filter(targets)
                    .boxed();

                self.layers.borrow_mut().push(stream_layer);
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

            match Targets::from_str(&entry.modules.join(",")) {
                Ok(file_targets) => {
                    let file_layer = fmt::Layer::new().with_writer(file_writer).with_ansi(false);
                    let layer = match entry.format {
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

    fn init_filelogger(&self) -> Result<()> {
        if let Some(file_logger_table) = &self.config.file_logger {
            file_logger_table
                .into_iter()
                .filter(|file| file.enabled)
                .for_each(|entry| {
                    self.register_filelogger_target(entry);
                });
        }

        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        self.init_stdout()?;
        self.init_filelogger()?;
        tracing_subscriber::registry()
            .with(self.layers.take())
            .init();
        Ok(())
    }
}
