//! Markers, steps and smoothing: Point markers, step lines, monotone spline smoothing and marker-only series.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Markers, steps and smoothing");
    let x: Vec<f64> = (0..12).map(|i| i as f64).collect();
    let y: Vec<f64> = x.iter().map(|v| (v * 0.7).sin() * 3.0 + 5.0).collect();
    ax.line(&x, &y).marker(Marker::Circle).label("markers");
    ax.line(&x, y.iter().map(|v| v + 4.0))
        .step(Step::Post)
        .label("step (post)");
    ax.line(&x, y.iter().map(|v| v - 4.0))
        .smooth(true)
        .marker(Marker::Diamond)
        .label("smooth");
    ax.line(&x, y.iter().map(|v| v + 8.0))
        .width(0.0)
        .marker(Marker::Star)
        .marker_size(11.0)
        .label("markers only");

    fig.save("markers_steps_smooth.png")?;
    fig.save("markers_steps_smooth.svg")?;
    Ok(())
}
