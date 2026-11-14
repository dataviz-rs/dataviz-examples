//! Pie and donut charts: Pie with exploded slice and labels; donut with centre text.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Pie");
    ax.pie(
        ["Chrome", "Safari", "Edge", "Firefox", "Other"],
        [64.0, 19.0, 5.0, 3.0, 9.0],
    )
    .explode([0.06, 0.0, 0.0, 0.0, 0.0])
    .labels(true);
    ax.legend(LegendPosition::Hidden);
    let ax = fig.at(0, 1);
    ax.title("Donut");
    ax.pie(
        ["Rent", "Food", "Transport", "Fun", "Savings"],
        [1200.0, 500.0, 200.0, 250.0, 600.0],
    )
    .donut(0.55)
    .center_text("$2750");
    ax.legend(LegendPosition::OutsideRight);

    fig.save("pie_donut.png")?;
    fig.save("pie_donut.svg")?;
    Ok(())
}
