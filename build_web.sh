#!/usr/bin/env bash
set -eu
cargo build --release --lib --target wasm32-unknown-unknown
wasm-bindgen "target\wasm32-unknown-unknown\release\egui_experiments.wasm" --out-dir "docs" --target no-modules
