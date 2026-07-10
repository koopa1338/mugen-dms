use common::models::document::Document;
use gloo_net::http::Method;
use leptos::prelude::*;
use leptos::html;
use leptos_router::hooks::use_params_map;
use leptos_use::{use_scroll_with_options, UseScrollOptions};
use wasm_bindgen_futures::spawn_local;

use crate::api::api_call;
use crate::components::grid::Grid;
use crate::ChronoFormat;

fn download_doc(doc_id: String) {
    if doc_id.is_empty() { return; }
    let url = format!("/api/documents/{}/download", doc_id);
    if let Some(window) = web_sys::window() {
        let _ = window.open_with_url(&url);
    }
}

#[component]
pub(crate) fn DocumentDetail() -> impl IntoView {
    let params = use_params_map();

    let (doc, set_doc) = signal::<Option<Document>>(None);
    let (loading, set_loading) = signal(true);
    let (error, set_error) = signal(false);

    let fetch_doc = move || {
        let id = params.get().get("id").clone().unwrap_or_default();
        if id.is_empty() { return; }
        spawn_local(async move {
            set_loading.set(true);
            match api_call::<Document, ()>(&format!("api/documents/{id}"), Method::GET, None, None).await {
                Ok(d) => { set_doc.set(Some(d)); set_error.set(false); }
                Err(_) => set_error.set(true),
            }
            set_loading.set(false);
        });
    };

    Effect::new(move |_| { fetch_doc(); });

    let show_content = move || !loading.get() && !error.get() && doc.get().is_some();

    view! {
        <div class="space-y-4">
            <Show when=show_content fallback=move || {
                if loading.get() {
                    view! { <p class="text-gray-400">"Loading..."</p> }.into_view()
                } else if error.get() {
                    view! { <p class="text-red-400">"Failed to load document"</p> }.into_view()
                } else {
                    view! { <p class="text-gray-400">"Document not found"</p> }.into_view()
                }
            }>
                <div class="rounded bg-gray-900 p-4">
                    <div class="flex items-center justify-between mb-3">
                        <div>
                            <h2 class="text-xl font-bold text-white">{doc.read().as_ref().map(|d| d.filename.clone()).unwrap_or_default()}</h2>
                            <p class="text-gray-500 text-sm mt-1">"ID: " {doc.read().as_ref().map(|d| d.id.clone()).unwrap_or_default()}</p>
                        </div>
                        <button class="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm" on:click=move |_| download_doc(doc.read().as_ref().and_then(|d| d.id.clone()).unwrap_or_default())>"Download"</button>
                    </div>
                    <div class="flex gap-6 text-sm text-gray-400">
                        <span>"Type: " {doc.read().as_ref().and_then(|d| d.filetype.clone()).unwrap_or_else(|| "\u{2014}".to_string())}</span>
                        <span>"Size: " {doc.read().as_ref().map(|d| d.size.to_string()).unwrap_or_default()}</span>
                    </div>
                    <div class="flex gap-6 text-sm text-gray-400 mt-2">
                        <span>"Created: " {doc.read().as_ref().map(|d| d.created_at.display()).unwrap_or_default()}</span>
                        <span>"Updated: " {doc.read().as_ref().map(|d| d.updated_at.display()).unwrap_or_default()}</span>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub(crate) fn Documents() -> impl IntoView {
    view! {
        <Grid classes="grid-cols-1 mb-4 h-full">
            <DocumentsTable/>
        </Grid>
    }
}

#[component]
pub(crate) fn DocumentsTable() -> impl IntoView {
    let el = NodeRef::<html::Tbody>::new();
    let page = RwSignal::new(1u32);
    let data = RwSignal::new(Vec::<Document>::new());
    let loading = RwSignal::new(false);
    let has_more = RwSignal::new(true);

    let user_scroll_return = use_scroll_with_options(el, UseScrollOptions::default());

    let fetch_documents = move |page_val: u32, append: bool| {
        spawn_local(async move {
            loading.set(true);
            // Frontend uses 1-indexed pages; backend expects 0-indexed
            let backend_page = page_val.saturating_sub(1);
            let result = api_call::<Vec<Document>, ()>(
                "api/documents",
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
        }
    });

    fetch_documents(1, false);

    view! {
        <div class="rounded bg-gray-900 p-3 h-full">
            <table class="table-fixed text-gray-500 w-full h-full">
                <thead class="table w-full table-fixed mb-1">
                    <tr class="border-b border-slate-100 font-medium">
                        <th>"ID"</th>
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
                    <Show when=move || !data.read().is_empty() fallback=|| view! { <p>"No documents"</p> }>
                        <tbody
                            node_ref=el
                            class="w-full h-[calc(100%-2em)] overflow-y-scroll overflow-x-auto overflow-x-wrap-break-word scrollbar-thin block"
                        >
                            <For each=move || data.read().clone() key=|doc| doc.id.clone() let:doc>
                                <tr class="table w-full table-fixed">
                                    <td class="py-2">{doc.id}</td>
                                    <td class="py-2">{doc.filename}</td>
                                    <td class="py-2">{doc.filetype}</td>
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
