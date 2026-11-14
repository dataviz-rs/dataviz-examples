//! Real-time benchmark harness: a rolling window of `n` points gets one new
//! point per frame and the chart is re-rendered to pixels.
#![allow(dead_code)]

pub struct Stream {
    pub x: std::collections::VecDeque<f64>,
    pub y: std::collections::VecDeque<f64>,
    t: u64,
    n: usize,
}

impl Stream {
    pub fn new(n: usize) -> Self {
        let mut s = Self { x: Default::default(), y: Default::default(), t: 0, n };
        for _ in 0..n {
            s.step();
        }
        s
    }
    pub fn step(&mut self) -> (f64, f64) {
        self.t += 1;
        let x = self.t as f64 * 0.01;
        let y = (x * 3.0).sin() * 5.0 + ((self.t * 2654435761) % 1000) as f64 / 1000.0;
        self.x.push_back(x);
        self.y.push_back(y);
        if self.x.len() > self.n {
            self.x.pop_front();
            self.y.pop_front();
        }
        (x, y)
    }
    pub fn vecs(&self) -> (Vec<f64>, Vec<f64>) {
        (self.x.iter().copied().collect(), self.y.iter().copied().collect())
    }
}

fn proc_cpu_ms() -> f64 {
    let s = std::fs::read_to_string("/proc/self/stat").unwrap();
    let rest = &s[s.rfind(')').unwrap() + 2..];
    let f: Vec<&str> = rest.split_whitespace().collect();
    // utime, stime are fields 14, 15 (1-based) -> index 11, 12 after the comm field.
    let ticks: f64 = f[11].parse::<f64>().unwrap() + f[12].parse::<f64>().unwrap();
    ticks * 10.0 // CLK_TCK = 100
}

fn status_kb(key: &str) -> f64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    s.lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0)
}

/// Runs `frames` frames (after `warmup`), each calling `frame` and returning a
/// checksum byte so the work cannot be optimized away. Prints a JSON line.
pub fn run(lib: &str, n: usize, frames: usize, mut frame: impl FnMut() -> u8) {
    let mut sink = 0u8;
    for _ in 0..5 {
        sink ^= frame();
    }
    let rss_start = status_kb("VmRSS:");
    let cpu0 = proc_cpu_ms();
    let wall0 = std::time::Instant::now();
    let mut lat = Vec::with_capacity(frames);
    // RT_SECONDS: run at least that long (and at least `frames` frames),
    // so that external samplers (GPU usage) see the steady state.
    let secs: f64 = std::env::var("RT_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    // RT_FPS: pace frames like a display would (sleep until the next frame).
    let fps: f64 = std::env::var("RT_FPS").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    while lat.len() < frames || wall0.elapsed().as_secs_f64() < secs {
        let t = std::time::Instant::now();
        sink ^= frame();
        let dt = t.elapsed();
        lat.push(dt.as_secs_f64() * 1000.0);
        if fps > 0.0 {
            let period = std::time::Duration::from_secs_f64(1.0 / fps);
            if let Some(rest) = period.checked_sub(dt) {
                std::thread::sleep(rest);
            }
        }
    }
    let frames = lat.len();
    let wall = wall0.elapsed().as_secs_f64() * 1000.0;
    let cpu = proc_cpu_ms() - cpu0;
    let rss_end = status_kb("VmRSS:");
    let peak = status_kb("VmHWM:");
    lat.sort_by(f64::total_cmp);
    let pct = |p: f64| lat[((lat.len() - 1) as f64 * p).round() as usize];
    let mean = lat.iter().sum::<f64>() / lat.len() as f64;
    println!(
        "{{\"lib\":\"{lib}\",\"n\":{n},\"frames\":{frames},\"p50\":{:.3},\"p95\":{:.3},\"max\":{:.3},\"mean\":{mean:.3},\"cpu_ms_per_frame\":{:.3},\"cpu_util\":{:.3},\"rss_peak_mb\":{:.1},\"rss_growth_mb\":{:.2},\"sink\":{sink}}}",
        pct(0.5),
        pct(0.95),
        lat[lat.len() - 1],
        cpu / frames as f64,
        cpu / wall,
        peak / 1024.0,
        (rss_end - rss_start) / 1024.0
    );
}

/// Image size from SIZE=WxH (default 800x600).
pub fn size() -> (u32, u32) {
    std::env::var("SIZE")
        .ok()
        .and_then(|v| v.split_once('x').and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?))))
        .unwrap_or((800, 600))
}

pub fn args() -> (usize, usize) {
    let a: Vec<String> = std::env::args().skip(1).collect();
    (a[0].parse().unwrap(), a.get(1).map_or(300, |v| v.parse().unwrap()))
}
