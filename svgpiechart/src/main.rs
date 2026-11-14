use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas, configuration::figureconfig::FigureConfig,
    drawers::drawer::Drawer, figuretypes::piechart::PieChart,
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
        num_axis_ticks: 10,           // Tick intervals per axis (10 + 1 labels)
        num_grid_horizontal: 10,      // Horizontal grid intervals
        num_grid_vertical: 10,        // Vertical grid intervals
        font_label: None, // None: an installed sans-serif font is used; or Some("path/to/font.ttf".to_string())
        font_title: None, // None: falls back to font_label / a system font
    };

    let mut pie_chart = PieChart::new("Market Share", figure_config);

    // Slices are drawn counter-clockwise, starting at 3 o'clock.
    pie_chart.add_slice("Product A", 40.0, [255, 99, 71]);
    pie_chart.add_slice("Product B", 25.0, [60, 179, 113]);
    pie_chart.add_slice("Product C", 20.0, [65, 105, 225]);
    pie_chart.add_slice("Product D", 10.0, [238, 180, 34]);
    pie_chart.add_slice("Other", 5.0, [180, 180, 180]);

    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    pie_chart.draw_svg(&mut svg_canvas);
    svg_canvas.save("pie_chart.svg").unwrap();
}
