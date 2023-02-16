use serde::{Deserializer, Deserialize};
use tracing::Level;

pub(crate) fn default_as_true() -> bool {
    true
}

pub(crate) const fn default_loglevel() -> Level {
    Level::INFO
}

pub(crate) fn deserialize_loglevel<'de, D>(deserializer: D) -> Result<Level, D::Error>
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
