.PHONY: all
all: format clippy check test

.PHONY: check-format
check-format:
	cd bank && cargo fmt --all -- --check
	cd eurorack && cargo fmt --all -- --check
	cd lib && cargo fmt --all -- --check

.PHONY: format
format:
	cd bank && cargo fmt --all
	cd eurorack && cargo fmt --all
	cd lib && cargo fmt --all

.PHONY: clippy
clippy:
	cd bank && cargo clippy --features fft --all -- -D warnings
	cd bank && cargo clippy --features svf --all -- -D warnings
	cd eurorack && cargo clippy --all -- -D warnings
	cd lib && cargo clippy --all -- -D warnings

.PHONY: check
check:
	cd bank && cargo check --features fft --all
	cd bank && cargo check --features svf --all
	cd eurorack && cargo check --all
	cd lib && cargo check --all
	cd lib && cargo check --benches --all

.PHONY: test
test:
	cd bank && cargo test --features fft --all
	cd bank && cargo test --features svf --all
	cd eurorack && cargo test --all
	cd lib && cargo test --all
	python -m unittest -v hack/calculate_adc_opamp_components.py
	python -m unittest -v hack/calculate_reference_voltage_current_limiter.py

.PHONY: update
update:
	cd bank && cargo update
	cd eurorack && cargo update
	cd lib && cargo update

.PHONY: manual
manual:
	make -C manual/build
	make -C manual/user

.PHONY: clean
clean:
	cd bank && cargo clean
	cd eurorack && cargo clean
	cd lib && cargo clean

.PHONY: flash
flash:
	cd eurorack && cargo objcopy --release -- -O binary target/achordion.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/achordion.bin -d ,0483:df11
