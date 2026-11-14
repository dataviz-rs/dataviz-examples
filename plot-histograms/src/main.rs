//! Histograms, KDE and ECDF: Overlapping density histograms with a KDE line; cumulative step histogram and ECDF.

use dataviz::plot::stats::Bins;
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
    let mut r = Rng(7);
    fig.grid(1, 2);
    let a: Vec<f64> = (0..3000).map(|_| r.normal()).collect();
    let b: Vec<f64> = (0..3000).map(|_| r.normal() * 0.7 + 1.5).collect();
    let ax = fig.at(0, 0);
    ax.title("Overlapping + KDE");
    ax.hist(&a).opacity(0.55).density(true).label("A");
    ax.hist(&b).opacity(0.55).density(true).label("B");
    ax.kde(&a, None).label("KDE A");
    let ax = fig.at(0, 1);
    ax.title("Cumulative step / ECDF");
    ax.hist(&a)
        .bins(Bins::Count(40))
        .cumulative(true)
        .density(true)
        .step(true)
        .label("cumulative hist");
    ax.ecdf(&b).label("ECDF B");
    ax.legend(LegendPosition::LowerRight);

    fig.save("histograms.png")?;
    fig.save("histograms.svg")?;
    Ok(())
}
