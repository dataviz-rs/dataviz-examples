//! Funnel and gauge: A sales funnel and a gauge with colored ranges.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Sales funnel");
    ax.funnel(
        ["Visits", "Sign-ups", "Trials", "Purchases"],
        [10000.0, 3200.0, 1100.0, 420.0],
    );
    let ax = fig.at(0, 1);
    ax.title("CPU load");
    ax.gauge(73.0, 0.0, 100.0)
        .bands([
            (50.0, Color::GREEN),
            (80.0, Color::ORANGE),
            (100.0, Color::RED),
        ])
        .label("percent");

    fig.save("funnel_gauge.png")?;
    fig.save("funnel_gauge.svg")?;
    Ok(())
}
