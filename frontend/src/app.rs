use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::sidebar::Sidebar;
use crate::routes::{about::About, dashboard::Dashboard, document::Documents};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Sidebar/>
            <main class="bg-gray-950 p-3 sm:ml-60 h-screen">
                <Routes>
                    <Route path="/" view=Dashboard/>
                    <Route path="/documents" view=Documents/>
                    <Route path="/documents/:id" view=Dashboard/>
                    <Route path="/categories/:id" view=Dashboard/>
                    <Route path="/categories" view=Dashboard/>
                    <Route path="/collections/:id" view=Dashboard/>
                    <Route path="/collections" view=Dashboard/>
                    <Route path="/settings" view=Dashboard/>
                    <Route path="/about" view=About/>
                </Routes>
            </main>
        </Router>
    }
}
