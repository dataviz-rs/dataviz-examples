use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::{dataset::Dataset, scattergraphdataset::ScatterGraphDataset},
    drawers::drawer::Drawer,
    figuretypes::scattergraph::ScatterGraph,
    utilities::scatterdottype::ScatterDotType,
};

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
        num_axis_ticks: 8,           // Tick intervals per axis (8 + 1 labels)
        num_grid_horizontal: 8,      // Horizontal grid intervals
        num_grid_vertical: 8,        // Vertical grid intervals
        font_label: None, // None: an installed sans-serif font is used; or Some("path/to/font.ttf".to_string())
        font_title: None, // None: falls back to font_label / a system font
    };

    let mut scatter_graph = ScatterGraph::new("Scatter Graph", "X Axis", "Y Axis", figure_config);

    let dot_types = [
        ([220, 40, 40], "Circle(4)", ScatterDotType::Circle(4)),
        ([40, 160, 40], "Square(7)", ScatterDotType::Square(7)),
        ([40, 90, 220], "Cross(5)", ScatterDotType::Cross(5)),
        ([230, 140, 0], "Triangle(7)", ScatterDotType::Triangle(7)),
    ];

    for (k, (color, label, dot_type)) in dot_types.into_iter().enumerate() {
        let mut dataset = ScatterGraphDataset::new(color, label, dot_type);
        for i in 0..20 {
            let x = i as f64 - 5.0;
            let y = (x * 0.6 + k as f64).sin() * 6.0 + k as f64 * 4.0;
            dataset.add_point((x, y));
        }
        scatter_graph.add_dataset(dataset);
    }

    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    scatter_graph.draw_svg(&mut svg_canvas);
    svg_canvas.save("scatter_graph.svg").unwrap();
}
