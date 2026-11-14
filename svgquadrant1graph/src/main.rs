use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::{cartesiangraphdataset::CartesianDataset, dataset::Dataset},
    drawers::drawer::Drawer,
    figuretypes::quadrant1graph::Quadrant1Graph,
    utilities::{linetype::LineType, scatterdottype::ScatterDotType},
};

use std::f64::consts::PI;

fn main() {
    let figure_config = FigureConfig {
        font_size_title: 20.0,           // Title font size
        font_size_label: 16.0,           // Axis label font size
        font_size_axis: 10.0,            // Tick value font size
        font_size_legend: 14.0,          // Legend font size
        color_axis: [0, 0, 0],           // Axis color (black)
        color_background: [255, 255, 255], // Plot area background (white)
        color_grid: [220, 220, 220],     // Grid color (light gray)
        color_title: [0, 0, 0],          // Title color (black)
        num_axis_ticks: 10,           // Tick intervals per axis (10 + 1 labels)
        num_grid_horizontal: 10,      // Horizontal grid intervals
        num_grid_vertical: 10,        // Vertical grid intervals
        font_label: None, // None: an installed sans-serif font is used; or Some("path/to/font.ttf".to_string())
        font_title: None, // None: falls back to font_label / a system font
    };

    // A Quadrant 1 graph only shows x >= 0 and y >= 0 (negative points are dropped).
    let mut graph = Quadrant1Graph::new("Quadrant 1 Graph", "X Axis", "Y Axis", figure_config);

    // Line styles: Solid, Dashed(dash length in px), Dotted(spacing between dots in px).
    let styles = [
        ([0, 90, 200], "sin(x) + 1 - Solid", LineType::Solid, 1.0),
        ([220, 120, 0], "cos(x) + 3 - Dashed(10)", LineType::Dashed(10), 3.0),
        ([0, 150, 60], "sin(x) + 5 - Dotted(6)", LineType::Dotted(6), 5.0),
    ];

    let num_points = 12;
    let step = 2.0 * PI / num_points as f64;
    for (k, (color, label, line_type, offset)) in styles.into_iter().enumerate() {
        let mut dataset = CartesianDataset::new(color, label, line_type);
        for i in 0..=num_points {
            let x = i as f64 * step;
            let y = if k == 1 { x.cos() } else { x.sin() };
            dataset.add_point((x, y + offset));
        }
        graph.add_dataset(dataset);
    }

    // Lines only (data points are not marked by default).
    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    graph.draw_svg(&mut svg_canvas);
    svg_canvas.save("quadrant1_graph.svg").unwrap();

    // The same graph with a marker at every data point, in the dataset color.
    // Also available: Square(size), Cross(arm length), Triangle(base);
    // use `draw_with_markers` for a PixelCanvas.
    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    graph.draw_svg_with_markers(&mut svg_canvas, ScatterDotType::Circle(4));
    svg_canvas.save("quadrant1_graph_markers.svg").unwrap();
}
