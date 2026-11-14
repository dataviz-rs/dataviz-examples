#[path = "../data.rs"]
mod data;
use data::*;
use kuva::prelude::*;

fn finish(plots: Vec<Plot>, title: &str, fmt: &str, path: &str) {
    let layout = Layout::auto_from_plots(&plots).with_title(title).with_width(W as f64).with_height(H as f64);
    if fmt == "svg" {
        std::fs::write(path, kuva::render_to_svg(plots, layout)).unwrap();
    } else {
        std::fs::write(path, kuva::render_to_raster(plots, layout, 1.0).unwrap()).unwrap();
    }
}

fn main() {
    let (scenario, n, fmt, out) = args();
    let path = format!("{out}.{fmt}");
    match scenario.as_str() {
        "line" => {
            let (x, y) = line(n);
            timed(|| {
                let p = LinePlot::new().with_data(x.iter().copied().zip(y.iter().copied()));
                finish(vec![p.into()], "line", &fmt, &path);
            });
        }
        "scatter" => {
            let (x, y) = scatter(n);
            timed(|| {
                let p = ScatterPlot::new().with_data(x.iter().copied().zip(y.iter().copied())).with_size(2.0);
                finish(vec![p.into()], "scatter", &fmt, &path);
            });
        }
        "bar" => {
            let (c, v) = bars(n);
            timed(|| {
                let p = BarPlot::new().with_bars(c.iter().cloned().zip(v.iter().copied()));
                finish(vec![p.into()], "bar", &fmt, &path);
            });
        }
        "hist" => {
            let v = hist(n);
            timed(|| {
                let (lo, hi) = v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
                let p = Histogram::new().with_data(v.iter().copied()).with_bins(BINS).with_range((lo, hi));
                finish(vec![p.into()], "hist", &fmt, &path);
            });
        }
        _ => println!("0"),
    }
}
