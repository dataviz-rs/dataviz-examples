//! dataviz in the browser: the chart is rendered by Rust compiled to
//! WebAssembly into an RGBA buffer that `www/index.html` puts on a canvas.
//! Wheel zoom, drag pan, double-click reset, hover readouts and legend
//! clicks call into Rust; new data streams in every frame.
//!
//! The functions use the plain C ABI (no wasm-bindgen): numbers in, a
//! pointer to the pixels (or to UTF-8 text) out.

use dataviz::plot::series::line::Line;
use dataviz::prelude::*;
use std::cell::RefCell;

struct App {
    fig: Figure,
    px: Pixels,
    t: f64,
    text: String,
}

thread_local! {
    static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}

fn with<R>(f: impl FnOnce(&mut App) -> R) -> Option<R> {
    APP.with(|a| a.borrow_mut().as_mut().map(f))
}

/// Builds the chart for a `width` × `height` canvas.
#[no_mangle]
pub extern "C" fn init(width: u32, height: u32) {
    let mut fig = Figure::new(width, height);
    fig.title("Streaming in the browser (Rust → WebAssembly)");
    let ax = fig.axes();
    ax.line(Vec::<f64>::new(), Vec::<f64>::new()).label("sensor A");
    ax.line(Vec::<f64>::new(), Vec::<f64>::new()).label("sensor B");
    ax.x_label("time [s]").y_label("value");
    ax.legend(LegendPosition::UpperLeft);
    APP.with(|a| {
        *a.borrow_mut() = Some(App {
            fig,
            px: Pixels::default(),
            t: 0.0,
            text: String::new(),
        })
    });
}

/// Adds `n` new samples to both lines (a rolling window of 3000).
#[no_mangle]
pub extern "C" fn step(n: u32) {
    with(|app| {
        for _ in 0..n {
            app.t += 0.01;
            let t = app.t;
            let a = (t * 1.7).sin() + 0.3 * (t * 9.0).sin();
            let b = (t * 0.6).cos() * 0.8 + 0.15 * (t * 23.0).sin();
            let ax = app.fig.axes();
            for (k, v) in [(0, a), (1, b)] {
                let line = ax.series_mut::<Line>(k).expect("line");
                line.push(t, v);
                line.keep_last(3000);
            }
        }
    });
}

/// Renders the chart; returns a pointer to `width * height * 4` RGBA bytes.
#[no_mangle]
pub extern "C" fn render() -> *const u8 {
    with(|app| {
        let _ = app.fig.render_into(&mut app.px);
        app.px.rgba.as_ptr()
    })
    .unwrap_or(std::ptr::null())
}

/// Wheel at canvas `(x, y)`: `delta` < 0 zooms in.
#[no_mangle]
pub extern "C" fn wheel(x: f64, y: f64, delta: f64) {
    with(|app| app.fig.zoom_at(x, y, if delta < 0.0 { 0.85 } else { 1.0 / 0.85 }));
}

/// Drag from `(x, y)` by `(dx, dy)` canvas pixels.
#[no_mangle]
pub extern "C" fn drag(x: f64, y: f64, dx: f64, dy: f64) {
    with(|app| app.fig.pan_by(x, y, dx, dy));
}

/// Back to following the data.
#[no_mangle]
pub extern "C" fn reset() {
    with(|app| app.fig.reset_view());
}

/// A click: legend rows hide or show their series.
#[no_mangle]
pub extern "C" fn click(x: f64, y: f64) {
    with(|app| app.fig.click(x, y));
}

/// Hover text at `(x, y)`: its length in bytes (0 = none); read it from
/// `text_ptr()`.
#[no_mangle]
pub extern "C" fn hover(x: f64, y: f64) -> u32 {
    with(|app| {
        app.text = app.fig.hover(x, y).map(|h| h.text).unwrap_or_default();
        app.text.len() as u32
    })
    .unwrap_or(0)
}

/// The text of the last `hover`.
#[no_mangle]
pub extern "C" fn text_ptr() -> *const u8 {
    with(|app| app.text.as_ptr()).unwrap_or(std::ptr::null())
}
