use plotly::Plot;
use plotly::Scatter;
use yew::prelude::*;

mod basic_charts;
mod custom_controls;
mod financial_charts;
mod images;
// mod maps;
// mod ndarray;
// mod scientific_charts;
// mod shapes;
// mod statistical_charts;
// mod subplots;
mod t3d_charts;

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut _plot = Plot::new();
        if false {
            //plot demo ++++++++++++++++++
            let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
            _plot.add_trace(trace);

            let layout = plotly::Layout::new()
                .title(plotly::common::Title::new("Displaying a Chart in Yew"));
            _plot.set_layout(layout);
            //plot demo -------------------------
        };
        if false {
            // // Scatter3D Plots
            t3d_charts::simple_scatter3d_plot(&mut _plot);
            // // Surface Plots
            // t3d_charts::surface_plot(&mut plot);
            // // Mesh Plots
            // t3d_charts::mesh_3d_plot(&mut plot);
        };

        if false {
            // // Scatter Plots
            basic_charts::line_and_scatter_plots(&mut _plot);

            // basic_charts::bubble_scatter_plots(&mut plot);
            // basic_charts::polar_scatter_plot(&mut plot);
            // basic_charts::large_data_sets(&mut plot);
            // basic_charts::filled_lines(&mut plot);

            // basic_charts::line_shape_options_for_interpolation(&mut plot);
        };
        if false {
            // // Bar Charts
            basic_charts::grouped_bar_chart(&mut _plot);
            // // Sankey Diagrams
            // basic_charts::basic_sankey_diagram(&mut plot);
            // // plot content -------------------------
        };

        if false {
            //custom_controls

            // custom_controls::bar_plot_with_dropdown_for_different_data(&mut _plot);
            // custom_controls::heat_map_with_modifiable_colorscale(&mut _plot);
            custom_controls::bar_chart_with_modifiable_bar_mode(&mut _plot);
        };
        if true {
            //TODO: not test  ok.
            //financial_charts
            // Time Series and Date Axes
            financial_charts::time_series_plot_with_custom_date_range(&mut _plot);
            // financial_charts::time_series_with_range_slider(&mut _plot);
            // financial_charts::time_series_with_range_selector_buttons(&mut _plot);
            // financial_charts::customizing_tick_label_formatting_by_zoom_level(&mut _plot);

            // Candlestick Charts
            // financial_charts::simple_candlestick_chart(&mut _plot);;

            // OHLC Charts
            // financial_charts::simple_ohlc_chart(&mut _plot);;
        };
        if false { //TODO: need test .
             //images
             // images::basic_image(&mut _plot);
             // images::trace_from_image_crate_rgb(&mut _plot);
             // images::trace_from_image_crate_rgba(&mut _plot);
             // images::trace_from_ndarray_rgb(&mut _plot);
             // images::trace_from_ndarray_rgba(&mut _plot);
        };
        // // if false {
        // //     //maps

        // //     maps::scatter_mapbox(&mut _plot);
        // // };
        // // if false {
        // //     //ndarray
        // //     single_ndarray_trace(&mut _plot);
        // //     // multiple_ndarray_traces_over_columns(&mut _plot);;
        // //     // multiple_ndarray_traces_over_rows(&mut _plot);;
        // // };
        // // if false {
        // //     //scientific_charts
        // //     single_ndarray_trace(&mut _plot);
        // //     // multiple_ndarray_traces_over_columns(&mut _plot);;
        // //     // multiple_ndarray_traces_over_rows(&mut _plot);;
        // // };
        // // if false {
        // //     //shapes
        // //     filled_area_chart(&mut _plot);
        // //     // vertical_and_horizontal_lines_positioned_relative_to_axes(&mut _plot);;
        // //     // lines_positioned_relative_to_the_plot_and_to_the_axes(&mut _plot);;
        // //     // creating_tangent_lines_with_shapes(&mut _plot);;
        // //     // rectangles_positioned_relative_to_the_axes(&mut _plot);;
        // //     // rectangle_positioned_relative_to_the_plot_and_to_the_axes(&mut _plot);;
        // //     // highlighting_time_series_regions_with_rectangle_shapes(&mut _plot);;
        // //     // circles_positioned_relative_to_the_axes(&mut _plot);;
        // //     // highlighting_clusters_of_scatter_points_with_circle_shapes(&mut _plot);;
        // //     // venn_diagram_with_circle_shapes(&mut _plot);;
        // //     // adding_shapes_to_subplots(&mut _plot);;
        // //     // svg_paths(&mut _plot);;
        // // };
        // // if false {
        // //     //statistical_charts
        // //     // Error Bars
        // //     basic_symmetric_error_bars(&mut _plot);
        // //     // asymmetric_error_bars(&mut _plot);;
        // //     // error_bars_as_a_percentage_of_the_y_value(&mut _plot);;
        // //     // asymmetric_error_bars_with_a_constant_offset(&mut _plot);;
        // //     // horizontal_error_bars(&mut _plot);;
        // //     // bar_chart_with_error_bars(&mut _plot);;
        // //     // colored_and_styled_error_bars(&mut _plot);;

        // //     // Box Plots
        // //     // basic_box_plot(&mut _plot);;
        // //     // box_plot_that_displays_the_underlying_data(&mut _plot);;
        // //     // horizontal_box_plot(&mut _plot);;
        // //     // grouped_box_plot(&mut _plot);;
        // //     // box_plot_styling_outliers(&mut _plot);;
        // //     // box_plot_styling_mean_and_standard_deviation(&mut _plot);;
        // //     // grouped_horizontal_box_plot(&mut _plot);;
        // //     // fully_styled_box_plot(&mut _plot);;

        // //     // Histograms
        // //     // basic_histogram(&mut _plot);;
        // //     // horizontal_histogram(&mut _plot);;
        // //     // overlaid_histogram(&mut _plot);;
        // //     // stacked_histograms(&mut _plot);;
        // //     // colored_and_styled_histograms(&mut _plot);;
        // //     // cumulative_histogram(&mut _plot);;
        // //     // normalized_histogram(&mut _plot);;
        // //     // specify_binning_function(&mut _plot);;
        // // };
        // // if false {
        // //     //subplots
        // //     // Subplots
        // //     simple_subplot(&mut _plot);
        // //     // custom_sized_subplot(&mut _plot);;
        // //     // multiple_subplots(&mut _plot);;
        // //     // stacked_subplots(&mut _plot);;
        // //     // stacked_subplots_with_shared_x_axis(&mut _plot);;
        // //     // multiple_custom_sized_subplots(&mut _plot);;

        // //     // Multiple Axes
        // //     // two_y_axes(&mut _plot);;
        // //     // multiple_axes(&mut _plot);;
        // // };
        // // if false {
        // //     //scientific
        // //     // Contour Plots
        // //     simple_contour_plot();
        // //     colorscale_for_contour_plot();
        // //     customizing_size_and_range_of_a_contour_plots_contours();
        // //     customizing_spacing_between_x_and_y_ticks();

        // //     // Heatmaps
        // //     basic_heat_map();
        // // };

        async move {
            plotly::bindings::new_plot(id, &_plot).await;
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
