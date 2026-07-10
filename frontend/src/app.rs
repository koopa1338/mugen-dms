use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::sidebar::Sidebar;
use crate::routes::{
    about::About,
    collection::{CollectionDetail, CollectionList},
    dashboard::Dashboard,
    document::{DocumentDetail, Documents},
    settings::Settings,
};

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
                    <Route path=path!("/documents/:id") view=DocumentDetail/>
                    <Route path=path!("/collections/:id") view=CollectionDetail/>
                    <Route path=path!("/collections") view=CollectionList/>
                    <Route path=path!("/settings") view=Settings/>
                    <Route path=path!("/about") view=About/>
                </Routes>
            </main>
        </Router>
    }
}
