//! Time series: Hourly data on a time axis, bars on a secondary y-axis, reference line and shaded span.

use dataviz::prelude::*;
use std::f64::consts::PI;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Time axis, annotations, twin axis");
    let t: Vec<f64> = (0..24 * 7)
        .map(|h| datetime(2024, 3, 4, 0, 0, 0.0) + h as f64 * 3600.0)
        .collect();
    let temp: Vec<f64> = t
        .iter()
        .enumerate()
        .map(|(i, _)| {
            12.0 + 6.0 * ((i as f64 - 6.0) / 24.0 * 2.0 * PI).sin() + i as f64 * 0.02
        })
        .collect();
    let rain: Vec<f64> = (0..t.len())
        .map(|i| {
            if (i / 13) % 4 == 0 {
                ((i % 13) as f64 * 0.4).sin().abs() * 3.0
            } else {
                0.0
            }
        })
        .collect();
    ax.line(&t, &temp).label("temperature °C");
    ax.bar_xy(&t, &rain)
        .width(3000.0)
        .color(Color::BLUE.with_alpha(0.5))
        .secondary(true)
        .label("rain mm");
    ax.x_axis().time();
    ax.y_label("°C");
    ax.y2_label("mm");
    ax.hline(15.0).label("threshold");
    ax.vspan(
        datetime(2024, 3, 6, 0, 0, 0.0),
        datetime(2024, 3, 7, 0, 0, 0.0),
    )
    .label("holiday");
    ax.legend(LegendPosition::OutsideBottom);

    fig.save("time_series.png")?;
    fig.save("time_series.svg")?;
    Ok(())
}
