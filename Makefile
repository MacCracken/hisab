.PHONY: check fmt clippy test build doc clean

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

build:
	cargo build --workspace --release

doc:
	cargo doc --workspace --no-deps --open

clean:
	cargo clean
