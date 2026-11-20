//! Scans as volumes: a head phantom (3D Shepp–Logan ellipsoids, in
//! Hounsfield-like units) written as NIfTI and as a DICOM series, read back
//! with `VolumeData::read` and rendered with gradient lighting.

use dataviz::plot::volume_file::VolumeData;
use dataviz::prelude::*;

/// (value, centre, half axes, rotation about z in degrees) of the
/// ellipsoids of a 3D Shepp–Logan head phantom (Kak & Slaney).
const PHANTOM: [(f64, [f64; 3], [f64; 3], f64); 10] = [
    (1000.0, [0.0, 0.0, 0.0], [0.69, 0.92, 0.9], 0.0),
    (-800.0, [0.0, -0.0184, 0.0], [0.63, 0.85, 0.84], 0.0),
    (-200.0, [0.22, 0.0, 0.0], [0.11, 0.31, 0.22], -18.0),
    (-200.0, [-0.22, 0.0, 0.0], [0.16, 0.41, 0.28], 18.0),
    (100.0, [0.0, 0.35, -0.15], [0.21, 0.25, 0.41], 0.0),
    (100.0, [0.0, 0.1, 0.25], [0.046, 0.046, 0.05], 0.0),
    (100.0, [-0.08, -0.605, 0.0], [0.046, 0.023, 0.05], 0.0),
    (100.0, [0.0, -0.1, 0.25], [0.046, 0.046, 0.05], 0.0),
    (100.0, [0.06, -0.605, 0.0], [0.023, 0.046, 0.02], 0.0),
    (100.0, [0.0, -0.605, 0.0], [0.023, 0.023, 0.02], 0.0),
];

fn phantom(n: usize, nz: usize) -> Vec<f32> {
    let mut v = vec![-1000.0f32; n * n * nz];
    for k in 0..nz {
        for j in 0..n {
            for i in 0..n {
                let c = |a: usize, m: usize| a as f64 / (m - 1) as f64 * 2.0 - 1.0;
                let (x, y, z) = (c(i, n), c(j, n), c(k, nz));
                let mut s = 0.0;
                for (val, p, a, deg) in PHANTOM {
                    let (sn, cs) = deg.to_radians().sin_cos();
                    let (dx, dy, dz) = (x - p[0], y - p[1], z - p[2]);
                    let (rx, ry) = (dx * cs + dy * sn, -dx * sn + dy * cs);
                    // Soft edges (about one voxel wide) as in a real
                    // scan, which also keeps the lighting smooth.
                    let q = ((rx / a[0]).powi(2) + (ry / a[1]).powi(2) + (dz / a[2]).powi(2)).sqrt();
                    let w = ((1.0 - q) * a[0].min(a[1]).min(a[2]) * n as f64 * 0.5 + 0.5).clamp(0.0, 1.0);
                    s += val * w;
                }
                // Cut-away: one octant removed to show the inside.
                if x > 0.0 && y < 0.0 && z > -0.1 {
                    continue;
                }
                // Shepp–Logan intensities to Hounsfield-like units: bone
                // 1000, brain 30, ventricles -70, lesions 80.
                // Air → bone → brain blend linearly through the edges.
                let hu = if s <= 200.0 {
                    -1000.0 + s / 200.0 * 1030.0
                } else if s >= 1000.0 {
                    1000.0
                } else {
                    (s - 200.0) * 0.5 + 30.0
                };
                // The skull (s between 200 and 1000 means ellipse 1 only).
                v[i + n * (j + n * k)] = hu as f32;
            }
        }
    }
    v
}

