//! 3D charts: a lit surface, a trajectory through a 3D
//! scatter, and a translucent mesh around points. All 3D series of a plot
//! share one scene, so they hide each other correctly.
//!
//! `cargo run --release` writes `three_d.png` and `three_d.svg`; with
//! `--window` the figure opens in a window: drag to turn, wheel to zoom,
//! `X`/`Y`/`Z` (Shift = reverse) to rotate around an axis, `R` to reset.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1200, 480);
    fig.grid(1, 3);

    let ax = fig.at(0, 0);
    ax.title("Surface");
    ax.surface_fn(
        |x, y| (x.hypot(y)).sin() / x.hypot(y).max(1e-9),
        (-8.0, 8.0),
        (-8.0, 8.0),
        60,
    )
    .colormap(Colormap::Viridis);
    ax.x_label("x").y_label("y").z_label("z");

    let ax = fig.at(0, 1);
    ax.title("Lorenz attractor");
    let (mut x, mut y, mut z) = (vec![1.0], vec![1.0], vec![1.0]);
    for _ in 0..6000 {
        let (a, b, c) = (*x.last().unwrap(), *y.last().unwrap(), *z.last().unwrap());
        let dt = 0.006;
        x.push(a + dt * 10.0 * (b - a));
        y.push(b + dt * (a * (28.0 - c) - b));
        z.push(c + dt * (a * b - 8.0 / 3.0 * c));
    }
    let t: Vec<f64> = (0..x.len()).map(|i| i as f64).collect();
    ax.line3d(&x, &y, &z).values(&t).colormap(Colormap::Plasma).size(1.2);
    ax.x_label("x").y_label("y").z_label("z");

    let ax = fig.at(0, 2);
    ax.title("Points in an ellipsoid");
    let n = 300;
    let (mut px, mut py, mut pz) = (vec![], vec![], vec![]);
    for i in 0..n {
        let u = (i as f64 * 0.618_034).fract() * 2.0 - 1.0;
        let v = (i as f64 * 0.414_214).fract() * std::f64::consts::TAU;
        let r = (i as f64 / n as f64).cbrt();
        let s = (1.0 - u * u).sqrt();
        px.push(3.0 * r * s * v.cos());
        py.push(2.0 * r * s * v.sin());
        pz.push(1.5 * r * u);
    }
    ax.scatter3d(&px, &py, &pz).values(&pz).colormap(Colormap::Turbo).size(6.0);
    ax.ellipsoid([0.0, 0.0, 0.0], [3.2, 2.2, 1.7]).color("#5dade2").opacity(0.2);
    ax.equal_aspect(true).perspective(0.4);
    ax.x_label("x").y_label("y").z_label("z");

    fig.save("three_d.png")?;
    fig.save("three_d.svg")?;
    volume()?;
    if std::env::args().any(|a| a == "--window") {
        fig.show("dataviz 3D: drag = turn, wheel = zoom, X/Y/Z = rotate, R = reset")?;
    }
    Ok(())
}

/// Volume rendering of an electron-density-like field (two lobes and a
/// ring on a 96³ grid): translucent cloud and maximum intensity projection.
fn volume() -> Result<(), Error> {
    let n = 96;
    let mut v = Vec::with_capacity(n * n * n);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let c = |a: usize| a as f64 / (n - 1) as f64 * 2.0 - 1.0;
                let (x, y, z) = (c(i), c(j), c(k));
                let lobe = |dz: f64| (-((x * x + y * y) * 9.0 + (z - dz) * (z - dz) * 14.0)).exp();
                let ring = (-(((x * x + y * y).sqrt() - 0.62).powi(2) * 40.0 + z * z * 30.0)).exp() * 0.8;
                v.push(lobe(0.45) + lobe(-0.45) + ring);
            }
        }
    }
    let mut fig = Figure::new(1200, 560);
    fig.grid(1, 2);
    let ax = fig.at(0, 0);
    ax.title("Volume rendering");
    ax.volume(v.clone(), [n, n, n])
        .extent((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0))
        .opacity(0.35)
        .colorbar("density");
    let ax = fig.at(0, 1);
    ax.title("Maximum intensity projection");
    ax.volume(v, [n, n, n])
        .extent((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0))
        .mode(VolumeMode::MaxIntensity)
        .colormap(Colormap::Viridis);
    // Seen from lower down: the lobes along z come apart.
    ax.camera(30.0, 10.0);
    fig.save("volume.png")
}
