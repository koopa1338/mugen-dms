use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="sidebar fixed top-0 bottom-0 lg:left-0 p-2 w-72 overflow-y-auto text-center bg-slate-900">
            <MenuSection label="Dashboard">
                <MenuEntry href="/" label="home"/>
                <MenuEntry href="/documents" label="Documents"/>
                <MenuEntry href="/categories" label="Categories"/>
            </MenuSection>
            <MenuSection label="Manage">
                <MenuEntry href="/settings" label="Settings"/>
                <MenuEntry href="/collections" label="Collections"/>
            </MenuSection>
            <MenuEntry href="/about" label="About"/>
        </aside>
    }
}

#[component]
pub fn MenuSection(#[prop()] label: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="my-5">
            <h2 class="text-red-400 font-bold">{label}</h2>
            <ul>{children()}</ul>
        </section>
    }
}

#[component]
pub fn MenuEntry<H: ToHref + 'static>(href: H, #[prop()] label: &'static str) -> impl IntoView {
    view! {
        <li class="list-none">
            <A href=href class="text-yellow-200 hover:text-yellow-400" >{label}</A>
        </li>
    }
}
