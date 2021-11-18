# Achordion

Chord-crafting quantizing wavetable oscillator module for Eurorack.

## Development

``` sh
# run formatting, linter and unit tests
make

# run benchmark
cargo bench --bench bench

# check size of firmware
cargo size -- -m
cargo size -- -A

# run the module via pure data
make puredata

# profiling example
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

## Bandlimiting algorithm

Bandlimiting is available with two different filtering algorithms - fast fourier
transform (FFT) removing high frequency bins and state variable filter (SVF).
While FFT provides crips clean and very sharp sound, SVF goes little harder on
filtering and leaves slighly more muted, but warmer sound. Preferred algorithm
can be selected through a feature in `Cargo.toml` of both Eurorack and Pure Data
modules.

# License

Software of Achordion is distributed under the terms of the General Public
License version 3. See [LICENSE-SOFTWARE](LICENSE-SOFTWARE) for details.

Schematics and PCB layout are distributed under the terms of Creative Commons
BY-SA. See [LICENSE-HARDWARE](LICENSE-HARDWARE) for details. Parts of the
schematics are based on [Emilie Gillet's Mutable
Instruments](https://github.com/pichenettes/eurorack), kudos to her for making
them open.

The manual is distributed under the terms of Creative Commons BY-SA too. See
[LICENSE-MANUAL](LICENSE-MANUAL) for details.

# Changelog

Read the [CHANGELOG.md](CHANGELOG.md) to learn about changes introduced in each
release.

# Versioning

See [VERSIONING.md](VERSIONING.md) to find detailed information about versioning
of the project and compatability between its software and hardware.
