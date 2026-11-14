//! Logarithmic scales: Log-log axes with minor ticks and a symmetric-log axis.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let x: Vec<f64> = (0..=60).map(|i| 10f64.powf(i as f64 / 10.0)).collect();
    let ax = fig.at(0, 0);
    ax.title("Log–log");
    ax.line(&x, x.iter().map(|v| v.powf(1.5))).label("x^1.5");
    ax.line(&x, x.iter().map(|v| v.sqrt() * 10.0))
        .label("10·√x");
    ax.x_axis().log().minor_ticks(true);
    ax.y_axis().log().minor_ticks(true);
    let ax = fig.at(0, 1);
    ax.title("Symmetric log");
    let x: Vec<f64> = (-100..=100).map(|i| i as f64).collect();
    ax.line(&x, x.iter().map(|v| v.powi(3))).label("x³");
    ax.y_scale(Scale::SymLog(1.0));

    fig.save("log_scales.png")?;
    fig.save("log_scales.svg")?;
    Ok(())
}
