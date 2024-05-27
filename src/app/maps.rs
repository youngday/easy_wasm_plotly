#![allow(dead_code)]

use plotly::{
    common::Marker,
    layout::{Center, DragMode, Mapbox, MapboxStyle, Margin},
    Layout, Plot, ScatterMapbox,
};

pub fn scatter_mapbox(_plot: &mut Plot) {
    let trace = ScatterMapbox::new(vec![45.5017], vec![-73.5673])
        .marker(Marker::new().size(25).opacity(0.9));

    let layout = Layout::new()
        .drag_mode(DragMode::Zoom)
        .margin(Margin::new().top(0).left(0).bottom(0).right(0))
        .mapbox(
            Mapbox::new()
                .style(MapboxStyle::OpenStreetMap)
                .center(Center::new(45.5017, -73.5673))
                .zoom(5),
        );
    _plot.add_trace(trace);
    _plot.set_layout(layout);
}
