use leptos::*;
pub mod api;
pub mod app;
pub mod components;
pub mod routes;

pub trait SignalToggle {
    fn toggle(&mut self);
}

impl SignalToggle for RwSignal<bool> {
    fn toggle(&mut self) {
        self.update(|value| *value = !*value);
    }
}

// TODO: get this to work so we can implement toggle for all signals where the containing type
// implements the `!` operator
// impl<T> SignalToggle for T
// where
//     T: SignalUpdate,
//     <T as leptos::SignalUpdate>::Value: Not,
// {
//     fn toggle(&mut self) {
//         self.update(|value| {
//             *value = !*value
//         });
//     }
// }

trait ChronoFormat {
    fn display(&self) -> String;
}

const DATETIME_FORMAT_STRING: &str = "%d.%m.%Y %H:%M";

impl ChronoFormat for common::DateTimeWithTimeZone {
    fn display(&self) -> String {
        self.format(DATETIME_FORMAT_STRING).to_string()
    }
}
