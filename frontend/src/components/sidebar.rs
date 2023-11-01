use leptos::*;
use leptos_router::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside
            id="separator-sidebar"
            class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
            aria-label="Sidebar"
        >
            <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
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
                class=move || {
                    if seperator {
                        "pt-4 mt-4 border-t border-gray-200 dark:border-gray-700"
                    } else {
                        ""
                    }
                }
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
                class="flex items-center p-2 text-gray-900 rounded-sm dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group"
            >
                <span class="ml-3">{label}</span>
            </A>
        </li>
    }
}
