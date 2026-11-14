"""3D benchmarks for matplotlib (mplot3d) and PyVista (VTK, OpenGL), with
the scenarios and output of the d3_* Rust binaries (run by run_all.py):

    bench3d.py <mpl|pyvista> <scenario> <n> png <out>  -> "first warm cpu extra_mb"

Scenarios: surface (n × n grid), scatter3d, line3d (Lorenz), molecule
(PDB 1BNA, ball-and-stick), rotate_surface / rotate_molecule (one frame of
an interactive rotation: the camera turns and the image is drawn again),
anim (90-frame GIF of the surface turning).

Both libraries are used the fast way: matplotlib with Agg, `view_init` +
`canvas.draw()` for rotation; PyVista off screen with one plotter per
static image (as a script saving images does) and `screenshot` per frame
(the image read back, like dataviz rendering into pixels).
"""
import os
import sys
import time

import numpy as np

W, H, DPI = 800, 600, 100
FRAMES, STEP = 90, 4.0


def status_mb(key):
    for line in open("/proc/self/status"):
        if line.startswith(key):
            return int(line.split()[1]) / 1024
    return 0.0


def cpu_ms():
    return time.process_time() * 1000


# ----- data (the same as src/d3.rs) -----------------------------------------

def grid(n):
    g = np.linspace(-8.0, 8.0, n)
    x, y = np.meshgrid(g, g)
    r = np.hypot(x, y)
    with np.errstate(invalid="ignore", divide="ignore"):
        z = np.where(r < 1e-9, 1.0, np.sin(r) / r)
    return x, y, z


def cloud(n):
    rng = np.random.default_rng(5)
    c = np.array([[0.0, 0.0, 0.0], [4.0, 1.0, 2.0], [1.0, 4.0, -2.0]])
    return c[np.arange(n) % 3] + rng.standard_normal((n, 3))


def lorenz(n):
    out = np.empty((n, 3))
    a, b, c = 1.0, 1.0, 1.0
    dt = 0.002
    for i in range(n):
        da, db, dc = 10.0 * (b - a), a * (28.0 - c) - b, a * b - 8.0 / 3.0 * c
        a, b, c = a + da * dt, b + db * dt, c + dc * dt
        out[i] = (a, b, c)
    return out


ELEMENTS = {
    "H": ((235, 235, 235), 0.31, 1.20), "C": ((144, 144, 144), 0.76, 1.70),
    "N": ((48, 80, 248), 0.71, 1.55), "O": ((255, 13, 13), 0.66, 1.52),
    "P": ((255, 128, 0), 1.07, 1.80), "S": ((230, 200, 40), 1.05, 1.80),
}


def pdb_text():
    path = os.environ.get("D3_PDB", os.path.join(os.path.dirname(__file__), "..", "plot-biology-3d", "data", "1BNA.pdb"))
    return open(path).read()


def molecule(text):
    """Atoms (positions, elements) and bonds by the same distance rule as
    dataviz (vectorized with numpy)."""
    pos, el = [], []
    for l in text.splitlines():
        if (l.startswith("ATOM") or l.startswith("HETATM")) and len(l) >= 54 and "HOH" not in l:
            pos.append((float(l[30:38]), float(l[38:46]), float(l[46:54])))
            e = l[76:78].strip() or l[12:16].strip()[:1]
            el.append(e)
    pos = np.array(pos)
    cov = np.array([ELEMENTS.get(e, ((255, 20, 147), 0.8, 1.7))[1] for e in el])
    d2 = ((pos[:, None, :] - pos[None, :, :]) ** 2).sum(-1)
    m = cov[:, None] + cov[None, :] + 0.45
    i, j = np.nonzero(np.triu((d2 > 0.16) & (d2 < m * m), 1))
    colors = np.array([ELEMENTS.get(e, ((255, 20, 147), 0.8, 1.7))[0] for e in el]) / 255.0
    vdw = np.array([ELEMENTS.get(e, ((255, 20, 147), 0.8, 1.7))[2] for e in el])
    return pos, colors, vdw, np.stack([i, j], 1)


# ----- matplotlib ------------------------------------------------------------

