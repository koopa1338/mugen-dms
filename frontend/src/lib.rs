use leptos::*;
pub mod app;
pub mod components;
pub mod routes;
pub mod api;


#[derive(Debug, Copy, Clone)]
pub struct ToggleSignal {
    inner: RwSignal<bool>,
}

impl ToggleSignal {
    pub fn new(value: bool) -> Self {
        Self {
            inner: create_rw_signal(value),
        }
    }

    pub fn toggle(&mut self) {
        self.inner.update(|inner| *inner = !*inner);
    }

    pub fn get(&self) -> bool {
        self.inner.get()
    }

    pub fn set(&self, value: bool) {
        self.inner.set(value);
    }
}
