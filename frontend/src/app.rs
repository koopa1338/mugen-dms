use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::sidebar::Sidebar;
use crate::routes::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Sidebar/>
            <main class="bg-gray-900 p-3 sm:ml-64">
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/documents/:id" view=Home />
                    <Route path="/categories/:id" view=Home />
                    <Route path="/collections/:id" view=Home />
                    <Route path="/settings" view=Home />
                    <Route path="/about" view=Home />
                </Routes>
            </main>
        </Router>
    }
}
