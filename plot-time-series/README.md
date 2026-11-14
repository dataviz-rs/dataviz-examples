# Time series

Hourly data on a time axis, bars on a secondary y-axis, reference line and shaded span.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/time_series.png" alt="Time series" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-time-series
cargo run --release
```

Writes `time_series.png` and `time_series.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
