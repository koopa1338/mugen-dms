use common::models::collection::Collection;
use common::models::document::Document;
use gloo_net::http::Method;
use leptos::html;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use leptos_use::{use_scroll_with_options, UseScrollOptions};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::api::api_call;
use crate::components::grid::Grid;
use crate::ChronoFormat;

#[component]
pub(crate) fn CollectionList() -> impl IntoView {
    let (collections, set_collections) = signal(Vec::<Collection>::new());
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(false);
    let (creating, set_creating) = signal(false);
    let (new_name, set_new_name) = signal(String::new());

    let fetch_collections = move || {
        spawn_local(async move {
            set_loading.set(true);
            match api_call::<Vec<Collection>, ()>("api/collections", Method::GET, None, None).await {
                Ok(cols) => { set_collections.set(cols); set_error.set(false); }
                Err(_) => set_error.set(true),
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| { fetch_collections(); });

    let show_collections = move || !loading.get() && !error.get() && !collections.read().is_empty();

    let handle_create = move || {
        let name = new_name.get_untracked();
        if name.is_empty() { return; }
        spawn_local(async move {
            let _ = api_call::<Collection, _>(
                "api/collections", Method::POST,
                Some(serde_json::json!({ "name": name })), None,
            ).await;
            set_creating.set(false);
            set_new_name.set(String::new());
            fetch_collections();
        });
    };

    view! {
        <div class="space-y-3">
            <div class="flex items-center justify-between">
                <h2 class="text-xl font-bold text-white">"Collections"</h2>
                <button class="px-3 py-1 bg-amber-600 text-white rounded hover:bg-amber-700 text-sm" on:click=move |_| set_creating.update(|v| *v = true)>"+ New Collection"</button>
            </div>

            <Show when=move || creating.get() fallback=|| ().into_view()>
                <div class="rounded bg-gray-900 p-3 flex gap-2">
                    <input type="text" placeholder="Collection name" class="bg-gray-800 text-white px-3 py-1 rounded border border-gray-700 focus:border-blue-500 focus:outline-none flex-1" prop:value=new_name on:input=move |ev| { if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) { set_new_name.set(input.value()); } } on:keydown=move |ev| if ev.key_code() == 13 { handle_create(); } />
                    <button class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm" on:click=move |_| handle_create()>"Create"</button>
                    <button class="px-3 py-1 bg-gray-600 text-white rounded hover:bg-gray-700 text-sm" on:click=move |_| set_creating.set(false)>"Cancel"</button>
                </div>
            </Show>

            <Show when=show_collections fallback=move || {
                if loading.get() {
                    view! { <p class="text-gray-400">"Loading..."</p> }.into_view()
                } else if error.get() {
                    view! { <p class="text-red-400">"Failed to load collections"</p> }.into_view()
                } else {
                    view! { <p class="text-gray-400">"No collections yet. Create one to get started."</p> }.into_view()
                }
            }>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                    <For each=move || collections.read().clone() key=|col| col.id.clone() let:col>
                        <A href=format!("/collections/{}", col.id.clone().unwrap_or_default())>
                            <div class="text-left rounded bg-gray-900 p-4 hover:bg-gray-800 transition-all duration-100 border border-transparent hover:border-slate-700">
                                <h3 class="text-lg font-semibold text-white">{col.name.clone()}</h3>
                                <p class="text-gray-500 text-sm mt-1">"Created: " {col.created_at.display()}</p>
                            </div>
                        </A>
                    </For>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn CollectionDetail() -> impl IntoView {
    view! {
        <Grid classes="grid-cols-1 mb-4 h-full">
            <CollectionInfo/>
            <DocumentsByCollection/>
        </Grid>
    }
}

#[component]
fn CollectionInfo() -> impl IntoView {
    let params = use_params_map();
    let collection_id = move || {
        let p = params.get();
        p.get("id").clone().unwrap_or_default()
    };

    let (collection, set_collection) = signal::<Option<Collection>>(None);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(false);
    let (editing, set_editing) = signal(false);
    let (new_name, set_new_name) = signal(String::new());

    let fetch_collection = move || {
        let id = collection_id();
        if id.is_empty() {
            return;
        }
        spawn_local(async move {
            set_loading.set(true);
            match api_call::<Collection, ()>(
                &format!("api/collections/{id}"),
                Method::GET,
                None,
                None,
            ).await {
                Ok(col) => {
                    set_collection.set(Some(col));
                    set_error.set(false);
                }
                Err(_) => set_error.set(true),
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| {
        fetch_collection();
    });

    let handle_rename = move || {
        let id = collection_id();
        let name = new_name.get_untracked();
        if name.is_empty() {
            return;
        }
        spawn_local(async move {
            let _result = api_call::<Collection, _>(
                &format!("api/collections/{id}"),
                Method::PUT,
                Some(serde_json::json!({ "name": name })),
                None,
            ).await;
            set_editing.set(false);
            fetch_collection();
        });
    };

    let handle_delete = move || {
        let id = collection_id();
        spawn_local(async move {
            let _result = api_call::<Collection, ()>(
                &format!("api/collections/{id}"),
                Method::DELETE,
                None,
                None,
            ).await;
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message("Collection deleted");
            }
        });
    };

    let show_collection = move || !loading.get() && !error.get() && collection.get().is_some();

    view! {
        <div class="rounded bg-gray-900 p-4 mb-3">
            <Show when=show_collection fallback=move || {
                if loading.get() {
                    view! { <p class="text-gray-400">"Loading..."</p> }.into_view()
                } else if error.get() {
                    view! { <p class="text-red-400">"Failed to load collection"</p> }.into_view()
                } else {
                    view! { <p class="text-gray-400">"Collection not found"</p> }.into_view()
                }
            }>
                <div class="flex items-center justify-between mb-3">
                    {collection.read().as_ref().map(|col| view! {
                        <div>
                            <h2 class="text-xl font-bold text-white">{col.name.clone()}</h2>
                            <p class="text-gray-500 text-sm mt-1">"ID: " {col.id.clone().unwrap_or_default()}</p>
                        </div>
                        <div class="flex gap-2">
                            <button
                                class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm"
                                on:click=move |_| set_editing.update(|v| *v = true)
                            >"Rename"</button>
                            <button
                                class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 text-sm"
                                on:click=move |_| handle_delete()
                            >"Delete"</button>
                        </div>
                    })}
                </div>
                <div class="flex gap-6 text-sm text-gray-400">
                    {collection.read().as_ref().map(|col| view! {
                        <>
                            <span>"Created: " {col.created_at.display()}</span>
                            <span>"Updated: " {col.updated_at.display()}</span>
                        </>
                    })}
                </div>
            </Show>

            // Rename form
            <Show when=move || editing.get() fallback=|| ().into_view()>
                <div class="flex gap-2 mt-2">
                    <input
                        type="text"
                        class="bg-gray-800 text-white px-3 py-1 rounded border border-gray-700 focus:border-blue-500 focus:outline-none"
                        prop:value=new_name
                        on:input=move |ev| {
                            if let Some(input) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                set_new_name.set(input.value());
                            }
                        }
                        on:keydown=move |ev| if ev.key_code() == 13 { handle_rename() }
                    />
                    <button
                        class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm"
                        on:click=move |_| handle_rename()
                    >"Save"</button>
                    <button
                        class="px-3 py-1 bg-gray-600 text-white rounded hover:bg-gray-700 text-sm"
                        on:click=move |_| set_editing.set(false)
                    >"Cancel"</button>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn DocumentsByCollection() -> impl IntoView {
    let params = use_params_map();
    let collection_id = move || {
        let p = params.get();
        p.get("id").clone().unwrap_or_default()
    };

    let el = NodeRef::<html::Tbody>::new();
    let page = RwSignal::new(1u32);
    let data = RwSignal::new(Vec::<Document>::new());
    let loading = RwSignal::new(false);
    let has_more = RwSignal::new(true);

    let user_scroll_return = use_scroll_with_options(el, UseScrollOptions::default());

    let fetch_documents = move |page_val: u32, append: bool| {
        let id = collection_id();
        if id.is_empty() {
            return;
        }
        spawn_local(async move {
            loading.set(true);
            // Frontend uses 1-indexed pages; backend expects 0-indexed
            let backend_page = page_val.saturating_sub(1);
            let result = api_call::<Vec<Document>, ()>(
                &format!("api/documents?collection_id={id}"),
                Method::GET,
                None,
                Some(vec![("page", &backend_page.to_string())]),
            ).await;
            loading.set(false);

            match result {
                Ok(docs) => {
                    if !append || page_val == 1 {
                        data.update(Vec::clear);
                        has_more.set(!docs.is_empty());
                    }
                    if docs.is_empty() {
                        has_more.set(false);
                    } else {
                        data.update(|d| d.extend_from_slice(&docs));
                    }
                }
                Err(_) => {
                    if append {
                        page.update(|p| {
                            if *p > 1 {
                                *p = p.saturating_sub(1);
                            }
                        });
                    }
                }
            }
        });
    };

    Effect::new(move |_| {
        if user_scroll_return.arrived_state.get().bottom && !user_scroll_return.is_scrolling.get() && has_more.get() && !loading.get() {
            page.update(|p| *p = p.saturating_add(1));
            (user_scroll_return.measure)();
            fetch_documents(page.get(), true);
        } else if page.get() == 1 {
            fetch_documents(1, false);
        }
    });

    fetch_documents(1, false);

    view! {
        <div class="rounded bg-gray-900 p-3 h-[calc(100vh-16rem)]">
            <h3 class="text-lg font-semibold text-white mb-2">"Documents"</h3>
            <table class="table-fixed text-gray-500 w-full h-full">
                <thead class="table w-full table-fixed mb-1">
                    <tr class="border-b border-slate-100 font-medium">
                        <th>"Filename"</th>
                        <th>"Type"</th>
                        <th>"Size"</th>
                        <th>"Created"</th>
                        <th>"Updated"</th>
                    </tr>
                </thead>
                <Suspense fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    <Show when=move || !data.read().is_empty() fallback=|| view! { <p class="text-gray-400">"No documents in this collection"</p> }>
                        <tbody
                            node_ref=el
                            class="w-full h-[calc(100%-3em)] overflow-y-scroll overflow-x-auto overflow-x-wrap-break-word scrollbar-thin block"
                        >
                            <For each=move || data.read().clone() key=|doc| doc.id.clone() let:doc>
                                <tr class="table w-full table-fixed hover:bg-gray-800 cursor-pointer">
                                    <td class="py-2 text-white">{doc.filename}</td>
                                    <td class="py-2">{doc.filetype.clone().unwrap_or_else(|| "—".to_string())}</td>
                                    <td class="py-2">{doc.size.to_string()}</td>
                                    <td class="py-2">{doc.created_at.display()}</td>
                                    <td class="py-2">{doc.updated_at.display()}</td>
                                </tr>
                            </For>
                        </tbody>
                    </Show>
                </Suspense>
            </table>
        </div>
    }
}
