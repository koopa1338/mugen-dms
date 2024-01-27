use gloo_file::{futures, FileList};
use leptos::{callback::Callback, *};

use crate::components::{grid::Grid, upload::Upload};

#[component]
pub(crate) fn Dashboard() -> impl IntoView {
    let upload_closure = move |file_list: FileList| async move {
        for file in file_list.iter() {
            let bytes = futures::read_as_bytes(&file).await.expect("filereaderror");
            logging::warn!("file bytes: {:?}", bytes);
        }
    };

    let upload_callback = Callback::new(upload_closure);

    view! {
        <Grid classes="grid-cols-2 mb-4 h-full">
            <Upload callback=upload_callback multiple=true accept="application/pdf"/>
        </Grid>
    }
}
