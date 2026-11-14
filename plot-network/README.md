# Network graph

Force-directed network layout with node groups.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/network.png" alt="Network graph" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-network
cargo run --release
```

Writes `network.png` and `network.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
