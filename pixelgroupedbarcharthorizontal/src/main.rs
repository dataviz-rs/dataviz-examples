use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas,          // For creating a pixel canvas to draw the chart
    configuration::figureconfig::FigureConfig, // For configuring visual properties of the chart
    datasets::bardataset::BarDataset,          // For defining datasets for bar charts
    drawers::drawer::Drawer,                   // For drawing functionality
    figuretypes::groupbarchart::GroupBarChart, // For creating grouped bar charts
    utilities::orientation::Orientation,       // For setting the chart orientation (vertical/horizontal)
};

fn main() {
    // Configuration for the figure (e.g., fonts, colors, grid, and axis settings)
    let figure_config = FigureConfig {
        font_size_title: 20.0,   // Font size for the chart title
        font_size_label: 16.0,   // Font size for axis labels
        font_size_legend: 14.0,  // Font size for legend
        font_size_axis: 10.0,    // Font size for axis tick labels
        color_axis: [0, 0, 0],   // Color of the axes (black)
        color_background: [0, 0, 0], // Background color of the chart (black)
        color_grid: [220, 220, 220], // Grid color (light gray)
        color_title: [0, 0, 0],      // Title color (black)
        num_axis_ticks: 20,          // Number of axis tick marks
        num_grid_horizontal: 20,     // Number of horizontal grid lines
        num_grid_vertical: 20,       // Number of vertical grid lines
        font_label: Some("C:/Users/samet/Desktop/Rust/rust-lab/dataviz/resources/fonts/Arial.ttf"
        .to_string()), // Path to the font for axis labels
        font_title: Some("C:/Users/samet/Desktop/Rust/rust-lab/dataviz/resources/fonts/Arial.ttf"
        .to_string()), // Path to the font for the title
    };

    // Create a grouped bar chart with the given configuration
    let mut barchart = GroupBarChart::new(
        "Grouped Bar Chart",    // Chart title
        "X Axis",               // X-axis label
        "Y Axis",               // Y-axis label
        Orientation::Horizontal,  // Orientation of the bar chart (Horizontal)
        figure_config,          // Pass the figure configuration
    );

    // Create a pixel canvas for rendering the chart
    let mut canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80); // 800x600 canvas with white background

    // Define datasets for the bar chart
    let mut dataset1 = BarDataset::new("Company A", [220, 0, 0]); // Dataset for Company A (red bars)
    dataset1.add_data(2020.0, 100.0); // Add data point: Year 2020, value 100
    dataset1.add_data(2021.0, 200.0); // Add data point: Year 2021, value 200
    dataset1.add_data(2022.0, 150.0); // Add data point: Year 2022, value 150

    let mut dataset2 = BarDataset::new("Company B", [0, 220, 0]); // Dataset for Company B (green bars)
    dataset2.add_data(2020.0, 120.0); // Add data point: Year 2020, value 120
    dataset2.add_data(2021.0, 180.0); // Add data point: Year 2021, value 180
    dataset2.add_data(2022.0, 220.0); // Add data point: Year 2022, value 220

    let mut dataset3 = BarDataset::new("Company C", [0, 0, 220]); // Dataset for Company C (blue bars)
    dataset3.add_data(2020.0, 150.0); // Add data point: Year 2020, value 150
    dataset3.add_data(2021.0, 250.0); // Add data point: Year 2021, value 250
    dataset3.add_data(2022.0, 400.0); // Add data point: Year 2022, value 400

    let mut dataset4 = BarDataset::new("Company D", [150, 100, 50]); // Dataset for Company D (brown bars)
    dataset4.add_data(2020.0, 50.0);  // Add data point: Year 2020, value 50
    dataset4.add_data(2021.0, 256.0); // Add data point: Year 2021, value 256
    dataset4.add_data(2022.0, 40.0);  // Add data point: Year 2022, value 40

    // Add datasets to the bar chart
    barchart.add_dataset(dataset1);
    barchart.add_dataset(dataset2);
    barchart.add_dataset(dataset3);
    barchart.add_dataset(dataset4);

    // Draw the bar chart onto the canvas
    barchart.draw(&mut canvas);

    // Save the canvas as an image file
    canvas.save_as_image("grouped_horizontal_bar_chart.png");
}
