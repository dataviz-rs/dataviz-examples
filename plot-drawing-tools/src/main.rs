//! Chart drawing tools of trading platforms: trend lines, rays, price
//! levels and Fibonacci retracements, in code or drawn with the mouse in a
//! window (`cargo run --release -- --show`).

use dataviz::prelude::*;

/// A day-by-day price path: a rally, a pull-back and a second leg.
fn prices(n: usize) -> (Vec<f64>, [Vec<f64>; 4]) {
    let (mut o, mut h, mut l, mut c) = (vec![], vec![], vec![], vec![]);
    let mut p = 100.0;
    let mut s = 7u64;
    let mut noise = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    for i in 0..n {
        let t = i as f64;
        let drift = (t * 0.06).sin() * 1.2 + 0.25;
        let open = p;
        p += drift + noise() * 2.2;
        o.push(open);
        c.push(p);
        h.push(open.max(p) + 0.4 + noise().abs() * 1.6);
        l.push(open.min(p) - 0.4 - noise().abs() * 1.6);
    }
    ((0..n).map(|i| i as f64).collect(), [o, h, l, c])
}

fn main() -> Result<(), Error> {
    let (x, [o, h, l, c]) = prices(160);
    let mut fig = Figure::new(1100, 600);
    let ax = fig.axes();
    ax.candlestick(x.clone(), o, h.clone(), l.clone(), c);
    ax.title("Trend lines, levels and a Fibonacci retracement");
    // Swings: the first low, the top of the rally, the pull-back low.
    let arg = |v: &[f64], a: usize, b: usize, max: bool| {
        (a..b)
            .max_by(|&i, &j| {
                let o = v[i].total_cmp(&v[j]);
                if max { o } else { o.reverse() }
            })
            .unwrap_or(a)
    };
    let low1 = arg(&l, 0, 30, false);
    let high1 = arg(&h, 30, 80, true);
    let low2 = arg(&l, 80, 125, false);
    let p = |i: usize, v: &[f64]| (i as f64, v[i]);
    // Rising support through the two lows, continued to the edge.
    ax.ray(p(low1, &l), p(low2, &l)).label("support line");
    ax.trendline(p(high1, &h), p(low2, &l)).dash(Dash::Dashed);
    ax.price_level(h[high1]).label("resistance");
    ax.fib_retracement(p(low1, &l), p(high1, &h));
    ax.legend(LegendPosition::UpperLeft);
    fig.save("drawing_tools.png")?;

    if std::env::args().any(|a| a == "--show") {
        // T: trend line (drag), L: price level (click), F: Fibonacci
        // (drag), Backspace: take back the last one.
        fig.show("Drawing tools — T, L, F, Backspace")?;
    }
    Ok(())
}
