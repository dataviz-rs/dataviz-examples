//! Bar charts: Grouped, stacked, percent (horizontal) and negative-value bar charts with value labels.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(2, 2);
    let cats = ["North", "South", "East", "West"];
    let data = || {
        vec![
            ("2022", vec![12.0, 9.0, 15.0, 7.0]),
            ("2023", vec![14.0, 11.0, 13.0, 9.0]),
            ("2024", vec![17.0, 10.0, 16.0, 12.0]),
        ]
    };
    let ax = fig.at(0, 0);
    ax.title("Grouped");
    for b in ax.bars(cats, data(), BarMode::Grouped, false) {
        b.value_labels(LabelPosition::Outside);
    }
    let ax = fig.at(0, 1);
    ax.title("Stacked");
    for b in ax.bars(cats, data(), BarMode::Stacked, false) {
        b.value_labels(LabelPosition::Inside);
    }
    let ax = fig.at(1, 0);
    ax.title("Percent (horizontal)");
    ax.bars(cats, data(), BarMode::Percent, true);
    ax.x_axis()
        .format(TickFormat::Affix(String::new(), "%".into()));
    ax.legend(LegendPosition::OutsideBottom);
    let ax = fig.at(1, 1);
    ax.title("Negative values");
    ax.bar(["Q1", "Q2", "Q3", "Q4", "Q5"], [4.0, -2.5, 6.0, -1.0, 3.0])
        .colors([
            Color::GREEN,
            Color::RED,
            Color::GREEN,
            Color::RED,
            Color::GREEN,
        ])
        .value_labels(LabelPosition::Outside)
        .corner_radius(3.0);

    fig.save("bars.png")?;
    fig.save("bars.svg")?;
    Ok(())
}
