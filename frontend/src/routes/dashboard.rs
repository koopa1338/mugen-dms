use charming::{
    component::{self, *},
    df,
    element::{self, *},
    series::{self, *},
    Animation as ResizeAnimation, Chart, ChartResize, Easing, WasmRenderer,
};
use gloo_file::{FileList, futures};
use leptos::{*, callback::Callback};

use crate::components::{chart::Chart, grid::Grid, upload::Upload};

#[component]
pub(crate) fn Dashboard() -> impl IntoView {
    let upload_closure =  move |file_list: FileList| async move {
        for file in file_list.iter() {
            let bytes = futures::read_as_bytes(&file).await.expect("filereaderror");
            logging::warn!("file bytes: {:?}", bytes);
        }
    };

    let upload_callback = Callback::new(upload_closure);

    // testing chart
    let chart = Chart::new()
        .title(Title::new().text("Demo: Leptos + Charming"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
        )
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(Line::new().data(vec![150, 230, 224, 218, 135, 147, 260]));

    view! {
        <Grid classes="grid-cols-2 mb-4 h-full">
            <Chart chart=chart id="chart-test"/>
            <Upload callback=upload_callback multiple=true/>
        </Grid>
    }
}
