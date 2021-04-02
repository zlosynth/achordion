.PHONY: all
all: clippy check test format

.PHONY: check-format
check-format:
	cd eurorack && cargo fmt -- --check
	cd lib && cargo fmt -- --check

.PHONY: format
format:
	cd eurorack && cargo fmt
	cd lib && cargo fmt

.PHONY: check-clippy
check-clippy:
	cd eurorack && cargo clippy -- -D warnings
	cd lib && cargo clippy -- -D warnings

.PHONY: clippy
clippy:
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
