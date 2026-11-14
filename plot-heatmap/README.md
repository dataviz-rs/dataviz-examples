# Heatmap (correlation matrix)

Annotated correlation matrix with a diverging colormap and colorbar.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/heatmap.png" alt="Heatmap (correlation matrix)" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-heatmap
cargo run --release
```

Writes `heatmap.png` and `heatmap.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
