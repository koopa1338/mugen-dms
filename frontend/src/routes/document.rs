use common::models::document::Document;
use gloo_net::http::Method;
use leptos::prelude::*;
use leptos::html;
use leptos_use::{use_scroll_with_options, UseScrollOptions};
use wasm_bindgen_futures::spawn_local;

use crate::api::api_call;
use crate::components::grid::Grid;
use crate::ChronoFormat;

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
            let result = api_call::<Vec<Document>, ()>(
                "api/documents",
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
