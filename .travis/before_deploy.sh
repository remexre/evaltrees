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
