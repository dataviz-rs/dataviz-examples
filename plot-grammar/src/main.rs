//! Grammar of graphics: charts described by data, a mark and encodings
//! (as in Vega-Lite, ggplot2 and Altair) with `dataviz::plot::grammar`.

use dataviz::plot::grammar::{Aggregate, Chart, Data, Field, Mark};
use dataviz::plot::series::bar::BarMode;
use dataviz::prelude::*;

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

/// A penguin-like table: species, island, sex, flipper length, body
/// mass and bill length.
fn penguins() -> Data {
    let mut r = Rng(11);
    let kinds = [
        ("Adelie", 190.0, 3700.0, 38.8, ["Torgersen", "Biscoe", "Dream"]),
        ("Chinstrap", 196.0, 3730.0, 48.8, ["Dream", "Dream", "Dream"]),
        ("Gentoo", 217.0, 5076.0, 47.5, ["Biscoe", "Biscoe", "Biscoe"]),
    ];
    let (mut sp, mut isl, mut sex, mut fl, mut mass, mut bill) =
        (vec![], vec![], vec![], vec![], vec![], vec![]);
    for (name, f0, m0, b0, islands) in kinds {
        for i in 0..110 {
            let f = f0 + 6.5 * r.normal();
            let male = i % 2 == 0;
            sp.push(name);
            isl.push(islands[i % 3]);
            sex.push(if male { "male" } else { "female" });
            fl.push((f * 10.0).round() / 10.0);
            mass.push(m0 + (f - f0) * 45.0 + if male { 280.0 } else { -280.0 } + 220.0 * r.normal());
            bill.push(b0 + if male { 1.4 } else { -1.4 } + 2.2 * r.normal());
        }
    }
    Data::new()
        .text("species", sp)
        .text("island", isl)
        .text("sex", sex)
        .num("flipper (mm)", fl)
        .num("body mass (g)", mass)
        .num("bill (mm)", bill)
}

fn main() -> Result<(), Error> {
    let data = penguins();

    // Four charts from one table: only the encodings differ.
    let mut fig = Figure::new(1200, 860);
    fig.title("One table, four encodings");
    fig.grid(2, 2);
    Chart::new(data.clone(), Mark::Point)
        .x("flipper (mm)")
        .y("body mass (g)")
        .color("species")
        .size("bill (mm)")
        .title("point: x, y, color, size")
        .draw_into(fig.at(0, 0))
        .map_err(Error::Render)?;
    Chart::new(data.clone(), Mark::Bar)
        .x("island")
        .y(Field::new("body mass (g)").aggregate(Aggregate::Mean))
        .color("species")
        .stack(BarMode::Grouped)
        .title("bar: mean per island, grouped by species")
        .draw_into(fig.at(0, 1))
        .map_err(Error::Render)?;
    Chart::new(data.clone(), Mark::Bar)
        .x(Field::new("body mass (g)").bin(30))
        .color("species")
        .title("bar with a binned x: histograms")
        .draw_into(fig.at(1, 0))
        .map_err(Error::Render)?;
    Chart::new(data.clone(), Mark::Boxplot)
        .x("species")
        .y("flipper (mm)")
        .title("boxplot")
        .draw_into(fig.at(1, 1))
        .map_err(Error::Render)?;
    fig.save("grammar.png")?;

    // Small multiples: a facet per species, colors the same in each.
    let mut fig = Figure::new(1200, 430);
    Chart::new(data.clone(), Mark::Point)
        .x("bill (mm)")
        .y("body mass (g)")
        .color("sex")
        .facet("species", 3)
        .title("facet: one plot per species")
        .draw(&mut fig)
        .map_err(Error::Render)?;
    fig.save("grammar_facets.png")?;

    // The same as a JSON spec (what Python, JavaScript, C and the MCP
    // server send), timed on a million rows.
    let n = 1_000_000;
    let mut r = Rng(5);
    let x: Vec<f64> = (0..n).map(|_| r.normal()).collect();
    let y: Vec<f64> = x.iter().map(|v| v * 0.6 + 0.8 * r.normal()).collect();
    let g: Vec<String> = (0..n).map(|i| ["a", "b", "c"][i % 3].to_string()).collect();
    let big = Data::new().num("x", x).num("y", y).text("group", g);
    let t = std::time::Instant::now();
    let mut fig = Figure::new(800, 600);
    Chart::new(big, Mark::Point)
        .x("x")
        .y("y")
        .color("group")
        .draw(&mut fig)
        .map_err(Error::Render)?;
    let built = t.elapsed();
    fig.save("grammar_million.png")?;
    println!(
        "1 000 000 rows in 3 color groups: built in {:.1} ms, built and saved in {:.1} ms",
        built.as_secs_f64() * 1e3,
        t.elapsed().as_secs_f64() * 1e3
    );
    Ok(())
}
