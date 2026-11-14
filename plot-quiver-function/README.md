# Vector field and function plots

Colored quiver plot and adaptively sampled function plots.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/quiver_function.png" alt="Vector field and function plots" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-quiver-function
cargo run --release
```

Writes `quiver_function.png` and `quiver_function.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
