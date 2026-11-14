"""matplotlib side of the benchmarks, with the same scenarios and output as
the Rust binaries (run by run_all.py with the interpreter in .venv):

    mpl_bench.py static <line|bar|scatter|hist> <n> png <out>   -> "first warm cpu extra_mb"
    mpl_bench.py rt <n> <frames>                                 -> JSON like the rt_* binaries (SIZE=WxH)
    mpl_bench.py anim <wave|stream> <out.gif>                    -> JSON like the anim_* binaries

matplotlib is used the fast way: the Agg backend, a reused figure with
`set_data` + `canvas.draw()` for live charts, numpy arrays throughout.
Memory is measured above what Python, numpy, matplotlib and the data take.
"""
import json
import os
import sys
import time

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
import numpy as np  # noqa: E402

W, H, DPI, BINS = 800, 600, 100, 50


def status_mb(key):
    for line in open("/proc/self/status"):
        if line.startswith(key):
            return int(line.split()[1]) / 1024
    return 0.0


def cpu_ms():
    return time.process_time() * 1000


def data(scenario, n):
    rng = np.random.default_rng(1)
    if scenario == "line":
        x = np.arange(n) / n * 100.0
        return x, np.sin(x * 0.3) * 10.0 + rng.random(n)
    if scenario == "scatter":
        x = rng.standard_normal(n)
        return x, x * 0.6 + rng.standard_normal(n) * 0.5
    if scenario == "bar":
        return [f"c{i}" for i in range(n)], [10.0 + (i * 37) % 23 for i in range(n)]
    return rng.standard_normal(n) * 2.0 + 5.0, None


def figure(w=W, h=H):
    return plt.figure(figsize=(w / DPI, h / DPI), dpi=DPI)


def draw(scenario, d, path):
    fig = figure()
    ax = fig.add_subplot()
    if scenario == "line":
        ax.plot(d[0], d[1], label="line")
        ax.set_title("line")
        ax.set_xlabel("x")
        ax.set_ylabel("y")
    elif scenario == "scatter":
        # 4 px markers like the other libraries (area in points²).
        ax.scatter(d[0], d[1], s=(4 * 72 / DPI) ** 2)
        ax.set_title("scatter")
    elif scenario == "bar":
        ax.bar(d[0], d[1])
        ax.set_title("bar")
    else:
        ax.hist(d[0], bins=BINS)
        ax.set_title("hist")
    fig.savefig(path)
    plt.close(fig)


def static(scenario, n, out):
    reps = max(1, int(os.environ.get("BENCH_REPS", "1")))
    d = data(scenario, n)
    path = out + ".png"
    base = status_mb("VmRSS:")
    times, cpus = [], []
    for _ in range(reps):
        t, c = time.perf_counter(), cpu_ms()
        draw(scenario, d, path)
        times.append((time.perf_counter() - t) * 1000)
        cpus.append(cpu_ms() - c)
    warm = sorted(times[1:] or times)
    wcpu = sorted(cpus[1:] or cpus)
    extra = max(0.0, status_mb("VmHWM:") - base)
    print(f"{times[0]:.3f} {warm[len(warm) // 2]:.3f} {wcpu[len(wcpu) // 2]:.3f} {extra:.1f}")


def realtime(n, frames):
    w, h = (int(v) for v in os.environ.get("SIZE", "800x600").split("x"))
    # The same stream as the Rust benchmarks: a rolling window of n points.
    t = np.arange(1, n + 1, dtype=np.float64)
    x = t * 0.01
    noise = lambda k: ((k * 2654435761) % 1000) / 1000.0  # noqa: E731
    y = np.sin(x * 3.0) * 5.0 + noise(t.astype(np.uint64))
    fig = figure(w, h)
    ax = fig.add_subplot()
    ax.set_title("real time")
    (line,) = ax.plot(x, y, color="tab:blue")
    k = n

    def frame():
        nonlocal x, y, k
        k += 1
        nx = k * 0.01
        x = np.append(x[1:], nx)
        y = np.append(y[1:], np.sin(nx * 3.0) * 5.0 + noise(np.uint64(k)))
        line.set_data(x, y)
        ax.relim()
        ax.autoscale_view()
        fig.canvas.draw()
        return bytes(memoryview(fig.canvas.buffer_rgba())[:4])

    for _ in range(5):
        frame()
    rss0 = status_mb("VmRSS:")
    c0, w0 = cpu_ms(), time.perf_counter()
    lat = []
    for _ in range(frames):
        s = time.perf_counter()
        frame()
        lat.append((time.perf_counter() - s) * 1000)
    wall = (time.perf_counter() - w0) * 1000
    cpu = cpu_ms() - c0
    lat.sort()
    pct = lambda p: lat[round((len(lat) - 1) * p)]  # noqa: E731
    print(json.dumps({"lib": "matplotlib", "n": n, "frames": frames, "p50": pct(0.5), "p95": pct(0.95),
                      "max": lat[-1], "mean": sum(lat) / len(lat), "cpu_ms_per_frame": cpu / frames,
                      "cpu_util": cpu / wall, "rss_peak_mb": status_mb("VmHWM:"),
                      "rss_growth_mb": status_mb("VmRSS:") - rss0, "sink": 0}))


def anim(scenario, out):
    from matplotlib.animation import FuncAnimation, PillowWriter

    frames, n = 120, 1000
    t0, c0 = time.perf_counter(), cpu_ms()
    fig = figure()
    ax = fig.add_subplot()
    ax.set_title("Animation")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    ax.set_xlim(0, 10)
    ax.set_ylim(-1.5, 1.5)
    (line,) = ax.plot([], [], color="tab:blue")

    def update(i):
        if scenario == "wave":
            x = np.arange(n) * 10.0 / n
            y = np.sin(x + i / 30 * 2 * np.pi)
        else:
            m = (i + 1) * n // frames
            x = np.arange(m) * 10.0 / n
            y = np.sin(x * 3.0) * (1.0 - x / 12.0)
        line.set_data(x, y)
        return (line,)

    FuncAnimation(fig, update, frames=frames).save(out, writer=PillowWriter(fps=30))
    wall, cpu = (time.perf_counter() - t0) * 1000, cpu_ms() - c0
    print(json.dumps({"lib": "matplotlib", "scenario": scenario, "file": out, "wall_ms": round(wall),
                      "cpu_ms": round(cpu), "rss_mb": round(status_mb("VmHWM:"), 1),
                      "kb": os.path.getsize(out) // 1024}))


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "static":
        static(sys.argv[2], int(sys.argv[3]), sys.argv[5])
    elif mode == "rt":
        realtime(int(sys.argv[2]), int(sys.argv[3]))
    else:
        anim(sys.argv[2], sys.argv[3])
