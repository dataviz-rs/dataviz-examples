use dataviz::figure::{
    canvas::svgcanvas::SvgCanvas, configuration::figureconfig::FigureConfig,
    drawers::drawer::Drawer, figuretypes::histogram::Histogram,
};

/// Small deterministic pseudo-random generator (no extra dependency).
struct Lcg(u64);

impl Lcg {
    fn uniform(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Approximately normal: sum of 12 uniforms minus 6.
    fn normal(&mut self) -> f64 {
        (0..12).map(|_| self.uniform()).sum::<f64>() - 6.0
    }
}

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

    // 20 bins; every value is counted in exactly one bin.
    let mut histogram = Histogram::new(
        "Histogram",
        "Value",
        "Frequency",
        20,
        [100, 120, 230],
        figure_config,
    );

    let mut rng = Lcg(42);
    let values: Vec<f64> = (0..5000).map(|_| 50.0 + rng.normal() * 8.0).collect();
    histogram.add_data_vec(values); // faster than calling add_data in a loop

    let mut svg_canvas = SvgCanvas::new(800, 600, "white", 80);
    histogram.draw_svg(&mut svg_canvas);
    svg_canvas.save("histogram.svg").unwrap();
}
