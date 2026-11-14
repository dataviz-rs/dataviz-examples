# Interactive window and live data

`Figure::show` opens a window: mouse wheel zooms around the cursor, left-drag pans, right-drag zooms to a box, clicking a legend row hides or shows its series, `R` resets, `C` toggles the crosshair (cursor values on the axes and the series values at that x), `H` toggles hover tooltips, `S` saves a PNG, `Esc` closes. Plots showing the same x range get a linked cursor and are zoomed and panned together. `Figure::show_live` then streams data into a rolling window.

<img src="./screenshots/interactive.png" alt="Interactive window and live data" width="400">

The screenshot shows the initial view; zoom, pan and the hover tooltip happen in the window.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-interactive
cargo run --release
```

## License

MIT
