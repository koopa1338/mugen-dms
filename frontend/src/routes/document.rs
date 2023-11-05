use common::models::document::Doc;
use gloo_net::http::Method;
use leptos::*;

use crate::api::{api_call, FetchError};

#[component]
pub(crate) fn Documents() -> impl IntoView {
    let documents = create_resource(
        move || (),
        move |_| async move {
            api_call::<Vec<Doc>, FetchError>("api/doc", Method::GET, None, None).await
        },
    );

    let documents_view = move || {
        documents.get().map(|doc| {
            doc.map(|doc| {
                view! {
                    <tbody>
                        <For each=move || doc.clone() key=|doc| doc.id let:doc>
                            <tr>
                                <th class="py-2">{doc.id}</th>
                                <th class="py-2">{doc.version}</th>
                                <th class="py-2">
                                    {doc.created.format("%d.%m.%Y %H:%M").to_string()}
                                </th>
                                <th class="py-2">
                                    {doc
                                        .updated
                                        .map(|date| date.format("%d.%m.%Y %H:%M").to_string())
                                        .unwrap_or("never".to_string())}
                                </th>
                                <th class="py-2">{doc.size}</th>
                            </tr>
                        </For>
                    </tbody>
                }
            })
        })
    };

    view! {
        <div class="grid grid-cols-1 gap-4 mb-4">
            <div class="rounded bg-gray-900 p-3">
                <table class="table-auto text-gray-500 w-full">
                    <thead>
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
                            let documents_view = documents_view.clone();
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
