use leptos::*;

#[component]
pub fn Grid(#[prop(optional)] classes: Option<&'static str>, children: Children) -> impl IntoView {
    let classes = classes.unwrap_or("grid-cols-1");

    view! {
        <div class=format!("grid gap-4 {classes}")>
            {children()}
        </div>
    }
}
