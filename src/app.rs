use plotly::Plot;

use yew::prelude::*;

mod basic_charts;
mod t3d_charts;
// use t3d_charts;

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();

        // //plot demo ++++++++++++++++++
        // let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
        // plot.add_trace(trace);

        // let layout =
        //     plotly::Layout::new().title(plotly::common::Title::new("Displaying a Chart in Yew"));
        // plot.set_layout(layout);
        // //plot demo -------------------------

        // // Scatter3D Plots
        // t3d_charts::simple_scatter3d_plot(&mut plot);
        // // Surface Plots
        // t3d_charts::surface_plot(&mut plot);
        // // Mesh Plots
        // t3d_charts::mesh_3d_plot(&mut plot);

        // // Scatter Plots
        basic_charts::line_and_scatter_plots(&mut plot);

        // basic_charts::bubble_scatter_plots(&mut plot);
        // basic_charts::polar_scatter_plot(&mut plot);
        // basic_charts::large_data_sets(&mut plot);
        // basic_charts::filled_lines(&mut plot);

        // basic_charts::line_shape_options_for_interpolation(&mut plot);

        // // Bar Charts
        // basic_charts::grouped_bar_chart(&mut plot);
        // // Sankey Diagrams
        // basic_charts::basic_sankey_diagram(&mut plot);
        // // plot content -------------------------

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });

    // Only on first render
    use_effect_with((), move |_| {
        p.run();
    });

    html! {
        <div id="plot-div"></div>
    }
}
