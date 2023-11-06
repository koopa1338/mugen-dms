use leptos::*;
use web_sys::FileList;

use crate::components::upload::Upload;

#[component]
pub(crate) fn Home() -> impl IntoView {
    let upload_callback = move |file_list: FileList| {
        logging::warn!("Number of uploaded files: {}", file_list.length());
    };
    view! {
        <div class="grid grid-cols-1 gap-4 mb-4">
            <div class="flex items-center justify-center h-24 rounded bg-gray-900">
                <Upload callback=upload_callback multiple=true>
                    "Drag and drop files here or click for file dialogue"
                </Upload>
            </div>
        </div>
    }
}
