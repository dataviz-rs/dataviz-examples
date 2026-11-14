//! Subplots with a shared x-axis: A 2 × 2 grid of plots sharing the x limits.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(2, 2).share_x(true);
    let x: Vec<f64> = (0..100).map(|i| i as f64 / 10.0).collect();
    for (i, name) in ["sin", "cos", "tanh", "exp"].iter().enumerate() {
        let ax = fig.at(i / 2, i % 2);
        ax.title(*name);
        let y: Vec<f64> = x
            .iter()
            .map(|v| match i {
                0 => v.sin(),
                1 => v.cos(),
                2 => (v - 5.0).tanh(),
                _ => (v / 3.0).exp(),
            })
            .collect();
        ax.line(&x, y);
    }

    fig.save("subplots_shared.png")?;
    fig.save("subplots_shared.svg")?;
    Ok(())
}
