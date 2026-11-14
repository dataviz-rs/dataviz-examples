# Interactive window and live data

`Figure::show` opens a window: mouse wheel zooms around the cursor, left-drag pans, right-drag zooms to a box, `R` resets, `H` toggles hover tooltips, `S` saves a PNG, `Esc` closes. `Figure::show_live` then streams data into a rolling window.

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
