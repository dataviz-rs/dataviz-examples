#!/usr/bin/env python3
"""Runs every dataviz benchmark against plotters, ruviz, kuva and charts-rs
and writes RESULTS.md (Markdown tables) and results.json.

    RUST_FONTCONFIG_DLOPEN=1 cargo build --release
    python3 run_all.py                 # everything (about 15 minutes)
    python3 run_all.py static realtime # some groups only

Groups: static (small charts), bigdata, realtime, animation.
Times are medians; CPU is user + system time of the process; memory is the
peak resident set (for big data: above what the data itself takes).
"""
import json
import os
import platform
import re
import statistics
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
BIN = os.path.join(HERE, "target", "release")
OUT = tempfile.mkdtemp(prefix="dataviz-bench-")
# One core for single-thread runs: steadier timings (no migrations).
PIN = ["taskset", "-c", "2"] if subprocess.run(["which", "taskset"], capture_output=True).returncode == 0 else []
# matplotlib runs from the virtualenv made by the README's setup commands.
PY = os.path.join(HERE, ".venv", "bin", "python")
MPL = [PY, os.path.join(HERE, "mpl_bench.py")] if os.path.exists(PY) else None
LIBS = [("dataviz", "dataviz"), ("plotters", "plotters"), ("ruviz", "ruviz"), ("kuva", "kuva"), ("charts-rs", "charts_rs")]
if MPL:
    LIBS.append(("matplotlib", "mpl"))


# ONLY=matplotlib,plotters: measure only these libraries and keep the other
# results of results.json.
ONLY = set(filter(None, os.environ.get("ONLY", "").split(",")))
PREV = {}


def skip(group, key, name):
    """True (and the previous result kept) when `name` is filtered out."""
    return bool(ONLY) and not any(o in name for o in ONLY)


def keep(group, key, name):
    return PREV.get(group, {}).get(key, {}).get(name)


def command(binary, mode):
    """The command line of a benchmark program (`mode`: static, rt, anim)."""
    if binary.endswith("mpl"):
        return MPL + [mode]
    prefix = {"static": "", "rt": "rt_", "anim": "anim_"}[mode]
    return [os.path.join(BIN, prefix + binary)]


def run(cmd, env=None, timeout=300, pin=False):
    """Runs a command under /usr/bin/time; returns (stdout, cpu s, peak RSS MB) or None."""
    e = dict(os.environ, **(env or {}))
    try:
        p = subprocess.run(["/usr/bin/time", "-f", "TIME %U %S %M"] + (PIN if pin else []) + cmd, env=e, capture_output=True,
                           text=True, timeout=timeout)
    except subprocess.TimeoutExpired:
        return None
    t = [l for l in p.stderr.splitlines() if l.startswith("TIME ")]
    if p.returncode != 0 or not t:
        return None
    _, u, s, m = t[-1].split()
    return p.stdout, float(u) + float(s), int(m) / 1024


def single_threaded(binary, env):
    """Runs pinned to one core: dataviz on one thread and the libraries that
    draw on one thread anyway (ruviz uses several threads)."""
    if "ruviz" in binary:
        return False
    return (env or {}).get("BACKEND", "cpu") == "cpu"


