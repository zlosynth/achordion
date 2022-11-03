CARGO = cargo +1.63.0

.PHONY: all
all: format clippy check test

.PHONY: check-format
check-format:
	cd bank && $(CARGO) fmt --all -- --check
	cd eurorack && $(CARGO) fmt --all -- --check
	cd lib && $(CARGO) fmt --all -- --check

.PHONY: format
format:
	cd bank && $(CARGO) fmt --all
	cd eurorack && $(CARGO) fmt --all
	cd lib && $(CARGO) fmt --all

.PHONY: clippy
clippy:
	cd bank && $(CARGO) clippy --features fft --all -- -D warnings
	cd bank && $(CARGO) clippy --features svf --all -- -D warnings
	cd eurorack && $(CARGO) clippy --all -- -D warnings
	cd lib && $(CARGO) clippy --all -- -D warnings

.PHONY: check
check:
	cd bank && $(CARGO) check --features fft --all
	cd bank && $(CARGO) check --features svf --all
	cd eurorack && $(CARGO) check --all
	cd lib && $(CARGO) check --all
	cd lib && $(CARGO) check --benches --all

.PHONY: test
test:
	cd bank && $(CARGO) test --features fft --all
	cd bank && $(CARGO) test --features svf --all
	cd eurorack && $(CARGO) test --all
	cd lib && $(CARGO) test --all
	python -m unittest -v hack/calculate_adc_opamp_components.py
	python -m unittest -v hack/calculate_reference_voltage_current_limiter.py

.PHONY: update
update:
	cd bank && $(CARGO) update
	cd eurorack && $(CARGO) update
	cd lib && $(CARGO) update

.PHONY: manual
manual:
	make -C manual/build
	make -C manual/user

.PHONY: clean
clean:
	cd bank && $(CARGO) clean
	cd eurorack && $(CARGO) clean
	cd lib && $(CARGO) clean

.PHONY: flash
flash:
	cd eurorack && $(CARGO) objcopy --release -- -O binary target/achordion.bin
	dfu-util -a 0 -s 0x08000000:leave -D eurorack/target/achordion.bin -d ,0483:df11
