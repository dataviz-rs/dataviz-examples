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
    dependencies()
}

/// Imports between the modules of five packages: an arc diagram and
/// hierarchical edge bundling of the same graph.
fn dependencies() -> Result<(), Error> {
    let packages = ["core", "io", "net", "ui", "util"];
    let names: Vec<String> = (0..40)
        .map(|i| format!("{}.{}", packages[i / 8], ["a", "b", "c", "d", "e", "f", "g", "h"][i % 8]))
        .collect();
    let groups: Vec<usize> = (0..40).map(|i| i / 8).collect();
    // A small deterministic generator: most imports stay in their package.
    let mut s = 7u64;
    let mut next = || {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (s >> 33) as usize
    };
    let mut edges = Vec::new();
    for _ in 0..120 {
        let a = next() % 40;
        let b = if next() % 3 == 0 { next() % 40 } else { (a / 8) * 8 + next() % 8 };
        if a != b {
            edges.push((a, b, 1.0 + (next() % 4) as f64));
        }
    }
    let mut fig = Figure::new(1400, 700);
    fig.grid(1, 2);
    fig.at(0, 0)
        .title("Arc diagram")
        .arc_diagram(&names, edges.clone())
        .groups(groups.clone())
        .group_names(packages);
    fig.at(0, 1)
        .title("Hierarchical edge bundling")
        .edge_bundling(&names, groups, edges)
        .group_names(packages);
    fig.save("dependencies.png")?;
    fig.save("dependencies.svg")
}