def static_chart(binary, scenario, n, reps, env=None, runs=5):
    """Median warm time and CPU per chart (ms), peak RSS (MB). `reps` is an
    upper bound: slow charts get fewer repetitions (about 2 s per run)."""
    probe = run(command(binary, "static") + [scenario, str(n), "png", os.path.join(OUT, binary)],
                dict(env or {}, BENCH_REPS="2"), pin=single_threaded(binary, env))
    if probe:
        est = float(probe[0].split()[1])
        reps = max(3, min(reps, int(2000 / max(est, 0.01))))
    rows = []
    for _ in range(runs):
        r = run(command(binary, "static") + [scenario, str(n), "png", os.path.join(OUT, binary)],
                dict(env or {}, BENCH_REPS=str(reps)), pin=single_threaded(binary, env))
        if r:
            f = list(map(float, r[0].split()))
            # matplotlib reports its memory above the data itself.
            rows.append((f[1], f[2], r[2], f[3] if len(f) > 3 else None))
    if not rows:
        return None
    rows.sort(key=lambda r: r[0])
    warm, cpu, rss, extra = rows[len(rows) // 2]
    out = {"ms": warm, "cpu_ms": cpu, "rss_mb": rss}
    if extra is not None:
        out["extra_mb"] = extra
    return out


def group_static():
    res = {}
    for sc, n in [("line", 1_000), ("bar", 20), ("scatter", 10_000), ("scatter", 100_000), ("line", 100_000), ("hist", 1_000_000)]:
        key = f"{sc} {n:,}"
        res[key] = {}
        for name, binary in LIBS:
            if skip("static", key, name):
                res[key][name] = keep("static", key, name)
                continue
            r = static_chart(binary, sc, n, 201 if n <= 100_000 else 31, {"BACKEND": "cpu"} if name == "dataviz" else None)
            res[key][name] = r
            print(f"static {key:16} {name:10} {r}", flush=True)
    return res


def group_bigdata():
    res = {}
    for sc, n in [("line", 10_000_000), ("scatter", 10_000_000), ("hist", 100_000_000)]:
        key = f"{sc} {n:,}"
        base = run([os.path.join(BIN, "baseline"), sc, str(n), "png", os.path.join(OUT, "base")], {"BENCH_REPS": "1"})
        base_rss = base[2] if base else 0.0
        res[key] = {"data_mb": base_rss}
        for name, binary in LIBS + [("dataviz all cores", "dataviz")]:
            if skip("bigdata", key, name):
                res[key][name] = keep("bigdata", key, name)
                continue
            env = {"BACKEND": "par" if "all" in name else "cpu"} if name.startswith("dataviz") else None
            r = static_chart(binary, sc, n, 3, env, runs=3)
            if r and "extra_mb" not in r:
                r["extra_mb"] = max(0.0, r["rss_mb"] - base_rss)
            res[key][name] = r
            print(f"bigdata {key:18} {name:18} {r}", flush=True)
    return res


def realtime(binary, n, frames, env):
    r = run(command(binary, "rt") + [str(n), str(frames)], env, timeout=900, pin=single_threaded(binary, env))
    if not r:
        return None
    try:
        d = json.loads(r[0].strip().splitlines()[-1])
    except (ValueError, IndexError):
        return None
    return {"p50_ms": d["p50"], "p95_ms": d["p95"], "cpu_ms": d["cpu_ms_per_frame"], "rss_mb": d["rss_peak_mb"]}


def gpu_available():
    r = realtime("backends", 1000, 5, {"BACKEND": "gpu", "SIZE": "320x240"})
    return r is not None


def group_realtime():
    res = {}
    gpu = gpu_available()
    for size in ["800x600", "1920x1080"]:
        for n in [1_000, 100_000]:
            key = f"{size} {n:,}"
            res[key] = {}
            variants = [("dataviz 1 thread", "backends", {"BACKEND": "cpu"}),
                        ("dataviz all cores", "backends", {"BACKEND": "par"})]
            if gpu:
                variants.append(("dataviz GPU", "backends", {"BACKEND": "gpu"}))
            variants += [(name, b, {}) for name, b in LIBS[1:]]
            for name, binary, env in variants:
                if skip("realtime", key, name):
                    res[key][name] = keep("realtime", key, name)
                    continue
                frames = 20 if (binary == "charts_rs" and n > 10_000) else 300
                r = realtime(binary, n, frames, dict(env, SIZE=size))
                res[key][name] = r
                print(f"realtime {key:18} {name:18} {r}", flush=True)
    return res


def group_animation():
    res = {}
    for sc in ["wave", "stream"]:
        res[sc] = {}
        for lib in ["dataviz", "plotters", "ruviz"] + (["matplotlib"] if MPL else []):
            if skip("animation", sc, lib):
                res[sc][lib] = keep("animation", sc, lib)
                continue
            out = os.path.join(OUT, f"{lib}_{sc}.gif")
            r = run(command("mpl" if lib == "matplotlib" else lib, "anim") + [sc, out], timeout=600)
            if r:
                d = json.loads(r[0].strip().splitlines()[-1])
                res[sc][lib] = {"ms": d["wall_ms"], "cpu_ms": d["cpu_ms"], "rss_mb": d["rss_mb"], "kb": d["kb"]}
            else:
                res[sc][lib] = None
            print(f"animation {sc:8} {lib:10} {res[sc][lib]}", flush=True)
    return res


def machine():
    cpu = "?"
    try:
        cpu = re.search(r"model name\s*:\s*(.*)", open("/proc/cpuinfo").read()).group(1)
    except (OSError, AttributeError):
        pass
    gpu = subprocess.run(["sh", "-c", "nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1"],
                         capture_output=True, text=True).stdout.strip()
    lock = open(os.path.join(HERE, "Cargo.lock")).read()
    versions = {}
    for crate in ["dataviz", "plotters", "ruviz", "kuva", "charts-rs"]:
        m = re.search(r'name = "%s"\nversion = "([^"]+)"' % re.escape(crate), lock)
        versions[crate] = m.group(1) if m else "?"
    if MPL:
        versions["matplotlib"] = subprocess.run(MPL[:1] + ["-c", "import matplotlib; print(matplotlib.__version__)"],
                                                capture_output=True, text=True).stdout.strip()
    return {"cpu": cpu, "cores": os.cpu_count(), "gpu": gpu or "none", "os": platform.platform(), "versions": versions}


def fmt_ms(v):
    if v is None:
        return "fails"
    return f"{v / 1000:.2f} s" if v >= 1000 else (f"{v:.1f} ms" if v >= 10 else f"{v:.2f} ms")


def best_row(cells, values):
    """Bolds the smallest value."""
    finite = [v for v in values if v is not None]
    if not finite:
        return cells
    m = min(finite)
    return [f"**{c}**" if v == m else c for c, v in zip(cells, values)]


def markdown(results, info):
    names = [n for n, _ in LIBS]
    lines = [f"# dataviz benchmarks\n",
             f"{info['cpu']} ({info['cores']} threads), GPU: {info['gpu']}, {info['os']}.  ",
             "Versions: " + ", ".join(f"{k} {v}" for k, v in info["versions"].items()) + ".\n"]
    if "static" in results:
        lines += ["## Static charts (800×600 PNG, one thread, time per chart)\n",
                  "| Chart | " + " | ".join(names) + " |", "|---|" + "---:|" * len(names)]
        for k, row in results["static"].items():
            vals = [row[n]["ms"] if row.get(n) else None for n in names]
            lines.append(f"| {k} | " + " | ".join(best_row([fmt_ms(v) for v in vals], vals)) + " |")
        lines.append("")
    if "bigdata" in results:
        lines += ["## Big data (800×600 PNG): time and memory above the data\n",
                  "| Chart | " + " | ".join(names + ["dataviz all cores"]) + " |", "|---|" + "---:|" * (len(names) + 1)]
        for k, row in results["bigdata"].items():
            cols = names + ["dataviz all cores"]
            vals = [row[n]["ms"] if row.get(n) else None for n in cols]
            mem = [row[n]["extra_mb"] if row.get(n) else None for n in cols]
            cells = [f"{fmt_ms(v)} / +{m:.0f} MB" if v is not None else "fails" for v, m in zip(vals, mem)]
            lines.append(f"| {k} | " + " | ".join(best_row(cells, vals)) + " |")
        lines.append("")
    if "realtime" in results:
        lines += ["## Real time (rolling window, one new point and a full redraw per frame)\n",
                  "Latency p50 / CPU time per frame / peak memory.\n"]
        for k, row in results["realtime"].items():
            lines += [f"**{k} points**\n", "| Library | p50 | CPU per frame | memory |", "|---|---:|---:|---:|"]
            for name, r in row.items():
                if r is None:
                    lines.append(f"| {name} | fails | | |")
                else:
                    lines.append(f"| {name} | {fmt_ms(r['p50_ms'])} | {fmt_ms(r['cpu_ms'])} | {r['rss_mb']:.0f} MB |")
            lines.append("")
    if "animation" in results:
        al = [l for l in ["dataviz", "plotters", "ruviz", "matplotlib"] if any(l in r for r in results["animation"].values())]
        lines += ["## Animated GIF (800×600, 120 frames)\n",
                  "| Scenario | " + " | ".join(al) + " |", "|---|" + "---:|" * len(al)]
        for sc, row in results["animation"].items():
            vals = [row[l]["ms"] if row.get(l) else None for l in al]
            cells = [f"{fmt_ms(row[l]['ms'])}, {row[l]['kb']} KB" if row.get(l) else "fails" for l in al]
            label = {"wave": "moving sine wave (whole plot changes)", "stream": "growing line (fixed axes)"}[sc]
            lines.append(f"| {label} | " + " | ".join(best_row(cells, vals)) + " |")
        lines.append("")
    return "\n".join(lines)


def main():
    groups = sys.argv[1:] or ["static", "bigdata", "realtime", "animation"]
    path = os.path.join(HERE, "results.json")
    results = json.load(open(path)) if os.path.exists(path) else {}
    PREV.update(results)
    for g in groups:
        results[g] = globals()[f"group_{g}"]()
        json.dump(results, open(path, "w"), indent=1)
    info = machine()
    results["machine"] = info
    json.dump(results, open(path, "w"), indent=1)
    open(os.path.join(HERE, "RESULTS.md"), "w").write(markdown(results, info))
    print(f"wrote {path} and RESULTS.md")


if __name__ == "__main__":
    main()
