//! Gantt chart: Project plan with progress on a time axis and a "today" marker.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Project plan");
    let d = |m: u32, day: u32| date(2024, m, day);
    ax.gantt(
        ["Research", "Design", "Build", "Test", "Launch"],
        [d(1, 1), d(1, 20), d(2, 10), d(3, 15), d(4, 5)],
        [d(1, 25), d(2, 20), d(3, 30), d(4, 10), d(4, 15)],
    )
    .progress([1.0, 1.0, 0.6, 0.1, 0.0]);
    ax.x_axis().time();
    ax.vline(d(3, 1)).label("today");

    fig.save("gantt.png")?;
    fig.save("gantt.svg")?;
    Ok(())
}
