#!/bin/bash

set -eu

cd $(dirname ${BASH_SOURCE[0]})/evaltrees-wasm

loudly() {
	echo $@
	$@
}

if [[ -z "${TRAVIS_TAG+x}" ]]; then
	loudly build --release
fi

if [[ "$TRAVIS_OS_NAME" = linux && "$TRAVIS_RUST_VERSION" = nightly ]]; then
	loudly npm i
	loudly npx webpack -p
fi
