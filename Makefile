.PHONY: all
all: format clippy check test

.PHONY: check-format
check-format:
	cd eurorack && cargo fmt --all -- --check
	cd lib && cargo fmt --all -- --check
	cd puredata && cargo fmt --all -- --check

.PHONY: format
format:
	cd eurorack && cargo fmt --all
	cd lib && cargo fmt --all
	cd puredata && cargo fmt --all

.PHONY: check-clippy
check-clippy:
	cd eurorack && cargo clippy --all -- -D warnings
	cd lib && cargo clippy --all -- -D warnings
	cd puredata && cargo clippy --all -- -D warnings

.PHONY: clippy
clippy:
	cd eurorack && cargo clippy --all
	cd lib && cargo clippy --all
	cd puredata && cargo clippy --all

.PHONY: check
check:
	cd eurorack && cargo check --all
	cd lib && cargo check --all
	cd lib && cargo check --benches --all
	cd puredata && cargo check --all

.PHONY: test
test:
	cd eurorack && cargo test --all
	cd lib && cargo test --all
	cd puredata && cargo test --all
