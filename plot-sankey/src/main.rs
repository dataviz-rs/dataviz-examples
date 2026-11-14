//! Sankey diagram: Energy flows between sources, carriers and uses.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Energy flow");
    ax.sankey([
        ("Coal", "Electricity", 30.0),
        ("Gas", "Electricity", 25.0),
        ("Gas", "Heat", 20.0),
        ("Solar", "Electricity", 15.0),
        ("Oil", "Transport", 40.0),
        ("Electricity", "Homes", 35.0),
        ("Electricity", "Industry", 30.0),
        ("Electricity", "Losses", 5.0),
        ("Heat", "Homes", 15.0),
        ("Heat", "Industry", 5.0),
        ("Transport", "Losses", 12.0),
        ("Transport", "Mobility", 28.0),
    ]);

    fig.save("sankey.png")?;
    fig.save("sankey.svg")?;
    Ok(())
}
