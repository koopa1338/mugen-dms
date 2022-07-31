use anyhow::Result;
use dotenv::dotenv;
use serde::Deserialize;
use std::fmt::Debug;
use std::fs::{create_dir_all, read_to_string};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::span::{Attributes, Record};
use tracing::subscriber::Interest;
use tracing::{Event, Id, Metadata, Subscriber};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{
    filter::{Filtered, Targets},
    fmt::{self, format::DefaultFields, format::Format},
    prelude::*,
    FmtSubscriber, Layer,
};

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

pub fn init_logging() -> Result<Vec<WorkerGuard>> {
    let conf: LogConfig = toml::from_str(&read_to_string(dotenv::var("LOGGING")?)?)?;

    let mut layers = Vec::new();
    let mut guards = Vec::new();

    match Targets::from_str(&conf.stream_logger.modules.join(",")) {
        Ok(targets) => {
            let stream_layer = fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_ansi(conf.stream_logger.color)
                .with_filter(targets)
                .boxed();

            layers.push(stream_layer);
        }
        Err(error) => {
            eprintln!(
                "Error parsing file targets. stdout logging failed to initialize, config has errors: {:#?}",
                &conf.stream_logger
            );
        }
    }

    if let Some(file_logger_table) = conf.file_logger {
        file_logger_table
            .into_iter()
            .filter(|file| file.enabled)
            .for_each(|entry| {
                let log_dir = &entry.path;
                if !log_dir.exists() {
                    if let Err(io) = create_dir_all(&log_dir) {
                        eprintln!("Error creating logging dir {}.", &log_dir.display());
                        return;
                    }
                }
                let appender = tracing_appender::rolling::never(log_dir, &entry.filename);
                let (file_writer, guard) = tracing_appender::non_blocking(appender);
                guards.push(guard);

                match Targets::from_str(&entry.modules.join(",")) {
                    Ok(file_targets) => {
                        let file_layer =
                            fmt::Layer::new().with_writer(file_writer).with_ansi(false);
                        let layer = match entry.format {
                            LogFormat::Full => file_layer.with_filter(file_targets).boxed(),
                            LogFormat::Compact => {
                                file_layer.compact().with_filter(file_targets).boxed()
                            }
                            LogFormat::Pretty => {
                                file_layer.pretty().with_filter(file_targets).boxed()
                            }
                            LogFormat::Json => file_layer.json().with_filter(file_targets).boxed(),
                        };

                        layers.push(layer);
                    }
                    Err(error) => {
                        eprintln!("Error parsing file targets. failed to initialize file logging for {:#?}", entry);
                        return;
                    }
                }
            });
    }
    tracing_subscriber::registry().with(layers).init();

    Ok(guards)
}
