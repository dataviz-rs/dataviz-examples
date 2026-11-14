# SVG Scatter Graph

Scatter graph with all four dot types (circle, square, cross, triangle), rendered as SVG.

## Features

- Uses [dataviz](https://crates.io/crates/dataviz) 0.1.10+
- Customizable colors, fonts, sizes, tick and grid counts through `FigureConfig`
- No font file needed: an installed sans-serif font is used by default

## Screenshots

<img src="./screenshots/scatter_graph.svg" alt="SVG Scatter Graph" width="600px">

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/dataviz-rs/dataviz-examples.git
    ```
2. Navigate to the project directory:
    ```sh
    cd dataviz-examples/svgscattergraph
    ```
3. Build the project:
    ```sh
    cargo build
    ```

## Usage

Run the application:
```sh
cargo run
```

The output is written to the current directory: `scatter_graph.svg`.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
