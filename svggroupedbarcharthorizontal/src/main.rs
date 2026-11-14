use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas,
    configuration::figureconfig::FigureConfig,
    datasets::bardataset::BarDataset,
    drawers::drawer::Drawer,
    figuretypes::groupbarchart::GroupBarChart,
    utilities::orientation::Orientation,
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

    // With Orientation::Horizontal, the categories (x values, here years) are
    // on the vertical axis and the values on the horizontal axis.
    let mut barchart = GroupBarChart::new(
        "Revenue by Year",
        "Year",
        "Revenue (k$)",
        Orientation::Horizontal,
        figure_config,
    );

    let companies = [
        ("Company A", [220, 0, 0], [100.0, 200.0, 150.0, 210.0]),
        ("Company B", [0, 160, 0], [120.0, 180.0, 220.0, 190.0]),
        ("Company C", [0, 0, 220], [150.0, 250.0, 400.0, 330.0]),
    ];

    for (name, color, revenue) in companies {
        let mut dataset = BarDataset::new(name, color);
        for (year, value) in [2021.0, 2022.0, 2023.0, 2024.0].into_iter().zip(revenue) {
            dataset.add_data(year, value);
        }
        barchart.add_dataset(dataset);
    }

    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    barchart.draw_svg(&mut svg_canvas);
    svg_canvas.save("grouped_horizontal_bar_chart.svg").unwrap();
}
