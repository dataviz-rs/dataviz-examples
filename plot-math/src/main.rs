//! Math notation: `$...$` in titles, labels, legends and annotations is
//! typeset (TeX subset) in PNG, SVG and PDF output alike.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 640);
    fig.title(r"Damped oscillation   $x(t) = A\,e^{-\gamma t}\cos(\omega t + \varphi)$");
    fig.grid(1, 2);

    let t: Vec<f64> = (0..=500).map(|i| i as f64 * 0.02).collect();
    let ax = fig.at(0, 0);
    for (g, w) in [(0.3, 4.0), (0.12, 2.0)] {
        ax.line(&t, t.iter().map(|v| (-g * v).exp() * (w * v).cos()))
            .label(format!(r"$\gamma = {g},\ \omega = {w}\,\mathrm{{s^{{-1}}}}$"));
    }
    ax.line(&t, t.iter().map(|v| (-0.3 * v).exp()))
        .dash(Dash::Dashed)
        .color(Color::rgb(120, 120, 120))
        .label(r"envelope $\pm A\,e^{-\gamma t}$");
    ax.x_label(r"time $t$ [s]")
        .y_label(r"displacement $x / x_0$");
    ax.legend(LegendPosition::UpperRight);

    // A panel of formulas.
    let ax = fig.at(0, 1);
    ax.title("Formulas");
    ax.x_limits(0.0, 10.0).y_limits(0.0, 10.0);
    ax.x_axis().hidden();
    ax.y_axis().hidden();
    for (i, f) in [
        r"$\bar{x} = \frac{1}{N}\sum_{i=1}^{N} x_i$",
        r"$\sigma^2 = \frac{1}{N-1}\sum_{i=1}^{N}(x_i - \bar{x})^2$",
        r"$f(x) = \frac{1}{\sigma\sqrt{2\pi}}\,e^{-\frac{(x-\mu)^2}{2\sigma^2}}$",
        r"$\int_0^\infty e^{-x^2}\,dx = \frac{\sqrt{\pi}}{2}$",
        r"$e^{i\pi} + 1 = 0,\quad \nabla \cdot \vec{E} = \frac{\rho}{\varepsilon_0}$",
        r"$x \in \mathbb{R},\ \lim_{n \to \infty}\left(1 + \frac{1}{n}\right)^n = e$",
    ]
    .iter()
    .enumerate()
    {
        ax.text(0.4, 9.0 - i as f64 * 1.6, *f).size(19.0);
    }

    fig.save("math.png")?;
    fig.save("math.svg")?;
    Ok(())
}
