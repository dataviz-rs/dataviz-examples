//! Live Binance market dashboard (BTC/USDT by default).
//!
//! A background thread polls Binance's public market-data REST API about once
//! a second (through the system's `curl`, no API key needed) and the window
//! redraws five charts:
//!
//! - 1-minute candlesticks with MA(7) and MA(25),
//! - volume per minute (total and taker buys),
//! - every trade of the last two minutes (buys green, sells red),
//! - order-book depth (cumulative bids and asks),
//! - taker buy minus sell volume per 10 seconds.
//!
//! ```text
//! cargo run --release                          # BTCUSDT
//! cargo run --release -- ETHUSDT               # any spot symbol
//! cargo run --release -- BTCUSDT --png out.png --seconds 10   # no window
//! cargo run --release -- BTCUSDT --log binance-log   # also record to CSV
//! ```
//!
//! With `--log DIR` every trade (`trades.csv`), an order-book summary per
//! second (`book.csv`) and every closed 1-minute candle (`candles.csv`) are
//! appended to CSV files (`dataviz::plot::data::CsvLog`, times in UTC).

use dataviz::plot::data::CsvLog;
use dataviz::prelude::*;
use std::collections::{BTreeMap, VecDeque};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Public market-data endpoint (same data as api.binance.com).
const API: &str = "https://data-api.binance.vision/api/v3";

// ---------------------------------------------------------------- JSON ---

/// Minimal JSON value (enough for Binance's responses).
#[derive(Debug, Clone)]
enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn parse(s: &str) -> Option<Json> {
        let mut p = Parser { s: s.as_bytes(), i: 0 };
        p.value()
    }
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    /// Numbers, and numbers in strings (Binance sends prices as strings).
    fn num(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            Json::Str(s) => s.parse().ok(),
            _ => None,
        }
    }
    fn arr(&self) -> &[Json] {
        match self {
            Json::Arr(a) => a,
            _ => &[],
        }
    }
    fn bool(&self) -> bool {
        matches!(self, Json::Bool(true))
    }
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn value(&mut self) -> Option<Json> {
        self.ws();
        match *self.s.get(self.i)? {
            b'{' => {
                self.i += 1;
                let mut kv = Vec::new();
                loop {
                    self.ws();
                    if self.s.get(self.i) == Some(&b'}') {
                        self.i += 1;
                        return Some(Json::Obj(kv));
                    }
                    let Json::Str(k) = self.value()? else { return None };
                    self.ws();
                    self.i += 1; // ':'
                    kv.push((k, self.value()?));
                    self.ws();
                    if self.s.get(self.i) == Some(&b',') {
                        self.i += 1;
                    }
                }
            }
            b'[' => {
                self.i += 1;
                let mut a = Vec::new();
                loop {
                    self.ws();
                    if self.s.get(self.i) == Some(&b']') {
                        self.i += 1;
                        return Some(Json::Arr(a));
                    }
                    a.push(self.value()?);
                    self.ws();
                    if self.s.get(self.i) == Some(&b',') {
                        self.i += 1;
                    }
                }
            }
            b'"' => {
                self.i += 1;
                let start = self.i;
                while *self.s.get(self.i)? != b'"' {
                    self.i += if self.s[self.i] == b'\\' { 2 } else { 1 };
                }
                let v = String::from_utf8_lossy(&self.s[start..self.i]).into_owned();
                self.i += 1;
                Some(Json::Str(v))
            }
            b't' => {
                self.i += 4;
                Some(Json::Bool(true))
            }
            b'f' => {
                self.i += 5;
                Some(Json::Bool(false))
            }
            b'n' => {
                self.i += 4;
                Some(Json::Null)
            }
            _ => {
                let start = self.i;
                while self.i < self.s.len() && matches!(self.s[self.i], b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9') {
                    self.i += 1;
                }
                std::str::from_utf8(&self.s[start..self.i]).ok()?.parse().ok().map(Json::Num)
            }
        }
    }
}

