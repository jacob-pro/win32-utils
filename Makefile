.PHONY: test
test:
	cargo fmt -- --check
	cargo-sort --check --workspace
	cargo clippy --all-features --workspace -- -D warnings
	cargo hack test --each-feature --workspace

.PHONY: format
format:
	cargo fmt
	cargo-sort --workspace
