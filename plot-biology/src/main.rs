//! Charts for biology, genomics and medicine: a DNA sequence logo, a gene
//! track, a Manhattan plot, a volcano plot, single-cell clusters, a
//! dose-response curve, Kaplan-Meier survival, a forest plot and a
//! clustermap of gene expression.

use dataviz::plot::series::bio::Gene;
use dataviz::plot::series::genome::Alteration;
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
    genomics()
}

/// Genomics: Circos, Hi-C, coverage, phylogenetic trees, oncoprint and a
/// mutation lollipop plot.
fn genomics() -> Result<(), Error> {
    let mut r = Rng(2024);
    let mut fig = Figure::new(1600, 1500);
    fig.grid(3, 3);

    // Circos: hg38 chromosome sizes (Mb), copy-number track, translocations.
    let sizes = [248.0, 242.0, 198.0, 190.0, 181.0, 171.0, 159.0, 145.0, 138.0, 134.0, 135.0, 133.0, 114.0, 107.0, 102.0, 90.0, 83.0, 80.0, 59.0, 64.0, 47.0, 51.0, 156.0];
    let chroms: Vec<(String, f64)> = sizes
        .iter()
        .enumerate()
        .map(|(i, s)| (if i == 22 { "X".to_string() } else { format!("{}", i + 1) }, *s))
        .collect();
    let (mut tc, mut tp, mut tv) = (Vec::new(), Vec::new(), Vec::new());
    for (name, len) in &chroms {
        let mut p = 0.0;
        while p < *len {
            tc.push(name.clone());
            tp.push(p);
            tv.push((1.0 + 0.6 * r.n()).abs() + if name == "8" && p > 120.0 { 2.5 } else { 0.0 });
            p += 2.0;
        }
    }
    let links = [("9", 130.0, "22", 23.0), ("8", 128.0, "14", 105.0), ("4", 50.0, "11", 70.0), ("1", 150.0, "17", 40.0), ("12", 30.0, "3", 160.0), ("5", 80.0, "X", 60.0), ("2", 30.0, "6", 120.0)];
    let ax = fig.at(0, 0);
    ax.title("Circos: copy number and translocations");
    ax.circos(&chroms).track(&tc, &tp, &tv).links(&links);

    // Hi-C: 120 bins of 25 kb, three domains (TADs).
    let n = 120;
    let tads = [(0usize, 38usize), (38, 85), (85, 120)];
    let m: Vec<Vec<f64>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    let d = (i as f64 - j as f64).abs();
                    let same = tads.iter().any(|t| (t.0..t.1).contains(&i) && (t.0..t.1).contains(&j));
                    (800.0 / (1.0 + d).powf(1.15)) * if same { 2.6 } else { 1.0 } * (0.7 + 0.6 * r.u())
                })
                .collect()
        })
        .collect();
    let ax = fig.at(0, 1);
    ax.title("Hi-C contact map with TADs");
    ax.hic(m, 1.0, 0.025).domains(&tads).max_distance(60);
    ax.x_label("chr7 position (Mb)");

    // Coverage over a 5 kb exome target with a dropout.
    let pos: Vec<f64> = (0..5000).map(|i| i as f64).collect();
    // Captured exons with smooth edges, slowly varying depth, a GC dropout.
    let mut wobble = 0.0;
    let depth: Vec<f64> = (0..5000)
        .map(|i| {
            let edge = [(300.0, 900.0), (1500.0, 2100.0), (2600.0, 3300.0), (4000.0, 4700.0)]
                .iter()
                .map(|e: &(f64, f64)| {
                    let x = i as f64;
                    let a = 1.0 / (1.0 + (-(x - e.0) / 40.0).exp());
                    let b = 1.0 / (1.0 + ((x - e.1) / 40.0).exp());
                    a * b
                })
                .fold(0.0, f64::max);
            wobble = 0.995 * wobble + 0.4 * r.n();
            let dip = if (2800..3000).contains(&i) { 0.18 } else { 1.0 };
            ((4.0 + 82.0 * edge) * dip + wobble).max(0.0)
        })
        .collect();
    let ax = fig.at(0, 2);
    ax.title("Coverage (CFTR exons), min depth 20×");
    ax.coverage(&pos, &depth).min_depth(20.0);
    ax.x_label("bp from chr7:117,559,000");

    // Primate tree (divergence, rectangular) and a mammal tree (circular).
    let primates = "((((Human:0.0067,Chimpanzee:0.0067):0.0022,Gorilla:0.0089):0.0083,Orangutan:0.0172):0.0098,((Rhesus_macaque:0.0040,Crab-eating_macaque:0.0040):0.0054,(Baboon:0.0060,Gelada:0.0060):0.0034):0.0176,(Marmoset:0.0280,Squirrel_monkey:0.0280):0.0150);";
    let ax = fig.at(1, 0);
    ax.title("Phylogeny of primates");
    ax.phylo_tree(primates).unwrap().color_leaves("Human", "#d62728").color_leaves("Chimp", "#d62728");
    let mammals = "(((((Human:0.1,Chimp:0.1):0.2,(Mouse:0.25,Rat:0.25):0.05):0.05,((Dog:0.2,Cat:0.2):0.05,(Cow:0.18,(Pig:0.15,Sheep:0.15):0.03):0.07,(Horse:0.22,Rhino:0.22):0.03):0.1):0.1,((Elephant:0.3,Manatee:0.3):0.1,(Armadillo:0.32,Sloth:0.32):0.08):0.05):0.15,(Kangaroo:0.45,(Koala:0.4,Opossum:0.4):0.05):0.2,Platypus:0.7);";
    let ax = fig.at(1, 1);
    ax.title("Mammals (circular tree)");
    ax.phylo_tree_circular(mammals).unwrap().color_leaves("Kangaroo", "#ff7f0e").color_leaves("Koala", "#ff7f0e").color_leaves("Opossum", "#ff7f0e").color_leaves("Platypus", "#2ca02c");

    // Oncoprint: 10 genes in 90 tumours.
    let genes = ["TP53", "KRAS", "PIK3CA", "APC", "PTEN", "EGFR", "BRAF", "CDKN2A", "MYC", "ERBB2"];
    let rates = [0.52, 0.31, 0.24, 0.20, 0.12, 0.10, 0.09, 0.14, 0.11, 0.07];
    let samples = 90;
    let mut events = Vec::new();
    for s in 0..samples {
        let kras = r.u() < rates[1];
        for (g, &rate) in rates.iter().enumerate() {
            // BRAF and KRAS mutations rarely together (mutual exclusivity).
            let rate = if g == 6 && kras { rate * 0.1 } else if g == 1 { if kras { 1.0 } else { 0.0 } } else { rate };
            if r.u() < rate {
                let kind = match g {
                    7 => Alteration::Deletion,
                    8 | 9 => Alteration::Amplification,
                    0 if r.u() < 0.3 => Alteration::Truncating,
                    3 if r.u() < 0.7 => Alteration::Truncating,
                    5 if r.u() < 0.3 => Alteration::Amplification,
                    _ if r.u() < 0.03 => Alteration::Fusion,
                    _ => Alteration::Missense,
                };
                events.push((g, s, kind));
            }
        }
    }
    let ax = fig.at(1, 2);
    ax.title("Oncoprint: 90 tumours");
    ax.oncoprint(&genes, samples, &events);

    // TP53 mutations along the protein (393 aa).
    let mut muts = vec![
        (175.0, 48.0, "R175H".to_string(), Alteration::Missense),
        (248.0, 41.0, "R248Q".into(), Alteration::Missense),
        (273.0, 38.0, "R273H".into(), Alteration::Missense),
        (245.0, 22.0, "G245S".into(), Alteration::Missense),
        (249.0, 20.0, "R249S".into(), Alteration::Missense),
        (282.0, 18.0, "R282W".into(), Alteration::Missense),
        (220.0, 14.0, "Y220C".into(), Alteration::Missense),
        (213.0, 12.0, "R213*".into(), Alteration::Truncating),
        (196.0, 9.0, "R196*".into(), Alteration::Truncating),
    ];
    for _ in 0..70 {
        let p = (r.u() * 393.0).round();
        let trunc = r.u() < 0.3;
        muts.push((p, (1.0 + r.u() * 5.0).round(), format!("p{p}"), if trunc { Alteration::Truncating } else { Alteration::Missense }));
    }
    let ax = fig.span(2, 0, 1, 3);
    ax.title("TP53 mutations along the protein");
    ax.mutation_lollipop(
        393.0,
        &[("TAD", 1.0, 61.0), ("PRD", 64.0, 92.0), ("DNA-binding", 102.0, 292.0), ("Tet", 325.0, 356.0)],
        &muts,
    );
    ax.legend(LegendPosition::UpperRight);
    fig.save("genomics.png")
}
