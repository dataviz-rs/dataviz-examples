//! Charts for traders: candlesticks on a trading-day axis with the Ichimoku
//! cloud and VWAP, a volume profile sharing the price axis, stochastic, ATR,
//! order book depth, drawdown and Renko.

use dataviz::plot::series::trading::{FootprintBar, OptionLeg};
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
    orderflow();
}

/// Small deterministic random numbers for the order-flow demo.
struct Rnd(u64);
impl Rnd {
    fn u(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn n(&mut self) -> f64 {
        let (a, b) = (self.u().max(1e-12), self.u());
        (-2.0 * a.ln()).sqrt() * (std::f64::consts::TAU * b).cos()
    }
}

/// Order flow and options: liquidity heatmap, footprint, market profile,
/// option payoffs and an implied volatility surface.
fn orderflow() {
    let mut r = Rnd(99);
    let mut fig = Figure::new(1600, 960);
    fig.grid(2, 3);

    // Liquidity heatmap: 400 seconds, prices 99.00–101.00 in 0.02 steps.
    let (steps, levels) = (400, 100);
    let mut price = 100.0;
    let mut path = Vec::with_capacity(steps);
    let mut depth = Vec::with_capacity(steps);
    for t in 0..steps {
        price += 0.012 * r.n();
        price = price.clamp(99.2, 100.8);
        path.push(price);
        let row: Vec<f64> = (0..levels)
            .map(|k| {
                let p = 99.0 + k as f64 * 0.02;
                let dist = (p - price).abs();
                // Walls at round numbers, thin near the price, noise.
                let wall = if ((p * 4.0).round() - p * 4.0).abs() < 0.04 { 400.0 * (1.0 + (t as f64 / 90.0).sin().abs()) } else { 0.0 };
                (20.0 + 60.0 * dist + wall) * (0.6 + 0.8 * r.u()) * if dist < 0.03 { 0.2 } else { 1.0 }
            })
            .collect();
        depth.push(row);
    }
    let ax = fig.at(0, 0);
    ax.title("Liquidity heatmap with traded price");
    ax.liquidity_heatmap((0.0, steps as f64), (99.0, 101.0), &depth);
    let t: Vec<f64> = (0..steps).map(|i| i as f64).collect();
    ax.line(&t, &path).color("#7df9ff").width(1.6).label("last price");
    ax.x_label("seconds").y_label("price");

    // Footprint: 7 one-minute bars, tick 0.25.
    let mut bars = Vec::new();
    let mut p = 4500.0;
    for _ in 0..5 {
        let open = p;
        let close = (open + (r.n() * 1.2 / 0.25).round() * 0.25).max(4490.0);
        let high = open.max(close) + (r.u() * 4.0).round() * 0.25;
        let low = open.min(close) - (r.u() * 4.0).round() * 0.25;
        let n = ((high - low) / 0.25).round() as usize + 1;
        let up = close >= open;
        let levels = (0..n)
            .map(|k| {
                let lv = low + k as f64 * 0.25;
                let centre = 1.0 - ((lv - (open + close) / 2.0) / (high - low + 0.25)).abs() * 1.6;
                let base = 40.0 + 160.0 * centre.max(0.05) * r.u();
                let (bid, ask) = if up { (base * (0.5 + 0.6 * r.u()), base * (0.9 + 0.8 * r.u())) } else { (base * (0.9 + 0.8 * r.u()), base * (0.5 + 0.6 * r.u())) };
                (lv, bid.round(), ask.round())
            })
            .collect();
        bars.push(FootprintBar { open, high, low, close, levels });
        p = close;
    }
    let ax = fig.at(0, 1);
    ax.title("Footprint: bid × ask per price level");
    ax.x_label("minute");
    ax.footprint(bars, 0.25);
    ax.x_label("minute").y_label("price");

    // Market profile: 13 half-hour periods.
    let mut mid = 4500.0;
    let periods: Vec<(f64, f64)> = (0..13)
        .map(|_| {
            mid += r.n() * 3.0;
            let half = 2.0 + r.u() * 4.0;
            ((mid - half).round(), (mid + half).round())
        })
        .collect();
    let ax = fig.at(0, 2);
    ax.title("Market profile (TPO): POC and value area");
    ax.market_profile(&periods, 1.0);
    ax.y_label("price");

    let ax = fig.at(1, 0);
    ax.title("Iron condor: payoff at expiry and 30 days before");
    ax.option_payoff(
        &[
            OptionLeg::Put { strike: 90.0, premium: 0.8, qty: 1.0 },
            OptionLeg::Put { strike: 95.0, premium: 2.0, qty: -1.0 },
            OptionLeg::Call { strike: 105.0, premium: 2.1, qty: -1.0 },
            OptionLeg::Call { strike: 110.0, premium: 0.9, qty: 1.0 },
        ],
        (82.0, 118.0),
        30.0,
        0.22,
        0.03,
    );
    ax.legend(LegendPosition::LowerCenter);

    let ax = fig.at(1, 1);
    ax.title("Covered call: stock + short 105 call");
    ax.option_payoff(
        &[OptionLeg::Stock { price: 100.0, qty: 1.0 }, OptionLeg::Call { strike: 105.0, premium: 2.5, qty: -1.0 }],
        (80.0, 125.0),
        45.0,
        0.25,
        0.03,
    );
    ax.legend(LegendPosition::LowerRight);

    // Implied volatility: smile, skew and term structure.
    let strikes: Vec<f64> = (0..21).map(|i| 70.0 + i as f64 * 3.0).collect();
    let maturities = [0.08, 0.17, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0];
    let iv: Vec<Vec<f64>> = maturities
        .iter()
        .map(|t: &f64| {
            strikes
                .iter()
                .map(|k| {
                    let m = (k / 100.0f64).ln();
                    0.18 + 0.03 / t.sqrt() * 0.4 + (0.9 * m * m - 0.25 * m) / t.sqrt().max(0.3)
                })
                .collect()
        })
        .collect();
    let ax = fig.at(1, 2);
    ax.title("Implied volatility surface");
    ax.volatility_surface(&strikes, &maturities, &iv);
    ax.camera(-130.0, 25.0);

    fig.save("orderflow.png").unwrap();
}
