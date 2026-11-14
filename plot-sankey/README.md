# Sankey diagram

Energy flows between sources, carriers and uses.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/sankey.png" alt="Sankey diagram" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-sankey
cargo run --release
```

Writes `sankey.png` and `sankey.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
