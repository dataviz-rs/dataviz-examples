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
    ax.choropleth(density.clone())
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

    // The same map in six projections (Equal Earth and Mollweide keep
    // areas true).
    let mut fig = Figure::new(1100, 900);
    fig.grid(3, 2);
    for (i, p) in [
        Projection::Robinson,
        Projection::NaturalEarth,
        Projection::EqualEarth,
        Projection::Mollweide,
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
    conic_and_polar(&density)?;
    globe()?;
    regions()?;
    Ok(())
}

/// Countries and continents on conic maps (Albers equal-area for the
/// USA, Lambert conformal for Europe) and the poles on azimuthal maps.
fn conic_and_polar(density: &[(String, f64)]) -> Result<(), Error> {
    let mut fig = Figure::new(1100, 900);
    fig.title("Conic and polar projections");
    fig.grid(2, 2);
    let views = [
        ("USA: Albers equal-area conic", Projection::USA, Some(((-126.0, -66.0), (23.0, 50.0)))),
        ("Europe: Lambert conformal conic", Projection::EUROPE, Some(((-12.0, 42.0), (34.0, 71.0)))),
        ("Arctic: stereographic", Projection::Stereographic { lon: 0.0, lat: 90.0 }, None),
        ("Antarctica: azimuthal equal-area", Projection::AzimuthalEqualArea { lon: 0.0, lat: -90.0 }, None),
    ];
    for (i, (title, p, bounds)) in views.into_iter().enumerate() {
        let ax = fig.at(i / 2, i % 2);
        ax.title(title);
        ax.map(p);
        let m = ax.choropleth(density.to_vec()).log(true).colormap(Colormap::Magma).graticule(15.0);
        if i == 1 {
            m.colorbar("people per km²");
        }
        if let Some((lon, lat)) = bounds {
            ax.map_bounds(lon, lat);
        }
    }
    fig.save("conic_polar.png")
}

/// The globe: population on the side facing Istanbul, a flight to New York
/// along the great circle, and a spinning globe as a GIF.
fn globe() -> Result<(), Error> {
    let population = [
        ("CHN", 1412.0), ("IND", 1408.0), ("USA", 333.0), ("IDN", 276.0), ("PAK", 236.0),
        ("NGA", 218.0), ("BRA", 215.0), ("BGD", 171.0), ("RUS", 144.0), ("MEX", 127.0),
        ("ETH", 123.0), ("JPN", 125.0), ("EGY", 111.0), ("PHL", 115.0), ("COD", 99.0),
        ("VNM", 98.0), ("IRN", 88.0), ("TUR", 85.0), ("DEU", 83.0), ("THA", 72.0),
        ("GBR", 67.0), ("FRA", 68.0), ("ITA", 59.0), ("ZAF", 60.0), ("KEN", 54.0),
    ];
    let mut fig = Figure::new(1100, 560);
    fig.grid(1, 2);
    for (i, (lon, lat)) in [(30.0, 35.0), (-75.0, 25.0)].into_iter().enumerate() {
        let ax = fig.at(0, i);
        ax.title(format!("Globe over {lon}°, {lat}°")).globe(lon, lat);
        let m = ax.choropleth(population).log(true).graticule(30.0);
        if i == 1 {
            m.colorbar("population (millions)");
        }
        ax.geo_line([28.98, -73.94], [41.01, 40.67]).color("#d62728").width(2.5);
        ax.geo_scatter([28.98, -73.94], [41.01, 40.67]).color("#d62728").size(8.0);
    }
    fig.save("globe.png")?;

    let mut fig = Figure::new(420, 420);
    fig.axes().globe(0.0, 20.0).world_map().graticule(30.0);
    fig.record("globe_spin.gif", 20.0, 60, |f, k| {
        f.axes().globe_center(k as f64 * 6.0, 20.0);
    })
}

