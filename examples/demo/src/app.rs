use plotly::Plot;
use plotly::Scatter;
use yew::prelude::*;

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        //plot content ++++++++++++++++++
        let id = "plot-div";
        let mut plot = Plot::new();
        let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
        plot.add_trace(trace);

        let layout =
            plotly::Layout::new().title(plotly::common::Title::new("Displaying a Chart in Yew"));
        plot.set_layout(layout);
        //plot content -------------------------
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
