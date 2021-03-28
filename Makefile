.PHONY: all
all: format lint check test

.PHONY: format
format:
	cd eurorack && cargo fmt
	cd lib && cargo fmt

.PHONY: lint
lint:
	cd eurorack && cargo clippy
	cd lib && cargo clippy

.PHONY: check
check:
	cd eurorack && cargo check
	cd lib && cargo check

.PHONY: test
test:
	cd eurorack && cargo test
	cd lib && cargo test
