//! Shared, deterministic data of the 3D benchmarks.
#![allow(dead_code)]

use crate::data::Rng;

/// Surface z = sin(r) / r on an `n × n` grid over [-8, 8]² (row = y).
pub fn surface(n: usize) -> Vec<Vec<f64>> {
    let at = |i: usize| -8.0 + 16.0 * i as f64 / (n - 1) as f64;
    (0..n)
        .map(|r| {
            (0..n)
                .map(|c| {
                    let d = at(c).hypot(at(r));
                    if d < 1e-9 { 1.0 } else { d.sin() / d }
                })
                .collect()
        })
        .collect()
}

pub fn grid_axis(n: usize) -> Vec<f64> {
    (0..n).map(|i| -8.0 + 16.0 * i as f64 / (n - 1) as f64).collect()
}

/// Three Gaussian clusters of `n` points in total.
pub fn cloud(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut r = Rng(5);
    let c = [[0.0, 0.0, 0.0], [4.0, 1.0, 2.0], [1.0, 4.0, -2.0]];
    let (mut x, mut y, mut z) = (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n));
    for i in 0..n {
        let m = c[i % 3];
        x.push(m[0] + r.normal());
        y.push(m[1] + r.normal());
        z.push(m[2] + r.normal());
    }
    (x, y, z)
}

/// The Lorenz attractor, `n` points.
pub fn lorenz(n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let (mut x, mut y, mut z) = (Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n));
    let (mut a, mut b, mut c) = (1.0, 1.0, 1.0);
    let dt = 0.002;
    for _ in 0..n {
        let (da, db, dc) = (10.0 * (b - a), a * (28.0 - c) - b, a * b - 8.0 / 3.0 * c);
        a += da * dt;
        b += db * dt;
        c += dc * dt;
        x.push(a);
        y.push(b);
        z.push(c);
    }
    (x, y, z)
}

pub struct Atom {
    pub pos: [f64; 3],
    pub element: String,
}

/// The PDB file of the molecule scenario (B-DNA 1BNA, 486 atoms).
pub fn pdb_text() -> String {
    let path = std::env::var("D3_PDB").unwrap_or_else(|_| "../plot-biology-3d/data/1BNA.pdb".into());
    std::fs::read_to_string(path).expect("PDB file (set D3_PDB)")
}

/// Atoms of the PDB text without water.
pub fn atoms(text: &str) -> Vec<Atom> {
    text.lines()
        .filter(|l| (l.starts_with("ATOM") || l.starts_with("HETATM")) && l.len() >= 54 && !l.contains("HOH"))
        .map(|l| {
            let f = |a: usize, b: usize| l[a..b].trim().parse::<f64>().unwrap();
            let e = l.get(76..78).unwrap_or("").trim();
            Atom {
                pos: [f(30, 38), f(38, 46), f(46, 54)],
                element: if e.is_empty() { l[12..16].trim()[..1].to_string() } else { e.to_string() },
            }
        })
        .collect()
}

/// CPK color, covalent radius and van der Waals radius.
pub fn element(e: &str) -> ([u8; 3], f64, f64) {
    match e {
        "H" => ([235, 235, 235], 0.31, 1.20),
        "C" => ([144, 144, 144], 0.76, 1.70),
        "N" => ([48, 80, 248], 0.71, 1.55),
        "O" => ([255, 13, 13], 0.66, 1.52),
        "P" => ([255, 128, 0], 1.07, 1.80),
        "S" => ([230, 200, 40], 1.05, 1.80),
        _ => ([255, 20, 147], 0.8, 1.7),
    }
}

/// Bonds from distances (the same rule dataviz uses), O(n²): setup only.
pub fn bonds(a: &[Atom]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            let d2: f64 = (0..3).map(|k| (a[i].pos[k] - a[j].pos[k]).powi(2)).sum();
            let m = element(&a[i].element).1 + element(&a[j].element).1 + 0.45;
            if d2 > 0.16 && d2 < m * m {
                out.push((i, j));
            }
        }
    }
    out
}

/// Viridis at `t` in 0..1 (a few stops, linear in between).
pub fn viridis(t: f64) -> [u8; 3] {
    const S: [[f64; 3]; 5] = [
        [68.0, 1.0, 84.0],
        [59.0, 82.0, 139.0],
        [33.0, 145.0, 140.0],
        [94.0, 201.0, 98.0],
        [253.0, 231.0, 37.0],
    ];
    let t = t.clamp(0.0, 1.0) * 4.0;
    let i = (t as usize).min(3);
    let f = t - i as f64;
    let c = |k: usize| (S[i][k] + (S[i + 1][k] - S[i][k]) * f) as u8;
    [c(0), c(1), c(2)]
}

/// Frames of the rotation GIF and degrees per frame.
pub const ANIM_FRAMES: usize = 90;
pub const ANIM_STEP: f64 = 4.0;
