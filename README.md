# dataviz examples

Runnable examples for [dataviz](https://github.com/dataviz-rs/dataviz)
([crates.io](https://crates.io/crates/dataviz), [docs](https://docs.rs/dataviz)),
a pure-Rust plotting library with 40+ chart types, SVG/PNG/JPEG/WebP/PDF
output, interactive windows and terminal output.

## Running an example

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-bars
cargo run --release
```

Every example writes its output (PNG and/or SVG) to the current directory;
the `screenshots/` folder of each example contains that output. The
`*display` and `plot-interactive` examples also open a window. No font file is
needed: an installed sans-serif font is used by default.

The repository is a Cargo workspace, so `cargo build --workspace` builds all
of them at once. Until dataviz 0.2 is published on crates.io, the workspace
builds dataviz from its `main` branch (see `[patch.crates-io]` in
`Cargo.toml`).

Linux needs the usual windowing libraries for the window examples, e.g.
`sudo apt-get install libxkbcommon-dev libwayland-dev`.

## Plot API examples (`dataviz::plot`, 0.2)

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
| [`plot-errorbars-band`](plot-errorbars-band) | Error bars and confidence bands | <img src="plot-errorbars-band/screenshots/errorbars_band.png" width="260"> |
| [`plot-heatmap`](plot-heatmap) | Heatmap (correlation matrix) | <img src="plot-heatmap/screenshots/heatmap.png" width="260"> |
| [`plot-hist2d-hexbin`](plot-hist2d-hexbin) | 2D histogram and hexbin | <img src="plot-hist2d-hexbin/screenshots/hist2d_hexbin.png" width="260"> |
| [`plot-contour`](plot-contour) | Contour plots | <img src="plot-contour/screenshots/contour.png" width="260"> |
| [`plot-candlestick`](plot-candlestick) | Candlestick chart | <img src="plot-candlestick/screenshots/candlestick.png" width="260"> |
| [`plot-time-series`](plot-time-series) | Time series | <img src="plot-time-series/screenshots/time_series.png" width="260"> |
| [`plot-log-scales`](plot-log-scales) | Logarithmic scales | <img src="plot-log-scales/screenshots/log_scales.png" width="260"> |
| [`plot-formats`](plot-formats) | Tick formats | <img src="plot-formats/screenshots/formats.png" width="260"> |
| [`plot-stem-waterfall`](plot-stem-waterfall) | Stem and waterfall charts | <img src="plot-stem-waterfall/screenshots/stem_waterfall.png" width="260"> |
| [`plot-quiver-function`](plot-quiver-function) | Vector field and function plots | <img src="plot-quiver-function/screenshots/quiver_function.png" width="260"> |
| [`plot-gantt`](plot-gantt) | Gantt chart | <img src="plot-gantt/screenshots/gantt.png" width="260"> |
| [`plot-pie-donut`](plot-pie-donut) | Pie and donut charts | <img src="plot-pie-donut/screenshots/pie_donut.png" width="260"> |
| [`plot-polar-radar`](plot-polar-radar) | Polar, radar and rose charts | <img src="plot-polar-radar/screenshots/polar_radar.png" width="260"> |
| [`plot-treemap-sunburst`](plot-treemap-sunburst) | Treemap and sunburst | <img src="plot-treemap-sunburst/screenshots/treemap_sunburst.png" width="260"> |
| [`plot-sankey`](plot-sankey) | Sankey diagram | <img src="plot-sankey/screenshots/sankey.png" width="260"> |
| [`plot-funnel-gauge`](plot-funnel-gauge) | Funnel and gauge | <img src="plot-funnel-gauge/screenshots/funnel_gauge.png" width="260"> |
| [`plot-network`](plot-network) | Network graph | <img src="plot-network/screenshots/network.png" width="260"> |
| [`plot-parallel-calendar`](plot-parallel-calendar) | Parallel coordinates and calendar heatmap | <img src="plot-parallel-calendar/screenshots/parallel_calendar.png" width="260"> |
| [`plot-three-d`](plot-three-d) | 3D surface and trajectory | <img src="plot-three-d/screenshots/three_d.png" width="260"> |
| [`plot-subplots-shared`](plot-subplots-shared) | Subplots with a shared x-axis | <img src="plot-subplots-shared/screenshots/subplots_shared.png" width="260"> |
| [`plot-annotations`](plot-annotations) | Annotations | <img src="plot-annotations/screenshots/annotations.png" width="260"> |
| [`plot-image`](plot-image) | Image display | <img src="plot-image/screenshots/image.png" width="260"> |
| [`plot-themes`](plot-themes) | Every built-in theme | <img src="plot-themes/screenshots/theme_dark.png" width="260"> |
| [`plot-interactive`](plot-interactive) | Interactive window (zoom, pan, hover) and live data | <img src="plot-interactive/screenshots/interactive.png" width="260"> |
| [`plot-terminal`](plot-terminal) | Charts in the terminal (half blocks / braille) | [terminal.txt](plot-terminal/screenshots/terminal.txt) |

## Classic API examples (`dataviz::figure`)


| Example | What it shows | Output |
|---|---|---|
| [`pixelareachart`](pixelareachart) | Area chart → PNG | <img src="pixelareachart/screenshots/area_chart.png" width="220"> |
| [`pixelareachartdisplay`](pixelareachartdisplay) | Area chart in an interactive / real-time window | <img src="pixelareachartdisplay/screenshots/areachartdisplay.png" width="220"> |
| [`pixelcartesiangraph`](pixelcartesiangraph) | Cartesian graph (solid, dashed, dotted lines) → PNG | <img src="pixelcartesiangraph/screenshots/cartesian_graph.png" width="220"> |
| [`pixelcartesiandisplay`](pixelcartesiandisplay) | Cartesian graph in an interactive / real-time window | <img src="pixelcartesiandisplay/screenshots/cartesiangraphdisplay.png" width="220"> |
| [`pixelgroupedbarchartvertical`](pixelgroupedbarchartvertical) | Grouped vertical bar chart → PNG | <img src="pixelgroupedbarchartvertical/screenshots/grouped_vertical_bar_chart.png" width="220"> |
| [`pixelgroupedbarcharthorizontal`](pixelgroupedbarcharthorizontal) | Grouped horizontal bar chart → PNG | <img src="pixelgroupedbarcharthorizontal/screenshots/grouped_horizontal_bar_chart.png" width="220"> |
| [`pixelgroupedbarchartfigurefactory`](pixelgroupedbarchartfigurefactory) | Bar chart created with `FigureFactory` → PNG | <img src="pixelgroupedbarchartfigurefactory/screenshots/grouped_vertical_bar_chart.png" width="220"> |
| [`pixelgroupedbarchartdisplay`](pixelgroupedbarchartdisplay) | Grouped bar chart in an interactive / real-time window | <img src="pixelgroupedbarchartdisplay/screenshots/interactivedisplay.png" width="220"> |
| [`pixelhistogram`](pixelhistogram) | Histogram → PNG | <img src="pixelhistogram/screenshots/histogram.png" width="220"> |
| [`pixelhistogramdisplay`](pixelhistogramdisplay) | Histogram in an interactive / real-time window | <img src="pixelhistogramdisplay/screenshots/histogramdisplay.png" width="220"> |
| [`pixelpiechartdisplay`](pixelpiechartdisplay) | Pie chart → PNG + real-time window | <img src="pixelpiechartdisplay/screenshots/piechart.png" width="220"> |
| [`pixelquadrant1graphdisplay`](pixelquadrant1graphdisplay) | Quadrant 1 graph in an interactive / real-time window | <img src="pixelquadrant1graphdisplay/screenshots/quadrant1graph.png" width="220"> |
| [`pixelscattergraphdisplay`](pixelscattergraphdisplay) | Scatter graph → PNG + real-time window | <img src="pixelscattergraphdisplay/screenshots/scatter_graph.png" width="220"> |
| [`svgareachart`](svgareachart) | Area chart → SVG | <img src="svgareachart/screenshots/area_chart.svg" width="220"> |
| [`svgcartesiangraph`](svgcartesiangraph) | Cartesian graph → SVG | <img src="svgcartesiangraph/screenshots/cartesian_graph.svg" width="220"> |
| [`svggroupedbarchartvertical`](svggroupedbarchartvertical) | Grouped vertical bar chart → SVG | <img src="svggroupedbarchartvertical/screenshots/grouped_vertical_bar_chart.svg" width="220"> |
| [`svggroupedbarcharthorizontal`](svggroupedbarcharthorizontal) | Grouped horizontal bar chart → SVG | <img src="svggroupedbarcharthorizontal/screenshots/grouped_horizontal_bar_chart.svg" width="220"> |
| [`svghistogram`](svghistogram) | Histogram → SVG | <img src="svghistogram/screenshots/histogram.svg" width="220"> |
| [`svgpiechart`](svgpiechart) | Pie chart → SVG | <img src="svgpiechart/screenshots/pie_chart.svg" width="220"> |
| [`svgquadrant1graph`](svgquadrant1graph) | Quadrant 1 graph, all line styles, data-point markers → SVG | <img src="svgquadrant1graph/screenshots/quadrant1_graph_markers.svg" width="220"> |
| [`svgscattergraph`](svgscattergraph) | Scatter graph with every dot type → SVG | <img src="svgscattergraph/screenshots/scatter_graph.svg" width="220"> |

## License

MIT, see [LICENSE](LICENSE).
