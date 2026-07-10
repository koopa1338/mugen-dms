pub mod api;
pub mod app;
pub mod components;
pub mod routes;

trait ChronoFormat {
    fn display(&self) -> String;
}

const DATETIME_FORMAT_STRING: &str = "%d.%m.%Y %H:%M";

use chrono::{DateTime, Utc};

impl ChronoFormat for DateTime<Utc> {
    fn display(&self) -> String {
        self.format(DATETIME_FORMAT_STRING).to_string()
    }
}
