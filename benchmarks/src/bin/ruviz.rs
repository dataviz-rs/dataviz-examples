#[path = "../data.rs"]
mod data;
use data::*;
use ruviz::prelude::*;

fn main() {
    let (scenario, n, fmt, out) = args();
    let path = format!("{out}.{fmt}");
    let save = |p: Plot| {
        if fmt == "svg" {
            p.export_svg(&path).unwrap();
        } else {
            p.save(&path).unwrap();
        }
    };
    match scenario.as_str() {
        "line" => {
            let (x, y) = data::line(n);
            timed(|| save(Plot::new().size_px(W, H).title("line").xlabel("x").ylabel("y").line(&x, &y).into()));
        }
        "scatter" => {
            let (x, y) = data::scatter(n);
            timed(|| save(Plot::new().size_px(W, H).title("scatter").scatter(&x, &y).into()));
        }
        "bar" => {
            let (c, v) = bars(n);
            timed(|| save(Plot::new().size_px(W, H).title("bar").bar(&c, &v).into()));
        }
        "hist" => {
            let v = hist(n);
            timed(|| save(Plot::new().size_px(W, H).title("hist").histogram(&v).into()));
        }
        _ => println!("0"),
    }
}
