use common::models::document::Doc;
use gloo_net::http::Method;
use leptos::*;
use leptos_use::{use_infinite_scroll_with_options, UseInfiniteScrollOptions};

use crate::api::{api_call, FetchError};
use crate::ChronoFormat;

#[component]
pub(crate) fn Documents() -> impl IntoView {
    let el = create_node_ref::<html::Tbody>();
    let (page, page_set) = create_signal(1);
    let (data, set_data) = create_signal(vec![]);

    let _ = use_infinite_scroll_with_options(
        el,
        move |_| async move {
            // this is called too often
            page_set.update(|page| *page += 1);
        },
        UseInfiniteScrollOptions::default(),
    );

    let documents = create_resource(
        move || page.get(),
        move |page| async move {
            api_call::<Vec<Doc>, FetchError>(
                "api/doc",
                Method::GET,
                None,
                Some(vec![("page", &page.to_string())]),
            )
            .await
        },
    );

    create_effect(move |_| {
        documents.and_then(|data| {
            if data.is_empty() {
                // we hit the last page, no more elements
                return;
            }
            set_data.update(|d| d.extend_from_slice(data));
        });
    });

    let documents_view = move || {
        view! {
            <tbody
                node_ref=el
                class="w-full h-[calc(100%-2em)] overflow-y-scroll overflow-x-auto overflow-x-wrap-break-word scrollbar-thin block"
            >
                <For each=move || data.get() key=|doc: &Doc| doc.id let:doc>
                    <tr class="table w-full table-fixed">
                        <th class="py-2">{doc.id}</th>
                        <th class="py-2">{doc.version}</th>
                        <th class="py-2">{doc.created.display()}</th>
                        <th class="py-2">
                            {doc.updated.map_or("never".to_string(), |date| date.display())}
                        </th>
                        <th class="py-2">{doc.filetype}</th>
                    </tr>
                </For>
            </tbody>
        }
    };

    view! {
        <div class="grid grid-cols-1 gap-4 mb-4 h-full">
            <div class="rounded bg-gray-900 p-3 h-full">
                <table class="table-fixed text-gray-500 w-full h-full">
                    <thead class="table w-full table-fixed mb-1">
                        <tr class="border-b border-slate-100 font-medium">
                            <th>"ID"</th>
                            <th>"Version"</th>
                            <th>"Created"</th>
                            <th>"Updated"</th>
                            <th>"Filetype"</th>
                        </tr>
                    </thead>
                    <Transition fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>

                        {
                            view! {
                                // TODO: make a error template for reuse
                                <ErrorBoundary fallback=|errors| {
                                    view! {
                                        {move || {
                                            errors
                                                .get()
                                                .into_iter()
                                                .map(|(_, e)| view! { <p>{e.to_string()}</p> })
                                                .collect::<Vec<_>>()
                                        }}
                                    }
                                }>{documents_view}</ErrorBoundary>
                            }
                        }
                    </Transition>
                </table>
            </div>
        </div>
    }
}
