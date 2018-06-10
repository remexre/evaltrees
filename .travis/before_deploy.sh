#!/bin/bash

set -eu

loudly() {
	echo $@
	$@
}

if [[ -n "$TRAVIS_TAG" ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/../evaltrees-cli
	loudly cargo build --all --release
fi

if [[ "$TRAVIS_OS_NAME" = linux && "$TRAVIS_RUST_VERSION" = nightly ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/../evaltrees-wasm
	command -v wasm-bindgen >/dev/null || loudly cargo install wasm-bindgen-cli
	loudly rustup target add wasm32-unknown-unknown
	loudly npm i
	loudly npx webpack -p
fi
