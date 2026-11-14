#[path = "../anim.rs"]
mod anim;
use plotters::prelude::*;

fn main() {
    anim::run("plotters", |scenario, out| {
        let data = if scenario == "wave" { anim::wave } else { anim::stream };
        let root = BitMapBackend::gif(out, (anim::W, anim::H), 33).unwrap().into_drawing_area();
        for i in 0..anim::FRAMES {
            let (x, y) = data(i);
            root.fill(&WHITE).unwrap();
            let mut c = ChartBuilder::on(&root)
                .caption("Animation", ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(40)
                .y_label_area_size(50)
                .build_cartesian_2d(0.0..10.0, -1.5..1.5)
                .unwrap();
            c.configure_mesh().x_desc("x").y_desc("y").draw().unwrap();
            c.draw_series(LineSeries::new(x.into_iter().zip(y), &BLUE)).unwrap();
            root.present().unwrap();
        }
    });
}
