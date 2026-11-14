//! Small multiples with `Figure::facet_wrap` and a scatter matrix with
//! `Figure::pair_plot`.

use dataviz::prelude::*;

struct Rng(u64);
impl Rng {
    fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn main() -> Result<(), Error> {
    let mut r = Rng(3);

    // Monthly temperatures of six cities, one plot each.
    // (city, (annual mean, seasonal amplitude))
    let cities = [
        ("Ankara", (12.0, 12.0)),
        ("Berlin", (10.0, 9.0)),
        ("Cairo", (22.0, 7.0)),
        ("Delhi", (25.0, 8.0)),
        ("Oslo", (6.0, 10.0)),
        ("Sydney", (18.0, -5.0)),
    ];
    let months: Vec<f64> = (1..=12).map(|m| m as f64).collect();
    let mut fig = Figure::new(1000, 620);
    fig.title("Monthly mean temperature [°C]");
    fig.facet_wrap(cities, 3, |ax, (mean, amp)| {
        let t: Vec<f64> = months
            .iter()
            .map(|m| mean - amp * ((m - 1.0) / 12.0 * std::f64::consts::TAU).cos())
            .collect();
        ax.line(&months, &t).marker(Marker::Circle);
        ax.hline(*mean).dash(Dash::Dashed).color(Color::rgb(150, 150, 150));
    });
    fig.save("facets.png")?;

    // Four measurements of 300 samples.
    let n = 300;
    let length: Vec<f64> = (0..n).map(|_| 4.0 + r.uniform() * 6.0).collect();
    let weight: Vec<f64> = length.iter().map(|l| l * l * 0.8 + r.uniform() * 12.0).collect();
    let phase: Vec<f64> = length.iter().map(|l| (l * 0.9).sin() * 2.0 + r.uniform()).collect();
    let noise: Vec<f64> = (0..n).map(|_| r.uniform() + r.uniform()).collect();
    let mut fig = Figure::new(820, 820);
    fig.pair_plot([("length", &length), ("weight", &weight), ("phase", &phase), ("noise", &noise)]);
    fig.save("pair_plot.png")?;
    Ok(())
}
