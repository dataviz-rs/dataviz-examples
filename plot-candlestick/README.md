# Candlestick chart

Daily candlesticks on a time axis with a 10-day moving average.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/candlestick.png" alt="Candlestick chart" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-candlestick
cargo run --release
```

Writes `candlestick.png` and `candlestick.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
