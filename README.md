# dataviz examples

Runnable examples for [dataviz](https://github.com/dataviz-rs/dataviz)
([crates.io](https://crates.io/crates/dataviz), [docs](https://docs.rs/dataviz)),
a Rust library for bar, line, scatter, area, pie and histogram charts with PNG
and SVG output and interactive windows.

## Running an example

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/svgquadrant1graph
cargo run
```

Every example writes its output (PNG or SVG) to the current directory; the
`*display` examples also open a window (`C`: hover tooltip, `H`: key hints,
`Esc`: next window / quit). No font file is needed: an installed sans-serif
font is used unless `FigureConfig::font_label` / `font_title` point to one.

The repository is a Cargo workspace, so `cargo build --workspace` builds all
of them at once.

Linux needs the usual windowing libraries for the display examples, e.g.
`sudo apt-get install libxkbcommon-dev libwayland-dev`.

## Examples

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
