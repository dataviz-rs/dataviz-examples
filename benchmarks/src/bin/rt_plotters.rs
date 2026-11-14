#[path = "../rt.rs"]
mod rt;
use plotters::prelude::*;

fn main() {
    let (n, frames) = rt::args();
    let mut s = rt::Stream::new(n);
    let (w, h) = rt::size();
    let mut buf = vec![0u8; (w * h * 3) as usize];
    rt::run("plotters", n, frames, || {
        s.step();
        {
            let root = BitMapBackend::with_buffer(&mut buf, (w, h)).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let x0 = *s.x.front().unwrap();
            let x1 = *s.x.back().unwrap();
            let (y0, y1) = s.y.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), v| (a.min(*v), b.max(*v)));
            let mut c = ChartBuilder::on(&root)
                .caption("real time", ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d(x0..x1, y0..y1)
                .unwrap();
            c.configure_mesh().draw().unwrap();
            c.draw_series(LineSeries::new(s.x.iter().copied().zip(s.y.iter().copied()), &BLUE)).unwrap();
            root.present().unwrap();
        }
        buf[buf.len() / 2]
    });
}
