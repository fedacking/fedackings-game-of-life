run:
	cargo run

format:
	cargo fmt

format-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy

test:
	cargo test