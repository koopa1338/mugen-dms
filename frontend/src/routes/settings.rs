use leptos::prelude::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <div class="rounded bg-gray-900 p-4">
            <h2 class="text-2xl font-bold text-white mb-4">"Settings"</h2>

            <div class="space-y-4">
                <div class="border-b border-gray-800 pb-3">
                    <h3 class="text-lg font-semibold text-white mb-2">"Application"</h3>
                    <div class="grid grid-cols-2 gap-2 text-sm">
                        <span class="text-gray-400">"Version"</span>
                        <span class="text-white">"0.1.0"</span>
                        <span class="text-gray-400">"Storage Backend"</span>
                        <span class="text-white">"Filesystem"</span>
                        <span class="text-gray-400">"Database"</span>
                        <span class="text-white">"SurrealDB"</span>
                    </div>
                </div>

                <div class="border-b border-gray-800 pb-3">
                    <h3 class="text-lg font-semibold text-white mb-2">"User"</h3>
                    <p class="text-sm text-gray-400">"User settings will be available in a future update."</p>
                </div>

                <div>
                    <h3 class="text-lg font-semibold text-white mb-2">"Advanced"</h3>
                    <p class="text-sm text-gray-400">"Advanced configuration options will be available in a future update."</p>
                </div>
            </div>
        </div>
    }
}
