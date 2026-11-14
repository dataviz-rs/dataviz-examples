//! Distributions: strip and swarm plots, a ridgeline of monthly
//! temperatures and a Q-Q plot against the normal distribution.

use dataviz::prelude::*;

/// Small deterministic random generator (xorshift) for the demo data.
struct Rng(u64);
impl Rng {
    fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        let (a, b) = (self.uniform().max(1e-12), self.uniform());
        (-2.0 * a.ln()).sqrt() * (std::f64::consts::TAU * b).cos()
    }
}

fn main() -> Result<(), Error> {
    let mut r = Rng(7);
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let waits: Vec<(&str, Vec<f64>)> = days
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let v = (0..90)
                .map(|_| 20.0 + i as f64 * 2.5 + r.normal() * (3.0 + i as f64))
                .collect();
            (*d, v)
        })
        .collect();

    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let temps: Vec<(&str, Vec<f64>)> = months
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let mean = 12.0 - 13.0 * (i as f64 / 12.0 * std::f64::consts::TAU).cos();
            (*m, (0..2000).map(|_| mean + r.normal() * 4.0).collect())
        })
        .collect();
    let incomes: Vec<f64> = (0..400).map(|_| (10.0 + 0.6 * r.normal()).exp() / 1000.0).collect();

    let mut fig = Figure::new(1200, 860);
    fig.title("Distributions");
    fig.grid(2, 2);
    fig.at(0, 0)
        .title("Strip plot")
        .y_label("waiting time [min]")
        .strip(waits.clone());
    fig.at(0, 1)
        .title("Swarm plot")
        .y_label("waiting time [min]")
        .swarm(waits.iter().map(|(d, v)| (*d, v[..60].to_vec())));
    fig.at(1, 0)
        .title("Ridgeline: daily temperature by month")
        .x_label("temperature [°C]")
        .ridgeline(temps)
        .color(Color::rgb(66, 133, 180));
    fig.at(1, 1)
        .title("Q-Q plot: incomes are not normal")
        .qq(&incomes);

    fig.save("distributions.png")?;
    fig.save("distributions.svg")?;
    Ok(())
}
