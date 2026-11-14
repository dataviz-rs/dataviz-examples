//! Box and violin plots: Notched box plots with means and outliers next to violin plots.

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
    let mut r = Rng(3);
    fig.grid(1, 2);
    let groups = |r: &mut Rng| -> Vec<(String, Vec<f64>)> {
        (0..4)
            .map(|g| {
                (
                    format!("group {}", g + 1),
                    (0..200)
                        .map(|_| r.normal() * (1.0 + g as f64 * 0.3) + g as f64)
                        .chain([8.0 + g as f64])
                        .collect(),
                )
            })
            .collect()
    };
    let ax = fig.at(0, 0);
    ax.title("Box plot");
    ax.boxplot(groups(&mut r)).show_mean(true).notch(true);
    let ax = fig.at(0, 1);
    ax.title("Violin plot");
    ax.violin(groups(&mut r));

    fig.save("box_violin.png")?;
    fig.save("box_violin.svg")?;
    Ok(())
}
