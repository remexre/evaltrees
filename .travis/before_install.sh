#!/bin/bash

set -eu

loudly() {
	echo $@
	$@
}

command -v wasm-bindgen >/dev/null || loudly cargo install wasm-bindgen-cli