/// States and provinces, flows, hexbin density and a tile basemap.
fn regions() -> Result<(), Error> {
    use dataviz::plot::geo_more::{regions, tile_dir, tile_range};
    let mut fig = Figure::new(1500, 1000);
    fig.grid(2, 3);

    // Provinces of Türkiye and German states, colored by their area (from
    // the outlines).
    let tr: Vec<(String, f64)> = regions("TUR").iter().map(|r| (r.code.clone(), r.area_km2())).collect();
    fig.at(0, 0).title("Provinces of Türkiye by area").region_map("TUR", tr).colormap(Colormap::YlGnBu).colorbar("km²");
    let de: Vec<(String, f64)> = regions("DEU").iter().map(|r| (r.code.clone(), r.area_km2())).collect();
    fig.at(0, 1).title("German states by area").region_map("DEU", de).colormap(Colormap::Greens).colorbar("km²");

    // Illustrative flows between airports (great circles, width by value).
    let cities = [
        ("IST", 28.82, 40.98), ("LHR", -0.45, 51.47), ("JFK", -73.78, 40.64), ("DXB", 55.36, 25.25),
        ("SIN", 103.99, 1.36), ("HND", 139.78, 35.55), ("GRU", -46.47, -23.43), ("JNB", 28.24, -26.13),
    ];
    let pairs = [(0, 1, 9.0), (0, 2, 6.0), (0, 3, 8.0), (3, 4, 7.0), (4, 5, 6.5), (1, 2, 10.0), (2, 6, 4.0), (3, 7, 3.0), (0, 5, 3.5), (1, 4, 5.0)];
    let flows: Vec<(f64, f64, f64, f64, f64)> = pairs.iter().map(|&(a, b, v)| (cities[a].1, cities[a].2, cities[b].1, cities[b].2, v)).collect();
    let names: Vec<String> = pairs.iter().map(|&(a, b, _)| format!("{}–{}", cities[a].0, cities[b].0)).collect();
    let ax = fig.at(0, 2);
    ax.title("Flow map (illustrative)");
    ax.world_map();
    ax.geo_flows(&flows, &names).colormap(Colormap::Plasma);
    ax.geo_scatter(cities.iter().map(|c| c.1), cities.iter().map(|c| c.2)).color("#222").size(5.0);

    // Illustrative events around European cities as hexagonal bins.
    let centers = [(2.35, 48.86), (13.40, 52.52), (12.50, 41.90), (-3.70, 40.42), (28.98, 41.01), (21.01, 52.23), (4.90, 52.37)];
    let (mut lon, mut lat) = (Vec::new(), Vec::new());
    let mut seed = 11u64;
    let mut u = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for k in 0..30_000 {
        let c = centers[k % centers.len()];
        // Gaussian around the city (Box–Muller), wider for a few.
        let s = if k % 5 == 0 { 2.0 } else { 0.6 };
        let (a, b) = (u().max(1e-12), u());
        let r = (-2.0 * a.ln()).sqrt() * s;
        let t = b * std::f64::consts::TAU;
        lon.push(c.0 + r * t.cos() * 1.4);
        lat.push(c.1 + r * t.sin());
    }
    let ax = fig.at(1, 0);
    ax.title("Hexbin density (illustrative)");
    ax.map(dataviz::plot::geo::Projection::Mercator);
    ax.region_map("FRA", Vec::<(&str, f64)>::new());
    for c in ["DEU", "ITA", "ESP", "POL", "NLD", "BEL", "CHE", "AUT", "CZE", "TUR", "GRC", "PRT"] {
        ax.region_map(c, Vec::<(&str, f64)>::new());
    }
    ax.geo_hexbin(&lon, &lat).gridsize(45).colorbar("events (log₁₀)");
    ax.x_limits(-10.0, 32.0);
    ax.y_limits(dataviz::plot::geo::Projection::Mercator.project(0.0, 35.0).1, dataviz::plot::geo::Projection::Mercator.project(0.0, 56.0).1);

    // OpenStreetMap tiles of Istanbul, downloaded once with --fetch-tiles.
    let (lon_r, lat_r, z) = ((28.90, 29.10), (40.98, 41.10), 12);
    if std::env::args().any(|a| a == "--fetch-tiles") {
        for (x, y) in tile_range(lon_r, lat_r, z) {
            let dir = format!("tiles/{z}/{x}");
            let path = format!("{dir}/{y}.png");
            if std::path::Path::new(&path).exists() {
                continue;
            }
            std::fs::create_dir_all(&dir)?;
            // OSM tile policy: identify the application, cache the tiles.
            let ok = std::process::Command::new("curl")
                .args(["-sSfL", "-A", "dataviz-examples/0.1 (+https://github.com/dataviz-rs/dataviz-examples)", "-o", &path])
                .arg(format!("https://tile.openstreetmap.org/{z}/{x}/{y}.png"))
                .status()
                .is_ok_and(|s| s.success());
            if !ok {
                let _ = std::fs::remove_file(&path);
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
    let ax = fig.span(1, 1, 1, 2);
    ax.title("Basemap: OpenStreetMap tiles of Istanbul");
    ax.basemap(lon_r, lat_r, z, tile_dir("tiles"), "© OpenStreetMap contributors").opacity(0.9);
    let spots = [("Hagia Sophia", 28.9802, 41.0086), ("Galata Tower", 28.9741, 41.0256), ("Dolmabahçe", 29.0003, 41.0392), ("Maiden's Tower", 29.0041, 41.0211), ("Kadıköy", 29.0270, 40.9903)];
    ax.geo_scatter(spots.iter().map(|s| s.1), spots.iter().map(|s| s.2)).color("#d62728").size(10.0);
    for s in spots {
        let (x, y) = dataviz::plot::geo::Projection::Mercator.project(s.1, s.2);
        // Label right of the marker (0.006 map units ≈ 10 px here).
        ax.text(x + 0.006, y, s.0).size(12.0).bold(true).anchor(Anchor::Start).background(Color::rgba(255, 255, 255, 0.85));
    }
    fig.save("regions.png")
}
