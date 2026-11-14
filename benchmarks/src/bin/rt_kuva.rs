#[path = "../rt.rs"]
mod rt;
use kuva::prelude::*;

fn main() {
    let (n, frames) = rt::args();
    let mut s = rt::Stream::new(n);
    let (w, h) = rt::size();
    rt::run("kuva (PNG)", n, frames, || {
        s.step();
        let p = LinePlot::new().with_data(s.x.iter().copied().zip(s.y.iter().copied()));
        let plots: Vec<Plot> = vec![p.into()];
        let layout = Layout::auto_from_plots(&plots).with_title("real time").with_width(w as f64).with_height(h as f64);
        let png = kuva::render_to_raster(plots, layout, 1.0).unwrap();
        png[png.len() / 2]
    });
}
