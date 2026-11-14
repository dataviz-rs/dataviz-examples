//! Vector field and function plots: Colored quiver plot and adaptively sampled function plots.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Vector field");
    let mut x = vec![];
    let mut y = vec![];
    for i in -8..=8 {
        for j in -8..=8 {
            x.push(i as f64 / 4.0);
            y.push(j as f64 / 4.0);
        }
    }
    let u: Vec<f64> = y.iter().map(|v| -v).collect();
    let v: Vec<f64> = x.clone();
    ax.quiver(&x, &y, u, v).colormap(Colormap::Plasma);
    ax.equal_aspect(true);
    let ax = fig.at(0, 1);
    ax.title("Function plots");
    ax.function(|x| x.sin() / x, -20.0, 20.0).label("sin(x)/x");
    ax.function(|x| (x / 4.0).tan() / 10.0, -20.0, 20.0)
        .label("tan(x/4)/10");
    ax.y_limits(-0.5, 1.2);

    fig.save("quiver_function.png")?;
    fig.save("quiver_function.svg")?;
    Ok(())
}