class Mpl:
    def __init__(self):
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
        from mpl_toolkits.mplot3d.art3d import Line3DCollection
        self.plt, self.Line3DCollection = plt, Line3DCollection

    def figure(self):
        fig = self.plt.figure(figsize=(W / DPI, H / DPI), dpi=DPI)
        return fig, fig.add_subplot(projection="3d")

    def surface(self, d, azim=-60.0):
        fig, ax = self.figure()
        x, y, z = d
        n = z.shape[0]
        # Every grid cell drawn (the default would thin it to 50 × 50).
        ax.plot_surface(x, y, z, cmap="viridis", rcount=n, ccount=n, linewidth=0, antialiased=True)
        ax.set_title("surface")
        ax.set_xlabel("x"), ax.set_ylabel("y"), ax.set_zlabel("z")
        ax.view_init(30, azim)
        return fig, ax

    def points(self, p, line, title):
        fig, ax = self.figure()
        if line:
            ax.plot(p[:, 0], p[:, 1], p[:, 2], lw=1.0)
        else:
            ax.scatter(p[:, 0], p[:, 1], p[:, 2], s=(3 * 72 / DPI) ** 2)
        ax.set_title(title)
        return fig, ax

    def molecule(self, m, azim=-60.0):
        pos, colors, vdw, bonds = m
        fig, ax = self.figure()
        a, b = pos[bonds[:, 0]], pos[bonds[:, 1]]
        mid = (a + b) / 2
        segs = np.concatenate([np.stack([a, mid], 1), np.stack([mid, b], 1)])
        cols = np.concatenate([colors[bonds[:, 0]], colors[bonds[:, 1]]])
        ax.add_collection3d(self.Line3DCollection(segs, colors=cols, linewidths=2.5))
        ax.scatter(pos[:, 0], pos[:, 1], pos[:, 2], s=(vdw * 0.24 * 20 * 72 / DPI) ** 2, c=colors, depthshade=True)
        ax.set_title("1BNA")
        ax.set_axis_off()
        ax.view_init(20, azim)
        return fig, ax

    def save(self, fig, path):
        fig.savefig(path)
        self.plt.close(fig)

    def rotate_frame(self, fig, ax, k):
        ax.view_init(ax.elev, -60.0 + k * STEP)
        fig.canvas.draw()
        return memoryview(fig.canvas.buffer_rgba())

    def anim(self, d, out):
        from matplotlib.animation import FuncAnimation, PillowWriter
        fig, ax = self.surface(d)

        def update(i):
            ax.view_init(30, -60.0 + i * STEP)
            return ()

        FuncAnimation(fig, update, frames=FRAMES).save(out, writer=PillowWriter(fps=30))
        self.plt.close(fig)


# ----- PyVista ---------------------------------------------------------------

