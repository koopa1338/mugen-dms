use leptos::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="rounded bg-gray-900 p-3 text-gray-500">
            <h2 class="text-2xl font-bold mb-4">"Overview"</h2>
            <p>
                "Mugen-dms is an open-source document management software, offering powerful and efficient solutions built entirely in Rust. It utilizes Leptos for the frontend and Axum with Sea ORM for the backend, ensuring a seamless and performant user experience."
            </p>

            <h2 class="text-2xl font-bold mt-6 mb-4">"Key Perks"</h2>
            <ul class="list-disc ml-6">
                <li>
                    "Outstanding Performance: Mugen-dms is designed for high-speed document handling and management."
                </li>
                <li>
                    "Emphasis on Safety: Rust's strong type system and memory safety guarantee secure operations."
                </li>
                <li>
                    "Low Resource Footprint: We prioritize efficiency and minimize resource consumption."
                </li>
                <li>
                    "Easy Setup: Mugen-dms comes with a straightforward and user-friendly installation process."
                </li>
                <li>
                    "Open Source: Developed and maintained by a single contributor, available for the community."
                </li>
            </ul>

            <h2 class="text-2xl font-bold mt-6 mb-4">"Contact the Maintainer"</h2>
            <p>
                "If you have any issues or feature requests, feel free to open an issue on github "
                <a
                    class="transition-all duration-100 ease-in-out text-bold text-white hover:text-amber-600"
                    href="https://github.com/koopa1338/mugen-dms"
                    target="_blank"
                    title="Link to github repository"
                >
                    here
                </a> "."
            </p>
        </div>
    }
}
