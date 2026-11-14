# dataviz benchmarks

dataviz against [plotters](https://crates.io/crates/plotters),
[ruviz](https://crates.io/crates/ruviz), [kuva](https://crates.io/crates/kuva)
and [charts-rs](https://crates.io/crates/charts-rs): the same charts drawn by
each library, measured in separate processes.

| Group | What is measured |
|---|---|
| `static` | small charts to an 800×600 PNG: median time and CPU per chart over 5 × 201 runs, peak memory |
| `bigdata` | 10 M-point line and scatter, 100 M-value histogram: time and memory above what the data itself takes |
| `realtime` | a rolling window (1 000 or 100 000 points) gets one new point per frame and is fully redrawn: p50 latency, CPU per frame, peak memory; dataviz on one thread, on all cores and on the GPU |
| `animation` | a 120-frame 800×600 animated GIF: total time and file size |

Results of the last run: [RESULTS.md](RESULTS.md) (raw: `results.json`).

## Run

```sh
# plotters needs fontconfig: install libfontconfig1-dev, or let it load at run time:
RUST_FONTCONFIG_DLOPEN=1 cargo build --release
python3 run_all.py                  # all groups (about 20 minutes)
python3 run_all.py static realtime  # some groups
```

Single-threaded runs are pinned to one core (`taskset`) for steadier
numbers; close other heavy programs while measuring. The binaries in
`src/bin` show exactly what each library draws.
