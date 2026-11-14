//! Real-time frame cost per dataviz backend: BACKEND=cpu|par|gpu,
//! SIZE=WxH, KIND=line|scatter.
#[path = "../rt.rs"]
mod rt;
use dataviz::plot::series::line::Line;
use dataviz::plot::series::scatter::Scatter;
use dataviz::prelude::*;

fn main() {
    let (n, frames) = rt::args();
    let env = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.into());
    let backend = env("BACKEND", "cpu");
    let size = env("SIZE", "800x600");
    let kind = env("KIND", "line");
    let (w, h) = size.split_once('x').map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap();
    let mut s = rt::Stream::new(n);
    let (x, y) = s.vecs();
    let mut fig = Figure::new(w, h);
    fig.backend(match backend.as_str() {
        "gpu" => Backend::Gpu,
        "par" => Backend::Cpu { threads: 0 },
        _ => Backend::Cpu { threads: 1 },
    });
    if kind == "scatter" {
        fig.axes().scatter(&x, &y).size(3.0).label("points");
    } else {
        fig.axes().line(&x, &y).label("signal");
    }
    fig.axes().title("real time").legend(LegendPosition::UpperLeft);
    let mut buf = Vec::new();
    let label = format!("{backend} {size} {kind}");
    rt::run(&label, n, frames, || {
        let (px, py) = s.step();
        if kind == "scatter" {
            let sc = fig.axes().series_mut::<Scatter>(0).unwrap();
            sc.push(px, py);
            sc.keep_last(n);
        } else {
            let l = fig.axes().series_mut::<Line>(0).unwrap();
            l.push(px, py);
            l.keep_last(n);
        }
        fig.render_0rgb(&mut buf).unwrap();
        buf[buf.len() / 2] as u8
    });
}
