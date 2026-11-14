# Charts for engineers

An FFT amplitude spectrum, the spectrogram of a chirp with a steady tone,
the Nyquist plot of a second-order system, impedances on a Smith chart,
and a Bode plot (magnitude and unwrapped phase) of two transfer functions.

<img src="./screenshots/engineering.png" alt="Charts for engineers" width="800">

<img src="./screenshots/bode.png" alt="Charts for engineers" width="800">

```rust
ax.spectrum(&signal, fs);
ax.spectrogram(&signal, fs, 512, 64);
let w = logspace(0.01, 100.0, 400);
let h = freq_response(&[1.0], &[1.0, 0.4, 1.0], &w);   // 1 / (s² + 0.4s + 1)
ax.nyquist(&h);
ax.smith(&impedances);
fig.bode("1/(s²+0.4s+1)", &w, &h);
```

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-engineering
cargo run --release
```

See [`src/main.rs`](src/main.rs) for the code.

## License

MIT
