//! Charts for traders: candlesticks on a trading-day axis with the Ichimoku
//! cloud and VWAP, a volume profile sharing the price axis, stochastic, ATR,
//! order book depth, drawdown and Renko.

use dataviz::prelude::*;
fn main() {
    let mut r = 77u64;
    // Small deterministic random generator for the demo data.
    let mut rnd = move || {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        (r >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    // Weekdays only (5 of 7 days) over ~7 months.
    let start = dataviz::plot::time::date(2025, 1, 6);
    let t: Vec<f64> = (0..200)
        .map(|d| start + ((d / 5) * 7 + d % 5) as f64 * 86400.0)
        .collect();
    let n = t.len();
    let (mut o, mut h, mut l, mut c, mut v) = (vec![], vec![], vec![], vec![], vec![]);
    let mut p = 100.0;
    for i in 0..n {
        let op = p;
        p *= 1.0 + rnd() * 0.035 + (i as f64 / 30.0).sin() * 0.004;
        o.push(op);
        c.push(p);
        h.push(op.max(p) * (1.0 + rnd().abs() * 0.015));
        l.push(op.min(p) * (1.0 - rnd().abs() * 0.015));
        v.push(1000.0 * (1.0 + rnd().abs() * 3.0));
    }
    let mut fig = Figure::new(1300, 900);
    fig.title("Trading toolkit");
    fig.grid(4, 5).share_y(false);
    let ax = fig.span(0, 0, 2, 4);
    let xi = ax.trading_axis(&t, "%d %b");
    ax.candlestick(&xi, &o, &h, &l, &c).label("price");
    ax.ichimoku(&xi, &h, &l, &c);
    ax.vwap(&xi, &h, &l, &c, &v);
    ax.legend(LegendPosition::UpperLeft).legend_columns(2);
    // The same price range for the chart and the profile next to it.
    let lo = l.iter().copied().fold(f64::INFINITY, f64::min);
    let hi = h.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let pad = (hi - lo) * 0.05;
    fig.span(0, 0, 2, 4).y_limits(lo - pad, hi + pad);
    let ax = fig.span(0, 4, 2, 1);
    ax.title("Volume profile");
    ax.volume_profile(&c, &v, 24);
    ax.y_limits(lo - pad, hi + pad);
    ax.y_axis().tick_labels(false);
    ax.legend(LegendPosition::LowerRight);
    let ax = fig.span(2, 0, 1, 2);
    ax.title("Stochastic (14, 3)");
    let xi2 = ax.trading_axis(&t, "%d %b");
    ax.stochastic(&xi2, &h, &l, &c, 14, 3);
    ax.legend(LegendPosition::Hidden);
    let ax = fig.span(2, 2, 1, 3);
    ax.title("ATR (14)");
    let xi3 = ax.trading_axis(&t, "%d %b");
    ax.atr(&xi3, &h, &l, &c, 14);
    ax.legend(LegendPosition::Hidden);
    let bids: Vec<(f64, f64)> = (0..40)
        .map(|i| (99.9 - i as f64 * 0.05, 1.0 + (rnd() + 0.5) * 4.0))
        .collect();
    let asks: Vec<(f64, f64)> = (0..40)
        .map(|i| (100.1 + i as f64 * 0.05, 1.0 + (rnd() + 0.5) * 4.0))
        .collect();
    let ax = fig.span(3, 0, 1, 2);
    ax.title("Order book depth");
    ax.depth_chart(&bids, &asks);
    ax.legend(LegendPosition::UpperCenter);
    let eq: Vec<f64> = {
        let mut e = 10000.0;
        c.iter()
            .map(|_| {
                e *= 1.0 + rnd() * 0.03 + 0.002;
                e
            })
            .collect()
    };
    let ax = fig.span(3, 2, 1, 2);
    ax.title("Drawdown");
    let xi4 = ax.trading_axis(&t, "%b");
    ax.drawdown(&xi4, &eq);
    let ax = fig.at(3, 4);
    ax.title("Renko");
    ax.renko(&c, 2.0);
    fig.save("trading.png").unwrap();
}
