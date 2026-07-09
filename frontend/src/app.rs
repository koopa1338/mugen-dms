use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;

use crate::components::sidebar::Sidebar;
use crate::routes::{about::About, dashboard::Dashboard, document::Documents};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Sidebar/>
            <main class="bg-gray-950 p-3 sm:ml-60 h-screen">
                <Routes fallback=|| view! { "Page not found" }>
                    <Route path=path!("/") view=Dashboard/>
                    <Route path=path!("/documents") view=Documents/>
                    <Route path=path!("/documents/:id") view=Dashboard/>
                    <Route path=path!("/collections/:id") view=Dashboard/>
                    <Route path=path!("/collections") view=Dashboard/>
                    <Route path=path!("/settings") view=Dashboard/>
                    <Route path=path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}
