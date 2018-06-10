#!/bin/bash

set -eu

loudly() {
	echo $@
	$@
}

if [[ -n "${TRAVIS_TAG+x}" ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/evaltrees-cli
	loudly build --all --release
fi

if [[ "$TRAVIS_OS_NAME" = linux && "$TRAVIS_RUST_VERSION" = nightly ]]; then
	cd $(dirname ${BASH_SOURCE[0]})/evaltrees-wasm
	loudly npm i
	loudly npx webpack -p
fi
