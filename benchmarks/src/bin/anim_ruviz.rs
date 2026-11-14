#[path = "../anim.rs"]
mod anim;
use ruviz::animation::RecordConfig;
use ruviz::prelude::*;
use ruviz::record;

fn main() {
    anim::run("ruviz", |scenario, out| {
        let data = if scenario == "wave" { anim::wave } else { anim::stream };
        let config = RecordConfig::new().dimensions(anim::W, anim::H).framerate(30);
        record!(out, anim::FRAMES, config: config, |t| {
            let (x, y) = data(t.count as usize);
            Plot::new()
                .line(&x, &y)
                .title("Animation")
                .xlabel("x")
                .ylabel("y")
                .xlim(0.0, 10.0)
                .ylim(-1.5, 1.5)
        })
        .unwrap();
    });
}
