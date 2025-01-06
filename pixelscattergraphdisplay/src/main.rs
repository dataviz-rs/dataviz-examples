use dataviz::figure::{
    canvas::pixelcanvas::PixelCanvas, configuration::figureconfig::FigureConfig,
    display::winop::Winop, drawers::drawer::Drawer, figuretypes::scattergraph::ScatterGraph,
    utilities::scatterdottype::ScatterDotType, datasets::scattergraphdataset::ScatterGraphDataset,
    datasets::dataset::Dataset
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

    // Scatter Graph
    let mut pixel_canvas = PixelCanvas::new(800, 600, [255, 255, 255], 80);
    let mut scatter_graph = ScatterGraph::new("Data Points", "X", "Y", figure_config.clone());
    let mut rng = rand::thread_rng();

    let mut dataset1 =
        ScatterGraphDataset::new([220, 0, 0], "Dataset 1", ScatterDotType::Circle(2));
    let mut dataset2 = ScatterGraphDataset::new([0, 220, 0], "Dataset 2", ScatterDotType::Cross(5));
    let mut dataset3 =
        ScatterGraphDataset::new([0, 0, 220], "Dataset 3", ScatterDotType::Square(5));
    let mut dataset4 =
        ScatterGraphDataset::new([50, 50, 50], "Dataset 4", ScatterDotType::Triangle(5));

    for _ in 0..10 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(0.5..2.0);
        dataset1.add_point((x, y));
    }

    for _ in 0..10 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(0.0..2.0);
        dataset2.add_point((x, y));
    }

    for _ in 0..10 {
        let x = rng.gen_range(0.0..3.0);
        let y = rng.gen_range(2.0..3.0);
        dataset3.add_point((x, y));
    }

    for _ in 0..10 {
        let x = rng.gen_range(1.5..3.0);
        let y = rng.gen_range(1.5..3.0);
        dataset4.add_point((x, y));
    }

    scatter_graph.add_dataset(dataset1);
    scatter_graph.add_dataset(dataset2);
    scatter_graph.add_dataset(dataset3);
    scatter_graph.add_dataset(dataset4);

    scatter_graph.draw(&mut pixel_canvas);
    pixel_canvas.save_as_image("scatter_graph.png");

    let mut rng = rand::thread_rng();
    let update_data = move |graph: &mut ScatterGraph| {
        for i in 0..graph.datasets.len() {
            let x = rng.gen_range(0.0..3.0);
            let y = rng.gen_range(0.0..3.0);
            graph.datasets[i].add_point((x, y));
        }
    };

    // Display the Scatter in real-time
    Winop::display_real_time(
        &mut pixel_canvas,
        &mut scatter_graph,
        "Real-Time Scatter",
        update_data,
        10,
    );
}