/// GET `API/path` with curl (5 s timeout).
fn get(path: &str) -> Option<Json> {
    let out = Command::new("curl").args(["-s", "-m", "5", &format!("{API}/{path}")]).output().ok()?;
    Json::parse(std::str::from_utf8(&out.stdout).ok()?)
}

// -------------------------------------------------------------- market ---

#[derive(Clone, Copy)]
struct Candle {
    t: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    taker_buy: f64,
}

#[derive(Clone, Copy)]
struct Trade {
    t: f64,
    price: f64,
    qty: f64,
    buy: bool,
}

/// Latest market data, shared between the poller and the window.
#[derive(Clone, Default)]
struct Market {
    candles: Vec<Candle>,
    trades: VecDeque<Trade>,
    last_trade_id: u64,
    bids: Vec<(f64, f64)>,
    asks: Vec<(f64, f64)>,
    change_pct: f64,
    high_24h: f64,
    low_24h: f64,
    volume_24h: f64,
    /// Incremented on every update (the window redraws when it changes).
    version: u64,
    error: Option<String>,
    /// Rows written to the CSV logs (`--log`).
    logged: Option<u64>,
}

/// The CSV files of `--log DIR`.
struct Logs {
    trades: CsvLog,
    book: CsvLog,
    candles: CsvLog,
    /// Open time (UTC) of the last candle in `candles.csv`.
    last_candle: f64,
}

impl Logs {
    fn open(dir: &str) -> std::io::Result<Logs> {
        let p = |f: &str| std::path::Path::new(dir).join(f);
        Ok(Logs {
            trades: CsvLog::create(p("trades.csv"), &["time", "price", "qty", "side"])?,
            book: CsvLog::create(
                p("book.csv"),
                &["time", "best_bid", "best_ask", "spread", "bid_depth", "ask_depth"],
            )?,
            candles: CsvLog::create(
                p("candles.csv"),
                &["time", "open", "high", "low", "close", "volume", "taker_buy"],
            )?,
            // Continue an existing log without repeating candles.
            last_candle: dataviz::plot::data::Table::read(p("candles.csv"))
                .ok()
                .and_then(|t| t.time_column("time"))
                .and_then(|t| t.into_iter().filter(|v| v.is_finite()).reduce(f64::max))
                .unwrap_or(f64::NEG_INFINITY),
        })
    }
    /// Writes buffered rows (once per poll, so closing the window loses
    /// nothing).
    fn flush(&mut self) {
        let _ = self.trades.flush();
        let _ = self.book.flush();
        let _ = self.candles.flush();
    }
    fn rows(&self) -> u64 {
        self.trades.rows() + self.book.rows() + self.candles.rows()
    }
}

/// Seconds to add to UTC timestamps to show local time (`date +%z`).
fn local_offset() -> f64 {
    let out = Command::new("date").arg("+%z").output().ok();
    let z = out.and_then(|o| String::from_utf8(o.stdout).ok()).unwrap_or_default();
    let z = z.trim();
    if z.len() != 5 {
        return 0.0;
    }
    let sign = if z.starts_with('-') { -1.0 } else { 1.0 };
    let (h, m): (f64, f64) = (z[1..3].parse().unwrap_or(0.0), z[3..5].parse().unwrap_or(0.0));
    sign * (h * 3600.0 + m * 60.0)
}

/// Current UTC time in seconds.
fn now_utc() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

