#!/bin/bash
set -e

echo "=== Building Dungeon Crawler WASM ==="

# Build for wasm32
cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen to generate JS bindings
wasm-bindgen \
    --out-dir web \
    --target web \
    --no-typescript \
    target/wasm32-unknown-unknown/release/dungeon-crawler.wasm

echo "=== Build complete! Files in web/ ==="
ls -la web/
