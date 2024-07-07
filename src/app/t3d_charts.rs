#![allow(dead_code)]
use std::vec;

use ndarray::Array;
use plotly::{
    common::{ColorScale, ColorScalePalette, Marker, MarkerSymbol, Mode, Title},
    layout::{Axis, Layout},
    Mesh3D, Plot, Scatter3D, Surface,
};

// 3D Scatter Plots
pub fn simple_scatter3d_plot(_plot: &mut Plot) {
    let n: usize = 100;
    let t: Vec<f64> = Array::linspace(0., 10., n).into_raw_vec();
    let y: Vec<f64> = t.iter().map(|x: &f64| x.sin()).collect();
    let z: Vec<f64> = t.iter().map(|x: &f64| x.cos()).collect();

    let _sizelookup = z.clone();
    let z_reverse = z.iter().map(|i| -i).collect();
    // let z_reverse=z.clone();//not reverse

    let trace0 = Scatter3D::new(t.clone(), y.clone(), z.clone()).mode(Mode::Markers);
    let trace1 = Scatter3D::new(t.clone(), y.clone(), z.clone()).mode(Mode::Lines);
    let trace2 = Scatter3D::new(t.clone(), y.clone(), z_reverse)
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .symbol(MarkerSymbol::Diamond)
                .size_array(
                    _sizelookup
                        .iter()
                        .map(|i| (i.abs() * 25f64) as usize)
                        .collect(),
                )
                //   .symbol(MarkerSymbol::Diamond)
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis)),
        );

    _plot.add_trace(trace0); //markers
    _plot.add_trace(trace1); //lines
    _plot.add_trace(trace2); //Diamond

    let layout = Layout::new()
        .title("Helix")
        .x_axis(Axis::new().title("x (A meaningful axis name goes here)"))
        .y_axis(Axis::new().title(Title::new()))
        .z_axis(Axis::new().title("z Axis"));
    _plot.set_layout(layout);
}

// 3D Surface Plot
pub fn surface_plot(_plot: &mut Plot) {
    let n: usize = 100;
    let x: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec();
    let y: Vec<f64> = Array::linspace(-10., 10., n).into_raw_vec();
    let z: Vec<Vec<f64>> = x
        .iter()
        .map(|i| {
            y.iter()
                .map(|j| 1.0 / (j * j + 5.0) * j.sin() + 1.0 / (i * i + 5.0) * i.cos())
                .collect()
        })
        .collect();

    let trace = Surface::new(z).x(x).y(y);
    _plot.add_trace(trace);
}

pub fn mesh_3d_plot(_plot: &mut Plot) {
    let trace = Mesh3D::new(
        vec![0, 1, 2, 0],
        vec![0, 0, 1, 2],
        vec![0, 2, 0, 1],
        vec![0, 0, 0, 1],
        vec![1, 2, 3, 2],
        vec![2, 3, 1, 3],
    )
    .intensity(vec![0.0, 0.33, 0.66, 1.0])
    .color_scale(ColorScale::Palette(ColorScalePalette::Rainbow));

    _plot.add_trace(trace);
}
