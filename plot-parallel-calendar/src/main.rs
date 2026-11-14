//! Parallel coordinates and calendar heatmap: Iris-like data in parallel coordinates and a year of daily values as a calendar.

use dataviz::prelude::*;

/// Deterministic pseudo-random numbers (no extra dependency).
struct Rng(u64);
impl Rng {
    fn uniform(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    #[allow(dead_code)]
    fn normal(&mut self) -> f64 {
        (0..12).map(|_| self.uniform()).sum::<f64>() - 6.0
    }
}

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let mut r = Rng(9);
    fig.grid(2, 1);
    let ax = fig.at(0, 0);
    ax.title("Parallel coordinates");
    let rows: Vec<Vec<f64>> = (0..60)
        .map(|i| {
            let c = (i % 3) as f64;
            vec![
                5.0 + c + r.normal() * 0.4,
                3.0 - c * 0.3 + r.normal() * 0.3,
                1.5 + c * 2.0 + r.normal() * 0.4,
                0.2 + c * 0.8 + r.normal() * 0.2,
            ]
        })
        .collect();
    ax.parallel_coordinates(["sepal L", "sepal W", "petal L", "petal W"], rows)
        .classes(
            (0..60).map(|i| i % 3),
            ["setosa", "versicolor", "virginica"],
        );
    ax.legend(LegendPosition::OutsideRight);
    let ax = fig.at(1, 0);
    ax.title("Calendar heatmap");
    let days: Vec<f64> = (0..365)
        .map(|d| date(2024, 1, 1) + d as f64 * 86400.0)
        .collect();
    let vals: Vec<f64> = (0..365)
        .map(|d| ((d as f64 / 20.0).sin() + 1.0) * 5.0 + r.uniform() * 4.0)
        .collect();
    ax.calendar(&days, &vals).colorbar("commits");

    fig.save("parallel_calendar.png")?;
    fig.save("parallel_calendar.svg")?;
    Ok(())
}
