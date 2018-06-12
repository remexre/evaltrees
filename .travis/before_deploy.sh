#!/bin/bash

set -eu

loudly() {
	echo $@
	$@
}

if [[ -n "$TRAVIS_TAG" ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/..
	loudly cargo build --all --release
	loudly cp target/release/evaltrees-cli evaltrees
	loudly tar czf "evaltrees-${TRAVIS_OS_NAME}.tar.gz" evaltrees
fi

if [[ "$TRAVIS_OS_NAME" = linux && "$TRAVIS_RUST_VERSION" = nightly ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/../evaltrees-wasm
	loudly rustup target add wasm32-unknown-unknown
	loudly cargo build --release --target wasm32-unknown-unknown
	loudly npm i
	loudly npx webpack -p --progress
fi
