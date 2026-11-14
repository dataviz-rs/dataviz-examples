//! Live dashboard for streaming data.
//!
//! A producer thread (simulated sensors, the system's real CPU and memory
//! load, or numbers piped into stdin) sends samples over a channel; the
//! window drains them every frame and redraws:
//!
//! - the last 30 seconds (`--window`) of every channel on a time axis,
//! - the distribution of the first channel in that window (histogram),
//! - the incoming sample rate.
//!
//! ```text
//! cargo run --release                     # simulated sensors (100 Hz)
//! cargo run --release -- cpu              # this machine's CPU / memory (Linux)
//! vmstat 1 | awk 'BEGIN{print "user system idle"} NR>2{print $13,$14,$15; fflush()}' \
//!     | cargo run --release -- stdin      # any numbers, one sample per line
//! cargo run --release -- sim --window 10                         # 10 s of history
//! cargo run --release -- sim --record streaming.gif --seconds 8   # GIF, no window
//! cargo run --release -- cpu --log cpu.csv          # also record every sample
//! ```

use dataviz::plot::data::CsvLog;
use dataviz::plot::series::line::Line;
use dataviz::plot::series::stats::Histogram;
use dataviz::prelude::*;
use std::io::BufRead;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Seconds of history shown by default (`--window`).
const WINDOW: f64 = 30.0;

/// Wall-clock time in seconds (the time axis shows it as UTC).
fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64())
}

/// What the producer threads send.
enum Msg {
    /// Channel names (sent first).
    Names(Vec<String>),
    /// One sample of every channel at time `t`.
    Sample { t: f64, values: Vec<f64> },
}

/// Simulated sensors at 100 Hz: temperature, humidity and a vibration
/// signal with occasional bursts.
fn simulated(tx: Sender<Msg>) {
    let names = ["temperature °C", "humidity %", "vibration"];
    let _ = tx.send(Msg::Names(names.iter().map(|s| s.to_string()).collect()));
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    let mut rnd = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    let (mut temp, mut hum) = (27.0, 52.0);
    let start = Instant::now();
    for i in 0u64.. {
        let t = start.elapsed().as_secs_f64();
        temp += rnd() * 0.08 + (27.0 + 4.0 * (t * 0.15).sin() - temp) * 0.01;
        hum += rnd() * 0.15 + (52.0 - hum) * 0.005;
        let burst = if (t * 0.4).sin() > 0.85 { 8.0 } else { 1.0 };
        let vib = 8.0 + rnd() * 4.0 * burst + (t * 40.0).sin() * burst;
        if tx.send(Msg::Sample { t: now(), values: vec![temp, hum, vib] }).is_err() {
            return; // the window was closed
        }
        // 100 samples per second, without drifting.
        let next = start + Duration::from_millis((i + 1) * 10);
        std::thread::sleep(next.saturating_duration_since(Instant::now()));
    }
}

/// The machine's CPU and memory use (Linux `/proc`), 10 samples per second.
fn system(tx: Sender<Msg>) {
    let cpu = || -> Option<(u64, u64)> {
        let stat = std::fs::read_to_string("/proc/stat").ok()?;
        let v: Vec<u64> = stat.lines().next()?.split_whitespace().skip(1).filter_map(|x| x.parse().ok()).collect();
        let idle = v.get(3)? + v.get(4).unwrap_or(&0);
        Some((v.iter().sum(), idle))
    };
    let memory = || -> Option<f64> {
        let info = std::fs::read_to_string("/proc/meminfo").ok()?;
        let kb = |key: &str| -> Option<f64> {
            info.lines().find(|l| l.starts_with(key))?.split_whitespace().nth(1)?.parse().ok()
        };
        Some(100.0 * (1.0 - kb("MemAvailable:")? / kb("MemTotal:")?))
    };
    let Some(mut last) = cpu() else {
        eprintln!("`cpu` reads /proc/stat (Linux); try `sim` or `stdin`");
        return;
    };
    let _ = tx.send(Msg::Names(vec!["CPU %".into(), "memory %".into()]));
    loop {
        std::thread::sleep(Duration::from_millis(100));
        let (Some(c), Some(mem)) = (cpu(), memory()) else { return };
        let (total, idle) = (c.0 - last.0, c.1 - last.1);
        last = c;
        let busy = if total > 0 { 100.0 * (1.0 - idle as f64 / total as f64) } else { 0.0 };
        if tx.send(Msg::Sample { t: now(), values: vec![busy, mem] }).is_err() {
            return;
        }
    }
}

