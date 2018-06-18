#!/bin/bash

set -eu

loudly() {
	echo $@
	$@
}

loudly rustup target add --toolchain nightly wasm32-unknown-unknown
command -v wasm-bindgen >/dev/null || cargo install wasm-bindgen-cli 
command -v wasm-gc >/dev/null || cargo install wasm-gc 

cd $(dirname ${BASH_SOURCE[0]})
mkdir -p src/generated

loudly cargo +nightly build --manifest-path evaltrees-wasm/Cargo.toml --release --target wasm32-unknown-unknown
loudly wasm-bindgen evaltrees-wasm/target/wasm32-unknown-unknown/release/evaltrees_wasm.wasm --out-dir src/generated
loudly npm run build
