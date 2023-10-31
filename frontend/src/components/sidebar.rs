use leptos::*;
use leptos_router::*;
// use mugen_frontend::ToggleSignal;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="sidebar">
            <ul>
                <SingleMenu href="/kekw" label="kekw"/>
                <SingleMenu href="/kekl" label="kekl"/>
                <SingleMenu href="/lul" label="lul"/>
                <SingleMenu href="/wat" label="wat"/>
                <SingleMenu href="/nononono" label="trolololo"/>
            </ul>
        </nav>
    }
}

/*
TODO: find a way to make the transition of collapsibles only with tailwind css or a feasable
workaround
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
            <ul class=move || if !collapsed.get() {
                    "transition-all duration-200 ease-in-out max-h-0 opacity-0"
                } else {
                    "transition-all duration-300 ease-in max-h-none opacity-100"
                } >
                {children()}
            </ul>
        </li>
    }
}
*/

#[component]
pub fn SingleMenu<H: ToHref + 'static>(
    href: H,
    #[prop(optional)] label: &'static str,
) -> impl IntoView {
    view! {
        <li>
            <A href=href>{label}</A>
        </li>
    }
}
