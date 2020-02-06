#!/bin/bash

# The initial build
wasm-pack build --release

echo "Initial size"

wc -c pkg/intersection_wasm_bg.wasm

wasm-snip --snip-rust-fmt-code --snip-rust-panicking-code \
pkg/intersection_wasm_bg.wasm -o pkg/intersection_wasm_bg.wasm

echo "Size after wasm-snip"
wc -c pkg/intersection_wasm_bg.wasm

# Optimize wasm
# NOTE: Again, setting -Os did not decrease the size,
# so might as well optimise for speed.
# You might need to install wasm-opt from binaryen:
# https://github.com/WebAssembly/binaryen/releases
wasm-opt -O4 --strip-debug --strip-producers \
pkg/intersection_wasm_bg.wasm -o pkg/intersection_wasm_bg.wasm

echo "Size after wasm-opt"
wc -c pkg/intersection_wasm_bg.wasm

echo "Size after gzip"
gzip -9 < pkg/intersection_wasm_bg.wasm | wc -c
