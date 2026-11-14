//! 2D histogram and hexbin: Density of 20 000 points as a 2D histogram and as hexagonal bins.

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
    let mut r = Rng(11);
    fig.grid(1, 2);
    let x: Vec<f64> = (0..20000).map(|_| r.normal()).collect();
    let y: Vec<f64> = x.iter().map(|v| v * 0.5 + r.normal() * 0.8).collect();
    let ax = fig.at(0, 0);
    ax.title("2D histogram");
    ax.hist2d(&x, &y, 40, 40).colormap(Colormap::Inferno);
    let ax = fig.at(0, 1);
    ax.title("Hexbin");
    ax.hexbin(&x, &y).gridsize(25).colormap(Colormap::Viridis);

    fig.save("hist2d_hexbin.png")?;
    fig.save("hist2d_hexbin.svg")?;
    Ok(())
}
