//! Area and fill between: Filled area under a curve and the band between two curves.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Area and fill between");
    let x: Vec<f64> = (0..=200).map(|i| i as f64 / 20.0).collect();
    ax.area(&x, x.iter().map(|v| v.sin() + 1.5)).label("area");
    ax.fill_between(
        &x,
        x.iter().map(|v| (v * 0.8).cos() - 1.0),
        x.iter().map(|v| (v * 0.8).cos() - 2.5),
    )
    .label("between");

    fig.save("area_fill.png")?;
    fig.save("area_fill.svg")?;
    Ok(())
}
