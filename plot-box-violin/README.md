# Box and violin plots

Notched box plots with means and outliers next to violin plots.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/box_violin.png" alt="Box and violin plots" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-box-violin
cargo run --release
```

Writes `box_violin.png` and `box_violin.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
