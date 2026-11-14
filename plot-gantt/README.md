# Gantt chart

Project plan with progress on a time axis and a "today" marker.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

<img src="./screenshots/gantt.png" alt="Gantt chart" width="800">

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-gantt
cargo run --release
```

Writes `gantt.png` and `gantt.svg` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
