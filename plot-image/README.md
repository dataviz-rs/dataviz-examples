# Image display

An RGBA image (Mandelbrot set) shown on axes.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/image.png" alt="Image display" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-image
cargo run --release
```

Writes `image.png` and `image.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
