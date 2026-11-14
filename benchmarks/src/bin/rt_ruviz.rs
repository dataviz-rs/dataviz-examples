#[path = "../rt.rs"]
mod rt;
use ruviz::prelude::*;

fn main() {
    let (n, frames) = rt::args();
    let mut s = rt::Stream::new(n);
    let (w, h) = rt::size();
    rt::run("ruviz", n, frames, || {
        s.step();
        let (x, y) = s.vecs();
        let p: Plot = Plot::new().size_px(w, h).title("real time").line(&x, &y).into();
        let img = p.render().unwrap();
        img.pixels[img.pixels.len() / 2]
    });
}