/// Numbers from stdin, one sample per line (spaces, commas or tabs). A first
/// line that is not numeric names the channels.
fn stdin(tx: Sender<Msg>) {
    let mut named = false;
    for line in std::io::stdin().lock().lines() {
        let Ok(line) = line else { return };
        let fields: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
            .filter(|f| !f.is_empty())
            .collect();
        if fields.is_empty() {
            continue;
        }
        let values: Vec<f64> = fields.iter().filter_map(|f| f.parse().ok()).collect();
        if !named {
            named = true;
            let names = if values.len() == fields.len() {
                (1..=values.len()).map(|i| format!("channel {i}")).collect()
            } else {
                fields.iter().map(|f| f.to_string()).collect()
            };
            let _ = tx.send(Msg::Names(names));
            if values.len() != fields.len() {
                continue; // header line
            }
        }
        if values.len() == fields.len() && tx.send(Msg::Sample { t: now(), values }).is_err() {
            return;
        }
    }
}

/// The dashboard: owns the receiving end and updates the figure.
struct Dashboard {
    rx: Receiver<Msg>,
    names: Vec<String>,
    latest: Vec<f64>,
    /// Arrival times of the last second's samples (rate).
    arrivals: std::collections::VecDeque<f64>,
    last_rate: f64,
    /// Seconds of history shown.
    window: f64,
    /// `--log FILE`: the path, then the open log once the channels are known.
    log_path: Option<String>,
    log: Option<CsvLog>,
}

impl Dashboard {
    fn new(source: &str, window: f64) -> Self {
        let (tx, rx) = channel();
        match source {
            "cpu" => std::thread::spawn(move || system(tx)),
            "stdin" => std::thread::spawn(move || stdin(tx)),
            _ => std::thread::spawn(move || simulated(tx)),
        };
        Dashboard {
            rx,
            names: Vec::new(),
            latest: Vec::new(),
            arrivals: Default::default(),
            last_rate: 0.0,
            window,
            log_path: None,
            log: None,
        }
    }

    fn figure(source: &str, window: f64) -> Figure {
        let mut fig = Figure::new(960, 600);
        fig.grid(2, 2);
        fig.title(format!("Live data · source: {source}"));
        let top = fig.span(0, 0, 1, 2);
        top.x_axis().time();
        top.legend(LegendPosition::UpperLeft);
        if source == "cpu" {
            top.y_limits(0.0, 100.0); // percentages
        }
        fig.at(1, 0).title(format!("Distribution (last {window} s)"));
        let rate = fig.at(1, 1);
        rate.title("Incoming samples per second");
        rate.x_axis().time();
        rate.line(Vec::<f64>::new(), Vec::<f64>::new()).color(Color::rgb(90, 90, 90));
        fig
    }

