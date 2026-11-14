# dataviz benchmarks

Intel(R) Core(TM) Ultra 7 255HX (20 threads), GPU: NVIDIA GeForce RTX 5070 Ti Laptop GPU, Linux-7.0.0-38-generic-x86_64-with-glibc2.39.  
Versions: dataviz 0.15.0, plotters 0.3.7, ruviz 0.15.2, kuva 0.6.0, charts-rs 2.3.0, matplotlib 3.11.2.

## Static charts (800×600 PNG, one thread, time per chart)

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib |
|---|---:|---:|---:|---:|---:|---:|
| line 1,000 | **1.62 ms** | 3.76 ms | 3.97 ms | 11.3 ms | 11.4 ms | 29.5 ms |
| bar 20 | **0.81 ms** | 0.83 ms | 3.98 ms | 3.64 ms | 2.14 ms | 41.6 ms |
| scatter 10,000 | **2.25 ms** | 2.90 ms | 4.38 ms | 8.71 ms | 76.6 ms | 37.5 ms |
| scatter 100,000 | **10.1 ms** | 16.7 ms | 16.6 ms | 17.4 ms | 776.4 ms | 80.7 ms |
| line 100,000 | **2.13 ms** | 5.99 ms | 8.22 ms | 28.4 ms | 831.8 ms | 49.8 ms |
| hist 1,000,000 | **1.99 ms** | 2.77 ms | 29.7 ms | 15.8 ms | 12.4 ms | 49.7 ms |

## Big data (800×600 PNG): time and memory above the data

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib | dataviz all cores |
|---|---:|---:|---:|---:|---:|---:|---:|
| line 10,000,000 | 122.3 ms / +8 MB | 486.1 ms / +315 MB | 329.5 ms / +389 MB | 2.71 s / +787 MB | fails | 565.7 ms / +1162 MB | **99.6 ms / +8 MB** |
| scatter 10,000,000 | 222.1 ms / +9 MB | 1.53 s / +12 MB | 1.23 s / +379 MB | 1.55 s / +788 MB | fails | 5.11 s / +910 MB | **176.5 ms / +9 MB** |
| hist 100,000,000 | 242.3 ms / +7 MB | 260.6 ms / +12 MB | 3.58 s / +1915 MB | 1.62 s / +1551 MB | 1.21 s / +1532 MB | 520.0 ms / +13 MB | **237.6 ms / +7 MB** |

## Real time (rolling window, one new point and a full redraw per frame)

Latency p50 / CPU time per frame / peak memory.

**800x600 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 0.74 ms | 0.73 ms | 8 MB |
| dataviz all cores | 0.52 ms | 2.53 ms | 9 MB |
| dataviz GPU | 0.83 ms | 0.43 ms | 238 MB |
| plotters | 0.63 ms | 0.63 ms | 14 MB |
| ruviz | 5.26 ms | 5.27 ms | 11 MB |
| kuva | 5.21 ms | 5.20 ms | 27 MB |
| charts-rs | 11.5 ms | 11.6 ms | 11 MB |
| matplotlib | 9.98 ms | 9.97 ms | 74 MB |

**800x600 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 2.06 ms | 2.03 ms | 12 MB |
| dataviz all cores | 1.81 ms | 4.50 ms | 14 MB |
| dataviz GPU | 1.76 ms | 1.30 ms | 242 MB |
| plotters | 5.39 ms | 5.43 ms | 16 MB |
| ruviz | 30.8 ms | 30.8 ms | 17 MB |
| kuva | 45.6 ms | 45.8 ms | 36 MB |
| charts-rs | 921.2 ms | 924.0 ms | 137 MB |
| matplotlib | 60.0 ms | 60.2 ms | 113 MB |

**1920x1080 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 1.60 ms | 1.63 ms | 16 MB |
| dataviz all cores | 0.77 ms | 5.83 ms | 18 MB |
| dataviz GPU | 1.71 ms | 1.37 ms | 244 MB |
| plotters | 1.44 ms | 1.43 ms | 18 MB |
| ruviz | 14.9 ms | 14.9 ms | 18 MB |
| kuva | 17.3 ms | 17.3 ms | 33 MB |
| charts-rs | 17.0 ms | 17.0 ms | 18 MB |
| matplotlib | 12.0 ms | 12.0 ms | 80 MB |

**1920x1080 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 5.15 ms | 5.17 ms | 19 MB |
| dataviz all cores | 3.14 ms | 14.1 ms | 21 MB |
| dataviz GPU | 2.80 ms | 2.40 ms | 249 MB |
| plotters | 8.15 ms | 8.17 ms | 21 MB |
| ruviz | 119.5 ms | 119.5 ms | 28 MB |
| kuva | 111.8 ms | 112.1 ms | 42 MB |
| charts-rs | 1.10 s | 1.11 s | 137 MB |
| matplotlib | 135.8 ms | 135.8 ms | 177 MB |

## Animated GIF (800×600, 120 frames)

| Scenario | dataviz | plotters | ruviz | matplotlib |
|---|---:|---:|---:|---:|
| moving sine wave (whole plot changes) | **252.0 ms, 1001 KB** | 1.08 s, 3227 KB | 2.93 s, 1741 KB | 5.90 s, 1119 KB |
| growing line (fixed axes) | **81.0 ms, 38 KB** | 2.38 s, 3257 KB | 2.93 s, 1701 KB | 6.10 s, 420 KB |
