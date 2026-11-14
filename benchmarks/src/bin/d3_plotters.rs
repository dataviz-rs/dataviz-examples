//! 3D benchmarks, plotters: `<scenario> <n> png <out>` (see run_all.py).
//! plotters draws 3D elements in the order given (no depth sorting) and
//! has no lighting; surfaces are colored by height.
#[path = "../data.rs"]
mod data;
#[path = "../d3.rs"]
mod d3;
use data::*;
use plotters::coord::Shift;
use plotters::prelude::*;

fn minmax(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

fn rgb(c: [u8; 3]) -> RGBColor {
    RGBColor(c[0], c[1], c[2])
}

fn surface<DB: DrawingBackend>(root: &DrawingArea<DB, Shift>, n: usize, yaw: f64) {
    root.fill(&WHITE).unwrap();
    let mut c = ChartBuilder::on(root)
        .caption("surface", ("sans-serif", 20))
        .margin(10)
        .build_cartesian_3d(-8.0..8.0, -0.3..1.0, -8.0..8.0)
        .unwrap();
    c.with_projection(|mut p| {
        p.yaw = yaw;
        p.pitch = 0.5;
        p.scale = 0.8;
        p.into_matrix()
    });
    c.configure_axes().draw().unwrap();
    let g = d3::grid_axis(n);
    let style = |z: &f64| rgb(d3::viridis((z + 0.25) / 1.25)).filled();
    c.draw_series(
        SurfaceSeries::xoz(g.iter().copied(), g.iter().copied(), |x: f64, y: f64| {
            let d = x.hypot(y);
            if d < 1e-9 { 1.0 } else { d.sin() / d }
        })
        .style_func(&style),
    )
    .unwrap();
}

fn points<DB: DrawingBackend>(root: &DrawingArea<DB, Shift>, title: &str, d: &(Vec<f64>, Vec<f64>, Vec<f64>), line: bool) {
    root.fill(&WHITE).unwrap();
    let (x, y, z) = d;
    let (rx, ry, rz) = (minmax(x), minmax(y), minmax(z));
    // plotters' 3D y-axis points up: data z goes there.
    let mut c = ChartBuilder::on(root)
        .caption(title, ("sans-serif", 20))
        .margin(10)
        .build_cartesian_3d(rx.0..rx.1, rz.0..rz.1, ry.0..ry.1)
        .unwrap();
    c.with_projection(|mut p| {
        p.yaw = 0.6;
        p.pitch = 0.5;
        p.scale = 0.8;
        p.into_matrix()
    });
    c.configure_axes().draw().unwrap();
    let it = (0..x.len()).map(|i| (x[i], z[i], y[i]));
    if line {
        c.draw_series(LineSeries::new(it, &BLUE)).unwrap();
    } else {
        c.draw_series(it.map(|p| Circle::new(p, 2, BLUE.filled()))).unwrap();
    }
}

fn molecule<DB: DrawingBackend>(root: &DrawingArea<DB, Shift>, atoms: &[d3::Atom], bonds: &[(usize, usize)], yaw: f64) {
    root.fill(&WHITE).unwrap();
    let r = |k: usize| minmax(&atoms.iter().map(|a| a.pos[k]).collect::<Vec<_>>());
    let (rx, ry, rz) = (r(0), r(1), r(2));
    let mut c = ChartBuilder::on(root)
        .caption("1BNA", ("sans-serif", 20))
        .margin(10)
        .build_cartesian_3d(rx.0..rx.1, rz.0..rz.1, ry.0..ry.1)
        .unwrap();
    c.with_projection(|mut p| {
        p.yaw = yaw;
        p.pitch = 0.3;
        p.scale = 0.9;
        p.into_matrix()
    });
    let p = |a: &d3::Atom| (a.pos[0], a.pos[2], a.pos[1]);
    c.draw_series(bonds.iter().flat_map(|&(i, j)| {
        let (a, b) = (&atoms[i], &atoms[j]);
        let m = ((a.pos[0] + b.pos[0]) / 2.0, (a.pos[2] + b.pos[2]) / 2.0, (a.pos[1] + b.pos[1]) / 2.0);
        [
            PathElement::new(vec![p(a), m], rgb(d3::element(&a.element).0).stroke_width(2)),
            PathElement::new(vec![m, p(b)], rgb(d3::element(&b.element).0).stroke_width(2)),
        ]
    }))
    .unwrap();
    c.draw_series(atoms.iter().map(|a| {
        let (col, _, vdw) = d3::element(&a.element);
        Circle::new(p(a), (vdw * 2.6) as i32, rgb(col).filled())
    }))
    .unwrap();
}

fn main() {
    let (scenario, n, _fmt, out) = args();
    let path = format!("{out}.png");
    let (w, h) = (W, H);
    match scenario.as_str() {
        "surface" => timed(|| {
            let root = BitMapBackend::new(&path, (w, h)).into_drawing_area();
            surface(&root, n, 0.6);
            root.present().unwrap();
        }),
        "scatter3d" | "line3d" => {
            let d = if scenario == "line3d" { d3::lorenz(n) } else { d3::cloud(n) };
            timed(|| {
                let root = BitMapBackend::new(&path, (w, h)).into_drawing_area();
                points(&root, &scenario, &d, scenario == "line3d");
                root.present().unwrap();
            });
        }
        "molecule" => {
            let text = d3::pdb_text();
            timed(|| {
                // Parsing and bond search timed as for dataviz.
                let atoms = d3::atoms(&text);
                let bonds = d3::bonds(&atoms);
                let root = BitMapBackend::new(&path, (w, h)).into_drawing_area();
                molecule(&root, &atoms, &bonds, 0.6);
                root.present().unwrap();
            });
        }
        // One interactive frame: everything is drawn again into an RGB buffer.
        "rotate_surface" | "rotate_molecule" => {
            let atoms = d3::atoms(&d3::pdb_text());
            let bonds = d3::bonds(&atoms);
            let mut buf = vec![0u8; (w * h * 3) as usize];
            let mut yaw = 0.6;
            timed(|| {
                yaw += 4f64.to_radians();
                let root = BitMapBackend::with_buffer(&mut buf, (w, h)).into_drawing_area();
                if scenario == "rotate_surface" {
                    surface(&root, n, yaw);
                } else {
                    molecule(&root, &atoms, &bonds, yaw);
                }
                root.present().unwrap();
            });
        }
        "anim" => timed(|| {
            let gif = format!("{out}.gif");
            let root = BitMapBackend::gif(&gif, (w, h), 33).unwrap().into_drawing_area();
            for i in 0..d3::ANIM_FRAMES {
                surface(&root, n, 0.6 + (i as f64 * d3::ANIM_STEP).to_radians());
                root.present().unwrap();
            }
        }),
        _ => println!("0"),
    }
}
