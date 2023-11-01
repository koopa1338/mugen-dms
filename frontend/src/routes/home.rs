use leptos::*;

#[component]
pub(crate) fn Home() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 gap-4 mb-4">
            <div class="flex items-center justify-center h-24 rounded bg-gray-800">
                <p class="text-2xl text-gray-500">TODO</p>
            </div>
        </div>
    }
}
