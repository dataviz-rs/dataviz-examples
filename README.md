# dataviz examples

Runnable examples for [dataviz](https://github.com/dataviz-rs/dataviz)
([crates.io](https://crates.io/crates/dataviz), [docs](https://docs.rs/dataviz)),
a pure-Rust plotting library with 40+ chart types, SVG/PNG/JPEG/WebP/PDF
output, interactive and live windows, terminal output and CPU / multi-core /
GPU rendering. The examples always use the latest dataviz release.

## Running an example

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-bars
cargo run --release
```

Every example writes its output (PNG and/or SVG) to the current directory;
the `screenshots/` folder of each example contains that output. The
`plot-interactive` and `plot-streaming` examples open a window
(`plot-streaming --record out.gif` works without one). No font file is needed: the
font is built into dataviz.

The repository is a Cargo workspace, so `cargo build --workspace` builds all
of them at once.

Linux needs the usual windowing libraries for the window examples, e.g.
`sudo apt-get install libxkbcommon-dev libwayland-dev`.

## Benchmarks

[`benchmarks/`](benchmarks) compares dataviz with plotters, ruviz, kuva and charts-rs
(static, big data, real time, animation); results in
[benchmarks/RESULTS.md](benchmarks/RESULTS.md), rerun with `python3 run_all.py`.

## Examples

| Example | What it shows | Output |
|---|---|---|
| [`plot-line`](plot-line) | Line styles | <img src="plot-line/screenshots/line.png" width="260"> |
| [`plot-markers-steps-smooth`](plot-markers-steps-smooth) | Markers, steps and smoothing | <img src="plot-markers-steps-smooth/screenshots/markers_steps_smooth.png" width="260"> |
| [`plot-area-fill`](plot-area-fill) | Area and fill between | <img src="plot-area-fill/screenshots/area_fill.png" width="260"> |
| [`plot-stacked-area`](plot-stacked-area) | Stacked areas | <img src="plot-stacked-area/screenshots/stacked_area.png" width="260"> |
| [`plot-scatter-bubble`](plot-scatter-bubble) | Scatter and bubble charts | <img src="plot-scatter-bubble/screenshots/scatter_bubble.png" width="260"> |
| [`plot-bars`](plot-bars) | Bar charts | <img src="plot-bars/screenshots/bars.png" width="260"> |
| [`plot-histograms`](plot-histograms) | Histograms, KDE and ECDF | <img src="plot-histograms/screenshots/histograms.png" width="260"> |
| [`plot-box-violin`](plot-box-violin) | Box and violin plots | <img src="plot-box-violin/screenshots/box_violin.png" width="260"> |
| [`plot-distributions`](plot-distributions) | Strip, swarm, ridgeline and Q-Q plots | <img src="plot-distributions/screenshots/distributions.png" width="260"> |
| [`plot-facets`](plot-facets) | Small multiples and pair plots | <img src="plot-facets/screenshots/facets.png" width="260"> |
| [`plot-errorbars-band`](plot-errorbars-band) | Error bars and confidence bands | <img src="plot-errorbars-band/screenshots/errorbars_band.png" width="260"> |
| [`plot-heatmap`](plot-heatmap) | Heatmap (correlation matrix) | <img src="plot-heatmap/screenshots/heatmap.png" width="260"> |
| [`plot-hist2d-hexbin`](plot-hist2d-hexbin) | 2D histogram and hexbin | <img src="plot-hist2d-hexbin/screenshots/hist2d_hexbin.png" width="260"> |
| [`plot-contour`](plot-contour) | Contour plots | <img src="plot-contour/screenshots/contour.png" width="260"> |
| [`plot-data-science`](plot-data-science) | ROC/PR curves, confusion matrix, regression band, density, dendrogram | <img src="plot-data-science/screenshots/data_science.png" width="260"> |
| [`plot-engineering`](plot-engineering) | FFT spectrum, spectrogram, Bode, Nyquist, Smith chart | <img src="plot-engineering/screenshots/engineering.png" width="260"> |
| [`plot-trading`](plot-trading) | Ichimoku, VWAP, volume profile, depth, stochastic, ATR, drawdown, Renko | <img src="plot-trading/screenshots/trading.png" width="260"> |
| [`plot-candlestick`](plot-candlestick) | Candlestick chart | <img src="plot-candlestick/screenshots/candlestick.png" width="260"> |
| [`plot-indicators`](plot-indicators) | Bollinger bands, RSI, MACD, Heikin-Ashi | <img src="plot-indicators/screenshots/indicators.png" width="260"> |
| [`plot-time-series`](plot-time-series) | Time series | <img src="plot-time-series/screenshots/time_series.png" width="260"> |
| [`plot-log-scales`](plot-log-scales) | Logarithmic scales | <img src="plot-log-scales/screenshots/log_scales.png" width="260"> |
| [`plot-broken-axis`](plot-broken-axis) | Broken axes | <img src="plot-broken-axis/screenshots/broken_axis.png" width="260"> |
| [`plot-formats`](plot-formats) | Tick formats | <img src="plot-formats/screenshots/formats.png" width="260"> |
| [`plot-stem-waterfall`](plot-stem-waterfall) | Stem and waterfall charts | <img src="plot-stem-waterfall/screenshots/stem_waterfall.png" width="260"> |
| [`plot-quiver-function`](plot-quiver-function) | Vector field and function plots | <img src="plot-quiver-function/screenshots/quiver_function.png" width="260"> |
| [`plot-gantt`](plot-gantt) | Gantt chart | <img src="plot-gantt/screenshots/gantt.png" width="260"> |
| [`plot-pie-donut`](plot-pie-donut) | Pie and donut charts | <img src="plot-pie-donut/screenshots/pie_donut.png" width="260"> |
| [`plot-polar-radar`](plot-polar-radar) | Polar, radar and rose charts | <img src="plot-polar-radar/screenshots/polar_radar.png" width="260"> |
| [`plot-treemap-sunburst`](plot-treemap-sunburst) | Treemap and sunburst | <img src="plot-treemap-sunburst/screenshots/treemap_sunburst.png" width="260"> |
| [`plot-maps`](plot-maps) | World choropleth, routes, projections | <img src="plot-maps/screenshots/maps.png" width="260"> |
| [`plot-sankey`](plot-sankey) | Sankey diagram | <img src="plot-sankey/screenshots/sankey.png" width="260"> |
| [`plot-funnel-gauge`](plot-funnel-gauge) | Funnel and gauge | <img src="plot-funnel-gauge/screenshots/funnel_gauge.png" width="260"> |
| [`plot-network`](plot-network) | Network graph | <img src="plot-network/screenshots/network.png" width="260"> |
| [`plot-parallel-calendar`](plot-parallel-calendar) | Parallel coordinates and calendar heatmap | <img src="plot-parallel-calendar/screenshots/parallel_calendar.png" width="260"> |
| [`plot-three-d`](plot-three-d) | 3D surface and trajectory | <img src="plot-three-d/screenshots/three_d.png" width="260"> |
| [`plot-subplots-shared`](plot-subplots-shared) | Subplots with a shared x-axis | <img src="plot-subplots-shared/screenshots/subplots_shared.png" width="260"> |
| [`plot-annotations`](plot-annotations) | Annotations | <img src="plot-annotations/screenshots/annotations.png" width="260"> |
| [`plot-image`](plot-image) | Image display | <img src="plot-image/screenshots/image.png" width="260"> |
| [`plot-themes`](plot-themes) | Every built-in theme | <img src="plot-themes/screenshots/theme_dark.png" width="260"> |
| [`plot-interactive`](plot-interactive) | Interactive window (zoom, pan, hover) and live data | <img src="plot-interactive/screenshots/interactive.gif" width="260"> |
| [`plot-egui`](plot-egui) | Embedded in an egui app: texture + zoom/pan/hover API | <img src="plot-egui/screenshots/egui.png" width="260"> |
| [`plot-html`](plot-html) | Interactive HTML: hover, crosshair, legend toggle | <img src="plot-html/screenshots/html.png" width="260"> |
| [`plot-wasm`](plot-wasm) | In the browser: Rust → WebAssembly, streaming, zoom, hover | <img src="plot-wasm/screenshots/wasm.png" width="260"> |
| [`plot-accessibility`](plot-accessibility) | Alt text, color vision simulation and checks | <img src="plot-accessibility/screenshots/risky.png" width="260"> |
| [`plot-international`](plot-international) | Text in any language: CJK, Arabic, Hebrew, Hindi, Thai, emoji | <img src="plot-international/screenshots/international.png" width="260"> |
| [`plot-math`](plot-math) | Math notation in labels, legends and annotations | <img src="plot-math/screenshots/math.png" width="260"> |
| [`plot-animation`](plot-animation) | Animations: GIF and animated PNG | <img src="plot-animation/screenshots/animation.gif" width="260"> |
| [`plot-streaming`](plot-streaming) | Live dashboard for streaming data (simulated sensors, CPU/memory, stdin) | <img src="plot-streaming/screenshots/streaming.gif" width="260"> |
| [`plot-binance`](plot-binance) | Live Binance BTC/USDT dashboard: candles, volume, trades, order book, buy/sell pressure | <img src="plot-binance/screenshots/binance.png" width="260"> |
| [`plot-terminal`](plot-terminal) | Charts in the terminal (half blocks / braille) | [terminal.txt](plot-terminal/screenshots/terminal.txt) |
| [`plot-backends`](plot-backends) | CPU, multi-core and GPU rendering with timings | <img src="plot-backends/screenshots/backends.png" width="260"> |

## License

MIT, see [LICENSE](LICENSE).
