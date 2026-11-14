//! Tick formats: SI-prefixed and percent tick labels.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Tick formats");
    let x: Vec<f64> = (0..10).map(|i| i as f64 * 250_000.0).collect();
    ax.line(&x, x.iter().map(|v| v / 3e6))
        .marker(Marker::Square);
    ax.x_axis().format(TickFormat::Si(1)).label("bytes");
    ax.y_axis().format(TickFormat::Percent(0)).label("share");

    fig.save("formats.png")?;
    fig.save("formats.svg")?;
    Ok(())
}
