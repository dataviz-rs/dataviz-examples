//! Line styles: Solid, dashed, dotted, dash-dot and long-dash lines with an outside legend.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Line styles").x_label("x").y_label("y");
    let x: Vec<f64> = (0..=100).map(|i| i as f64 / 10.0).collect();
    for (k, d) in [
        Dash::Solid,
        Dash::Dashed,
        Dash::Dotted,
        Dash::DashDot,
        Dash::LongDash,
    ]
    .into_iter()
    .enumerate()
    {
        ax.line(&x, x.iter().map(|v| (v + k as f64).sin() + k as f64 * 0.6))
            .dash(d.clone())
            .label(format!("{d:?}"));
    }
    ax.legend(LegendPosition::OutsideRight);

    fig.save("line.png")?;
    fig.save("line.svg")?;
    Ok(())
}
