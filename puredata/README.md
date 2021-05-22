# Achordion Pure Data

This Pure Data external is meant for simple testing of the module without a need
to flash it to the real thing.

Build and install on Linux with:

```sh
mkdir -p ~/.local/lib/pd/extra
cargo build --release && cp target/release/libachordion_pd.so ~/.local/lib/pd/extra/achordion~.pd_linux
```

