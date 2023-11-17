use charming::{
    component::{self, *},
    df,
    element::{self, *},
    series::{self, *},
    Animation as ResizeAnimation, Chart, ChartResize, Easing, WasmRenderer,
};
use leptos::*;
use leptos_use::use_resize_observer;

#[component]
pub fn Chart(#[prop()] chart: Chart, #[prop()] id: &'static str) -> impl IntoView {
    let (size, size_set) = create_signal((0, 0));
    let renderer = WasmRenderer::new(None, None);
    // TODO: this runs before the view is rendered and results in `id not found` error.
    // let echart = StoredValue::new(renderer.render(id, &chart).expect("charterror"));
    // NOTE: this also requires Chart + ECharts to be clone

    let el = create_node_ref::<html::Div>();
    use_resize_observer(el, move |entries, observer| {
        let rect = entries[0].content_rect();
        size_set.set((rect.width() as u32, rect.height() as u32));
    });

    // create_effect(move |_| {
    //     let (width, height) = size.get();
    //     WasmRenderer::resize_chart(
    //         &echart.get_value(),
    //         ChartResize::new(
    //             width as u32,
    //             height as u32,
    //             false,
    //             Some(ResizeAnimation {
    //                 duration: 20,
    //                 easing: Some(Easing::Linear),
    //             }),
    //         ),
    //     );
    // });

    view! {
            <div class="rounded bg-gray-900 p-3 h-full">
                <div node_ref=el class="transition-all w-full h-5/6" id={id}></div>
                <button class="px-3 py-2 rounded-md bg-gray-700 hover:bg-gray-800 transition-all duration 150 text-white" on:click=move |_| size_set.set((800, 600))>"Resize to 800x600"</button>
            </div>
    }
}
