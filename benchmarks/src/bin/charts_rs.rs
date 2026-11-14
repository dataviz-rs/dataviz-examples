#[path = "../data.rs"]
mod data;
use charts_rs::{svg_to_png, BarChart, HistogramChart, LineChart, ScatterChart};
use data::*;

fn finish(svg: String, fmt: &str, path: &str) {
    if fmt == "svg" {
        std::fs::write(path, svg).unwrap();
    } else {
        std::fs::write(path, svg_to_png(&svg).unwrap()).unwrap();
    }
}

fn main() {
    let (scenario, n, fmt, out) = args();
    let path = format!("{out}.{fmt}");
    match scenario.as_str() {
        "line" => {
            let (x, y) = line(n);
            timed(|| {
                // charts-rs line charts use one category label per point.
                let labels: Vec<String> = x.iter().map(|v| format!("{v:.1}")).collect();
                let ys: Vec<f32> = y.iter().map(|v| *v as f32).collect();
                let mut c = LineChart::new(vec![("line", ys).into()], labels);
                c.width = W as f32;
                c.height = H as f32;
                c.title.text = "line".into();
                finish(c.svg().unwrap(), &fmt, &path);
            });
        }
        "scatter" => {
            let (x, y) = scatter(n);
            timed(|| {
                let flat: Vec<f32> = x.iter().zip(&y).flat_map(|(a, b)| [*a as f32, *b as f32]).collect();
                let mut c = ScatterChart::new(vec![("scatter", flat).into()]);
                c.width = W as f32;
                c.height = H as f32;
                c.title.text = "scatter".into();
                finish(c.svg().unwrap(), &fmt, &path);
            });
        }
        "bar" => {
            let (cats, v) = bars(n);
            timed(|| {
                let vs: Vec<f32> = v.iter().map(|x| *x as f32).collect();
                let mut c = BarChart::new(vec![("bar", vs).into()], cats.clone());
                c.width = W as f32;
                c.height = H as f32;
                c.title.text = "bar".into();
                finish(c.svg().unwrap(), &fmt, &path);
            });
        }
        "hist" => {
            let v = hist(n);
            timed(|| {
                let vs: Vec<f32> = v.iter().map(|x| *x as f32).collect();
                let mut c = HistogramChart::new(vec![("hist", vs).into()]);
                c.width = W as f32;
                c.height = H as f32;
                c.title.text = "hist".into();
                c.bin_count = BINS;
                finish(c.svg().unwrap(), &fmt, &path);
            });
        }
        _ => println!("0"),
    }
}
