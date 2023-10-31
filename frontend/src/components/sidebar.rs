use leptos::*;
use leptos_router::*;
use mugen_frontend::ToggleSignal;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="sidebar">
            <ul>
                <MultiMenu label={"Home"}>
                    <SingleMenu href="/kekw" label="kekw" />
                    <SingleMenu href="/kekl" label="kekl" />
                    <SingleMenu href="/lul" label="lul" />
                    <SingleMenu href="/wat" label="wat" />
                </MultiMenu>
                <SingleMenu href="/nononono" label="trolololo" />
            </ul>
        </nav>
    }
}

#[component]
pub fn MultiMenu(#[prop(optional)] label: &'static str, children: Children) -> impl IntoView {
    let mut collapsed = ToggleSignal::new(false);
    let toggle = move |_| collapsed.toggle();

    // Collapse dropdown then the url has been changed
    let location = use_location();
    create_effect(move |_| {
        location.pathname.track();
        collapsed.set(false);
    });

    view! {
        <li>
            <a on:click=toggle>{label}</a>
            <ul class="transition-all" class=("hidden", move || !collapsed.get())>
                {children()}
            </ul>
        </li>
    }
}

#[component]
pub fn SingleMenu<H: ToHref + 'static>(
    href: H,
    #[prop(optional)] label: &'static str,
) -> impl IntoView {
    view! {
        <li>
            <A href=href>
                {label}
            </A>
        </li>
    }
}
