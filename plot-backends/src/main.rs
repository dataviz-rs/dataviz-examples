//! Raster backends: the same real-time chart drawn on one CPU thread, on all
//! cores and (with `--features gpu`) on the GPU, with the time and CPU time
//! per frame. Also writes `backends.png`.

use dataviz::plot::series::line::Line;
use dataviz::prelude::*;
use std::time::{Duration, Instant};

/// CPU time of this process on Linux (user + system), `None` elsewhere.
fn cpu_time() -> Option<Duration> {
    let stat = std::fs::read_to_string("/proc/self/stat").ok()?;
    let f: Vec<&str> = stat[stat.rfind(')')? + 2..].split_whitespace().collect();
    let ticks: u64 = f.get(11)?.parse::<u64>().ok()? + f.get(12)?.parse::<u64>().ok()?;
    Some(Duration::from_millis(ticks * 10))
}

fn signal(t: usize) -> f64 {
    (t as f64 * 0.00025).sin() * 5.0 + ((t * 7919) % 1000) as f64 / 1600.0
}

fn chart(n: usize, backend: Backend) -> Figure {
    let x: Vec<f64> = (0..n).map(|t| t as f64).collect();
    let y: Vec<f64> = (0..n).map(signal).collect();
    let mut fig = Figure::new(1920, 1080);
    fig.backend(backend);
    let ax = fig.axes();
    ax.line(x, y).label("signal");
    ax.title("Rolling window, 100 000 points").legend(LegendPosition::UpperLeft);
    fig
}

fn main() -> Result<(), Error> {
    let n = 100_000;
    chart(n, Backend::Auto).save("backends.png")?;

    let mut backends = vec![
        ("CPU, 1 thread", Backend::Cpu { threads: 1 }),
        ("CPU, all cores", Backend::Cpu { threads: 0 }),
    ];
    if Backend::gpu_available() {
        backends.push(("GPU", Backend::Gpu));
    } else {
        println!("(no GPU backend: build with `--features gpu` on a machine with a GPU)");
    }

    println!("1920×1080, {n} points, one new point and a full redraw per frame:");
    for (name, backend) in backends {
        let mut fig = chart(n, backend);
        let mut frame = Vec::new();
        fig.render_0rgb(&mut frame)?; // warm-up (caches, GPU start)
        let frames = std::env::args().nth(1).and_then(|v| v.parse().ok()).unwrap_or(100u32);
        let (cpu0, t0) = (cpu_time(), Instant::now());
        for t in n..n + frames as usize {
            let line = fig.axes().series_mut::<Line>(0).expect("line series");
            line.push(t as f64, signal(t));
            line.keep_last(n);
            fig.render_0rgb(&mut frame)?;
        }
        let wall = t0.elapsed() / frames;
        match cpu0.zip(cpu_time()).map(|(a, b)| (b - a) / frames) {
            Some(cpu) => println!("  {name:<15} {wall:>9.2?} per frame, {cpu:>9.2?} CPU"),
            None => println!("  {name:<15} {wall:>9.2?} per frame"),
        }
    }
    Ok(())
}
