//! Contour plots: Labelled contour lines and filled contours with a colorbar.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let g = |x: f64, y: f64| (x * x + y - 11.0).powi(2) + (x + y * y - 7.0).powi(2);
    let ax = fig.at(0, 0);
    ax.title("Contour lines (Himmelblau)");
    ax.contour_fn(|x, y| g(x, y).ln(), (-5.0, 5.0), (-5.0, 5.0), 120)
        .level_count(12)
        .labels(true);
    let ax = fig.at(0, 1);
    ax.title("Filled contours");
    ax.contour_fn(
        |x, y| (x * y).sin() * (-0.05 * (x * x + y * y)).exp(),
        (-6.0, 6.0),
        (-6.0, 6.0),
        150,
    )
    .level_count(14)
    .filled(true)
    .colormap(Colormap::Coolwarm)
    .colorbar("z");

    fig.save("contour.png")?;
    fig.save("contour.svg")?;
    Ok(())
}
