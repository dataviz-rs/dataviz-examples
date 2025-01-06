use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::{cartesiangraphdataset::CartesianDataset, dataset::Dataset},
    drawers::drawer::Drawer,
    figuretypes::cartesiangraph::CartesianGraph,
    utilities::linetype::LineType,
    display::winop::Winop,
};

use std::f64::consts::PI;

fn main() {
    // Initialize the figure configuration with specific settings
    let figure_config = FigureConfig {
        font_size_title: 20.0,       // Set font size for chart title
        font_size_label: 16.0,       // Set font size for axis labels
        font_size_legend: 14.0,      // Set font size for legend text
        font_size_axis: 10.0,        // Set font size for axis tick labels
        color_axis: [0, 0, 0],       // Define axis color (black)
        color_background: [0, 0, 0], // Define chart background color (black)
        color_grid: [220, 220, 220], // Define grid line color (light gray)
        color_title: [0, 0, 0],      // Define title text color (black)
        num_axis_ticks: 20,          // Set number of tick marks on the axis
        num_grid_horizontal: 20,     // Set number of horizontal grid lines
        num_grid_vertical: 20,       // Set number of vertical grid lines
        font_label: Some(
            "C:/Users/samet/Desktop/Rust/rust-lab/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for axis labels
        font_title: Some(
            "C:/Users/samet/Desktop/Rust/rust-lab/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for the title
    };

    // Create a new pixel-based canvas with specified dimensions and background color
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);

    // Create a CartesianGraph
    let mut cartesian_graph =
        CartesianGraph::new("Math Functions", "X Axis", "Y Axis", &figure_config.clone());

    // Add datasets to CartesianGraph
    let mut sine_wave = CartesianDataset::new([0, 0, 255], "sin(x)", LineType::Dashed(10));

    let num_points = 1000;
    let step = 2.0 * PI / num_points as f64;
    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
        sine_wave.add_point((xf, xf.sin()));
    }

    cartesian_graph.add_dataset(sine_wave);

    // Draw CartesianGraph
    cartesian_graph.draw(&mut pixel_canvas);

    // Display the bar chart interactively
    Winop::display_interactive(&mut pixel_canvas, &mut cartesian_graph, "Interactive Cartesian Graph");

    // Create a new pixel canvas for real-time display
    // Closure to update sine wave data
    let mut x_value: f64 = 4.0;
    let update_data = move |graph: &mut CartesianGraph| {
        for i in 0..graph.datasets.len() {
            let y: f64 = x_value.sin();
            graph.datasets[i].add_point((x_value, y));
        }
        x_value += 0.1;
        graph.update_range();
    };

    // Display the Cartesian graph in real-time
    Winop::display_real_time(
        &mut pixel_canvas,
        &mut cartesian_graph,
        "Real-Time Cartesian Graph",
        update_data,
        30,
    );
}