    /// Takes the queued samples into the figure; `true` when it changed.
    fn update(&mut self, fig: &mut Figure) -> bool {
        let mut changed = false;
        for msg in self.rx.try_iter() {
            match msg {
                Msg::Names(names) => {
                    let top = fig.span(0, 0, 1, 2);
                    for name in &names {
                        top.line(Vec::<f64>::new(), Vec::<f64>::new()).label(name.clone());
                    }
                    let first = names.first().cloned().unwrap_or_default();
                    let dist = fig.at(1, 0);
                    dist.title(format!("{first}: distribution (last {} s)", self.window));
                    dist.hist(Vec::<f64>::new()).bin_count(30);
                    self.latest = vec![f64::NAN; names.len()];
                    if let Some(path) = &self.log_path {
                        let header: Vec<&str> =
                            std::iter::once("time").chain(names.iter().map(String::as_str)).collect();
                        match CsvLog::create(path, &header) {
                            Ok(log) => self.log = Some(log),
                            Err(e) => eprintln!("cannot log to {path}: {e}"),
                        }
                    }
                    self.names = names;
                }
                Msg::Sample { t, values } => {
                    let top = fig.span(0, 0, 1, 2);
                    for (i, v) in values.iter().enumerate().take(self.names.len()) {
                        if let Some(line) = top.series_mut::<Line>(i) {
                            line.push(t, *v);
                        }
                        self.latest[i] = *v;
                    }
                    self.arrivals.push_back(t);
                    if let Some(log) = &mut self.log {
                        let _ = log.record(t, &values);
                    }
                }
            }
            changed = true;
        }
        if !changed || self.names.is_empty() {
            return changed;
        }
        // Slide the windows.
        let t = now();
        while self.arrivals.front().is_some_and(|a| *a < t - 1.0) {
            self.arrivals.pop_front();
        }
        let top = fig.span(0, 0, 1, 2);
        for i in 0..self.names.len() {
            if let Some(line) = top.series_mut::<Line>(i) {
                line.keep_since(t - self.window);
            }
        }
        top.x_limits(t - self.window, t);
        let values: Vec<String> = self
            .names
            .iter()
            .zip(&self.latest)
            .map(|(n, v)| format!("{n} {v:.1}"))
            .collect();
        let rec = self.log.as_ref().map_or(String::new(), |l| format!("   ● REC {} rows", l.rows()));
        top.title(values.join("   ") + &rec);
        // Histogram of the first channel's window.
        let first: Vec<f64> = top.series_mut::<Line>(0).map_or(Vec::new(), |l| l.data().1.to_vec());
        if let Some(h) = fig.at(1, 0).series_mut::<Histogram>(0) {
            h.set_values(first);
        }
        // Sample rate, a point every quarter second.
        if t - self.last_rate >= 0.25 {
            self.last_rate = t;
            let rate = fig.at(1, 1);
            let mut top_rate = 1.0f64;
            if let Some(line) = rate.series_mut::<Line>(0) {
                line.push(t, self.arrivals.len() as f64).keep_since(t - self.window);
                top_rate = line.data().1.iter().fold(top_rate, |a, b| a.max(*b));
            }
            rate.x_limits(t - self.window, t);
            rate.y_limits(0.0, top_rate * 1.25);
        }
        true
    }
}

/// Renders `seconds` of the dashboard at `fps` into an animation (GIF, or
/// APNG / MP4 / WebM by extension; see `dataviz::plot::anim`) and the last
/// frame as PNG, without opening a window.
fn record(source: &str, window: f64, path: &str, seconds: f64, fps: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut dash = Dashboard::new(source, window);
    let mut fig = Dashboard::figure(source, window);
    fig.scale(0.75); // smaller GIF frames
    let mut rec = Recorder::create(path, fps as f64)?;
    let start = Instant::now();
    let period = Duration::from_secs_f64(1.0 / fps as f64);
    let mut next = start;
    while start.elapsed().as_secs_f64() < seconds {
        next += period;
        std::thread::sleep(next.saturating_duration_since(Instant::now()));
        dash.update(&mut fig);
        rec.frame(&mut fig)?;
    }
    let frames = rec.frames();
    rec.finish()?;
    fig.scale(1.0);
    fig.save("streaming.png")?;
    println!("wrote {path} ({frames} frames) and streaming.png");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let source = args.first().filter(|a| !a.starts_with("--")).map_or("sim", |s| s.as_str()).to_string();
    let flag = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let window = flag("--window").and_then(|s| s.parse().ok()).unwrap_or(WINDOW);
    let log = flag("--log");
    if let Some(path) = flag("--record") {
        let seconds = flag("--seconds").and_then(|s| s.parse().ok()).unwrap_or(8.0);
        return record(&source, window, &path, seconds, 10);
    }
    let mut dash = Dashboard::new(&source, window);
    dash.log_path = log;
    let mut fig = Dashboard::figure(&source, window);
    fig.show_live("dataviz live data", 30, |fig| dash.update(fig))?;
    Ok(())
}
