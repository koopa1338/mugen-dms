use leptos::*;
use web_sys::FileList;
use charming::{
    component::{self, *},
    df,
    element::{self, *},
    series::{self, *},
    Animation as ResizeAnimation, Chart, ChartResize, Easing, WasmRenderer,
};

use crate::components::{chart::Chart, grid::Grid, upload::Upload};


#[component]
pub(crate) fn Dashboard() -> impl IntoView {
    let upload_callback = move |file_list: FileList| {
        logging::warn!("Number of uploaded files: {}", file_list.length());
        for idx in 0..file_list.length() {
            logging::warn!(
                "filename: {:?}",
                file_list.item(idx).map(|file| file.name()).unwrap()
            );
        }
    };

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
