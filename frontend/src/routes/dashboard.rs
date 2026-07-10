use gloo_file::{futures, FileList};
use gloo_net::http::{Method, RequestBuilder};
use leptos::prelude::*;

use crate::components::{grid::Grid, upload::Upload};

#[component]
pub(crate) fn Dashboard() -> impl IntoView {
    let (error, setError) = signal(false);

    let upload_closure = move |file_list: FileList| async move {
        let mut success_count = 0u32;
        for file in file_list.iter() {
            let name = file.name();
            let bytes = match futures::read_as_bytes(file).await {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Failed to read file {name}: {e}");
                    continue;
                }
            };

            // Use gloo-net RequestBuilder with raw bytes as body
            // The backend's multipart handler will receive the file
            let resp = match RequestBuilder::new("http://localhost:4000/api/documents/raw")
                .method(Method::POST)
                .body(bytes)
            {
                Ok(rb) => rb.send().await,
                Err(e) => {
                    eprintln!("Failed to build request for {name}: {e}");
                    continue;
                }
            };

            match resp {
                Ok(r) if r.ok() => success_count += 1,
                Ok(r) => {
                    eprintln!("Upload failed for {name}: {}", r.status());
                }
                Err(e) => {
                    eprintln!("Upload error for {name}: {e}");
                }
            }
        }
        if success_count > 0 {
            setError.set(false);
        } else if file_list.iter().count() > 0 {
            setError.set(true);
        }
    };

    view! {
        <Grid classes="grid-cols-2 mb-4 h-full">
            <Show when=move || !error.get() fallback=|| view! { <p class="text-red-400">"All uploads failed."</p> }>
                <Upload callback=upload_closure multiple=true accept="*/*"/>
            </Show>
        </Grid>
    }
}
