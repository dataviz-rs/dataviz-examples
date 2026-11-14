//! Accessibility: a text alternative for screen readers, color warnings,
//! and the chart as seen with color vision deficiencies — first with red
//! and green series, then with the colour-blind safe Okabe–Ito palette.

use dataviz::prelude::*;

fn chart(colors: [Color; 3], title: &str) -> Figure {
    let mut fig = Figure::new(520, 320);
    fig.title(title);
    let ax = fig.axes();
    let q = [1.0, 2.0, 3.0, 4.0];
    for (k, (name, c)) in ["north", "south", "east"].iter().zip(colors).enumerate() {
        ax.line(q, q.iter().map(|v| 10.0 + v * (k as f64 + 1.0) + (v * 1.7 + k as f64).sin() * 2.0))
            .label(*name)
            .color(c)
            .width(3.0);
    }
    ax.x_label("quarter").y_label("sales [k€]");
    ax.legend(LegendPosition::UpperLeft);
    fig
}

/// The figure as seen normally and with three color vision deficiencies,
/// side by side (2 × 2).
fn as_seen(fig: &mut Figure) -> Result<Pixels, Error> {
    let base = fig.to_pixels()?;
    let (w, h) = (base.width as usize, base.height as usize);
    let mut out = Pixels {
        width: (w * 2) as u32,
        height: (h * 2) as u32,
        rgba: vec![255; w * h * 16],
    };
    for (k, cvd) in [None, Some(Cvd::Deuteranopia), Some(Cvd::Protanopia), Some(Cvd::Tritanopia)]
        .into_iter()
        .enumerate()
    {
        let mut px = base.clone();
        if let Some(c) = cvd {
            px.simulate(c);
        }
        let (ox, oy) = ((k % 2) * w, (k / 2) * h);
        for y in 0..h {
            let src = &px.rgba[y * w * 4..(y + 1) * w * 4];
            let at = ((oy + y) * w * 2 + ox) * 4;
            out.rgba[at..at + w * 4].copy_from_slice(src);
        }
    }
    Ok(out)
}

fn main() -> Result<(), Error> {
    let risky = [Color::rgb(214, 39, 40), Color::rgb(44, 160, 44), Color::rgb(255, 127, 14)];
    let mut fig = chart(risky, "Red / green / orange");
    println!("Alt text: {}\n", fig.alt_text());
    for w in fig.color_warnings() {
        println!("warning: {w}");
    }
    std::fs::write("risky.png", as_seen(&mut fig)?.encode(Format::Png)?)?;
    fig.save("risky.svg")?; // with <title> and <desc> for screen readers

    let safe = Palette::OkabeIto.colors();
    let mut fig = chart([safe[0], safe[1], safe[2]], "Okabe–Ito palette");
    println!("\nwith Okabe–Ito: {} warnings", fig.color_warnings().len());
    std::fs::write("safe.png", as_seen(&mut fig)?.encode(Format::Png)?)?;
    Ok(())
}
