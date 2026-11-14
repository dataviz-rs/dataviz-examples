//! Heatmap (correlation matrix): Annotated correlation matrix with a diverging colormap and colorbar.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Correlation matrix");
    let names = ["temp", "humidity", "wind", "pressure", "rain"];
    let m = vec![
        vec![1.0, -0.62, 0.15, -0.31, -0.22],
        vec![-0.62, 1.0, -0.08, 0.12, 0.71],
        vec![0.15, -0.08, 1.0, -0.45, 0.33],
        vec![-0.31, 0.12, -0.45, 1.0, -0.54],
        vec![-0.22, 0.71, 0.33, -0.54, 1.0],
    ];
    ax.heatmap_labeled(names, names, m)
        .colormap(Colormap::RdBu)
        .range(-1.0, 1.0)
        .annotate(true)
        .decimals(2)
        .colorbar("r");

    fig.save("heatmap.png")?;
    fig.save("heatmap.svg")?;
    Ok(())
}
