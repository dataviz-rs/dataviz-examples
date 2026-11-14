# Charts for data scientists

ROC curves with the AUC in the legend, a precision-recall curve, a
normalized confusion matrix, a regression line with its 95 % confidence
band and R², 2D density contours of 20 000 points and a dendrogram of
hierarchical clustering.

<img src="./screenshots/data_science.png" alt="Charts for data scientists" width="800">

```rust
ax.roc_curve("gradient boosting", &labels, &scores);
ax.pr_curve("gradient boosting", &labels, &scores);
ax.confusion_matrix(["cat", "dog", "bird"], &actual, &predicted, true);
ax.regplot(&x, &y);
ax.density_contour(&a, &b);
ax.dendrogram(names, &features);
```

Built with the [`dataviz::plot`](https://docs.rs/dataviz/latest/dataviz/plot/) API.

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-data-science
cargo run --release
```

See [`src/main.rs`](src/main.rs) for the code.

## License

MIT
