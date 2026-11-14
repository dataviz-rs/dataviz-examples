//! Animations: a travelling wave packet and its phase portrait recorded as a
//! GIF and as a lossless animated PNG with `Figure::record`.
//!
//! `animation.mp4` / `.webm` work the same way when `ffmpeg` is installed.

use dataviz::prelude::*;
use std::f64::consts::TAU;

const FRAMES: usize = 90;

fn frame(fig: &mut Figure, i: usize) {
    let t = i as f64 / FRAMES as f64;
    *fig = Figure::new(820, 520);
    fig.title(format!("Wave packet   t = {:.2} s", t * 3.0));
    fig.grid(1, 2);

    // A Gaussian wave packet moving to the right.
    let x: Vec<f64> = (0..=600).map(|k| k as f64 / 60.0).collect();
    let centre = 1.5 + 7.0 * t;
    let env = |v: f64| (-(v - centre).powi(2) / 1.2).exp();
    let y: Vec<f64> = x.iter().map(|&v| env(v) * (6.0 * v - 18.0 * t * TAU).cos()).collect();
    let ax = fig.at(0, 0);
    ax.title("Signal").x_label("x").y_label("amplitude");
    ax.fill_between(&x, x.iter().map(|&v| -env(v)), x.iter().map(|&v| env(v)))
        .color(Color::rgb(31, 119, 180))
        .opacity(0.15)
        .label("envelope");
    ax.line(&x, &y).color(Color::rgb(31, 119, 180)).label("signal");
    ax.x_limits(0.0, 10.0).y_limits(-1.3, 1.3);
    ax.legend(LegendPosition::LowerLeft);

    // Phase portrait of a damped oscillator, drawn up to time t.
    let n = 1 + (i + 1) * 400 / FRAMES;
    let s: Vec<f64> = (0..n).map(|k| k as f64 * 0.03).collect();
    let px: Vec<f64> = s.iter().map(|&u| (-0.08 * u).exp() * u.cos()).collect();
    let py: Vec<f64> = s.iter().map(|&u| (-0.08 * u).exp() * -u.sin()).collect();
    let ax = fig.at(0, 1);
    ax.title("Phase portrait").x_label("position").y_label("velocity");
    ax.line(&px, &py).color(Color::rgb(214, 39, 40));
    ax.scatter([px[n - 1]], [py[n - 1]]).color(Color::rgb(214, 39, 40)).size(9.0);
    ax.x_limits(-1.1, 1.1).y_limits(-1.1, 1.1);
    ax.equal_aspect(true);
}

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(820, 520);
    for path in ["animation.gif", "animation.png"] {
        let start = std::time::Instant::now();
        fig.record(path, 30.0, FRAMES, frame)?;
        let kb = std::fs::metadata(path).map(|m| m.len() / 1024).unwrap_or(0);
        println!("{path}: {FRAMES} frames, {kb} KB in {:.0?}", start.elapsed());
    }
    Ok(())
}
