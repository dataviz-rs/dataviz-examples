#[path = "../rt.rs"]
mod rt;
use dataviz::plot::series::line::Line;
use dataviz::prelude::*;

fn main() {
    let (n, frames) = rt::args();
    let mut s = rt::Stream::new(n);
    let (x, y) = s.vecs();
    let mut fig = Figure::new(800, 600);
    fig.axes().line(&x, &y).label("signal");
    fig.axes().title("real time").legend(LegendPosition::UpperLeft);
    let mut buf = Vec::new();
    rt::run("dataviz", n, frames, || {
        let (px, py) = s.step();
        let l = fig.axes().series_mut::<Line>(0).unwrap();
        l.push(px, py);
        l.keep_last(n);
        // Render straight into the window frame buffer (reused).
        fig.render_0rgb(&mut buf).unwrap();
        buf[buf.len() / 2] as u8
    });
}
