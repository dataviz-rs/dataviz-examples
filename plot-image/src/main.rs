//! Image display: An RGBA image (Mandelbrot set) shown on axes.

use dataviz::prelude::*;

fn main() -> Result<(), Error> {
    let mut fig = Figure::new(1000, 620);
    let ax = fig.axes();
    ax.title("Image (Mandelbrot)");
    let (w, h) = (300u32, 200u32);
    let mut px = Vec::with_capacity((w * h * 4) as usize);
    for j in 0..h {
        for i in 0..w {
            let (cx, cy) = (
                -2.2 + 3.2 * i as f64 / w as f64,
                -1.0 + 2.0 * j as f64 / h as f64,
            );
            let (mut x, mut y, mut k) = (0.0, 0.0, 0);
            while x * x + y * y < 4.0 && k < 60 {
                (x, y) = (x * x - y * y + cx, 2.0 * x * y + cy);
                k += 1;
            }
            let c = Colormap::Magma.at(k as f64 / 60.0);
            px.extend_from_slice(&[c.r, c.g, c.b, 255]);
        }
    }
    ax.image(w, h, px).smooth(true);

    fig.save("image.png")?;
    fig.save("image.svg")?;
    Ok(())
}
