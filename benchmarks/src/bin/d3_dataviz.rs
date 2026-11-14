//! 3D benchmarks, dataviz: `<scenario> <n> png <out>` (see run_all.py).
#[path = "../data.rs"]
mod data;
#[path = "../d3.rs"]
mod d3;
use data::*;
use dataviz::prelude::*;

fn backend() -> Backend {
    match std::env::var("BACKEND").as_deref() {
        Ok("gpu") => Backend::Gpu,
        Ok("par") => Backend::Cpu { threads: 0 },
        _ => Backend::Cpu { threads: 1 },
    }
}

fn surface_fig(n: usize) -> Figure {
    let mut fig = Figure::new(W, H);
    fig.backend(backend());
    let ax = fig.axes();
    ax.title("surface");
    ax.surface((-8.0, 8.0), (-8.0, 8.0), d3::surface(n)).colormap(Colormap::Viridis);
    ax.x_label("x").y_label("y").z_label("z");
    fig
}

fn molecule_fig(text: &str) -> Figure {
    let mut fig = Figure::new(W, H);
    fig.backend(backend());
    fig.axes().title("1BNA").molecule(Molecule::from_pdb(text));
    fig
}

fn main() {
    let (scenario, n, _fmt, out) = args();
    let path = format!("{out}.png");
    match scenario.as_str() {
        "surface" => timed(|| surface_fig(n).save(&path).unwrap()),
        "scatter3d" => timed_with(|| d3::cloud(n), |(x, y, z)| {
            let mut fig = Figure::new(W, H);
            fig.backend(backend());
            fig.axes().title("scatter3d").scatter3d(x, y, z).size(3.0);
            fig.save(&path).unwrap();
        }),
        "line3d" => timed_with(|| d3::lorenz(n), |(x, y, z)| {
            let mut fig = Figure::new(W, H);
            fig.backend(backend());
            fig.axes().title("Lorenz").line3d(x, y, z).size(1.0);
            fig.save(&path).unwrap();
        }),
        // Parsing, bond search and drawing are all timed.
        "molecule" => {
            let text = d3::pdb_text();
            timed(|| molecule_fig(&text).save(&path).unwrap());
        }
        // One interactive frame: the camera turns, the scene is drawn into
        // RGBA pixels (what a window shows).
        "rotate_surface" | "rotate_molecule" => {
            let text = d3::pdb_text();
            let mut fig = if scenario == "rotate_surface" { surface_fig(n) } else { molecule_fig(&text) };
            let mut px = Pixels::default();
            fig.render_into(&mut px).unwrap();
            timed(|| {
                fig.axes().rotate(Axis3::Z, 4.0);
                fig.render_into(&mut px).unwrap();
            });
        }
        "anim" => timed(|| {
            let mut fig = surface_fig(n);
            fig.record(&format!("{out}.gif"), 30.0, d3::ANIM_FRAMES, |f, _| {
                f.axes().rotate(Axis3::Z, d3::ANIM_STEP);
            })
            .unwrap();
        }),
        _ => println!("0"),
    }
}
