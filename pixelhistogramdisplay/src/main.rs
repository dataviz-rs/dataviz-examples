use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas, configuration::figureconfig::FigureConfig,
    display::winop::Winop, drawers::drawer::Drawer, figuretypes::histogram::Histogram,
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
        color_background: [255, 255, 255], // Define chart background color (white)
        color_grid: [220, 220, 220], // Define grid line color (light gray)
        color_title: [0, 0, 0],      // Define title text color (black)
        num_axis_ticks: 20,          // Set number of tick marks on the axis
        num_grid_horizontal: 20,     // Set number of horizontal grid lines
        num_grid_vertical: 20,       // Set number of vertical grid lines
        font_label: None, // None: an installed sans-serif font is used; or Some("path/to/font.ttf".to_string())
        font_title: None, // None: an installed sans-serif font is used; or Some("path/to/font.ttf".to_string())
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
    pixel_canvas.save_as_image("histogram.png"); // the same image the window shows
    // Display the bar chart interactively
    Winop::display_interactive(&mut pixel_canvas, &mut histogram, "Interactive Histogram");

    let update_data = move |graph: &mut Histogram| {
        graph.add_data(rng.gen_range(-3.0..3.0));
    };

    // Display the histogram graph in real-time
    Winop::display_real_time(
        &mut pixel_canvas,
        &mut histogram,
        "Real-Time Histogram Graph",
        update_data,
        30,
    );
}
