//! Charts for data scientists: ROC and precision-recall curves, a confusion
//! matrix, regression with a confidence band, 2D density and a dendrogram.

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
}
