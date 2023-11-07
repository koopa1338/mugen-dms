use leptos::*;
use web_sys::FileList;

#[component]
pub fn Upload(
    #[prop(optional, into)] accept: MaybeSignal<String>,
    #[prop(optional, into)] multiple: MaybeSignal<bool>,
    #[prop(into)] callback: Callback<FileList, ()>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let on_file_addition = move |files: FileList| {
        Callback::call(&callback, files);
    };
    let input_ref = create_node_ref::<html::Input>();

    let on_change = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            if let Some(files) = input_ref.files() {
                on_file_addition(files);
            }
        }
    };
    let on_click = move |_| {
        if let Some(input_ref) = input_ref.get_untracked() {
            let _ = input_ref.show_picker();
        }
    };

    let is_trigger_dragover = create_rw_signal(false);
    let on_trigger_drop = move |event: ev::DragEvent| {
        event.prevent_default();
        if let Some(data) = event.data_transfer() {
            if let Some(files) = data.files() {
                on_file_addition(files);
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
    };
    let on_trigger_dragleave = move |event: ev::DragEvent| {
        event.prevent_default();
        is_trigger_dragover.set(false);
    };

    view! {
        <div class="w-full rounded-md bg-gray-900 p-3">
            <div class="w-full p-5 rounded-md border border-2 border-dotted"
                class=("border-amber-600", move|| is_trigger_dragover.get())
                on:drop=on_trigger_drop
                on:dragover=on_trigger_dragover
                on:dragenter=on_trigger_dragenter
                on:dragleave=on_trigger_dragleave
            >
                <input class="hidden invisible"
                    ref=input_ref
                    type="file"
                    accept=move || accept.get()
                    multiple=move || multiple.get()
                    on:change=on_change
                    on:click=move |ev| ev.stop_propagation()
                />
                <div class="text-gray-500 p-2 mt-2 flex flex-justify">
                    <button
                        class="rounded-md bg-amber-600 text-semibold cursor-pointer text-white p-2 m-2"
                        on:click=on_click>
                        "Select files"
                    </button>
                    <p class="text-gray-500">"Drag and drop files here or click for file dialogue"</p>
                </div>

                <div>{children.map(|c| c())}</div>
            </div>
        </div>
    }
}
