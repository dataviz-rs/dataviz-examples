# 2D histogram and hexbin

Density of 20 000 points as a 2D histogram and as hexagonal bins.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/hist2d_hexbin.png" alt="2D histogram and hexbin" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-hist2d-hexbin
cargo run --release
```

Writes `hist2d_hexbin.png` and `hist2d_hexbin.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
