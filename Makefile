.PHONY: default
default:
	cargo clean
	cargo build
	cargo test
	cargo fmt
