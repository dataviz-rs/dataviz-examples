//! Stacked areas: Stacked and 100 % stacked area charts side by side.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    fig.grid(1, 2);
    let x: Vec<f64> = (0..20).map(|i| i as f64).collect();
    let layers = || {
        vec![
            (
                "Solar",
                (0..20).map(|i| 2.0 + i as f64 * 0.5).collect::<Vec<_>>(),
            ),
            (
                "Wind",
                (0..20)
                    .map(|i| 5.0 + (i as f64 * 0.5).sin() * 2.0)
                    .collect(),
            ),
            ("Hydro", vec![4.0; 20]),
        ]
    };
    let ax = fig.at(0, 0);
    ax.title("Stacked area");
    ax.stacked_area(&x, layers(), false);
    ax.legend(LegendPosition::UpperLeft);
    let ax = fig.at(0, 1);
    ax.title("100 % stacked");
    ax.stacked_area(&x, layers(), true);
    ax.y_axis()
        .format(TickFormat::Affix(String::new(), "%".into()));
    ax.legend(LegendPosition::Hidden);

    fig.save("stacked_area.png")?;
    fig.save("stacked_area.svg")?;
    Ok(())
}
