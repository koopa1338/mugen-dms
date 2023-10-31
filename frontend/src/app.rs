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
            <main>
                <Routes>
                    <Route path="" view=move || view! { <Home/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! { <div class="">"asdfasdf"</div> }
}
