use serde::Deserialize;
use std::path::PathBuf;
use tracing::Level;

use crate::utils::{default_as_true, default_loglevel, deserialize_loglevel};

#[derive(Debug, Deserialize)]
pub(crate) struct GlobalLogger {
    #[serde(deserialize_with = "deserialize_loglevel")]
    #[serde(default = "default_loglevel")]
    pub(crate) log_level: Level,
    #[serde(flatten)]
    pub(crate) options: SharedOptions,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FileLogger {
    pub(crate) filename: String,
    pub(crate) path: PathBuf,
    #[serde(flatten)]
    pub(crate) options: SharedOptions,
    pub(crate) modules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SharedOptions {
    #[serde(default = "default_as_true")]
    pub(crate) enabled: bool,
    #[serde(default)]
    pub(crate) format: LogFormat,
    #[serde(default)]
    pub(crate) line_number: bool,
    #[serde(default)]
    pub(crate) file: bool,
    #[serde(default)]
    pub(crate) thread_name: bool,
    #[serde(default)]
    pub(crate) thread_id: bool,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StreamLogger {
    #[serde(default)]
    pub(crate) color: bool,
    #[serde(flatten)]
    pub(crate) options: SharedOptions,
    pub(crate) modules: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub(crate) enum LogFormat {
    #[default]
    Full,
    Compact,
    Pretty,
    Json,
}