/// Polls Binance forever, updating `market`.
fn poll(symbol: String, market: Arc<Mutex<Market>>, mut logs: Option<Logs>) {
    let tz = local_offset();
    let mut tick = 0u64;
    loop {
        let started = Instant::now();
        let trades = get(&format!("trades?symbol={symbol}&limit=1000"));
        let depth = get(&format!("depth?symbol={symbol}&limit=500"));
        let candles = (tick % 2 == 0).then(|| get(&format!("klines?symbol={symbol}&interval=1m&limit=90"))).flatten();
        let ticker = (tick % 5 == 0).then(|| get(&format!("ticker/24hr?symbol={symbol}"))).flatten();
        {
            let mut m = market.lock().unwrap_or_else(|e| e.into_inner());
            match &trades {
                Some(t) if !t.arr().is_empty() => {
                    m.error = None;
                    for tr in t.arr() {
                        let id = tr.get("id").and_then(Json::num).unwrap_or(0.0) as u64;
                        if id <= m.last_trade_id {
                            continue;
                        }
                        m.last_trade_id = id;
                        let (Some(price), Some(qty), Some(time)) = (
                            tr.get("price").and_then(Json::num),
                            tr.get("qty").and_then(Json::num),
                            tr.get("time").and_then(Json::num),
                        ) else {
                            continue;
                        };
                        // isBuyerMaker = the seller was the taker.
                        let buy = !tr.get("isBuyerMaker").is_some_and(Json::bool);
                        m.trades.push_back(Trade { t: time / 1000.0 + tz, price, qty, buy });
                        if let Some(l) = &mut logs {
                            let side = if buy { "buy" } else { "sell" };
                            let _ = l.trades.row([
                                dataviz::plot::data::format_timestamp(time / 1000.0),
                                price.to_string(),
                                qty.to_string(),
                                side.to_string(),
                            ]);
                        }
                    }
                    let now = m.trades.back().map_or(0.0, |t| t.t);
                    while m.trades.front().is_some_and(|t| t.t < now - 120.0) {
                        m.trades.pop_front();
                    }
                }
                Some(Json::Obj(o)) => m.error = Some(format!("Binance: {:?}", o)),
                _ => m.error = Some("no answer from Binance".into()),
            }
            if let Some(d) = &depth {
                let side = |k: &str| -> Vec<(f64, f64)> {
                    d.get(k)
                        .map(|a| a.arr().iter().filter_map(|p| Some((p.arr().first()?.num()?, p.arr().get(1)?.num()?))).collect())
                        .unwrap_or_default()
                };
                m.bids = side("bids");
                m.asks = side("asks");
                if let (Some(l), Some(b), Some(a)) = (&mut logs, m.bids.first(), m.asks.first()) {
                    // Rounded to 8 decimals (Binance's precision).
                    let r8 = |v: f64| (v * 1e8).round() / 1e8;
                    let depth = |s: &[(f64, f64)]| r8(s.iter().map(|p| p.1).sum::<f64>());
                    let row = [b.0, a.0, r8(a.0 - b.0), depth(&m.bids), depth(&m.asks)];
                    let _ = l.book.record(now_utc(), &row);
                }
            }
            if let Some(k) = &candles {
                let c: Vec<Candle> = k
                    .arr()
                    .iter()
                    .filter_map(|r| {
                        let f = |i: usize| r.arr().get(i).and_then(Json::num);
                        Some(Candle {
                            t: f(0)? / 1000.0 + tz,
                            open: f(1)?,
                            high: f(2)?,
                            low: f(3)?,
                            close: f(4)?,
                            volume: f(5)?,
                            taker_buy: f(9)?,
                        })
                    })
                    .collect();
                if let Some(l) = &mut logs {
                    // Every candle but the last (still open), once.
                    for k in c.iter().take(c.len().saturating_sub(1)) {
                        if k.t - tz > l.last_candle {
                            l.last_candle = k.t - tz;
                            let _ = l.candles.record(
                                k.t - tz,
                                &[k.open, k.high, k.low, k.close, k.volume, k.taker_buy],
                            );
                        }
                    }
                }
                if !c.is_empty() {
                    m.candles = c;
                }
            }
            // The current minute follows the trades between kline polls.
            if let (Some(last), Some(tr)) = (m.candles.last().copied(), m.trades.back().copied()) {
                if tr.t >= last.t && tr.t < last.t + 60.0 {
                    let n = m.candles.len() - 1;
                    let c = &mut m.candles[n];
                    c.close = tr.price;
                    c.high = c.high.max(tr.price);
                    c.low = c.low.min(tr.price);
                }
            }
            if let Some(t) = &ticker {
                let f = |k: &str| t.get(k).and_then(Json::num).unwrap_or(f64::NAN);
                m.change_pct = f("priceChangePercent");
                m.high_24h = f("highPrice");
                m.low_24h = f("lowPrice");
                m.volume_24h = f("volume");
            }
            if let Some(l) = &mut logs {
                l.flush();
            }
            m.logged = logs.as_ref().map(Logs::rows);
            m.version += 1;
        }
        tick += 1;
        std::thread::sleep(Duration::from_secs(1).saturating_sub(started.elapsed()));
    }
}

