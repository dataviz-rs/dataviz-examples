//! Interactive HTML: one self-contained file with hover readouts, a
//! crosshair, legend toggling and a PNG download, viewable in any browser
//! without a server or internet access.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let start = date(2025, 1, 1);
    let t: Vec<f64> = (0..24 * 14).map(|h| start + h as f64 * 3600.0).collect();
    let day = |h: usize| (h as f64 / 24.0 * std::f64::consts::TAU).sin();
    let temp: Vec<f64> = (0..t.len()).map(|h| 12.0 + 6.0 * day(h) + (h as f64 / 60.0).sin() * 3.0).collect();
    let hum: Vec<f64> = (0..t.len()).map(|h| 65.0 - 15.0 * day(h) + (h as f64 / 45.0).cos() * 5.0).collect();
    let rain: Vec<f64> = (0..14).map(|d| ((d * 7 + 3) % 11) as f64 * 0.8).collect();
    let days: Vec<f64> = (0..14).map(|d| start + d as f64 * 86400.0 + 43200.0).collect();

    let mut fig = Figure::new(1000, 640);
    fig.title("Two weeks of weather (hover me)");
    fig.grid(3, 1).share_x(true);
    let ax = fig.span(0, 0, 2, 1);
    ax.line(&t, &temp).label("temperature [°C]");
    ax.line(&t, &hum).label("humidity [%]");
    ax.x_axis().time();
    ax.legend(LegendPosition::UpperRight);
    let ax = fig.at(2, 0);
    ax.bar_xy(&days, &rain).width(60_000.0).label("rain [mm]");
    ax.x_axis().time();
    fig.save("weather.html")?;
    fig.save("weather.png")?;
    println!("open weather.html in a browser");
    Ok(())
}
