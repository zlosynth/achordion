# Development

## Cheat sheet

Run formatting, linter and unit tests:

``` sh
make
```

Run the module via Pure Data:

```sh
make puredata
```

Check firmware size:

``` sh
cargo size -- -m
cargo size -- -A
```

Flash:

``` sh
cd eurorack
openocd &
cargo run --release
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

And more under `hack/` and in the `Makefile`.

## Bandlimiting algorithm

Bandlimiting is available with two different filtering algorithms - fast fourier
transform (FFT) removing high frequency bins and state variable filter (SVF).
While FFT provides crips clean and very sharp sound, SVF goes little harder on
filtering and leaves slighly more muted, but warmer sound. Preferred algorithm
can be selected through a feature in `Cargo.toml` of both Eurorack and Pure Data
modules.
