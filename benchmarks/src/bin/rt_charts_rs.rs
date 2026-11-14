#[path = "../rt.rs"]
mod rt;
use charts_rs::{svg_to_png, LineChart};

fn main() {
    let (n, frames) = rt::args();
    let mut s = rt::Stream::new(n);
    let (w, h) = rt::size();
    rt::run("charts-rs (PNG)", n, frames, || {
        s.step();
        let labels: Vec<String> = s.x.iter().map(|v| format!("{v:.2}")).collect();
        let ys: Vec<f32> = s.y.iter().map(|v| *v as f32).collect();
        let mut c = LineChart::new(vec![("signal", ys).into()], labels);
        c.width = w as f32;
        c.height = h as f32;
        c.title.text = "real time".into();
        let png = svg_to_png(&c.svg().unwrap()).unwrap();
        png[png.len() / 2]
    });
}
