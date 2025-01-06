use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::{areachartdataset::AreaChartDataset, dataset::Dataset},
    drawers::drawer::Drawer,
    figuretypes::areachart::AreaChart,
};

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

    // Create a new svg-based canvas with specified dimensions and background color
    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);

    // Create a new area chart with a title and axis labels
    let mut area_chart = AreaChart::new(
        "Area Chart Example",  // Title of the chart
        "X Axis",              // Label for the x-axis
        "Y Axis",              // Label for the y-axis
        figure_config.clone(), // Use the configured figure settings
    );

    // Define the first dataset for the area chart
    let mut dataset1 = AreaChartDataset::new([220, 0, 0], "Dataset 1", 0.5); // Dataset with red color and 50% opacity
    dataset1.add_point((0.0, 0.0)); // Add a point to the dataset
    dataset1.add_point((1.0, 2.0)); // Add another point
    dataset1.add_point((2.0, 1.0)); // Continue adding points
    dataset1.add_point((3.0, 3.0)); // Final point for the dataset

    // Define the second dataset for the area chart
    let mut dataset2 = AreaChartDataset::new([0, 220, 0], "Dataset 2", 0.5); // Dataset with green color and 50% opacity
    dataset2.add_point((0.0, 1.0)); // Add points to the second dataset
    dataset2.add_point((1.0, 1.5));
    dataset2.add_point((2.0, 0.5));
    dataset2.add_point((3.0, 2.0));

    // Define the third dataset for the area chart
    let mut dataset3 = AreaChartDataset::new([0, 0, 220], "Dataset 3", 0.5); // Dataset with blue color and 50% opacity
    dataset3.add_point((0.0, 2.5)); // Add points to the third dataset
    dataset3.add_point((1.0, 0.5));
    dataset3.add_point((2.0, 0.5));
    dataset3.add_point((3.0, 1.5));

    // Add datasets to the area chart
    area_chart.add_dataset(dataset1); // Add the first dataset
    area_chart.add_dataset(dataset2); // Add the second dataset
    area_chart.add_dataset(dataset3); // Add the third dataset

    // Render the area chart onto the canvas
    area_chart.draw_svg(&mut svg_canvas);

    // Save the rendered chart as an svg file
    svg_canvas.save("area_chart.svg").unwrap();
}
