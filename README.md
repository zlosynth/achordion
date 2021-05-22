# Achordion

Chord-crafting quantizing wavetable oscillator module for Eurorack.

## Development

``` sh
# run formatting, linter and unit tests
make

# run benchmark
cargo bench --bench bench

# run the module via pure data
make puredata

# profiling example
rm -f target/release/deps/bench-*
rm -f callgrind.out.*
RUSTFLAGS="-g" cargo bench --bench bench --no-run
BENCH=$(find target/release/deps -type f -executable -name 'bench-*')
TEST=oscillator
valgrind \
    --tool=callgrind \
    --dump-instr=yes \
    --collect-jumps=yes \
    --simulate-cache=yes \
    ${BENCH} --bench --profile-time 10 ${TEST}
kcachegrind callgrind.out.*
```


# License

Software of Achordion is distributed under the terms of the General Public
License version 3. See [LICENSE-SOFTWARE](LICENSE-SOFTWARE) for details.

Schematics and PCB layout are distributed under the terms of Creative Commons
BY-SA. See [LICENSE-HARDWARE](LICENSE-HARDWARE) for details. Parts of the
schematics are based on [Emilie Gillet's Mutable
Instruments](https://github.com/pichenettes/eurorack), kudos to her for making
them open.
