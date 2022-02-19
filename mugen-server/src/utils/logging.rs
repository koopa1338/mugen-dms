#![allow(unused_macros)]

use std::fs::{create_dir_all,read_to_string};
use std::str::FromStr;
use std::path::PathBuf;

use serde::Deserialize;

use anyhow::Result;
use dotenv::dotenv;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::{
    filter::{Filtered, Targets},
    fmt::{self, format::DefaultFields, format::Format},
    prelude::*,
    FmtSubscriber, Layer,
};

// #[derive(Debug, Deserialize)]
// #[serde(rename_all(deserialize = "lowercase"))]
// pub enum LogFormat {
//     Full,
//     Compact, Pretty, Json, }

#[derive(Debug, Deserialize)]
pub struct FileLogger {
    pub enabled: bool,
    pub filename: String,
    pub path: PathBuf,
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamLogger {
    pub enabled: bool,
    pub modules: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub stream_logger: StreamLogger,
    pub file_logger: Option<Vec<FileLogger>>,
}

macro_rules! register_layers {
    () => {
        todo!()
    };
}

pub fn init_logging() -> Result<Vec<WorkerGuard>> {
    let conf: LogConfig = toml::from_str(&read_to_string(dotenv::var("LOGGING")?)?)?;

    let mut layers: Vec<
        Filtered<fmt::Layer<_, DefaultFields, Format, NonBlocking>, Targets, FmtSubscriber>,
    > = Vec::new();
    let mut guards = Vec::new();

    if let Some(file_logger_table) = conf.file_logger {
        file_logger_table
            .into_iter()
            .filter(|file| file.enabled)
            .for_each(|entry| {
                let log_dir = entry.path;
                if !log_dir.exists() {
                    create_dir_all(&log_dir).unwrap();
                }
                let appender = tracing_appender::rolling::never(log_dir, entry.filename);
                let (file_writer, guard) = tracing_appender::non_blocking(appender);
                guards.push(guard);

                let file_targets = Targets::from_str(&entry.modules.join(",")).unwrap();
                let file_layer = fmt::layer()
                    .with_writer(file_writer)
                    .with_filter(file_targets);

                layers.push(file_layer);
            });
    }

    let targets = Targets::from_str(&conf.stream_logger.modules.join(","))?;
    // TODO: call fn on layer with parsed format
    let stream_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(targets);

    tracing_subscriber::registry().with(stream_layer).init();

    Ok(guards)
}
