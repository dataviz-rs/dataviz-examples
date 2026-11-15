# dataviz benchmarks

Intel(R) Core(TM) Ultra 7 255HX (20 threads), GPU: NVIDIA GeForce RTX 5070 Ti Laptop GPU, Linux-7.0.0-38-generic-x86_64-with-glibc2.39.  
Versions: dataviz 0.16.0, plotters 0.3.7, ruviz 0.15.2, kuva 0.6.0, charts-rs 2.3.0, PyVista 0.49.2, matplotlib 3.11.2.

## Static charts (800×600 PNG, one thread, time per chart)

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib |
|---|---:|---:|---:|---:|---:|---:|
| line 1,000 | **1.53 ms** | 1.69 ms | 3.58 ms | 5.09 ms | 11.6 ms | 30.2 ms |
| bar 20 | **0.82 ms** | 0.84 ms | 3.83 ms | 3.66 ms | 2.15 ms | 43.1 ms |
| scatter 10,000 | **2.26 ms** | 2.93 ms | 4.29 ms | 8.86 ms | 78.6 ms | 38.7 ms |
| scatter 100,000 | **10.2 ms** | 17.0 ms | 18.0 ms | 18.0 ms | 808.3 ms | 83.8 ms |
| line 100,000 | **2.21 ms** | 6.12 ms | 8.08 ms | 29.1 ms | 861.2 ms | 52.2 ms |
| hist 1,000,000 | **2.16 ms** | 2.83 ms | 30.3 ms | 16.6 ms | 13.2 ms | 50.8 ms |

## Big data (800×600 PNG): time and memory above the data

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib | dataviz all cores |
|---|---:|---:|---:|---:|---:|---:|---:|
| line 10,000,000 | 126.8 ms / +8 MB | 491.9 ms / +315 MB | 330.8 ms / +389 MB | 2.78 s / +787 MB | fails | 597.0 ms / +1162 MB | **106.2 ms / +8 MB** |
| scatter 10,000,000 | 236.4 ms / +9 MB | 1.64 s / +12 MB | 1.30 s / +377 MB | 1.61 s / +788 MB | fails | 5.16 s / +910 MB | **183.2 ms / +8 MB** |
| hist 100,000,000 | 240.0 ms / +8 MB | 247.2 ms / +12 MB | 3.72 s / +1915 MB | 1.67 s / +1551 MB | 1.26 s / +1532 MB | 535.0 ms / +13 MB | **239.9 ms / +7 MB** |

## Real time (rolling window, one new point and a full redraw per frame)

Latency p50 / CPU time per frame / peak memory.

**800x600 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 0.78 ms | 0.77 ms | 8 MB |
| dataviz all cores | 0.50 ms | 2.60 ms | 9 MB |
| dataviz GPU | 0.67 ms | 0.40 ms | 239 MB |
| plotters | 0.65 ms | 0.63 ms | 14 MB |
| ruviz | 5.33 ms | 5.33 ms | 11 MB |
| kuva | 5.44 ms | 5.43 ms | 27 MB |
| charts-rs | 12.0 ms | 12.1 ms | 11 MB |
| matplotlib | 10.1 ms | 10.2 ms | 73 MB |

**800x600 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 2.15 ms | 2.13 ms | 12 MB |
| dataviz all cores | 1.99 ms | 4.60 ms | 13 MB |
| dataviz GPU | 1.73 ms | 1.30 ms | 243 MB |
| plotters | 5.53 ms | 5.53 ms | 16 MB |
| ruviz | 33.3 ms | 33.3 ms | 18 MB |
| kuva | 47.5 ms | 47.5 ms | 36 MB |
| charts-rs | 958.6 ms | 958.5 ms | 137 MB |
| matplotlib | 61.4 ms | 61.5 ms | 112 MB |

**1920x1080 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 1.67 ms | 1.70 ms | 16 MB |
| dataviz all cores | 0.82 ms | 5.87 ms | 18 MB |
| dataviz GPU | 1.52 ms | 1.30 ms | 245 MB |
| plotters | 1.48 ms | 1.50 ms | 18 MB |
| ruviz | 17.4 ms | 17.4 ms | 18 MB |
| kuva | 18.1 ms | 18.2 ms | 33 MB |
| charts-rs | 17.5 ms | 17.5 ms | 18 MB |
| matplotlib | 12.0 ms | 11.9 ms | 79 MB |

**1920x1080 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 5.37 ms | 5.40 ms | 19 MB |
| dataviz all cores | 3.26 ms | 14.4 ms | 22 MB |
| dataviz GPU | 2.82 ms | 2.50 ms | 250 MB |
| plotters | 8.45 ms | 8.47 ms | 21 MB |
| ruviz | 121.9 ms | 121.5 ms | 29 MB |
| kuva | 117.8 ms | 117.8 ms | 42 MB |
| charts-rs | 1.15 s | 1.15 s | 137 MB |
| matplotlib | 139.3 ms | 140.1 ms | 177 MB |

## Animated GIF (800×600, 120 frames)

| Scenario | dataviz | plotters | ruviz | matplotlib |
|---|---:|---:|---:|---:|
| moving sine wave (whole plot changes) | **301.0 ms, 1001 KB** | 995.0 ms, 3227 KB | 3.87 s, 1741 KB | 5.45 s, 1119 KB |
| growing line (fixed axes) | **84.0 ms, 38 KB** | 2.98 s, 3257 KB | 4.00 s, 1701 KB | 6.53 s, 420 KB |

## 3D charts (800×600 PNG; dataviz, plotters and matplotlib on one core)

Time per image; a rotation frame is one frame of an interactive rotation (camera turned, image drawn again); the GIF has 90 frames.

| Chart | dataviz | dataviz GPU | plotters | matplotlib | PyVista |
|---|---:|---:|---:|---:|---:|
| surface 60² | 6.89 ms | **2.66 ms** | 6.39 ms | 61.8 ms | 95.2 ms |
| surface 200² | 30.9 ms | **22.0 ms** | 22.4 ms | 170.4 ms | 101.7 ms |
| scatter 10,000 | 2.84 ms | **1.90 ms** | 3.46 ms | 99.8 ms | 88.6 ms |
| scatter 1,000,000 | 141.7 ms | **107.0 ms** | 177.6 ms | 5.87 s | 118.0 ms |
| line 100,000 (Lorenz) | 9.72 ms | 13.4 ms | **8.62 ms** | 62.1 ms | 85.2 ms |
| molecule 1BNA (486 atoms) | 4.48 ms | 4.95 ms | **1.92 ms** | 38.7 ms | 82.1 ms |
| rotation frame: surface 60² | 5.08 ms | 2.07 ms | 5.00 ms | 25.0 ms | **0.60 ms** |
| rotation frame: molecule | 3.01 ms | 3.67 ms | 0.88 ms | 11.4 ms | **0.60 ms** |
| GIF, 90 frames turning | **885.2 ms, 3949 KB** | 946.8 ms, 3440 KB | 2.80 s, 4544 KB | 7.49 s, 5713 KB | 6.59 s, 3441 KB |

Peak memory of the process:

| Chart | dataviz | dataviz GPU | plotters | matplotlib | PyVista |
|---|---:|---:|---:|---:|---:|
| scatter 1,000,000 | 103 MB | 352 MB | **37 MB** | 620 MB | 345 MB |
| molecule 1BNA (486 atoms) | **11 MB** | 248 MB | 14 MB | 100 MB | 282 MB |
| surface 200² | 33 MB | 267 MB | **13 MB** | 132 MB | 291 MB |
