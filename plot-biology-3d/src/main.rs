//! 3D biology: a DNA double helix, real structures from the Protein Data
//! Bank (B-DNA 1BNA as ball-and-stick, the protein crambin 1CRN as a
//! backbone tube), a cell with nucleus, mitochondria and ribosomes, an
//! organoid of cells and single-cell clusters on a 3D UMAP.
//!
//! `cargo run --release` writes `biology_3d.png` and `rotate.gif` (the
//! scenes turning around x, y and z). With `--window` the figure opens in a
//! window: drag to turn, wheel to zoom, `X`/`Y`/`Z` (Shift = reverse) to
//! rotate around an axis, `R` to reset.

use dataviz::prelude::*;

const BNA: &str = include_str!("../data/1BNA.pdb");
const CRN: &str = include_str!("../data/1CRN.pdb");

/// Small deterministic random numbers (xorshift).
struct Rng(u64);

impl Rng {
    fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        let (u, v) = (self.uniform().max(1e-12), self.uniform());
        (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
    }
}

fn dna(ax: &mut Axes) {
    ax.title("DNA double helix");
    ax.dna_helix("ATGCGTATAAAAGCCATGGCATTCG")
        .highlight(5..11, "#ffd400", Some("TATA box"));
    ax.legend(LegendPosition::OutsideBottom).legend_columns(2);
}

/// A eukaryotic cell: translucent membrane, nucleus with nucleolus,
/// mitochondria, endoplasmic-reticulum vesicles and ribosomes.
fn cell(ax: &mut Axes) {
    let mut r = Rng(7);
    ax.title("Cell");
    ax.ellipsoid([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])
        .color("#8ec3e6")
        .opacity(0.16)
        .label("membrane");
    ax.ellipsoid([1.0, 0.5, 0.0], [3.6, 3.2, 3.0])
        .color("#8e44ad")
        .opacity(0.55)
        .label("nucleus");
    ax.ellipsoid([1.6, 0.9, 0.4], [1.1, 1.0, 1.0]).color("#4a235a");
    let mito = [
        ([-5.5, -2.5, 1.0], [1.8, 0.7, 0.7]),
        ([-4.0, 3.5, -1.0], [0.7, 1.7, 0.7]),
        ([5.5, -3.0, -0.5], [1.6, 0.7, 0.7]),
        ([6.0, 3.0, 1.5], [0.7, 0.7, 1.4]),
        ([-1.0, -5.5, 0.5], [1.7, 0.7, 0.7]),
        ([1.5, 5.5, 1.0], [1.5, 0.7, 0.7]),
    ];
    for (k, (c, rad)) in mito.into_iter().enumerate() {
        let m = ax.ellipsoid(c, rad);
        m.color("#e67e22");
        if k == 0 {
            m.label("mitochondria");
        }
    }
    let (mut x, mut y, mut z) = (vec![], vec![], vec![]);
    let (mut vx, mut vy, mut vz) = (vec![], vec![], vec![]);
    let mut k = 0;
    while x.len() < 160 {
        let p = [r.uniform() * 2.0 - 1.0, r.uniform() * 2.0 - 1.0, r.uniform() * 2.0 - 1.0];
        let d = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
        let (px, py, pz) = (p[0] * 10.0, p[1] * 8.0, p[2] * 6.0);
        let nucleus = ((px - 1.0) / 3.9).powi(2) + ((py - 0.5) / 3.5).powi(2) + (pz / 3.3).powi(2);
        if d > 0.85 || nucleus < 1.0 {
            continue;
        }
        k += 1;
        if k % 8 == 0 {
            vx.push(px);
            vy.push(py);
            vz.push(pz);
        } else {
            x.push(px);
            y.push(py);
            z.push(pz);
        }
    }
    ax.spheres3d(&vx, &vy, &vz, [0.7]).color("#27ae60").label("vesicles");
    ax.spheres3d(&x, &y, &z, [0.28]).color("#2e86c1").label("ribosomes");
    ax.box3d(false).equal_aspect(true).depth_cue(0.2);
}