// ----------------------------------------------------------- dashboard ---

const GREEN: Color = Color::rgb(38, 166, 91);
const RED: Color = Color::rgb(220, 53, 69);

/// Builds the whole dashboard from a snapshot of the market.
fn dashboard(symbol: &str, m: &Market) -> Figure {
    let mut fig = Figure::new(1400, 860);
    fig.theme(Theme::dark());
    fig.grid(3, 2);
    let price = m.trades.back().map(|t| t.price).or(m.candles.last().map(|c| c.close)).unwrap_or(f64::NAN);
    let sign = if m.change_pct >= 0.0 { "+" } else { "" };
    let rec = m.logged.map_or(String::new(), |n| format!("   ● REC {n} rows"));
    fig.title(match &m.error {
        Some(e) => format!("{symbol}  ·  {e}{rec}"),
        None => format!(
            "{symbol}  {price:.2}   24h {sign}{:.2} %   high {:.2}   low {:.2}   volume {:.0}{rec}",
            m.change_pct, m.high_24h, m.low_24h, m.volume_24h
        ),
    });

    // 1-minute candlesticks with moving averages.
    let ax = fig.span(0, 0, 2, 1);
    ax.title("1-minute candles");
    if !m.candles.is_empty() {
        let col = |f: fn(&Candle) -> f64| m.candles.iter().map(f).collect::<Vec<f64>>();
        let t = col(|c| c.t + 30.0);
        let close = col(|c| c.close);
        ax.candlestick(&t, col(|c| c.open), col(|c| c.high), col(|c| c.low), &close)
            .colors(GREEN, RED)
            .width(45.0);
        ax.moving_average(&t, &close, 7).color(Color::rgb(240, 185, 11));
        ax.moving_average(&t, &close, 25).color(Color::rgb(160, 120, 240));
        ax.hline(price).color(Color::rgb(200, 200, 200));
        ax.legend(LegendPosition::UpperLeft);
    }
    ax.x_axis().time();
    ax.y_label("USDT");

    // Volume per minute.
    let ax = fig.at(2, 0);
    ax.title("Volume per minute (BTC): total and taker buys");
    if !m.candles.is_empty() {
        let t: Vec<f64> = m.candles.iter().map(|c| c.t + 30.0).collect();
        ax.bar_xy(&t, m.candles.iter().map(|c| c.volume).collect::<Vec<_>>())
            .width(45.0)
            .color(Color::rgb(90, 110, 140))
            .label("total");
        ax.bar_xy(&t, m.candles.iter().map(|c| c.taker_buy).collect::<Vec<_>>())
            .width(45.0)
            .color(GREEN.with_alpha(0.8))
            .label("taker buys");
        ax.legend(LegendPosition::UpperLeft);
    }
    ax.x_axis().time();

    // Every trade of the last two minutes.
    let ax = fig.at(0, 1);
    ax.title("Trades, last 2 minutes (green buy, red sell; size ~ amount)");
    if !m.trades.is_empty() {
        let (t, p): (Vec<f64>, Vec<f64>) = m.trades.iter().map(|tr| (tr.t, tr.price)).unzip();
        ax.line(&t, &p).color(Color::rgb(170, 170, 170)).width(1.0);
        for (buy, color) in [(true, GREEN), (false, RED)] {
            let sel: Vec<&Trade> = m.trades.iter().filter(|tr| tr.buy == buy).collect();
            ax.scatter(sel.iter().map(|tr| tr.t), sel.iter().map(|tr| tr.price))
                .sizes(sel.iter().map(|tr| (2.0 + tr.qty.sqrt() * 12.0).min(18.0)))
                .color(color.with_alpha(0.7));
        }
    }
    ax.x_axis().time();

    // Order-book depth.
    let ax = fig.at(1, 1);
    ax.title("Order book depth (cumulative BTC)");
    let cum = |side: &[(f64, f64)]| {
        let mut acc = 0.0;
        side.iter().map(|&(p, q)| {
            acc += q;
            (p, acc)
        }).collect::<Vec<_>>()
    };
    let (bids, asks) = (cum(&m.bids), cum(&m.asks));
    if !bids.is_empty() && !asks.is_empty() {
        let mut b = bids.clone();
        b.reverse(); // increasing price
        ax.line(b.iter().map(|p| p.0), b.iter().map(|p| p.1))
            .step(Step::Pre)
            .fill_to(0.0)
            .color(GREEN)
            .label("bids");
        ax.line(asks.iter().map(|p| p.0), asks.iter().map(|p| p.1))
            .step(Step::Post)
            .fill_to(0.0)
            .color(RED)
            .label("asks");
        ax.legend(LegendPosition::UpperRight);
        ax.x_label("price (USDT)");
    }

    // Taker buy minus sell volume per 10 s.
    let ax = fig.at(2, 1);
    ax.title("Buy − sell pressure per 10 s (BTC)");
    let mut flow: BTreeMap<i64, f64> = BTreeMap::new();
    for tr in &m.trades {
        *flow.entry((tr.t / 10.0).floor() as i64).or_default() += if tr.buy { tr.qty } else { -tr.qty };
    }
    let (pos, neg): (Vec<_>, Vec<_>) = flow.iter().map(|(k, v)| (*k as f64 * 10.0 + 5.0, *v)).partition(|(_, v)| *v >= 0.0);
    if !pos.is_empty() {
        ax.bar_xy(pos.iter().map(|p| p.0), pos.iter().map(|p| p.1)).width(8.0).color(GREEN);
    }
    if !neg.is_empty() {
        ax.bar_xy(neg.iter().map(|p| p.0), neg.iter().map(|p| p.1)).width(8.0).color(RED);
    }
    ax.hline(0.0).color(Color::rgb(150, 150, 150));
    ax.x_axis().time();
    fig
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let symbol = args.first().filter(|a| !a.starts_with("--")).map_or("BTCUSDT", |s| s.as_str()).to_uppercase();
    let flag = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();

    let logs = match flag("--log") {
        Some(dir) => {
            let l = Logs::open(&dir)?;
            println!("logging to {dir}/trades.csv, book.csv and candles.csv");
            Some(l)
        }
        None => None,
    };
    let market = Arc::new(Mutex::new(Market::default()));
    {
        let (symbol, market) = (symbol.clone(), market.clone());
        std::thread::spawn(move || poll(symbol, market, logs));
    }
    let snapshot = || market.lock().unwrap_or_else(|e| e.into_inner()).clone();

    // Without a window: wait, then save the dashboard.
    if let Some(path) = flag("--png") {
        let secs: f64 = flag("--seconds").and_then(|s| s.parse().ok()).unwrap_or(10.0);
        std::thread::sleep(Duration::from_secs_f64(secs));
        dashboard(&symbol, &snapshot()).save(&path)?;
        println!("wrote {path}");
        return Ok(());
    }

    let mut seen = 0;
    let mut fig = dashboard(&symbol, &Market::default());
    fig.show_live(&format!("{symbol} live · Binance"), 10, |fig| {
        let m = snapshot();
        if m.version == seen {
            return false;
        }
        seen = m.version;
        *fig = dashboard(&symbol, &m);
        true
    })?;
    Ok(())
}
