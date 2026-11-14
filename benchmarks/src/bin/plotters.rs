#[path = "../data.rs"]
mod data;
use data::*;
use plotters::coord::Shift;
use plotters::prelude::*;

fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

fn draw<DB: DrawingBackend>(root: DrawingArea<DB, Shift>, scenario: &str, n: usize, d: &Data) {
    root.fill(&WHITE).unwrap();
    match scenario {
        "line" | "scatter" => {
            let (x, y) = if scenario == "line" { (&d.x, &d.y) } else { (&d.x, &d.y) };
            let (x0, x1) = minmax(x);
            let (y0, y1) = minmax(y);
            let mut c = ChartBuilder::on(&root)
                .caption(scenario, ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d(x0..x1, y0..y1)
                .unwrap();
            c.configure_mesh().x_desc("x").y_desc("y").draw().unwrap();
            if scenario == "line" {
                c.draw_series(LineSeries::new(x.iter().copied().zip(y.iter().copied()), &BLUE)).unwrap();
            } else {
                c.draw_series(x.iter().zip(y.iter()).map(|(a, b)| Circle::new((*a, *b), 2, BLUE.filled()))).unwrap();
            }
        }
        "bar" => {
            let mut c = ChartBuilder::on(&root)
                .caption("bar", ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(40)
                .build_cartesian_2d((0..n as i32).into_segmented(), 0.0..40.0)
                .unwrap();
            c.configure_mesh().draw().unwrap();
            c.draw_series(d.bars.iter().enumerate().map(|(i, v)| {
                let mut r = Rectangle::new([(SegmentValue::Exact(i as i32), 0.0), (SegmentValue::Exact(i as i32 + 1), *v)], BLUE.filled());
                r.set_margin(0, 0, 5, 5);
                r
            }))
            .unwrap();
        }
        "hist" => {
            // plotters has no float binning: bin here (part of the work).
            let (lo, hi) = minmax(&d.hist);
            let w = (hi - lo) / BINS as f64;
            let mut counts = vec![0u32; BINS];
            for v in &d.hist {
                counts[(((v - lo) / w) as usize).min(BINS - 1)] += 1;
            }
            let top = *counts.iter().max().unwrap() as f64;
            let mut c = ChartBuilder::on(&root)
                .caption("hist", ("sans-serif", 20))
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(50)
                .build_cartesian_2d(lo..hi, 0.0..top * 1.05)
                .unwrap();
            c.configure_mesh().draw().unwrap();
            c.draw_series(counts.iter().enumerate().map(|(i, k)| {
                Rectangle::new([(lo + i as f64 * w, 0.0), (lo + (i + 1) as f64 * w, *k as f64)], BLUE.filled())
            }))
            .unwrap();
        }
        _ => {}
    }
    root.present().unwrap();
}

struct Data {
    x: Vec<f64>,
    y: Vec<f64>,
    bars: Vec<f64>,
    hist: Vec<f64>,
}

fn main() {
    let (scenario, n, fmt, out) = args();
    let path = format!("{out}.{fmt}");
    let (x, y) = match scenario.as_str() {
        "line" => line(n),
        "scatter" => data::scatter(n),
        _ => (vec![], vec![]),
    };
    let d = Data {
        x,
        y,
        bars: if scenario == "bar" { bars(n).1 } else { vec![] },
        hist: if scenario == "hist" { hist(n) } else { vec![] },
    };
    timed(|| {
        if fmt == "svg" {
            draw(SVGBackend::new(&path, (W, H)).into_drawing_area(), &scenario, n, &d);
        } else {
            draw(BitMapBackend::new(&path, (W, H)).into_drawing_area(), &scenario, n, &d);
        }
    });
}
