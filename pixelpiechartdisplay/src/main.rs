use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas, configuration::figureconfig::FigureConfig,
    display::winop::Winop, drawers::drawer::Drawer, figuretypes::piechart::PieChart,
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
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for axis labels
        font_title: Some(
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
        ), // Path to font file for the title
    };

    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    let mut pie_chart = PieChart::new("Real-Time Market Share", figure_config.clone());

    pie_chart.add_slice("Company A", 33.0, [200, 0, 0]);
    pie_chart.add_slice("Company B", 33.0, [0, 200, 0]);
    pie_chart.add_slice("Company C", 33.0, [50, 100, 200]);
    pie_chart.add_slice("Company D", 33.0, [200, 200, 200]);
    pie_chart.add_slice("Company E", 33.0, [100, 50, 225]);

    pie_chart.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("piechart.png");

    let mut rng = rand::thread_rng();
    let update_data = move |chart: &mut PieChart| {
        chart.datasets[0].1 += rng.gen_range(0.0..4.0); // Increment Company A's share
        chart.datasets[1].1 += rng.gen_range(0.0..4.0); // Increment Company B's share
        chart.datasets[2].1 += rng.gen_range(0.0..4.0); // Increment Company C's share
        chart.datasets[3].1 += rng.gen_range(0.0..4.0); // Increment Company D's share
        chart.datasets[4].1 += rng.gen_range(0.0..4.0); // Increment Company E's share
    };

    // Display the pie chart in real-time
    Winop::display_real_time(
        &mut pixel_canvas,
        &mut pie_chart,
        "Real-Time Pie Chart",
        update_data,
        10,
    );
}