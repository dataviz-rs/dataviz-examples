use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas, // For creating a SVG canvas to draw the chart
    datasets::bardataset::BarDataset, // For defining datasets for bar charts
    drawers::drawer::Drawer,          // For drawing functionality
    figurefactory::FigureFactory,     // For creating figures
    figurefactory::FigureType,        // For specifying the type of figure to create
    figuretypes::groupbarchart::GroupBarChart, // For creating grouped bar charts
};

fn main() {
    let mut figure = FigureFactory::create_figure(FigureType::GroupBarChartVertical);

    // Attempt to downcast the Drawer trait object to GroupBarChart
    // Dereference the Box and use downcast_mut
    if let Some(group_bar_chart) = figure.as_any().downcast_mut::<GroupBarChart>() {
        group_bar_chart.config.set_font_paths(
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
            "path/to/dataviz/resources/fonts/Arial.ttf".to_string(),
        );

        // Create a SVG canvas for rendering the chart
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
        dataset4.add_data(2020.0, 50.0); // Add data point: Year 2020, value 50
        dataset4.add_data(2021.0, 256.0); // Add data point: Year 2021, value 256
        dataset4.add_data(2022.0, 40.0); // Add data point: Year 2022, value 40

        // Add datasets to the bar chart
        group_bar_chart.add_dataset(dataset1);
        group_bar_chart.add_dataset(dataset2);
        group_bar_chart.add_dataset(dataset3);
        group_bar_chart.add_dataset(dataset4);

        // Draw the bar chart onto the canvas
        group_bar_chart.draw(&mut canvas);

        // Save the canvas as an image file
        canvas.save_as_image("grouped_vertical_bar_chart.png");
    } else {
        println!("Failed to downcast to GroupBarChart.");
    }
}
