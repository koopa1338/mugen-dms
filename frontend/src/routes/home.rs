use leptos::*;
use web_sys::FileList;

use crate::components::upload::Upload;

#[component]
pub(crate) fn Home() -> impl IntoView {
    let upload_callback = move |file_list: FileList| {
        logging::warn!("Number of uploaded files: {}", file_list.length());
        for idx in 0..file_list.length() {
            logging::warn!(
                "filename: {:?}",
                file_list
                    .item(idx)
                    .map(|file| file.name())
                    .unwrap()
            );
        }
    };
    view! {
        <Upload callback=upload_callback multiple=true/>
    }
}
