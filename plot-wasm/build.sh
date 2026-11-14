#!/bin/sh
# Builds the WebAssembly module into www/ and serves the page.
set -e
cd "$(dirname "$0")"
rustup target add wasm32-unknown-unknown
cargo build --profile wasm --target wasm32-unknown-unknown -p plot-wasm
cp ../target/wasm32-unknown-unknown/wasm/plot_wasm.wasm www/
echo "open http://127.0.0.1:8000"
cd www && python3 -m http.server 8000 --bind 127.0.0.1
