all: check doc test
clean:
	cargo clean
watch TARGET="all":
	watchexec -cre rs,toml "just {{TARGET}}"

bench:
	cargo +nightly bench --all
build: build-debug build-release
build-debug:
	cargo build --all
build-release:
	cargo build --all --release
check:
	cargo check --all
clippy:
	cargo +nightly clippy --all
doc:
	cargo doc --all
run +ARGS="":
	cargo run -- {{ARGS}}
run-release +ARGS="":
	cargo run --release -- {{ARGS}}
test:
	cargo test --all

open-docs:
	cargo doc --open -p common
outdated-deps:
	cargo outdated -R
