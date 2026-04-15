use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside
            id="separator-sidebar"
            class=r"
            fixed top-0 left-0 z-40
            w-60 h-screen
            transition-transform -translate-x-full sm:translate-x-0
            border-r-2 border-r-solid border-r-slate-900"
            aria-label="Sidebar"
        >
            <div class="h-full px-2 py-4 overflow-y-auto bg-gray-950 ">
                <img src="assets/logo.png" class="space-y-2 mx-auto mb-3 w-20 rounded-full"/>
                <MenuSection seperator=false>
                    <MenuEntry href="/" label="Dashboard"/>
                    <MenuEntry href="/documents" label="Documents"/>
                    <MenuEntry href="/categories" label="Categories"/>
                </MenuSection>
                <MenuSection seperator=true>
                    <MenuEntry href="/settings" label="Settings"/>
                    <MenuEntry href="/collections" label="Collections"/>
                </MenuSection>
                <MenuEntry href="/about" label="About"/>
            </div>
        </aside>
    }
}

#[component]
pub fn MenuSection(#[prop(optional)] seperator: bool, children: Children) -> impl IntoView {
    view! {
        <section>
            <ul
                class="space-y-2 font-medium"
                class=move || { if seperator { "pt-4 mt-4 border-t border-gray-700" } else { "" } }
            >

                {children()}
            </ul>
        </section>
    }
}

#[component]
pub fn MenuEntry<H: ToHref + 'static>(href: H, #[prop()] label: &'static str) -> impl IntoView {
    view! {
        <li class="list-none">
            <A
                href=href
                class=r"
                transition-all duration-100 ease-in-out
                flex items-center p-2 group
                rounded-md hover:bg-slate-900
                text-white hover:text-amber-600 hover:font-semibold"
            >
                <span class="ml-2">{label}</span>
            </A>
        </li>
    }
}
