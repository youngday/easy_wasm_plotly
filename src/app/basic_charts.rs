#![allow(dead_code)]
use std::f64::consts::PI;
use std::vec;

use ndarray::Array;
use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode,
        Orientation,
    },
    layout::{Axis, BarMode, Layout, Legend, TicksDirection, TraceOrder},
    sankey::{Line as SankeyLine, Link, Node},
    Bar, Plot, Sankey, Scatter, ScatterPolar,
};
use rand_distr::{Distribution, Normal, Uniform};

// Scatter Plots

pub fn line_and_scatter_plots(_plot: &mut Plot) {
    let n: usize = 100;
    let mut rng = rand::thread_rng();
    let random_x: Vec<f64> = Array::linspace(0., 1., n).into_raw_vec();
    let random_y0: Vec<f64> = Normal::new(5., 1.)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();
    let random_y1: Vec<f64> = Normal::new(0., 1.)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();
    let y: Vec<f64> = random_x
        .iter()
        .map(|random_x| (random_x * PI * 20.0).sin())
        .collect();

    let trace1 = Scatter::new(random_x.clone(), random_y0)
        .mode(Mode::Markers)
        .name("markers")
        .marker(Marker::new().size(12).color(Rgb::new(164, 194, 244)))
        .text_array(vec!["A-1", "A-2", "A-3", "A-4", "A-5"]); //data_labels_on_the_plot

    let trace2 = Scatter::new(random_x.clone(), random_y1)
        .mode(Mode::LinesMarkers)
        .name("linex+markers");
    let trace3 = Scatter::new(random_x, y)
        .mode(Mode::Lines)
        .name("lines")
        .line(
            Line::new()
                .color(Rgb::new(55, 128, 191))
                .width(3.0)
                .dash(DashType::DashDot),
        );

    _plot.add_trace(trace1);
    _plot.add_trace(trace2);
    _plot.add_trace(trace3);

    let layout = Layout::new()
        .title("Data Labels Hover".into()) //data_labels_hover
        .width(1280)
        .height(600)
        .x_axis(Axis::new().title("x".into()).range(vec![-0.1, 1.1]))
        .y_axis(Axis::new().title("y".into()).range(vec![-8.0, 8.]));
    _plot.set_layout(layout);
}

pub fn bubble_scatter_plots(_plot: &mut Plot) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 11, 12, 13])
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size_array(vec![40, 60, 80, 100])
                .color_array(vec![
                    NamedColor::Red,
                    NamedColor::Blue,
                    NamedColor::Cyan,
                    NamedColor::OrangeRed,
                ]),
        );

    _plot.add_trace(trace1);
}

pub fn polar_scatter_plot(_plot: &mut Plot) {
    let n: usize = 400;
    let theta: Vec<f64> = Array::linspace(0., 360., n).into_raw_vec();
    let r: Vec<f64> = theta
        .iter()
        .map(|x| {
            let x = x / 360. * core::f64::consts::TAU;
            let x = x.cos();
            1. / 8. * (63. * x.powf(5.) - 70. * x.powf(3.) + 15. * x).abs()
        })
        .collect();

    let trace = ScatterPolar::new(theta, r).mode(Mode::Lines);
    _plot.add_trace(trace);
}

pub fn large_data_sets(_plot: &mut Plot) {
    let n: usize = 100_000;
    let mut rng = rand::thread_rng();
    let r: Vec<f64> = Uniform::new(0., 1.).sample_iter(&mut rng).take(n).collect();
    let theta: Vec<f64> = Normal::new(0., 2. * std::f64::consts::PI)
        .unwrap()
        .sample_iter(&mut rng)
        .take(n)
        .collect();

    let x: Vec<f64> = r
        .iter()
        .zip(theta.iter())
        .map(|args| args.0 * args.1.cos())
        .collect();
    let y: Vec<f64> = r
        .iter()
        .zip(theta.iter())
        .map(|args| args.0 * args.1.sin())
        .collect();
    let trace = Scatter::new(x, y)
        .web_gl_mode(true)
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .color_scale(ColorScale::Palette(ColorScalePalette::Viridis))
                .line(Line::new().width(1.)),
        );

    _plot.add_trace(trace);
}

// Line Charts

pub fn line_shape_options_for_interpolation(_plot: &mut Plot) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4, 5], vec![1, 3, 2, 3, 1])
        .mode(Mode::LinesMarkers)
        .name("linear")
        .line(Line::new().shape(LineShape::Linear));
    let trace2 = Scatter::new(vec![1, 2, 3, 4, 5], vec![6, 8, 7, 8, 6])
        .mode(Mode::LinesMarkers)
        .name("spline")
        .line(Line::new().shape(LineShape::Spline));
    let trace3 = Scatter::new(vec![1, 2, 3, 4, 5], vec![11, 13, 12, 13, 11])
        .mode(Mode::LinesMarkers)
        .name("vhv")
        .line(Line::new().shape(LineShape::Vhv));
    let trace4 = Scatter::new(vec![1, 2, 3, 4, 5], vec![16, 18, 17, 18, 16])
        .mode(Mode::LinesMarkers)
        .name("hvh")
        .line(Line::new().shape(LineShape::Hvh));
    let trace5 = Scatter::new(vec![1, 2, 3, 4, 5], vec![21, 23, 22, 23, 21])
        .mode(Mode::LinesMarkers)
        .name("vh")
        .line(Line::new().shape(LineShape::Vh));
    let trace6 = Scatter::new(vec![1, 2, 3, 4, 5], vec![26, 28, 27, 28, 26])
        .mode(Mode::LinesMarkers)
        .name("hv")
        .line(Line::new().shape(LineShape::Hv));

    let layout = Layout::new().legend(
        Legend::new()
            .y(0.5)
            .trace_order(TraceOrder::Reversed)
            .font(Font::new().size(16)),
    );
    _plot.set_layout(layout);
    _plot.add_trace(trace1);
    _plot.add_trace(trace2);
    _plot.add_trace(trace3);
    _plot.add_trace(trace4);
    _plot.add_trace(trace5);
    _plot.add_trace(trace6);
}

