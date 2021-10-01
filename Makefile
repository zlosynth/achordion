.PHONY: all
all: format clippy check test

.PHONY: check-format
check-format:
	cd bank && cargo fmt --all -- --check
	cd eurorack && cargo fmt --all -- --check
	cd lib && cargo fmt --all -- --check
	cd puredata && cargo fmt --all -- --check

.PHONY: format
format:
	cd bank && cargo fmt --all
	cd eurorack && cargo fmt --all
	cd lib && cargo fmt --all
	cd puredata && cargo fmt --all

.PHONY: clippy
clippy:
	cd bank && cargo clippy --features fft --all -- -D warnings
	cd bank && cargo clippy --features svf --all -- -D warnings
	cd eurorack && cargo clippy --all -- -D warnings
	cd lib && cargo clippy --all -- -D warnings
	cd puredata && cargo clippy --all -- -D warnings

.PHONY: check
check:
	cd bank && cargo check --features fft --all
	cd bank && cargo check --features svf --all
	cd eurorack && cargo check --all
	cd lib && cargo check --all
	cd lib && cargo check --benches --all
	cd puredata && cargo check --all

.PHONY: test
test:
	cd bank && cargo test --features fft --all
	cd bank && cargo test --features svf --all
	cd eurorack && cargo test --all
	cd lib && cargo test --all
	cd puredata && cargo test --all
	python -m unittest -v hack/calculate_adc_opamp_components.py
	python -m unittest -v hack/calculate_reference_voltage_current_limiter.py

.PHONY: puredata
puredata:
	mkdir -p ~/.local/lib/pd/extra
	cd puredata && cargo build --release
	cp puredata/target/release/libachordion_puredata.so ~/.local/lib/pd/extra/achordion~.pd_linux
	pd puredata/achordion.pd
