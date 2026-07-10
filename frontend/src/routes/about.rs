use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="rounded bg-gray-900 p-3 text-gray-500">
            <h2 class="text-2xl font-bold mb-4">"About Mugen-DMS"</h2>
            <p>
                "Mugen-DMS is an open-source document management system written in Rust. The frontend uses Leptos (compiled to WASM), and the backend runs on Axum with SurrealDB for persistence."
            </p>

            <h2 class="text-2xl font-bold mt-6 mb-4">"Tech Stack"</h2>
            <ul class="list-disc ml-6 space-y-1">
                <li>"Frontend: Leptos 0.8 / WASM / Tailwind CSS"</li>
                <li>"Backend: Axum 0.8 / Tokio"</li>
                <li>"Database: SurrealDB 3.x"</li>
                <li>"Authentication: JWT + Argon2 password hashing"</li>
                <li>"Storage: Pluggable filesystem/S3 backend"</li>
            </ul>

            <h2 class="text-2xl font-bold mt-6 mb-4">"Getting Help"</h2>
            <p>
                "Report issues or request features on GitHub:"
                <a
                    class="text-white hover:text-amber-600 underline"
                    href="https://github.com/koopa1338/mugen-dms"
                    target="_blank"
                >
                    github.com/koopa1338/mugen-dms
                </a>
            </p>
        </div>
    }
}