pub fn filled_lines(_plot: &mut Plot) {
    let x1 = vec![
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
        2.0, 1.0,
    ];
    let x2 = (1..=10).map(|iv| iv as f64).collect::<Vec<f64>>();
    let trace1 = Scatter::new(
        x1.clone(),
        vec![
            2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0,
            2.0, 1.0, 0.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 100, 80, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace2 = Scatter::new(
        x1.clone(),
        vec![
            5.5, 3.0, 5.5, 8.0, 6.0, 3.0, 8.0, 5.0, 6.0, 5.5, 4.75, 5.0, 4.0, 7.0, 2.0, 4.0, 7.0,
            4.4, 2.0, 4.5,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(0, 176, 246, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Premium")
    .show_legend(false);
    let trace3 = Scatter::new(
        x1,
        vec![
            11.0, 9.0, 7.0, 5.0, 3.0, 1.0, 3.0, 5.0, 3.0, 1.0, -1.0, 1.0, 3.0, 1.0, -0.5, 1.0, 3.0,
            5.0, 7.0, 9.0,
        ],
    )
    .fill(Fill::ToZeroX)
    .fill_color(Rgba::new(231, 107, 243, 0.2))
    .line(Line::new().color(NamedColor::Transparent))
    .name("Fair")
    .show_legend(false);
    let trace4 = Scatter::new(
        x2.clone(),
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    )
    .line(Line::new().color(Rgb::new(0, 100, 80)))
    .name("Fair");
    let trace5 = Scatter::new(
        x2.clone(),
        vec![5.0, 2.5, 5.0, 7.5, 5.0, 2.5, 7.5, 4.5, 5.5, 5.0],
    )
    .line(Line::new().color(Rgb::new(0, 176, 246)))
    .name("Premium");
    let trace6 = Scatter::new(x2, vec![10.0, 8.0, 6.0, 4.0, 2.0, 0.0, 2.0, 4.0, 2.0, 0.0])
        .line(Line::new().color(Rgb::new(231, 107, 243)))
        .name("Ideal");

    let layout = Layout::new()
        .paper_background_color(Rgb::new(255, 255, 255))
        .plot_background_color(Rgb::new(229, 229, 229))
        .x_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .range(vec![1.0, 10.0])
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .grid_color(Rgb::new(255, 255, 255))
                .show_grid(true)
                .show_line(false)
                .show_tick_labels(true)
                .tick_color(Rgb::new(127, 127, 127))
                .ticks(TicksDirection::Outside)
                .zero_line(false),
        );

    _plot.set_layout(layout);
    _plot.add_trace(trace1);
    _plot.add_trace(trace2);
    _plot.add_trace(trace3);
    _plot.add_trace(trace4);
    _plot.add_trace(trace5);
    _plot.add_trace(trace6);
}

// Bar Charts

pub fn grouped_bar_chart(_plot: &mut Plot) {
    let animals1 = vec!["giraffes", "orangutans", "monkeys"];
    let trace1 = Bar::new(animals1, vec![20, 14, 23]).name("SF Zoo");

    let animals2 = vec!["giraffes", "orangutans", "monkeys"];
    let trace2 = Bar::new(animals2, vec![12, 18, 29]).name("LA Zoo");

    let layout = Layout::new().bar_mode(BarMode::Group);
    // let layout = Layout::new().bar_mode(BarMode::Stack);

    _plot.add_trace(trace1);
    _plot.add_trace(trace2);
    _plot.set_layout(layout);
}

// Sankey Diagrams
pub fn basic_sankey_diagram(_plot: &mut Plot) {
    // https://plotly.com/javascript/sankey-diagram/#basic-sankey-diagram
    let trace = Sankey::new()
        .orientation(Orientation::Horizontal)
        .node(
            Node::new()
                .pad(15)
                .thickness(30)
                .line(SankeyLine::new().color(NamedColor::Black).width(0.5))
                .label(vec!["A1", "A2", "B1", "B2", "C1", "C2"])
                .color_array(vec![
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                ]),
        )
        .link(
            Link::new()
                .value(vec![8, 4, 2, 8, 4, 2])
                .source(vec![0, 1, 0, 2, 3, 3])
                .target(vec![2, 3, 3, 4, 4, 5]),
        );

    let layout = Layout::new()
        .title("Basic Sankey".into())
        .font(Font::new().size(10));

    _plot.add_trace(trace);
    _plot.set_layout(layout);
}
