#[path = "../data.rs"]
mod data;
use data::*;
use dataviz::prelude::*;

fn main() {
    let (scenario, n, fmt, out) = args();
    let path = format!("{out}.{fmt}");
    let backend = match std::env::var("BACKEND").as_deref() {
        Ok("gpu") => Backend::Gpu,
        Ok("par") => Backend::Cpu { threads: 0 },
        Ok("cpu") => Backend::Cpu { threads: 1 },
        _ => Backend::Auto,
    };
    match scenario.as_str() {
        "line" => {
            timed_with(|| line(n), |(x, y)| {
                let mut fig = Figure::new(W, H);
                fig.backend(backend);
                fig.axes().line(x, y).label("line");
                fig.axes().title("line").x_label("x").y_label("y");
                fig.save(&path).unwrap();
            });
        }
        "scatter" => {
            timed_with(|| scatter(n), |(x, y)| {
                let mut fig = Figure::new(W, H);
                fig.backend(backend);
                fig.axes().scatter(x, y).size(4.0);
                fig.axes().title("scatter");
                fig.save(&path).unwrap();
            });
        }
        "bar" => {
            let (c, v) = bars(n);
            timed(|| {
                let mut fig = Figure::new(W, H);
                fig.backend(backend);
                fig.axes().bar(&c, &v);
                fig.axes().title("bar");
                fig.save(&path).unwrap();
            });
        }
        "hist" => {
            timed_with(|| hist(n), |v| {
                let mut fig = Figure::new(W, H);
                fig.backend(backend);
                fig.axes().hist(v).bin_count(BINS);
                fig.axes().title("hist");
                fig.save(&path).unwrap();
            });
        }
        _ => println!("0"),
    }
}
