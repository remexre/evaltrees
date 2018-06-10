#!/bin/bash

set -eu
cd $(dirname ${BASH_SOURCE[0]})/..
cargo build --manifest-path evaltrees-wasm/Cargo.toml --release --target wasm32-unknown-unknown
wasm-gc target/wasm32-unknown-unknown/release/evaltrees_wasm.wasm
