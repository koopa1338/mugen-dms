use crate::components::sidebar::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Sidebar/>
            <main class="bg-gray-900 p-3 sm:ml-64">
                <Routes>
                    <Route path="/" view=move || view! { <Home/> }/>
                    <Route path="/documents" view=move || view! { <Home/> }/>
                    <Route path="/categories" view=move || view! { <Home/> }/>
                    <Route path="/settings" view=move || view! { <Home/> }/>
                    <Route path="/collections" view=move || view! { <Home/> }/>
                    <Route path="/about" view=move || view! { <Home/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 gap-4 mb-4">
            <div class="flex items-center justify-center h-24 rounded bg-gray-800">
                <p class="text-2xl text-gray-500">TODO</p>
            </div>
        </div>
    }
}
