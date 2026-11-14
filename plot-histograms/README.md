# Histograms, KDE and ECDF

Overlapping density histograms with a KDE line; cumulative step histogram and ECDF.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/histograms.png" alt="Histograms, KDE and ECDF" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-histograms
cargo run --release
```

Writes `histograms.png` and `histograms.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
