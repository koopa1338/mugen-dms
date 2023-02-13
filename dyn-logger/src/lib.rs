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
}

#[derive(Clone, Debug, Deserialize)]
struct SharedOptions {
    #[serde(default = "default_as_true")]
    enabled: bool,
    modules: Vec<String>,
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
}

#[derive(Debug, Deserialize)]
struct LogConfig {
    stream_logger: StreamLogger,
    file_logger: Option<Vec<FileLogger>>,
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
        

        match Targets::from_str(&options.modules.join(",")) {
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
            match Targets::from_str(&options.modules.join(",")) {
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
        self.init_stdout()?;
        self.init_filelogger();
        tracing_subscriber::registry()
            .with(self.layers.take())
            .init();
        Ok(())
    }
}
