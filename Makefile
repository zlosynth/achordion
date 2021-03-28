.PHONY: all
all: format lint check test

.PHONY: format
format:
	cd eurorack && cargo fmt

.PHONY: lint
lint:
	cd eurorack && cargo clippy

.PHONY: check
check:
	cd eurorack && cargo check

.PHONY: test
test:
	cd eurorack && cargo test
