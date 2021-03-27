.PHONY: all
all: format lint check test

.PHONY: format
format:
	cargo fmt --all

.PHONY: lint
lint:
	cargo clippy --all

.PHONY: check
check:
	cargo check --all

.PHONY: test
test:
	cargo test --all
