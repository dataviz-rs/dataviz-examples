//! Candlestick chart: Daily candlesticks on a time axis with a 10-day moving average.

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
    let mut r = Rng(5);
    let ax = fig.axes();
    ax.title("Candlestick + moving average");
    let n = 60;
    let t: Vec<f64> = (0..n)
        .map(|i| date(2024, 1, 1) + i as f64 * 86400.0)
        .collect();
    let mut price = 100.0;
    let (mut o, mut h, mut l, mut c) = (vec![], vec![], vec![], vec![]);
    for _ in 0..n {
        let open = price;
        let close = open + r.normal() * 2.0;
        o.push(open);
        c.push(close);
        h.push(open.max(close) + r.uniform() * 2.0);
        l.push(open.min(close) - r.uniform() * 2.0);
        price = close;
    }
    ax.candlestick(&t, &o, &h, &l, &c).label("ACME");
    ax.moving_average(&t, &c, 10);
    ax.x_axis().time();
    ax.y_label("price ($)");

    fig.save("candlestick.png")?;
    fig.save("candlestick.svg")?;
    Ok(())
}