/// Cells on the surface of an organoid, colored by a marker gene that
/// rises from one pole to the other.
fn organoid(ax: &mut Axes) {
    let mut r = Rng(11);
    ax.title("Organoid: marker expression");
    let n = 420;
    let (mut x, mut y, mut z, mut v) = (vec![], vec![], vec![], vec![]);
    for i in 0..n {
        // Fibonacci sphere: evenly spread cells.
        let t = (1.0 - 2.0 * (i as f64 + 0.5) / n as f64).acos();
        let p = i as f64 * 2.399963;
        let rad = 10.0 + r.normal() * 0.3;
        x.push(rad * t.sin() * p.cos());
        y.push(rad * t.sin() * p.sin());
        z.push(rad * t.cos());
        v.push((t.cos() + 1.0) * 2.0 + r.normal() * 0.35);
    }
    ax.spheres3d(&x, &y, &z, [1.0]).values(&v, Colormap::Viridis);
    ax.box3d(false).equal_aspect(true).depth_cue(0.3);
}

fn umap(ax: &mut Axes) {
    let mut r = Rng(3);
    ax.title("Single-cell clusters (3D UMAP)");
    let types = [
        ("CD4 T", [0.0, 0.0, 0.0], 1.0),
        ("CD8 T", [3.5, 1.5, 0.5], 0.9),
        ("NK", [6.0, 3.0, -1.0], 0.8),
        ("B", [-4.0, 5.0, 2.0], 0.9),
        ("Monocytes", [-2.0, -5.0, 4.0], 1.2),
        ("DC", [6.0, -4.0, -3.0], 0.6),
    ];
    let (mut x, mut y, mut z, mut c) = (vec![], vec![], vec![], vec![]);
    for i in 0..4000 {
        let (name, m, s) = types[i % types.len()];
        x.push(m[0] + r.normal() * s);
        y.push(m[1] + r.normal() * s);
        z.push(m[2] + r.normal() * s);
        c.push(name);
    }
    ax.embedding3d(&x, &y, &z, &c);
}

fn figure() -> Figure {
    let mut fig = Figure::new(1500, 1000);
    fig.grid(2, 3);
    dna(fig.at(0, 0));
    fig.at(0, 1)
        .title("B-DNA dodecamer (PDB 1BNA)")
        .molecule(Molecule::from_pdb(BNA));
    fig.at(0, 2)
        .title("Crambin backbone (PDB 1CRN)")
        .molecule(Molecule::from_pdb(CRN))
        .style(MolStyle::Backbone)
        .coloring(MolColor::Rainbow);
    cell(fig.at(1, 0));
    organoid(fig.at(1, 1));
    umap(fig.at(1, 2));
    fig
}

/// The DNA and the cell turning a full circle around z, then x, then y.
fn rotate_gif() -> Result<(), Error> {
    let mut fig = Figure::new(840, 400);
    fig.grid(1, 2);
    dna(fig.at(0, 0));
    cell(fig.at(0, 1));
    fig.at(0, 1).legend(LegendPosition::Hidden);
    let steps = 30;
    let mut rec = Recorder::create("rotate.gif", 24.0)?;
    for axis in [Axis3::Z, Axis3::X, Axis3::Y] {
        let name = format!("{axis:?}").to_lowercase();
        for _ in 0..steps {
            fig.title(format!("Rotating around the {name} axis"));
            fig.at(0, 0).rotate(axis, 360.0 / steps as f64);
            fig.at(0, 1).rotate(axis, 360.0 / steps as f64);
            rec.frame(&mut fig)?;
        }
    }
    rec.finish()
}

fn main() -> Result<(), Error> {
    let mut fig = figure();
    fig.save("biology_3d.png")?;
    if std::env::args().any(|a| a == "--window") {
        return fig.show("dataviz 3D: drag = turn, wheel = zoom, X/Y/Z = rotate, R = reset");
    }
    let start = std::time::Instant::now();
    rotate_gif()?;
    let kb = std::fs::metadata("rotate.gif").map(|m| m.len() / 1024).unwrap_or(0);
    println!("rotate.gif: {kb} KB in {:.1?}", start.elapsed());
    Ok(())
}
