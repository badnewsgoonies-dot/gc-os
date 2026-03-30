#!/bin/bash
set -e

echo "Building Platform Jumper for WASM..."

# Build the WASM binary
RUSTFLAGS="--cfg=web_sys_unstable_apis" cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen to generate JS bindings
wasm-bindgen \
    --out-dir web \
    --target web \
    --no-typescript \
    target/wasm32-unknown-unknown/release/platform-jumper.wasm

echo "Build complete! Output in web/"
echo "Files:"
ls -lh web/platform-jumper_bg.wasm web/platform-jumper.js 2>/dev/null || true
