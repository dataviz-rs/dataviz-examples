//! Polar, radar and rose charts: Polar curves, a radar (spider) chart and a wind rose.

use dataviz::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 3);
    let ax = fig.at(0, 0);
    ax.title("Polar");
    let t: Vec<f64> = (0..=360).map(|i| (i as f64).to_radians()).collect();
    ax.polar_line(&t, t.iter().map(|a| (4.0 * a).sin().abs() * 2.0 + 1.0))
        .label("rose curve");
    ax.polar_line(&t, t.iter().map(|a| 1.0 + a / PI))
        .label("spiral")
        .dash(Dash::Dashed);
    ax.legend(LegendPosition::Hidden);
    let ax = fig.at(0, 1);
    ax.title("Radar");
    ax.radar(
        ["Speed", "Power", "Range", "Comfort", "Price", "Safety"],
        [
            ("Model A", vec![8.0, 6.0, 7.0, 5.0, 4.0, 9.0]),
            ("Model B", vec![6.0, 8.0, 5.0, 8.0, 7.0, 6.0]),
        ],
    );
    ax.r_limits(0.0, 10.0);
    ax.legend(LegendPosition::OutsideBottom);
    let ax = fig.at(0, 2);
    ax.title("Wind rose");
    ax.rose(
        ["N", "NE", "E", "SE", "S", "SW", "W", "NW"],
        [12.0, 7.0, 5.0, 9.0, 14.0, 20.0, 16.0, 8.0],
    );
    ax.legend(LegendPosition::Hidden);

    fig.save("polar_radar.png")?;
    fig.save("polar_radar.svg")?;
    Ok(())
}
