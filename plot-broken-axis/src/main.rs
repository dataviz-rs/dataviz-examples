//! Broken axes: one very large bar next to small ones, and a sensor
//! recording with a long outage, each with the empty range left out.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 440);
    fig.grid(1, 2);

    let ax = fig.at(0, 0);
    ax.title("Revenue by product");
    ax.bar(["A", "B", "C", "D", "E"], [12.0, 18.0, 9.0, 950.0, 15.0])
        .value_labels(LabelPosition::Outside);
    ax.y_label("revenue [k€]");
    ax.y_axis().break_at(25.0, 920.0);

    let ax = fig.at(0, 1);
    ax.title("Sensor with an outage");
    let t: Vec<f64> = (0..400)
        .map(|i| if i < 200 { i as f64 * 0.05 } else { 90.0 + (i - 200) as f64 * 0.05 })
        .collect();
    let v: Vec<f64> = t.iter().map(|x| (x * 0.8).sin() + 0.1 * (x * 7.0).sin()).collect();
    ax.line(&t, &v);
    ax.x_label("time [h]");
    ax.x_axis().break_at(10.5, 89.5);

    fig.save("broken_axis.png")?;
    fig.save("broken_axis.svg")?;
    Ok(())
}
