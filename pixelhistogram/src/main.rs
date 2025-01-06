use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas,
    configuration::figureconfig::FigureConfig,
    drawers::drawer::Drawer,
    figuretypes::histogram::Histogram,
};

use rand::Rng;

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
    // Generate random data
    let mut rng = rand::thread_rng();
    let data: Vec<f64> = (0..1000).map(|_| rng.gen_range(-3.0..3.0)).collect();

    // Create a Histogram
    let mut histogram = Histogram::new(
        "Histogram Example",
        "Values",
        "Frequency",
        30,
        [135, 206, 250], // Skyblue
        figure_config.clone(),
    );
    histogram.add_data_vec(data);

    // Draw the Histogram
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80); // White background

    histogram.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("histogram.png");
}
