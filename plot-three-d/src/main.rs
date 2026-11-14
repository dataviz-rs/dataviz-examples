//! 3D surface and trajectory: A 3D surface colored by height and a 3D spiral.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Surface");
    ax.surface_fn(
        |x, y| (x.hypot(y)).sin() / x.hypot(y).max(1e-9),
        (-8.0, 8.0),
        (-8.0, 8.0),
        40,
    )
    .colormap(Colormap::Viridis)
    .labels("x", "y", "z");
    let ax = fig.at(0, 1);
    ax.title("3D trajectory");
    let t: Vec<f64> = (0..600).map(|i| i as f64 / 30.0).collect();
    ax.line3d(
        t.iter().map(|v| v.cos() * (1.0 + v / 10.0)),
        t.iter().map(|v| v.sin() * (1.0 + v / 10.0)),
        &t,
    )
    .colormap(Colormap::Plasma)
    .labels("x", "y", "t");

    fig.save("three_d.png")?;
    fig.save("three_d.svg")?;
    Ok(())
}
