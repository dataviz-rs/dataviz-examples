//! Maps: a world choropleth of population density, flight routes along
//! great circles, and the same countries in four projections.

use dataviz::plot::geo::{self, Projection};
use dataviz::prelude::*;

/// Area of a country's outline in km² (spherical polygon area, good enough
/// for a demo).
fn area_km2(c: &geo::Country) -> f64 {
    let r = 6371.0f64;
    let mut a = 0.0;
    for poly in c.polygons.iter() {
        for (k, ring) in poly.iter().enumerate() {
            let mut s = 0.0;
            for w in ring.windows(2) {
                let (l1, p1) = (w[0].0.to_radians(), w[0].1.to_radians());
                let (l2, p2) = (w[1].0.to_radians(), w[1].1.to_radians());
                s += (l2 - l1) * (2.0 + p1.sin() + p2.sin());
            }
            let ring_area = (s * r * r / 2.0).abs();
            a += if k == 0 { ring_area } else { -ring_area };
        }
    }
    a
}

fn main() -> Result<(), Error> {
    // Population density per country from the built-in world data.
    let density: Vec<(String, f64)> = geo::world()
        .iter()
        .filter(|c| c.iso3 != "ATA")
        .map(|c| (c.iso3.clone(), c.population / area_km2(c).max(1.0)))
        .collect();

    let mut fig = Figure::new(1100, 600);
    fig.title("Population density and flight routes");
    let ax = fig.axes();
    ax.map(Projection::Robinson);
    ax.choropleth(density)
        .log(true)
        .colormap(Colormap::Magma)
        .graticule(30.0)
        .colorbar("people per km²");
    // Istanbul → New York, Istanbul → Tokyo, Istanbul → Sydney.
    let ist = (28.98, 41.01);
    for (lon, lat) in [(-74.0, 40.7), (139.7, 35.7), (151.2, -33.9)] {
        ax.geo_line([ist.0, lon], [ist.1, lat])
            .color(Color::rgb(0, 200, 255))
            .width(1.6);
    }
    ax.geo_scatter([ist.0, -74.0, 139.7, 151.2], [ist.1, 40.7, 35.7, -33.9])
        .color(Color::rgb(0, 200, 255))
        .size(7.0)
        .label("airports");
    fig.save("maps.png")?;
    fig.save("maps.svg")?;

    // The same map in four projections.
    let mut fig = Figure::new(1100, 640);
    fig.grid(2, 2);
    for (i, p) in [
        Projection::Robinson,
        Projection::NaturalEarth,
        Projection::Equirectangular,
        Projection::Mercator,
    ]
    .into_iter()
    .enumerate()
    {
        let ax = fig.at(i / 2, i % 2);
        ax.title(format!("{p:?}"));
        ax.map(p);
        ax.world_map().graticule(30.0);
    }
    fig.save("projections.png")?;
    Ok(())
}
