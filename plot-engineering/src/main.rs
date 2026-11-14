//! Charts for engineers: FFT spectrum, spectrogram, Nyquist plot, Smith
//! chart and a Bode plot of two transfer functions.

use dataviz::plot::series::signal::{freq_response, logspace};
use dataviz::prelude::*;
use std::f64::consts::PI;
fn main() {
    let fs = 8000.0;
    let n = 4 * 8000;
    let mut r = 12345u64;
    let mut rnd = move || {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        (r >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    let s: Vec<f64> = (0..n)
        .map(|i| {
            let t = i as f64 / fs;
            (2.0 * PI * (300.0 + 400.0 * t) * t).sin()
                + 0.5 * (2.0 * PI * 1500.0 * t).sin()
                + 0.3 * rnd()
        })
        .collect();
    let mut fig = Figure::new(1200, 820);
    fig.grid(2, 2);
    fig.at(0, 0)
        .title("Spectrum (FFT, Hann window)")
        .spectrum(&s[..8192], fs);
    fig.at(0, 1)
        .title("Spectrogram: chirp + 1.5 kHz tone")
        .spectrogram(&s, fs, 512, 64);
    let w = logspace(0.01, 100.0, 400);
    let h = freq_response(&[1.0], &[1.0, 0.4, 1.0], &w);
    fig.at(1, 0).title("Nyquist: 1/(s² + 0.4s + 1)").nyquist(&h);
    let z: Vec<(f64, f64)> = (0..100)
        .map(|i| {
            let f = i as f64 / 99.0;
            (0.3 + 1.5 * f, -1.5 + 3.0 * f)
        })
        .collect();
    fig.at(1, 1)
        .title("Smith chart")
        .smith(&z)
        .label("load sweep");
    fig.save("engineering.png").unwrap();
    let mut b = Figure::new(900, 600);
    b.title("Bode plot");
    b.bode("1/(s²+0.4s+1)", &w, &h);
    b.bode("1/(s+1)", &w, &freq_response(&[1.0], &[1.0, 1.0], &w));
    b.save("bode.png").unwrap();
}
