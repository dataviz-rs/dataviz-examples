use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::{cartesiangraphdataset::CartesianDataset, dataset::Dataset},
    drawers::drawer::Drawer,
    figuretypes::cartesiangraph::CartesianGraph,
    utilities::linetype::LineType,
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
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for axis labels
        font_title: Some(
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for the title
    };

    // Create a new pixel-based canvas with specified dimensions and background color
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);

    // Create a CartesianGraph
    let mut cartesian_graph =
        CartesianGraph::new("Math Functions", "X Axis", "Y Axis", &figure_config.clone());

    // Add datasets to CartesianGraph
    let mut sine_wave = CartesianDataset::new([0, 0, 255], "sin(x)", LineType::Dashed(10));
    let mut cosine_wave = CartesianDataset::new([255, 0, 0], "cos(x)", LineType::Solid);

    // Add datasets
    let mut line5 = CartesianDataset::new([0, 0, 220], "line1", LineType::Solid);
    let mut line6 = CartesianDataset::new([220, 0, 0], "line2", LineType::Dashed(10));
    let mut line7 = CartesianDataset::new([0, 220, 0], "line3", LineType::Dotted(10));
    let mut line8 = CartesianDataset::new([150, 100, 50], "line4", LineType::Solid);

    let num_points = 10;
    let step = 2.0 * PI / num_points as f64;
    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
        line5.add_point((xf, xf));
        line6.add_point((-xf, xf * 2.0));
        line7.add_point((-2.0 * xf, -xf * 3.0));
        line8.add_point((xf, xf * 4.0));
    }

    let num_points = 1000;
    let step = 2.0 * PI / num_points as f64;
    for x in 0..=num_points {
        let xf = -PI + x as f64 * step;
        sine_wave.add_point((xf, xf.sin()));
        cosine_wave.add_point((xf, xf.cos()));
    }

    cartesian_graph.add_dataset(sine_wave);
    cartesian_graph.add_dataset(cosine_wave);
    cartesian_graph.add_dataset(line5);
    cartesian_graph.add_dataset(line6);
    cartesian_graph.add_dataset(line7);
    cartesian_graph.add_dataset(line8);

    // Draw CartesianGraph
    cartesian_graph.draw(&mut pixel_canvas);
    cartesian_graph.draw_legend(&mut pixel_canvas);
    pixel_canvas.save_as_image("cartesian_graph.png");
}
