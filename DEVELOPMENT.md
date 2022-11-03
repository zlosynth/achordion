# Development

## Rust version

The firmware of this project fits really tight into the flash of Daisy Patch SM.
With anything newer than Rust 1.63.0, it just won't fit. Before working with
this project, please make sure to install needed Rust toolchains:

``` sh
rustup toolchain install 1.63.0-x86_64-unknown-linux-gnu
rustup target add thumbv7em-none-eabihf --toolchain 1.63.0
cargo +1.63.0 install cargo-binutils
rustup +1.63.0 component add llvm-tools-preview
```

## Cheat sheet

Run formatting, linter and unit tests:

``` sh
make
```

Check firmware size:

``` sh
cargo size -- -m
cargo size -- -A
cargo bloat --release -n 50
```

Analyze the call stack:

``` sh
cargo +nightly call-stack --bin achordion-eurorack > cg.dot
dot -Tsvg cg.dot > cg.svg
firefox cg.svg
```

Flash using ST-Link:

``` sh
cd eurorack
openocd &
cargo run --release
```

Flash using DFU:

``` sh
make flash
```

Benchmark:

``` sh
cargo bench --bench bench
```

Profiling:

``` sh
rm -f target/release/deps/bench-*
rm -f callgrind.out.*
RUSTFLAGS="-g" cargo bench --bench bench --no-run
BENCH=$(find target/release/deps -type f -executable -name 'bench-*')
TEST=instrument
valgrind \
    --tool=callgrind \
    --dump-instr=yes \
    --collect-jumps=yes \
    --simulate-cache=yes \
    ${BENCH} --bench --profile-time 10 ${TEST}
kcachegrind callgrind.out.*
```

Build the manual. Find built PDFs under `manual/user` and `manual/build`:

``` sh
make manual
```

And more under `hack/` and in the `Makefile`.

## Gerbers, BOM and CPL

I extensivelly use https://github.com/Bouni/kicad-jlcpcb-tools to deal with the
matters listed in the title, and to prepare project for manufacture.

## Bandlimiting algorithm

Bandlimiting is available with two different filtering algorithms - fast fourier
transform (FFT) removing high frequency bins and state variable filter (SVF).
While FFT provides crips clean and very sharp sound, SVF goes little harder on
filtering and leaves slighly more muted, but warmer sound. Preferred algorithm
can be selected through a feature in `Cargo.toml` of both Eurorack and Pure Data
modules.
