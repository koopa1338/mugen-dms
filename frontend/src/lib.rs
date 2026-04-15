use leptos::*;
use std::ops::Not;
pub mod api;
pub mod app;
pub mod components;
pub mod routes;

pub trait SignalToggle {
    fn toggle(&mut self);
}

impl<T> SignalToggle for T
where
    T: SignalUpdate<Value = T> + Not<Output = T>,
{
    fn toggle(&mut self) {
        self.update(|&mut mut value| value = !value);
    }
}

trait ChronoFormat {
    fn display(&self) -> String;
}

const DATETIME_FORMAT_STRING: &str = "%d.%m.%Y %H:%M";

impl ChronoFormat for common::DateTimeWithTimeZone {
    fn display(&self) -> String {
        self.format(DATETIME_FORMAT_STRING).to_string()
    }
}
