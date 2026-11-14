# 3D biology: DNA, molecules, cells

A DNA double helix with base pairs colored by base and a marked TATA box,
real structures from the Protein Data Bank (the B-DNA dodecamer 1BNA as
ball-and-stick with CPK colors, the protein crambin 1CRN as a backbone tube
colored from N- to C-terminus), a cell (translucent membrane, nucleus with
nucleolus, mitochondria, vesicles, ribosomes), an organoid of cells colored
by a marker gene, and single-cell clusters on a 3D UMAP.

<img src="./screenshots/rotate.gif" alt="DNA and a cell turning around the z, x and y axes" width="840">

<img src="./screenshots/biology_3d.png" alt="DNA helix, PDB structures, cell, organoid, 3D UMAP" width="900">

```rust
ax.dna_helix("ATGCGTATAAAAGCC").highlight(5..11, "#ffd400", Some("TATA box"));
ax.molecule(Molecule::from_pdb(&pdb)).style(MolStyle::BallAndStick);
ax.molecule(Molecule::from_pdb(&pdb)).style(MolStyle::Backbone).coloring(MolColor::Rainbow);
ax.ellipsoid([0.0, 0.0, 0.0], [10.0, 8.0, 6.0]).opacity(0.16);   // membrane
ax.spheres3d(&x, &y, &z, [1.0]).values(&expression, Colormap::Viridis);
ax.embedding3d(&umap1, &umap2, &umap3, cell_types);
ax.rotate(Axis3::X, 30.0);
```

Molecules are read from PDB (`ATOM`/`HETATM`) or XYZ text; bonds are found
from covalent radii with a spatial grid (linear time). Styles: ball-and-stick,
sticks, space-filling and backbone; colors by element, chain or along the
chain. Structures open with their longest extent across the screen
(principal axes) and far parts fade into the background for depth.

## Turning the scenes

`cargo run --release -- --window` opens the figure in a window: drag to
turn, wheel to zoom, `X`/`Y`/`Z` to rotate by 15° around that axis
(Shift+key the other way), `R` to reset, hover an atom or base pair for its
name. In code: `ax.rotate(Axis3::Z, 15.0)`, `ax.roll(..)`,
`ax.camera(azimuth, elevation)`; `rotate.gif` is recorded that way.

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-biology-3d
cargo run --release              # biology_3d.png and rotate.gif
cargo run --release -- --window  # interactive
```

See [`src/main.rs`](src/main.rs) for the code.

## Data

`data/1BNA.pdb` and `data/1CRN.pdb` are from the
[RCSB Protein Data Bank](https://www.rcsb.org/) (public domain, CC0):
1BNA, Drew et al. 1981, *Structure of a B-DNA dodecamer*; 1CRN, Teeter
1984, *Crambin*.

## License

MIT
