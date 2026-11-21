"""dataviz.pyplot against matplotlib.pyplot on the same scripts:

    python pyplot_bench.py mpl     # matplotlib (Agg backend)
    python pyplot_bench.py dv      # dataviz.pyplot  (pip install dataviz-rs)

Prints the import time, then for each script the first call and the median
of warm calls in the same process (imports excluded). Both save a PNG.
"""
import sys, time, statistics, os, tempfile
which = sys.argv[1]
t0 = time.perf_counter()
if which == "mpl":
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
else:
    import dataviz.pyplot as plt
import numpy as np
t_import = (time.perf_counter() - t0) * 1000

rng = np.random.default_rng(1)
x = np.linspace(0, 10, 500)
a, b = rng.standard_normal(2000), rng.standard_normal(2000)
samples = rng.standard_normal(20000)
big_x = np.linspace(0, 100, 1_000_000)
big_y = np.sin(big_x) + rng.random(1_000_000)
out = tempfile.mkdtemp()

def four_panel(path):
    fig, axs = plt.subplots(2, 2, figsize=(10, 7))
    axs[0, 0].plot(x, np.sin(x), "r--", label="sin"); axs[0, 0].legend()
    axs[0, 1].scatter(a, b, s=12, c=a * b, cmap="viridis")
    axs[1, 0].hist(samples, bins=40); axs[1, 0].set_yscale("log")
    axs[1, 1].bar(["a", "b", "c"], [3, 5, 2], color="tab:purple")
    fig.suptitle("Report"); fig.savefig(path); plt.close(fig)

def line_1m(path):
    fig, ax = plt.subplots(figsize=(8, 6))
    ax.plot(big_x, big_y); fig.savefig(path); plt.close(fig)

def bench(f, name, reps=15):
    t = time.perf_counter(); f(os.path.join(out, name + "0.png")); first = (time.perf_counter() - t) * 1000
    ws = []
    for i in range(reps):
        t = time.perf_counter(); f(os.path.join(out, f"{name}{i+1}.png")); ws.append((time.perf_counter() - t) * 1000)
    return first, statistics.median(ws)

r4 = bench(four_panel, "four")
rl = bench(line_1m, "line", 7)
print(f"{which}: import {t_import:.0f} ms | 4-panel first {r4[0]:.1f} warm {r4[1]:.1f} ms | 1M line first {rl[0]:.1f} warm {rl[1]:.1f} ms")
