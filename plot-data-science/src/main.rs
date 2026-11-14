//! Charts for data scientists: ROC and precision-recall curves, a confusion
//! matrix, regression with a confidence band, 2D density and a dendrogram;
//! model evaluation: SHAP summary, calibration, learning curve, gain,
//! raincloud and UpSet (`evaluation.png`).

use dataviz::prelude::*;
/// Small deterministic random generator for the demo data.
struct R(u64);
impl R {
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
fn main() {
    let mut r = R(5);
    let mut fig = Figure::new(1200, 1100);
    fig.grid(3, 2);
    let labels: Vec<bool> = (0..600).map(|_| r.u() < 0.4).collect();
    let s1: Vec<f64> = labels
        .iter()
        .map(|&l| if l { 0.65 } else { 0.35 } + r.n() * 0.18)
        .collect();
    let s2: Vec<f64> = labels
        .iter()
        .map(|&l| if l { 0.55 } else { 0.45 } + r.n() * 0.2)
        .collect();
    let ax = fig.at(0, 0);
    ax.title("ROC curves");
    ax.roc_curve("gradient boosting", &labels, &s1);
    ax.roc_curve("logistic regression", &labels, &s2);
    ax.legend(LegendPosition::LowerRight);
    let ax = fig.at(0, 1);
    ax.title("Precision-recall");
    ax.pr_curve("gradient boosting", &labels, &s1);
    ax.legend(LegendPosition::LowerLeft);
    let act: Vec<usize> = (0..500).map(|_| (r.u() * 3.0) as usize).collect();
    let pred: Vec<usize> = act
        .iter()
        .map(|&a| {
            if r.u() < 0.8 {
                a
            } else {
                (r.u() * 3.0) as usize
            }
        })
        .collect();
    fig.at(1, 0).title("Confusion matrix").confusion_matrix(
        ["cat", "dog", "bird"],
        &act,
        &pred,
        true,
    );
    let x: Vec<f64> = (0..250).map(|_| r.u() * 10.0).collect();
    let y: Vec<f64> = x.iter().map(|v| 1.5 * v + 3.0 + r.n() * 3.0).collect();
    let ax = fig.at(1, 1);
    ax.title("Regression with 95 % band");
    ax.regplot(&x, &y);
    ax.legend(LegendPosition::UpperLeft);
    let a: Vec<f64> = (0..20000)
        .map(|i| if i % 2 == 0 { r.n() } else { 3.0 + r.n() * 0.7 })
        .collect();
    let b: Vec<f64> = a.iter().map(|v| v * 0.5 + r.n()).collect();
    let ax = fig.at(2, 0);
    ax.title("2D density (20 000 points)");
    ax.density_contour(&a, &b).colorbar("density");
    let names = [
        "Ankara", "Berlin", "Cairo", "Delhi", "Oslo", "Paris", "Rome", "Tokyo",
    ];
    let data: Vec<Vec<f64>> = names
        .iter()
        .enumerate()
        .map(|(i, _)| vec![(i % 3) as f64 * 5.0 + r.u(), (i % 2) as f64 * 3.0 + r.u()])
        .collect();
    let ax = fig.at(2, 1);
    ax.title("Dendrogram");
    ax.dendrogram(names, &data);
    fig.title("Model evaluation and exploration");
    fig.save("data_science.png").unwrap();
    fig.save("data_science.svg").unwrap();
    evaluation();
}

/// Model evaluation and explanation: SHAP summary, calibration, learning
/// curve, cumulative gain and lift, raincloud and UpSet.
fn evaluation() {
    let mut r = R(42);
    // A credit model: 2 000 customers, 6 features, a logistic score.
    let n = 2000;
    let names = ["income", "debt ratio", "age", "late payments", "credit lines", "tenure"];
    let weights = [-1.2, 1.6, -0.5, 2.1, 0.4, -0.7];
    let features: Vec<Vec<f64>> = (0..6).map(|_| (0..n).map(|_| r.n()).collect()).collect();
    let shap: Vec<Vec<f64>> = (0..6)
        .map(|f| features[f].iter().map(|v| weights[f] * v * (1.0 + 0.2 * r.n())).collect())
        .collect();
    let logit: Vec<f64> = (0..n).map(|i| (0..6).map(|f| shap[f][i]).sum::<f64>() * 0.6 - 1.0).collect();
    let prob: Vec<f64> = logit.iter().map(|z| 1.0 / (1.0 + (-z).exp())).collect();
    let labels: Vec<bool> = prob.iter().map(|p| r.u() < *p).collect();
    // An over-confident second model for the calibration plot.
    let sharp: Vec<f64> = logit.iter().map(|z| 1.0 / (1.0 + (-2.2 * z).exp())).collect();

    let mut fig = Figure::new(1500, 980);
    fig.grid(2, 3);
    fig.at(0, 0).title("SHAP summary: what drives default risk").shap_summary(&names, &shap, &features, 10);
    let ax = fig.at(0, 1);
    ax.title("Calibration");
    ax.calibration_curve("logistic", &labels, &prob, 10);
    ax.calibration_curve("boosted (over-confident)", &labels, &sharp, 10);
    ax.legend(LegendPosition::UpperLeft);
    let sizes = [100.0, 250.0, 500.0, 1000.0, 1500.0, 2000.0];
    let train: Vec<Vec<f64>> = sizes.iter().map(|s| (0..5).map(|_| 0.99 - 0.12 * (s / 2000.0) + 0.01 * r.n()).collect()).collect();
    let val: Vec<Vec<f64>> = sizes.iter().map(|s| (0..5).map(|_| 0.86 - 0.22 * (-s / 500.0f64).exp() + 0.015 * r.n()).collect()).collect();
    fig.at(0, 2).title("Learning curve").learning_curve(sizes, &train, &val);
    let ax = fig.at(1, 0);
    ax.title("Cumulative gain");
    ax.gain_curve("logistic", &labels, &prob);
    ax.legend(LegendPosition::LowerRight);
    let ax = fig.at(1, 1);
    ax.title("Raincloud: response time by server (ms)");
    let lat = |m: f64, s: f64, r: &mut R| -> Vec<f64> { (0..300).map(|_| (m + s * r.n()).max(1.0) + if r.u() < 0.05 { 60.0 * r.u() } else { 0.0 }).collect() };
    let (a, b, c) = (lat(42.0, 6.0, &mut r), lat(55.0, 12.0, &mut r), lat(38.0, 4.0, &mut r));
    ax.raincloud([("eu-west", a), ("us-east", b), ("ap-south", c)]);
    ax.x_label("ms");
    let genes = |k: usize, m: usize, r: &mut R| -> Vec<String> { (0..m).map(|_| format!("g{}", (r.u() * k as f64) as usize)).collect() };
    let sets = [
        ("RNA-seq", genes(900, 700, &mut r)),
        ("ATAC-seq", genes(700, 500, &mut r)),
        ("ChIP-seq", genes(600, 350, &mut r)),
        ("proteomics", genes(500, 250, &mut r)),
    ];
    fig.at(1, 2).title("UpSet: genes found by each assay").upset(&sets, 12);
    fig.save("evaluation.png").unwrap();
}
