#!/bin/bash

set -eu

cd $(dirname ${BASH_SOURCE[0]})/..

loudly() {
	echo $@
	$@
}

loudly cargo build --manifest-path evaltrees-wasm/Cargo.toml --release --target wasm32-unknown-unknown
loudly wasm-gc target/wasm32-unknown-unknown/release/evaltrees_wasm.wasm
