pub mod api;
pub mod app;
pub mod components;
pub mod routes;

trait ChronoFormat {
    fn display(&self) -> String;
}

const DATETIME_FORMAT_STRING: &str = "%d.%m.%Y %H:%M";

impl ChronoFormat for common::DateTimeWithTimeZone {
    fn display(&self) -> String {
        self.format(DATETIME_FORMAT_STRING).to_string()
    }
}
