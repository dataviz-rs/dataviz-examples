# Animations

A travelling wave packet and the phase portrait of a damped oscillator,
recorded with `Figure::record` as a GIF and as a lossless animated PNG.

<img src="./screenshots/animation.gif" alt="Animation" width="720">

Only the part of each frame that changed is stored, so recording 90 frames
at 820×520 takes about 0.2 s. The same call writes `.mp4` / `.webm` when
`ffmpeg` is installed. For data arriving over time, use `Recorder::push_at`
(see [`plot-streaming`](../plot-streaming)); in a window `V` records a GIF.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-animation
cargo run --release
```

Writes `animation.gif` and `animation.png` to the current directory. See
[`src/main.rs`](src/main.rs) for the code.

## License

MIT
