//! Scatter and bubble charts: Colormapped scatter with a colorbar and regression line; bubble chart on a log axis.

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
    let mut r = Rng(1);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Scatter, colored by value");
    let x: Vec<f64> = (0..300).map(|_| r.normal()).collect();
    let y: Vec<f64> = x.iter().map(|v| v * 0.6 + r.normal() * 0.5).collect();
    let c: Vec<f64> = x.iter().zip(&y).map(|(a, b)| a.hypot(*b)).collect();
    ax.scatter(&x, &y)
        .values(c)
        .colorbar("distance")
        .opacity(0.85);
    ax.regression(&x, &y, 1);
    let ax = fig.at(0, 1);
    ax.title("Bubble chart");
    let gdp = [1.2, 3.5, 7.0, 15.0, 30.0, 45.0];
    let life = [62.0, 68.0, 72.0, 76.0, 80.0, 82.0];
    let pop = [80.0, 300.0, 50.0, 120.0, 30.0, 330.0];
    ax.scatter(gdp, life)
        .bubble(pop, 8.0, 60.0)
        .names(["A", "B", "C", "D", "E", "F"])
        .label("countries");
    ax.x_axis().log().label("GDP per capita (k$)");
    ax.y_label("Life expectancy");

    fig.save("scatter_bubble.png")?;
    fig.save("scatter_bubble.svg")?;
    Ok(())
}
