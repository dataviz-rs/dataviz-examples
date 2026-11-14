//! Error bars and confidence bands: Measurements with error bars over a model line with a ±1 band.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Error bars and confidence band");
    let x: Vec<f64> = (0..10).map(|i| i as f64).collect();
    let y: Vec<f64> = x.iter().map(|v| v * 0.8 + (v * 1.3).sin()).collect();
    ax.error_bars(&x, &y, x.iter().map(|v| 0.3 + v * 0.05))
        .label("measurements");
    let xs: Vec<f64> = (0..=90).map(|i| i as f64 / 10.0).collect();
    let fit: Vec<f64> = xs.iter().map(|v| v * 0.8).collect();
    ax.error_band(
        &xs,
        &fit,
        fit.iter().map(|v| v - 1.0),
        fit.iter().map(|v| v + 1.0),
    )
    .label("model ± 1");

    fig.save("errorbars_band.png")?;
    fig.save("errorbars_band.svg")?;
    Ok(())
}
