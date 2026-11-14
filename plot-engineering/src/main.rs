//! Charts for engineers: FFT spectrum, spectrogram, Nyquist plot, Smith
//! chart and a Bode plot of two transfer functions; streamlines, a
//! ternary diagram, a 3D waterfall, an SPC control chart, a Pareto chart, 3D
//! vectors and 3D bars (`science.png`).

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
    science();
}

fn science() {
    let mut fig = Figure::new(1600, 820);
    fig.grid(2, 4);
    // Flow around a cylinder (potential flow, radius 1).
    let flow = |x: f64, y: f64| -> (f64, f64) {
        let r2 = x * x + y * y;
        if r2 < 1.0 {
            return (f64::NAN, f64::NAN);
        }
        let r4 = r2 * r2;
        (1.0 - (x * x - y * y) / r4, -2.0 * x * y / r4)
    };
    let ax = fig.at(0, 0);
    ax.title("Streamlines: flow around a cylinder");
    ax.streamplot(flow, (-3.0, 3.0), (-2.0, 2.0), 28);
    ax.equal_aspect(true);
    // Field of two opposite charges.
    let dipole = |x: f64, y: f64| -> (f64, f64) {
        let f = |cx: f64, q: f64| {
            let (dx, dy) = (x - cx, y);
            let r3 = (dx * dx + dy * dy).powf(1.5).max(1e-3);
            (q * dx / r3, q * dy / r3)
        };
        let (a, b) = (f(-1.0, 1.0), f(1.0, -1.0));
        (a.0 + b.0, a.1 + b.1)
    };
    let ax = fig.at(0, 1);
    ax.title("Electric field of a dipole");
    ax.streamplot(dipole, (-3.0, 3.0), (-2.0, 2.0), 30).colormap(Colormap::Magma);
    ax.equal_aspect(true);

    // Soil samples: sand / silt / clay.
    let mut seed = 7u64;
    let mut u = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let (mut sa, mut si, mut cl, mut om) = (vec![], vec![], vec![], vec![]);
    for k in 0..160 {
        let c = [[70.0, 20.0, 10.0], [30.0, 50.0, 20.0], [20.0, 30.0, 50.0]][k % 3];
        sa.push(c[0] + 14.0 * (u() - 0.5));
        si.push(c[1] + 14.0 * (u() - 0.5));
        cl.push(c[2] + 14.0 * (u() - 0.5));
        om.push(1.0 + 4.0 * u() * (c[2] / 50.0));
    }
    fig.at(0, 2)
        .title("Ternary: soil texture")
        .ternary(&sa, &si, &cl)
        .names("sand %", "silt %", "clay %")
        .values(&om, Colormap::YlGnBu);

    // Vibration spectra over time (a resonance drifting up).
    let f: Vec<f64> = (0..160).map(|i| i as f64 * 2.5).collect();
    let t: Vec<f64> = (0..24).map(|i| i as f64 * 5.0).collect();
    let z: Vec<Vec<f64>> = t
        .iter()
        .map(|tt| {
            let peak = 120.0 + tt * 1.2;
            f.iter()
                .map(|ff| 1.0 / (1.0 + ((ff - peak) / 8.0).powi(2)) + 0.4 / (1.0 + ((ff - 2.0 * peak) / 10.0).powi(2)) + 0.03 * u())
                .collect()
        })
        .collect();
    let ax = fig.at(0, 3);
    ax.title("3D waterfall: vibration spectra");
    ax.waterfall3d(&f, &t, z);
    ax.x_label("Hz").y_label("s").z_label("amplitude");

    // Shaft diameter measurements, a shift after sample 30.
    let d: Vec<f64> = (0..50)
        .map(|i| 25.0 + 0.02 * (u() - 0.5) * 2.0 + if i >= 30 { 0.012 } else { 0.0 } + if i == 18 { 0.05 } else { 0.0 })
        .collect();
    let ax = fig.at(1, 0);
    ax.title("SPC control chart: shaft diameter (mm)");
    ax.control_chart(&d);
    ax.legend(LegendPosition::OutsideBottom).legend_columns(3);

    fig.at(1, 1).title("Pareto: defect causes").pareto(
        ["scratch", "dent", "misalignment", "crack", "discolor", "burr", "other"],
        [41.0, 28.0, 17.0, 9.0, 6.0, 4.0, 3.0],
    );

    // Magnetic field around a wire along z.
    let (mut p, mut v) = (vec![], vec![]);
    for i in -3..=3 {
        for j in -3..=3 {
            for k in 0..3 {
                let (x, y) = (i as f64, j as f64);
                let r2 = x * x + y * y;
                if r2 == 0.0 {
                    continue;
                }
                p.push([x, y, k as f64 * 2.0]);
                v.push([-y / r2, x / r2, 0.0]);
            }
        }
    }
    let ax = fig.at(1, 2);
    ax.title("3D vectors: field around a wire");
    ax.quiver3d(p, v);
    ax.x_label("x").y_label("y").z_label("z");

    let ax = fig.at(1, 3);
    ax.title("3D bars: energy use by month and site");
    ax.bar3d(
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun"],
        ["Plant A", "Plant B", "Plant C"],
        vec![
            vec![42.0, 39.0, 35.0, 28.0, 22.0, 18.0],
            vec![30.0, 31.0, 29.0, 27.0, 25.0, 24.0],
            vec![15.0, 17.0, 21.0, 26.0, 31.0, 36.0],
        ],
    );
    ax.z_label("MWh");
    fig.save("science.png").unwrap();
}
