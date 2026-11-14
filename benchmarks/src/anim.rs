//! Animation benchmark harness: `<bin> <scenario> <out-file>`.
//! Scenarios: `wave` (whole plot changes each frame) and `stream` (a line
//! grows on fixed axes). 800×600, 120 frames at 30 fps, 1000 points.
#![allow(dead_code)]
pub const W: u32 = 800;
pub const H: u32 = 600;
pub const FRAMES: usize = 120;
pub const N: usize = 1000;

pub fn wave(frame: usize) -> (Vec<f64>, Vec<f64>) {
    let phase = frame as f64 / 30.0 * std::f64::consts::TAU;
    let x: Vec<f64> = (0..N).map(|i| i as f64 * 10.0 / N as f64).collect();
    let y = x.iter().map(|v| (v + phase).sin()).collect();
    (x, y)
}

pub fn stream(frame: usize) -> (Vec<f64>, Vec<f64>) {
    let n = (frame + 1) * N / FRAMES;
    let x: Vec<f64> = (0..n).map(|i| i as f64 * 10.0 / N as f64).collect();
    let y = x.iter().map(|v| (v * 3.0).sin() * (1.0 - v / 12.0)).collect();
    (x, y)
}

fn cpu_ms() -> f64 {
    let s = std::fs::read_to_string("/proc/self/stat").unwrap();
    let rest = &s[s.rfind(')').unwrap() + 2..];
    let f: Vec<&str> = rest.split_whitespace().collect();
    let tick = 100.0;
    (f[11].parse::<f64>().unwrap() + f[12].parse::<f64>().unwrap()) * 1000.0 / tick
}

fn rss_mb() -> f64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let l = s.lines().find(|l| l.starts_with("VmHWM")).unwrap();
    l.split_whitespace().nth(1).unwrap().parse::<f64>().unwrap() / 1024.0
}

pub fn run(lib: &str, f: impl FnOnce(&str, &str)) {
    let a: Vec<String> = std::env::args().collect();
    let (scenario, out) = (a[1].clone(), a[2].clone());
    let (t, c) = (std::time::Instant::now(), cpu_ms());
    f(&scenario, &out);
    let wall = t.elapsed().as_secs_f64() * 1000.0;
    let cpu = cpu_ms() - c;
    let size = std::fs::metadata(&out).map(|m| m.len()).unwrap_or(0);
    println!(
        "{{\"lib\":\"{lib}\",\"scenario\":\"{scenario}\",\"file\":\"{out}\",\"wall_ms\":{wall:.0},\"cpu_ms\":{cpu:.0},\"rss_mb\":{:.1},\"kb\":{}}}",
        rss_mb(),
        size / 1024
    );
}
