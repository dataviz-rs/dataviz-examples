//! Technical indicators: candlesticks with Bollinger bands, RSI and MACD
//! in aligned panels sharing the time axis, and Heikin-Ashi candles.

use dataviz::prelude::*;

/// A random walk with some trend for the demo (deterministic).
fn prices(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut r = 99u64;
    let mut rnd = || {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        (r >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    let (mut o, mut h, mut l, mut c) = (vec![], vec![], vec![], vec![]);
    let mut p = 100.0;
    for i in 0..n {
        let open = p;
        p *= 1.0 + rnd() * 0.04 + (i as f64 / 25.0).sin() * 0.006;
        o.push(open);
        c.push(p);
        h.push(open.max(p) * (1.0 + rnd().abs() * 0.02));
        l.push(open.min(p) * (1.0 - rnd().abs() * 0.02));
    }
    (o, h, l, c)
}

fn main() -> Result<(), Error> {
    let n = 160;
    let start = date(2025, 1, 1);
    let t: Vec<f64> = (0..n).map(|i| start + i as f64 * 86400.0).collect();
    let (o, h, l, c) = prices(n);

    let mut fig = Figure::new(1100, 820);
    fig.title("Price with Bollinger bands, RSI and MACD");
    fig.grid(5, 1).share_x(true);
    let ax = fig.span(0, 0, 3, 1);
    ax.candlestick(&t, &o, &h, &l, &c).label("price");
    ax.bollinger(&t, &c, 20, 2.0);
    ax.x_axis().time();
    ax.legend(LegendPosition::UpperLeft);
    let ax = fig.at(3, 0);
    ax.rsi(&t, &c, 14);
    ax.x_axis().time();
    let ax = fig.at(4, 0);
    ax.macd(&t, &c, 12, 26, 9);
    ax.x_axis().time();
    ax.legend(LegendPosition::UpperLeft);
    fig.save("indicators.png")?;

    let mut fig = Figure::new(1000, 420);
    fig.title("Heikin-Ashi candles");
    fig.axes().heikin_ashi(&t, &o, &h, &l, &c);
    fig.axes().x_axis().time();
    fig.save("heikin_ashi.png")?;
    Ok(())
}
