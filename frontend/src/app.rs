use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::sidebar::Sidebar;
use crate::routes::{document::Documents, home::Home};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Sidebar/>
            <main class="bg-gray-950 p-3 sm:ml-60">
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/documents" view=Documents/>
                    <Route path="/documents/:id" view=Home/>
                    <Route path="/categories/:id" view=Home/>
                    <Route path="/categories" view=Home/>
                    <Route path="/collections/:id" view=Home/>
                    <Route path="/collections" view=Home/>
                    <Route path="/settings" view=Home/>
                    <Route path="/about" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
