#![allow(unused_macros)]

use std::fmt::Debug;
use std::fs::{create_dir_all, read_to_string};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;

use serde::Deserialize;

use anyhow::Result;
use dotenv::dotenv;
use tracing::metadata::LevelFilter;
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

// #[derive(Debug, Deserialize)]
// #[serde(rename_all(deserialize = "lowercase"))]
// pub enum LogFormat {
//     Full,
//     Compact,
//     Pretty,
//     Json,
// }

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

pub fn init_logging() -> Result<Vec<WorkerGuard>> {
    let conf: LogConfig = toml::from_str(&read_to_string(dotenv::var("LOGGING")?)?)?;

    let mut layers: Vec<Filtered<fmt::Layer<_, DefaultFields, Format, _>, Targets, FmtSubscriber>> =
        Vec::new();
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
                // TODO: call fn on layer with parsed format
                let file_layer = fmt::Layer::new()
                    .with_writer(file_writer)
                    .with_ansi(false)
                    .with_filter(file_targets);

                layers.push(file_layer);
            });
    }

    let targets = Targets::from_str(&conf.stream_logger.modules.join(","))?;
    // TODO: call fn on layer with parsed format
    let stream_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_filter(targets);

    // TODO: how to register multiple layers on the same subsciber dynamically?
    // already tried:
    //      - combine layers with `and_then(layer)`
    //      - concat `with(layer)` calls
    //          - not possible because types of stdout and layer with file writers are different
    tracing_subscriber::registry().with(stream_layer).init();

    Ok(guards)
}
