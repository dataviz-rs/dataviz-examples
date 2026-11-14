//! The same chart in every built-in theme.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    for (name, theme) in Theme::presets() {
        let mut fig = Figure::new(800, 500);
        fig.theme(theme);
        let ax = fig.axes();
        ax.title(format!("Theme: {name}"));
        let x: Vec<f64> = (0..=60).map(|i| i as f64 / 6.0).collect();
        for k in 0..4 {
            ax.line(&x, x.iter().map(|v| (v + k as f64).sin() * (1.0 + k as f64 * 0.3)))
                .label(format!("series {k}"));
        }
        ax.x_label("x").y_label("y");
        fig.save(format!("theme_{name}.png"))?;
    }
    Ok(())
}
