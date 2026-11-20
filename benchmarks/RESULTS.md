# dataviz benchmarks

Intel(R) Core(TM) Ultra 7 255HX (20 threads), GPU: NVIDIA GeForce RTX 5070 Ti Laptop GPU, Linux-7.0.0-43-generic-x86_64-with-glibc2.39.  
Versions: dataviz 0.19.1, plotters 0.3.7, ruviz 0.15.2, kuva 0.6.0, charts-rs 2.3.0, PyVista 0.49.2, matplotlib 3.11.2.

## Static charts (800×600 PNG, one thread, time per chart)

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib |
|---|---:|---:|---:|---:|---:|---:|
| line 1,000 | **1.50 ms** | 1.66 ms | 3.60 ms | 4.98 ms | 11.1 ms | 29.3 ms |
| bar 20 | **0.79 ms** | 0.83 ms | 3.90 ms | 3.55 ms | 2.08 ms | 41.8 ms |
| scatter 10,000 | **2.12 ms** | 2.83 ms | 4.37 ms | 8.53 ms | 76.2 ms | 37.5 ms |
| scatter 100,000 | **9.88 ms** | 16.6 ms | 16.9 ms | 17.4 ms | 784.5 ms | 81.8 ms |
| line 100,000 | **2.17 ms** | 5.91 ms | 8.39 ms | 28.2 ms | 829.7 ms | 50.2 ms |
| hist 1,000,000 | **2.06 ms** | 2.76 ms | 30.2 ms | 16.1 ms | 12.8 ms | 49.6 ms |

## Big data (800×600 PNG): time and memory above the data

| Chart | dataviz | plotters | ruviz | kuva | charts-rs | matplotlib | dataviz all cores |
|---|---:|---:|---:|---:|---:|---:|---:|
| line 10,000,000 | 129.0 ms / +7 MB | 481.9 ms / +315 MB | 332.5 ms / +389 MB | 2.70 s / +787 MB | fails | 533.1 ms / +1162 MB | **103.1 ms / +6 MB** |
| scatter 10,000,000 | 230.3 ms / +8 MB | 1.53 s / +12 MB | 1.23 s / +376 MB | 1.56 s / +788 MB | fails | 5.08 s / +910 MB | **181.6 ms / +7 MB** |
| hist 100,000,000 | 243.5 ms / +7 MB | 264.6 ms / +12 MB | 3.64 s / +1915 MB | 1.64 s / +1551 MB | 1.23 s / +1533 MB | 516.6 ms / +13 MB | **235.2 ms / +6 MB** |

## Real time (rolling window, one new point and a full redraw per frame)

Latency p50 / CPU time per frame / peak memory.

**800x600 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 0.77 ms | 0.77 ms | 7 MB |
| dataviz all cores | 0.49 ms | 2.57 ms | 8 MB |
| dataviz GPU | 0.67 ms | 0.47 ms | 214 MB |
| plotters | 0.65 ms | 0.63 ms | 14 MB |
| ruviz | 4.66 ms | 4.67 ms | 11 MB |
| kuva | 5.26 ms | 5.23 ms | 27 MB |
| charts-rs | 11.7 ms | 11.7 ms | 11 MB |
| matplotlib | 10.1 ms | 10.1 ms | 73 MB |

**800x600 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 2.20 ms | 2.17 ms | 12 MB |
| dataviz all cores | 2.02 ms | 4.67 ms | 13 MB |
| dataviz GPU | 2.04 ms | 1.63 ms | 219 MB |
| plotters | 5.40 ms | 5.37 ms | 16 MB |
| ruviz | 34.6 ms | 34.6 ms | 17 MB |
| kuva | 47.0 ms | 47.1 ms | 36 MB |
| charts-rs | 944.7 ms | 945.0 ms | 137 MB |
| matplotlib | 62.0 ms | 62.0 ms | 112 MB |

