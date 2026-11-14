//! Charts for biology, genomics and medicine: a DNA sequence logo, a gene
//! track, a Manhattan plot, a volcano plot, single-cell clusters, a
//! dose-response curve, Kaplan-Meier survival, a forest plot and a
//! clustermap of gene expression.

use dataviz::plot::series::bio::Gene;
use dataviz::prelude::*;

/// Small deterministic random generator for the demo data.
struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn n(&mut self) -> f64 {
        let (a, b) = (self.u().max(1e-12), self.u());
        (-2.0 * a.ln()).sqrt() * (std::f64::consts::TAU * b).cos()
    }
}

fn main() -> Result<(), Error> {
    let mut r = Rng(11);
    let mut fig = Figure::new(1500, 1250);
    fig.title("Biology, genomics and medicine");
    fig.grid(3, 3);

    // TATA box-like motif: conserved core, variable flanks.
    let motif = "TATAAAAG";
    let seqs: Vec<String> = (0..60)
        .map(|_| {
            motif
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let keep = if (1..6).contains(&i) { 0.92 } else { 0.55 };
                    if r.u() < keep { c } else { ['A', 'C', 'G', 'T'][(r.u() * 4.0) as usize] }
                })
                .collect()
        })
        .collect();
    let seq_refs: Vec<&str> = seqs.iter().map(String::as_str).collect();
    fig.at(0, 0).title("DNA sequence logo").sequence_logo(&seq_refs);

    let genes = vec![
        Gene { name: "GENE1".into(), start: 1_000.0, end: 9_000.0, forward: true, exons: vec![(1_000.0, 1_600.0), (3_200.0, 3_700.0), (5_100.0, 5_400.0), (8_200.0, 9_000.0)] },
        Gene { name: "GENE2".into(), start: 6_000.0, end: 14_000.0, forward: false, exons: vec![(6_000.0, 6_900.0), (9_500.0, 10_200.0), (13_100.0, 14_000.0)] },
        Gene { name: "GENE3".into(), start: 15_000.0, end: 19_500.0, forward: true, exons: vec![(15_000.0, 15_800.0), (17_000.0, 17_400.0), (18_800.0, 19_500.0)] },
    ];
    fig.at(0, 1).title("Gene track").gene_track(&genes);

    // GWAS: 22 chromosomes, a few peaks.
    let (mut chrom, mut pos, mut p) = (Vec::new(), Vec::new(), Vec::new());
    for c in 1..=22 {
        let len = 250.0 - c as f64 * 8.0;
        for k in 0..400 {
            let x = k as f64 / 400.0 * len;
            let peak = [(3, 120.0), (8, 60.0), (17, 30.0)].iter().any(|&(pc, px)| pc == c && (x - px).abs() < 6.0);
            let v = if peak { 10f64.powf(-(4.0 + r.u() * 8.0)) } else { r.u() };
            chrom.push(c.to_string());
            pos.push(x * 1e6);
            p.push(v);
        }
    }
    fig.span(0, 2, 1, 1).title("Manhattan plot (GWAS)").manhattan(chrom, pos, &p);

    // Differential expression.
    let n = 3000;
    let fc: Vec<f64> = (0..n).map(|_| r.n() * 1.2).collect();
    let pv: Vec<f64> = fc.iter().map(|f| (10f64).powf(-(f.abs() * 2.2 + r.u() * 1.5))).collect();
    let names: Vec<String> = (0..n).map(|i| format!("G{i}")).collect();
    fig.at(1, 0).title("Volcano plot").volcano(names, &fc, &pv, 1.0, 1e-3, 6);

    // Single-cell clusters.
    let centres = [("T cells", -4.0, 2.0), ("B cells", 3.0, 4.0), ("NK", 0.0, -4.0), ("Monocytes", 5.0, -2.0), ("DC", -5.0, -3.0)];
    let (mut ux, mut uy, mut cl) = (Vec::new(), Vec::new(), Vec::new());
    for _ in 0..6000 {
        let (name, cx, cy) = centres[(r.u() * centres.len() as f64) as usize];
        ux.push(cx + r.n() * 0.9);
        uy.push(cy + r.n() * 0.9);
        cl.push(name);
    }
    fig.at(1, 1).title("Single-cell UMAP (6 000 cells)").embedding(&ux, &uy, cl);

    let dose: Vec<f64> = (0..10).map(|i| 10f64.powf(-2.0 + i as f64 * 0.5)).collect();
    let resp: Vec<f64> = dose.iter().map(|d| 4.0 + 92.0 / (1.0 + (d / 0.8f64).powf(1.2)) + r.n() * 3.0).collect();
    fig.at(1, 2).title("Dose-response").dose_response(&dose, &resp);

    let surv = |r: &mut Rng, scale: f64| -> (Vec<f64>, Vec<bool>) {
        (0..80).map(|_| {
            let t = -scale * r.u().max(1e-9).ln();
            if t > 60.0 { (60.0, false) } else { (t, r.u() > 0.15) }
        }).unzip()
    };
    let (ta, ea) = surv(&mut r, 20.0);
    let (tb, eb) = surv(&mut r, 38.0);
    let ax = fig.at(2, 0);
    ax.title("Kaplan-Meier survival").kaplan_meier([("standard", ta, ea), ("new treatment", tb, eb)]);
    ax.x_label("months");

    let ax = fig.at(2, 1);
    ax.title("Forest plot (hazard ratios)");
    ax.forest_plot(
        ["Study A", "Study B", "Study C", "Study D", "Pooled"],
        [0.72, 0.85, 0.64, 0.91, 0.78],
        [0.55, 0.62, 0.45, 0.70, 0.68],
        [0.94, 1.16, 0.91, 1.18, 0.89],
        1.0,
    )
    .summary_last(true);
    ax.x_axis().log();
    ax.x_label("hazard ratio");

    fig.at(2, 2).title("p-value Q-Q").qq_pvalues(&p);
    fig.save("biology.png")?;
    fig.save("biology.svg")?;

    // Clustermap of 30 genes × 12 samples in three groups.
    let genes: Vec<String> = (0..30).map(|i| format!("gene{i}")).collect();
    let samples: Vec<String> = (0..12).map(|i| format!("S{i}")).collect();
    let data: Vec<Vec<f64>> = (0..30)
        .map(|g| (0..12).map(|s| ((g % 3) as f64 - 1.0) * ((s % 3) as f64 - 1.0) * 1.5 + r.n() * 0.4).collect())
        .collect();
    let mut cm = Figure::new(900, 900);
    cm.title("Clustermap: gene expression");
    cm.clustermap(genes, samples, &data);
    cm.save("clustermap.png")?;
    Ok(())
}
