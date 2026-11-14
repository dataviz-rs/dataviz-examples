//! Interactive window and live (streaming) data.

use dataviz::plot::series::line::Line;
use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(900, 560);
    let ax = fig.axes();
    ax.title("Interactive: wheel = zoom, drag = pan, Shift+drag = measure, R = reset");
    let x: Vec<f64> = (0..2000).map(|i| i as f64 / 100.0).collect();
    ax.line(&x, x.iter().map(|v| v.sin() * (v / 5.0).exp())).label("growing sine");
    ax.scatter((0..20).map(|i| i as f64), (0..20).map(|i| (i as f64 * 0.7).cos() * 5.0))
        .label("points");
    // Overview of all data below the plot; drag it to pan, its edges to zoom.
    ax.range_slider(true);
    ax.x_limits(4.0, 12.0).auto_scale_y(true);
    fig.save("interactive.png")?;
    if std::env::args().any(|a| a == "--demo") {
        return demo(&mut fig);
    }
    fig.show("dataviz interactive")?;

    let mut live = Figure::new(900, 400);
    live.axes()
        .title("Live data (rolling window)")
        .line(Vec::<f64>::new(), Vec::<f64>::new())
        .label("signal");
    let mut t = 0.0;
    live.show_live("dataviz live", 30, |fig| {
        t += 0.05;
        let line = fig.axes().series_mut::<Line>(0).unwrap();
        line.push(t, (t * 2.0).sin() + (t * 0.3).cos() * 0.5);
        line.keep_last(300);
        true
    })
}

/// `--demo`: records `interactive.gif` without a window: zooming in with
/// the wheel, panning by dragging and resetting, through the same calls a
/// window or GUI makes (`zoom_at`, `pan_by`, `reset_view`).
fn demo(fig: &mut Figure) -> Result<(), Error> {
    let mut rec = Recorder::create("interactive.gif", 25.0)?;
    rec.frame(fig)?; // also records the layout for the calls below
    // Wheel zoom at the centre of the plot, then a drag to the right
    // (earlier data); y keeps fitting the visible data (auto scale).
    let (cx, cy) = (460.0, 250.0);
    for _ in 0..12 {
        rec.frame(fig)?;
    }
    for _ in 0..14 {
        fig.zoom_at(cx, cy, 0.92);
        rec.frame(fig)?;
    }
    for _ in 0..30 {
        fig.pan_by(cx, cy, 10.0, 0.0);
        rec.frame(fig)?;
    }
    for _ in 0..15 {
        rec.frame(fig)?;
    }
    fig.reset_view();
    for _ in 0..20 {
        rec.frame(fig)?;
    }
    rec.finish()
}
