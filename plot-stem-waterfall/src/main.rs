//! Stem and waterfall charts: A stem plot and a waterfall chart with a total.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Stem");
    let x: Vec<f64> = (0..30).map(|i| i as f64).collect();
    ax.stem(&x, x.iter().map(|v| (v * 0.4).sin() * (-v / 15.0).exp()));
    let ax = fig.at(0, 1);
    ax.title("Waterfall");
    ax.waterfall(
        ["Start", "Sales", "Services", "Costs", "Tax", "Total"],
        [100.0, 45.0, 20.0, -60.0, -15.0, 0.0],
    )
    .total_at(5);

    fig.save("stem_waterfall.png")?;
    fig.save("stem_waterfall.svg")?;
    Ok(())
}
