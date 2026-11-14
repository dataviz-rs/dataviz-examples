//! Network graph: Force-directed network layout with node groups.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Network (force-directed)");
    let names: Vec<String> = (0..16).map(|i| format!("n{i}")).collect();
    let mut edges = vec![];
    for i in 0..16usize {
        edges.push((i, (i + 1) % 6 + (i / 6) * 6, 1.0));
        if i % 6 == 0 {
            edges.push((i, (i + 6) % 16, 2.0));
        }
    }
    ax.network(names, edges).groups((0..16).map(|i| i / 6));

    fig.save("network.png")?;
    fig.save("network.svg")?;
    Ok(())
}
