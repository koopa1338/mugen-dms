use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside
            id="separator-sidebar"
            class="fixed top-0 left-0 z-40 w-60 h-screen transition-transform -translate-x-full sm:translate-x-0 border-r-2 border-r-solid border-r-slate-900"
            aria_label="Sidebar"
        >
            <div class="h-full px-2 py-4 overflow-y-auto bg-gray-950 ">
                <img src="assets/logo.png" class="space-y-2 mx-auto mb-3 w-20 rounded-full"/>
                <MenuSection separator=false>
                    <MenuEntry href="/" label="Dashboard"/>
                    <MenuEntry href="/documents" label="Documents"/>
                    <MenuEntry href="/collections" label="Collections"/>
                </MenuSection>
                <MenuSection separator=true>
                    <MenuEntry href="/settings" label="Settings"/>
                </MenuSection>
                <MenuEntry href="/about" label="About"/>
            </div>
        </aside>
    }
}

#[component]
pub fn MenuSection(#[prop(optional)] separator: bool, children: Children) -> impl IntoView {
    let classes = move || {
        if separator {
            "pt-4 mt-4 border-t border-gray-700"
        } else {
            ""
        }
    };
    view! {
        <section>
            <ul class=move || format!("space-y-2 font-medium {}", classes())>
                {children()}
            </ul>
        </section>
    }
}

#[component]
pub fn MenuEntry(href: &'static str, #[prop()] label: &'static str) -> impl IntoView {
    let location = use_location();
    let is_active = Memo::new(move |_| location.pathname.get() == href);
    let link_classes = move || {
        if is_active.get() {
            "transition-all duration-100 ease-in-out flex items-center p-2 group rounded-md bg-slate-900 text-amber-600 font-semibold"
        } else {
            "transition-all duration-100 ease-in-out flex items-center p-2 group rounded-md hover:bg-slate-900 text-white hover:text-amber-600 hover:font-semibold"
        }
    };
    view! {
        <li class="list-none">
            <A href=href>
                <span class=link_classes()>{label}</span>
            </A>
        </li>
    }
}