/// A DICOM slice (explicit VR little endian, 16-bit signed, rescaled):
/// just the elements a viewer needs to stack the series.
fn dicom_slice(cols: usize, rows: usize, spacing: f64, pos: [f64; 3], hu: &[f32]) -> Vec<u8> {
    let mut b = vec![0u8; 128];
    b.extend(b"DICM");
    let mut el = |g: u16, e: u16, vr: &[u8; 2], v: &[u8]| {
        let mut v = v.to_vec();
        if v.len() % 2 == 1 {
            v.push(if vr == b"UI" { 0 } else { b' ' });
        }
        b.extend(g.to_le_bytes());
        b.extend(e.to_le_bytes());
        b.extend(vr);
        if vr == b"OW" {
            b.extend([0, 0]);
            b.extend((v.len() as u32).to_le_bytes());
        } else {
            b.extend((v.len() as u16).to_le_bytes());
        }
        b.extend(v);
    };
    el(0x0002, 0x0010, b"UI", b"1.2.840.10008.1.2.1");
    el(0x0020, 0x0032, b"DS", format!("{}\\{}\\{}", pos[0], pos[1], pos[2]).as_bytes());
    el(0x0028, 0x0002, b"US", &1u16.to_le_bytes());
    el(0x0028, 0x0010, b"US", &(rows as u16).to_le_bytes());
    el(0x0028, 0x0011, b"US", &(cols as u16).to_le_bytes());
    el(0x0028, 0x0030, b"DS", format!("{spacing}\\{spacing}").as_bytes());
    el(0x0028, 0x0100, b"US", &16u16.to_le_bytes());
    el(0x0028, 0x0103, b"US", &1u16.to_le_bytes());
    el(0x0028, 0x1052, b"DS", b"-1024");
    el(0x0028, 0x1053, b"DS", b"1");
    let px: Vec<u8> = hu.iter().flat_map(|v| ((v + 1024.0).round() as i16).to_le_bytes()).collect();
    el(0x7FE0, 0x0010, b"OW", &px);
    b
}

fn main() -> Result<(), Error> {
    let (n, nz) = (128, 96);
    let scan = VolumeData {
        values: phantom(n, nz),
        dims: [n, n, nz],
        // 1.5 mm voxels in plane, 2 mm slices: the volume keeps its
        // physical proportions.
        spacing: [1.5, 1.5, 2.0],
        origin: [-96.0, -96.0, -96.0],
    };

    // As NIfTI (what MRI/CT pipelines exchange) …
    std::fs::create_dir_all("out/dicom").map_err(Error::Io)?;
    std::fs::write("out/head.nii", scan.to_nifti()).map_err(Error::Io)?;
    // … and as a DICOM series: one file per slice, rescaled to HU.
    for k in 0..nz {
        let slice = &scan.values[k * n * n..(k + 1) * n * n];
        let pos = [-96.0, -96.0, -96.0 + 2.0 * k as f64];
        let bytes = dicom_slice(n, n, 1.5, pos, slice);
        std::fs::write(format!("out/dicom/IM{k:04}.dcm"), bytes).map_err(Error::Io)?;
    }

    let t = std::time::Instant::now();
    let nifti = VolumeData::read("out/head.nii").map_err(Error::Render)?;
    let t_nifti = t.elapsed();
    let t = std::time::Instant::now();
    let dicom = VolumeData::read("out/dicom").map_err(Error::Render)?;
    let t_dicom = t.elapsed();
    println!(
        "read NIfTI in {:.1} ms, {} DICOM slices in {:.1} ms ({}×{}×{} voxels)",
        t_nifti.as_secs_f64() * 1e3,
        nz,
        t_dicom.as_secs_f64() * 1e3,
        dicom.dims[0],
        dicom.dims[1],
        dicom.dims[2]
    );

    let mut fig = Figure::new(1260, 470);
    fig.title("Head phantom read from NIfTI and DICOM (one octant cut away)");
    fig.grid(1, 3);
    // Bone window from the NIfTI file, with and without lighting.
    for (col, lit) in [(0, true), (1, false)] {
        let ax = fig.at(0, col);
        ax.title(if lit { "bone, lit (default)" } else { "bone, without lighting" });
        ax.volume_data(&nifti)
            .colormap(Colormap::Magma)
            .range(-1000.0, 1000.0)
            .threshold(0.62)
            .opacity(0.6)
            .shading(lit);
        ax.camera(-50.0, 25.0);
    }
    // Soft-tissue window from the DICOM series: lesions inside the brain.
    let ax = fig.at(0, 2);
    ax.title("soft tissue (DICOM series)");
    ax.volume_data(&dicom)
        .colormap(Colormap::Inferno)
        .range(-100.0, 150.0)
        .threshold(0.45)
        .opacity(0.25)
        .colorbar("HU");
    ax.camera(-50.0, 25.0);
    let t = std::time::Instant::now();
    fig.save("volume_files.png")?;
    println!("three volume renderings saved in {:.0} ms", t.elapsed().as_secs_f64() * 1e3);
    Ok(())
}
