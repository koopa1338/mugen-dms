use common::models::collection::Collection;
use common::models::document::Document;
use gloo_net::http::Method;
use leptos::html;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_use::{use_scroll_with_options, UseScrollOptions};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::api::api_call;
use crate::components::grid::Grid;
use crate::ChronoFormat;

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

    let (collection, set_collection) = create_signal::<Option<Collection>>(None);
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(false);
    let (editing, set_editing) = create_signal(false);
    let (new_name, set_new_name) = create_signal(String::new());

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

    create_effect(move |_| {
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

    let col_for_view = collection.clone();
    let loading_for_view = loading.clone();
    let error_for_view = error.clone();
    let editing_for_view = editing.clone();
    let set_editing_for_view = set_editing.clone();

    view! {
        <div class="rounded bg-gray-900 p-4 mb-3">
            <Show when=move || loading_for_view.get() fallback=move || {
                view! {
                    <Show when=move || error_for_view.get() fallback=move || {
                        view! {
                            <Show when=move || col_for_view.get().is_some() fallback=move || view! { <p class="text-gray-400">"Collection not found"</p> }>
                                <div class="flex items-center justify-between mb-3">
                                    {col_for_view.read().as_ref().map(|col| view! {
                                        <div>
                                            <h2 class="text-xl font-bold text-white">{col.name.clone()}</h2>
                                            <p class="text-gray-500 text-sm mt-1">"ID: " {col.id.clone().unwrap_or_default()}</p>
                                        </div>
                                        <div class="flex gap-2">
                                            <button
                                                class="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 text-sm"
                                                on:click=move |_| set_editing_for_view.update(|v| *v = true)
                                            >"Rename"</button>
                                            <button
                                                class="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 text-sm"
                                                on:click=move |_| handle_delete()
                                            >"Delete"</button>
                                        </div>
                                    })}
                                </div>
                                <div class="flex gap-6 text-sm text-gray-400">
                                    {col_for_view.read().as_ref().map(|col| view! {
                                        <>
                                            <span>"Created: " {col.created_at.display()}</span>
                                            <span>"Updated: " {col.updated_at.display()}</span>
                                        </>
                                    })}
                                </div>
                            </Show>
                        }
                    }>
                        <p class="text-red-400">Failed to load collection</p>
                    </Show>
                }
            }>
                <p class="text-gray-400">Loading...</p>
            </Show>

            // Rename form
            <Show when=move || editing_for_view.get() fallback=|| view! {}.into_view()>
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
            let result = api_call::<Vec<Document>, ()>(
                &format!("api/documents?collection_id={id}"),
                Method::GET,
                None,
                Some(vec![("page", &page_val.to_string())]),
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

    let initial_fetch = true;

    Effect::new(move |_| {
        if user_scroll_return.arrived_state.get().bottom && !user_scroll_return.is_scrolling.get() && has_more.get() && !loading.get() {
            page.update(|p| *p = p.saturating_add(1));
            (user_scroll_return.measure)();
            fetch_documents(page.get(), true);
        } else if initial_fetch && page.get() == 1 {
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
