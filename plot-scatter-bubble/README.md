# Scatter and bubble charts

Colormapped scatter with a colorbar and regression line; bubble chart on a log axis.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/scatter_bubble.png" alt="Scatter and bubble charts" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-scatter-bubble
cargo run --release
```

Writes `scatter_bubble.png` and `scatter_bubble.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
