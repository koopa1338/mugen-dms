use common::models::document::Doc;
use gloo_net::http::Method;
use leptos::*;
use leptos_use::{use_scroll_with_options, UseScrollOptions};

use crate::api::{api_call, FetchError};
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
    let el = create_node_ref::<html::Tbody>();
    let (page, page_set) = create_signal(1u32);
    let (data, data_set) = create_signal(vec![]);
    let (error, error_set) = create_signal(false);

    let user_scroll_return = use_scroll_with_options(el, UseScrollOptions::default());

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
        if user_scroll_return.arrived_state.get().bottom && !user_scroll_return.is_scrolling.get() {
            page_set.update(|page| *page = page.saturating_add(1));
            (user_scroll_return.measure)();
            if error.get_untracked() {
                documents.refetch();
            }
        }
    });

    create_effect(move |_| {
        if let Some(docs) = documents.get() {
            match docs {
                Ok(data) if !data.is_empty() => {
                    data_set.update(|d| d.extend_from_slice(&data));
                    error_set.set_untracked(false);
                }
                Err(_e) => {
                    error_set.set_untracked(true);
                    page_set.update_untracked(|page| {
                        if *page > 1 {
                            *page = page.saturating_sub(1);
                        }
                    });
                }
                _ => return,
            }
        }
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
    }
}
