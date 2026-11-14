//! Annotations: Arrow, text, shaded band, rectangle and reference line.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Annotations");
    let x: Vec<f64> = (0..=100).map(|i| i as f64 / 10.0).collect();
    ax.line(&x, x.iter().map(|v| (v * 0.8).sin() * v));
    ax.arrow((2.0, 6.0), (5.9, -4.6), Some("minimum"));
    ax.text(7.5, 6.0, "peak region").bold(true);
    ax.hspan(-1.0, 1.0).label("noise band");
    ax.rectangle(6.5, 3.5, 8.8, 7.5);
    ax.vline(5.0);

    fig.save("annotations.png")?;
    fig.save("annotations.svg")?;
    Ok(())
}