class Vista:
    def __init__(self):
        import pyvista as pv
        self.pv = pv

    def plotter(self):
        pl = self.pv.Plotter(off_screen=True, window_size=(W, H))
        pl.set_background("white")
        return pl

    def surface(self, d, pl=None):
        pv = self.pv
        x, y, z = d
        pl = pl or self.plotter()
        g = pv.StructuredGrid(x, y, z)
        pl.add_mesh(g, scalars=z.ravel(order="F"), cmap="viridis", smooth_shading=True, show_scalar_bar=False)
        pl.show_grid(xtitle="x", ytitle="y", ztitle="z", color="gray")
        pl.add_title("surface", font_size=10, color="black")
        pl.camera_position = "iso"
        return pl

    def points(self, p, line, title):
        pv = self.pv
        pl = self.plotter()
        if line:
            poly = pv.PolyData(p)
            poly.lines = np.concatenate([[len(p)], np.arange(len(p))])
            pl.add_mesh(poly, color="tab:blue", line_width=1)
        else:
            pl.add_mesh(pv.PolyData(p), color="tab:blue", point_size=3, render_points_as_spheres=True)
        pl.show_grid(color="gray")
        pl.add_title(title, font_size=10, color="black")
        return pl

    def molecule(self, m, pl=None):
        pv = self.pv
        pos, colors, vdw, bonds = m
        pl = pl or self.plotter()
        atoms = pv.PolyData(pos)
        atoms["r"] = vdw * 0.24
        atoms["rgb"] = (colors * 255).astype(np.uint8)
        balls = atoms.glyph(geom=pv.Sphere(theta_resolution=16, phi_resolution=12), scale="r", orient=False, factor=1.0)
        pl.add_mesh(balls, scalars="rgb", rgb=True, smooth_shading=True)
        # Bonds: two halves per bond colored by their atom, as tubes.
        a, b = pos[bonds[:, 0]], pos[bonds[:, 1]]
        mid = (a + b) / 2
        pts = np.concatenate([a, mid, mid, b])
        k = len(bonds)
        lines = np.concatenate([np.stack([np.full(k, 2), np.arange(k), np.arange(k) + k], 1),
                                np.stack([np.full(k, 2), np.arange(k) + 2 * k, np.arange(k) + 3 * k], 1)]).ravel()
        sticks = pv.PolyData(pts, lines=lines)
        sticks["rgb"] = (np.concatenate([colors[bonds[:, 0]]] * 2 + [colors[bonds[:, 1]]] * 2) * 255).astype(np.uint8)
        pl.add_mesh(sticks.tube(radius=0.14, n_sides=10), scalars="rgb", rgb=True, smooth_shading=True)
        pl.add_title("1BNA", font_size=10, color="black")
        return pl

    def save(self, pl, path):
        pl.screenshot(path)
        pl.close()

    def rotate_frame(self, pl, _ax, k):
        pl.camera.azimuth = pl.camera.azimuth + STEP
        return pl.screenshot(return_img=True)

    def anim(self, d, out):
        pl = self.surface(d)
        pl.open_gif(out, fps=30)
        for _ in range(FRAMES):
            pl.camera.azimuth = pl.camera.azimuth + STEP
            pl.write_frame()
        pl.close()


def main():
    lib, scenario, n, out = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[5]
    impl = Mpl() if lib == "mpl" else Vista()
    reps = max(1, int(os.environ.get("BENCH_REPS", "1")))
    path = out + ".png"
    base = status_mb("VmRSS:")
    times, cpus = [], []

    def measure(f):
        t, c = time.perf_counter(), cpu_ms()
        f()
        times.append((time.perf_counter() - t) * 1000)
        cpus.append(cpu_ms() - c)

    if scenario == "surface":
        d = grid(n)
        draw = (lambda: impl.save(impl.surface(d)[0], path)) if lib == "mpl" else (lambda: impl.save(impl.surface(d), path))
        for _ in range(reps):
            measure(draw)
    elif scenario in ("scatter3d", "line3d"):
        p = lorenz(n) if scenario == "line3d" else cloud(n)
        for _ in range(reps):
            def draw():
                r = impl.points(p, scenario == "line3d", scenario)
                impl.save(r[0] if lib == "mpl" else r, path)
            measure(draw)
    elif scenario == "molecule":
        text = pdb_text()
        for _ in range(reps):
            def draw():
                # Parsing and bond search timed, as for dataviz.
                m = molecule(text)
                r = impl.molecule(m)
                impl.save(r[0] if lib == "mpl" else r, path)
            measure(draw)
    elif scenario in ("rotate_surface", "rotate_molecule"):
        if scenario == "rotate_surface":
            built = impl.surface(grid(n))
        else:
            built = impl.molecule(molecule(pdb_text()))
        fig, ax = built if lib == "mpl" else (built, None)
        impl.rotate_frame(fig, ax, 0)
        for k in range(reps):
            measure(lambda: impl.rotate_frame(fig, ax, k + 1))
    elif scenario == "anim":
        d = grid(n)
        measure(lambda: impl.anim(d, out + ".gif"))
    warm = sorted(times[1:] or times)
    wcpu = sorted(cpus[1:] or cpus)
    extra = max(0.0, status_mb("VmHWM:") - base)
    print(f"{times[0]:.3f} {warm[len(warm) // 2]:.3f} {wcpu[len(wcpu) // 2]:.3f} {extra:.1f}")


if __name__ == "__main__":
    main()
