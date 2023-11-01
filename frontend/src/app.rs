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
            <main class="ps-72">
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
    view! { <div class="w-full h-screen bg-slate-800 text-yellow-200">"TODO"</div> }
}
