//! Business and editorial charts: bullet, dumbbell, slope, bump, waffle,
//! marimekko, chord diagram, streamgraph and horizon chart.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1500, 1260);
    fig.grid(3, 3);

    fig.at(0, 0)
        .title("Bullet: KPIs against target")
        .bullet(
            ["Revenue", "Profit", "Avg order", "New customers", "Satisfaction"],
            [270.0, 22.5, 310.0, 1650.0, 4.3],
            [250.0, 26.0, 330.0, 2100.0, 4.5],
            &[
                vec![150.0, 225.0, 300.0],
                vec![20.0, 25.0, 30.0],
                vec![220.0, 300.0, 400.0],
                vec![1400.0, 2000.0, 2500.0],
                vec![3.5, 4.25, 5.0],
            ],
        );
    // Rows have different units: each is drawn in % of its own range.
    fig.at(0, 0).x_label("% of each row's range (value / target)");

    fig.at(0, 1)
        .title("Dumbbell: life expectancy 2000 → 2023")
        .dumbbell(
            ["Japan", "Türkiye", "Brazil", "India", "Nigeria", "Germany", "USA"],
            [81.1, 70.0, 70.1, 62.7, 46.3, 77.9, 76.6],
            [84.7, 77.5, 75.9, 72.0, 54.5, 81.1, 78.4],
        )
        .labels("2000", "2023");
    fig.at(0, 1).x_label("years").legend(LegendPosition::OutsideBottom);

    fig.at(0, 2)
        .title("Slope: market share (%)")
        .slope(
            ["Alpha", "Beta", "Gamma", "Delta", "Epsilon"],
            [32.0, 24.0, 18.0, 15.0, 11.0],
            [27.0, 29.0, 21.0, 9.0, 14.0],
            "2020",
            "2025",
        );

    fig.at(1, 0).title("Bump: league position by season").bump(
        ["Lions", "Eagles", "Wolves", "Sharks", "Bears"],
        ["2020", "2021", "2022", "2023", "2024", "2025"],
        &[
            vec![1.0, 1.0, 2.0, 3.0, 2.0, 1.0],
            vec![2.0, 3.0, 1.0, 1.0, 1.0, 2.0],
            vec![3.0, 2.0, 3.0, 2.0, 4.0, 3.0],
            vec![4.0, 5.0, 5.0, 4.0, 3.0, 4.0],
            vec![5.0, 4.0, 4.0, 5.0, 5.0, 5.0],
        ],
    );

    fig.at(1, 1)
        .title("Waffle: energy mix")
        .waffle(["Solar", "Wind", "Hydro", "Gas", "Coal"], [21.0, 27.0, 14.0, 23.0, 15.0]);

    fig.at(1, 2).title("Marimekko: segment × region sales").marimekko(
        ["North", "South", "East", "West"],
        ["Retail", "Online", "B2B"],
        &[vec![40.0, 25.0, 15.0], vec![20.0, 30.0, 5.0], vec![12.0, 8.0, 20.0], vec![30.0, 20.0, 10.0]],
    );

    fig.at(2, 0).title("Chord: migration between regions").chord(
        ["Europe", "Asia", "Africa", "Americas", "Oceania"],
        &[
            vec![6.0, 2.0, 1.0, 3.0, 0.5],
            vec![4.0, 9.0, 1.0, 5.0, 1.5],
            vec![3.5, 1.0, 4.0, 1.5, 0.2],
            vec![2.0, 1.5, 0.3, 7.0, 0.4],
            vec![0.4, 0.8, 0.1, 0.5, 0.6],
        ],
    );

    let weeks: Vec<f64> = (0..60).map(|i| i as f64).collect();
    let wave = |a: f64, f: f64, p: f64| -> Vec<f64> {
        weeks.iter().map(|t| a * (1.2 + (t / f + p).sin()) * (1.0 + t / 120.0)).collect()
    };
    let ax = fig.at(2, 1);
    ax.title("Streamgraph: listening by genre");
    ax.streamgraph(
        &weeks,
        &[
            ("Pop", wave(5.0, 9.0, 0.0)),
            ("Rock", wave(3.5, 13.0, 1.0)),
            ("Hip-hop", wave(4.0, 7.0, 2.0)),
            ("Jazz", wave(1.5, 11.0, 3.0)),
            ("Electronic", wave(3.0, 8.0, 4.0)),
        ],
    );
    ax.x_label("week");

    let days: Vec<f64> = (0..250).map(|i| i as f64).collect();
    let stock = |seed: f64, drift: f64| -> Vec<f64> {
        let mut v = 0.0;
        days.iter()
            .map(|t| {
                v += ((t * 12.9898 + seed).sin() * 43758.5453).rem_euclid(1.0) - 0.5 + drift;
                v
            })
            .collect()
    };
    let ax = fig.at(2, 2);
    ax.title("Horizon: 6 stocks, return in %");
    ax.horizon(
        &days,
        &[
            ("AAA", stock(1.0, 0.05)),
            ("BBB", stock(2.0, -0.04)),
            ("CCC", stock(3.0, 0.02)),
            ("DDD", stock(4.0, 0.0)),
            ("EEE", stock(5.0, 0.08)),
            ("FFF", stock(6.0, -0.02)),
        ],
    );
    ax.x_label("trading day");

    fig.save("business.png")?;
    fig.save("business.svg")?;
    Ok(())
}
