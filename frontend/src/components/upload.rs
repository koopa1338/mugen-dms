use gloo_file::FileList;
use leptos::prelude::*;
use leptos::{html, ev};
use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

#[component]
pub fn Upload<T>(
    #[prop(optional, into)] accept: Signal<String>,
    #[prop(optional, into)] multiple: Signal<bool>,
    #[prop(into)] callback: Callback<FileList, T>,
    #[prop(optional)] children: Option<ChildrenFn>,
) -> impl IntoView
where
    T: Future<Output = ()> + 'static,
{
    let on_file_addition = move |files: FileList| {
        spawn_local(callback.run(files));
    };

    let input_ref = NodeRef::<html::Input>::new();

    let on_change = move |_| {
        if let Some(input_el) = input_ref.get()
            && let Some(files) = input_el.files() {
                on_file_addition(FileList::from(files));
            }
    };
    let on_click = move |_| {
        if let Some(input_el) = input_ref.get() {
            let _ = HtmlInputElement::show_picker(&input_el);
        }
    };

    let is_trigger_dragover = RwSignal::new(false);
    let on_trigger_drop = move |event: ev::DragEvent| {
        event.prevent_default();
        if let Some(data) = event.data_transfer()
            && let Some(input_el) = input_ref.get()
                && let Some(files) = data.files() {
                    let ft = input_el.accept();
                    let accepted = ft.split(',').collect::<Vec<_>>();
                    let mut all_files_valid = true;
                    for idx in 0..files.length() {
                        if let Some(file) = files.item(idx) {
                            let file_type = file.type_();
                            let contain = accepted.contains(&file_type.as_str());
                            if !contain {
                                leptos::logging::warn!("filetype of file was {file_type} but only one of the following filetypes is allowed: {accepted:#?}");
                            }
                            all_files_valid &= contain;
                        }
                    }
                    if all_files_valid {
                        on_file_addition(FileList::from(files));
                    }
                }

        is_trigger_dragover.set(false);
    };
    let on_trigger_dragover = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(true);
    };
    let on_trigger_dragenter = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(true);
    };
    let on_trigger_dragleave = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(false);
    };

    view! {
        <div class="w-full rounded-md bg-gray-900 p-3">
            <div class="p-5">
                <div
                    class="w-full rounded-md border border-2 border-dotted transition-all duration 200"
                    class=("border-amber-600", move || is_trigger_dragover.get())
                    on:drop=on_trigger_drop
                    on:dragover=on_trigger_dragover
                    on:dragenter=on_trigger_dragenter
                    on:dragleave=on_trigger_dragleave
                >
                    <input
                        class="hidden invisible"
                        node_ref=input_ref
                        type="file"
                        accept=move || accept.get()
                        multiple=move || multiple.get()
                        on:change=on_change
                        on:click=move |ev| ev.stop_propagation()
                    />
                    <div class="text-gray-500 p-2 mt-2 flex flex-justify items-center">
                        <button
                            class="rounded-md bg-amber-600 hover:bg-amber-700 text-semibold text-4xl cursor-pointer text-white px-3 m-2 transition-all duration-200"
                            on:click=on_click
                        >
                            ""
                        </button>
                        <p
                            class="transition-all duration-200"
                            class=("text-amber-600", move || is_trigger_dragover.get())
                        >
                            "Drag and drop files here or click for file dialogue"
                        </p>
                    </div>

                    <div>{children.map(|c| c())}</div>
                </div>
            </div>
        </div>
    }
}
