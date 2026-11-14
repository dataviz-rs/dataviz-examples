# Terminal output

Charts printed in the terminal with 24-bit colored half blocks or braille dots (`Figure::to_terminal`, `Figure::print`), e.g. over SSH or in CI logs.



Braille output (`TerminalMode::BrailleMono`) is saved to [`screenshots/terminal.txt`](screenshots/terminal.txt).

## Run

```sh
git clone https://github.com/dataviz-rs/dataviz-examples.git
cd dataviz-examples/plot-terminal
cargo run --release
```

## License

MIT