**1920x1080 1,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 1.69 ms | 1.70 ms | 15 MB |
| dataviz all cores | 0.77 ms | 5.83 ms | 17 MB |
| dataviz GPU | 1.39 ms | 1.23 ms | 221 MB |
| plotters | 1.48 ms | 1.50 ms | 18 MB |
| ruviz | 17.4 ms | 17.4 ms | 18 MB |
| kuva | 18.0 ms | 18.0 ms | 33 MB |
| charts-rs | 17.9 ms | 18.0 ms | 17 MB |
| matplotlib | 12.4 ms | 12.6 ms | 79 MB |

**1920x1080 100,000 points**

| Library | p50 | CPU per frame | memory |
|---|---:|---:|---:|
| dataviz 1 thread | 5.51 ms | 5.53 ms | 18 MB |
| dataviz all cores | 3.25 ms | 14.1 ms | 20 MB |
| dataviz GPU | 2.70 ms | 2.53 ms | 225 MB |
| plotters | 8.43 ms | 8.43 ms | 21 MB |
| ruviz | 119.9 ms | 119.9 ms | 28 MB |
| kuva | 115.8 ms | 115.8 ms | 42 MB |
| charts-rs | 1.13 s | 1.13 s | 137 MB |
| matplotlib | 136.8 ms | 136.8 ms | 177 MB |

## Animated GIF (800×600, 120 frames)

| Scenario | dataviz | plotters | ruviz | matplotlib |
|---|---:|---:|---:|---:|
| moving sine wave (whole plot changes) | **282.0 ms, 1001 KB** | 1.09 s, 3227 KB | 3.00 s, 1741 KB | 5.24 s, 1119 KB |
| growing line (fixed axes) | **81.0 ms, 38 KB** | 2.40 s, 3257 KB | 2.91 s, 1701 KB | 5.75 s, 420 KB |

## 3D charts (800×600 PNG; dataviz, plotters and matplotlib on one core)

Time per image; a rotation frame is one frame of an interactive rotation (camera turned, image drawn again); the GIF has 90 frames.

| Chart | dataviz | dataviz GPU | plotters | matplotlib | PyVista |
|---|---:|---:|---:|---:|---:|
| surface 60² | 6.34 ms | **2.37 ms** | 6.05 ms | 57.0 ms | 93.7 ms |
| surface 200² | 29.9 ms | **5.41 ms** | 21.3 ms | 146.2 ms | 101.0 ms |
| scatter 10,000 | 2.81 ms | **2.00 ms** | 3.19 ms | 96.3 ms | 89.2 ms |
| scatter 1,000,000 | 147.0 ms | **39.9 ms** | 170.3 ms | 5.71 s | 119.2 ms |
| line 100,000 (Lorenz) | 9.93 ms | **3.79 ms** | 8.41 ms | 59.3 ms | 87.8 ms |
| molecule 1BNA (486 atoms) | 3.35 ms | 2.31 ms | **1.91 ms** | 37.6 ms | 82.1 ms |
| rotation frame: surface 60² | 5.10 ms | **0.45 ms** | 4.98 ms | 25.0 ms | 0.55 ms |
| rotation frame: molecule | 2.13 ms | **0.46 ms** | 0.88 ms | 11.4 ms | 0.61 ms |
| GIF, 90 frames turning | 885.7 ms, 3949 KB | **665.9 ms, 3453 KB** | 2.80 s, 4544 KB | 7.38 s, 5713 KB | 4.14 s, 3441 KB |

Peak memory of the process:

| Chart | dataviz | dataviz GPU | plotters | matplotlib | PyVista |
|---|---:|---:|---:|---:|---:|
| scatter 1,000,000 | 103 MB | 299 MB | **36 MB** | 620 MB | 345 MB |
| molecule 1BNA (486 atoms) | **10 MB** | 223 MB | 14 MB | 116 MB | 284 MB |
| surface 200² | 32 MB | 233 MB | **14 MB** | 159 MB | 296 MB |
