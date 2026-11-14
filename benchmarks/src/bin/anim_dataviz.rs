#[path = "../anim.rs"]
mod anim;
use dataviz::prelude::*;

fn main() {
    anim::run("dataviz", |scenario, out| {
        let mut fig = Figure::new(anim::W, anim::H);
        let data = if scenario == "wave" { anim::wave } else { anim::stream };
        fig.record(out, 30.0, anim::FRAMES, |fig, i| {
            let (x, y) = data(i);
            *fig = Figure::new(anim::W, anim::H);
            fig.title("Animation");
            let ax = fig.axes();
            ax.line(x, y);
            ax.x_limits(0.0, 10.0).y_limits(-1.5, 1.5);
            ax.x_label("x").y_label("y");
        })
        .unwrap();
    });
}
