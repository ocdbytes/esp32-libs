# Rust on ESP32: Build and Flash

How to set up the toolchain, build the firmware in `demos/firmware/`, flash it to an
ESP32, and debug it.

Read it on GitHub starting at [src/SUMMARY.md](./src/SUMMARY.md), or as a
website with [mdBook](https://rust-lang.github.io/mdBook/):

```sh
cargo install mdbook --locked
mdbook serve rust-esp32-book --open    # from the repository root
```

Links from the book to `docs/` work on GitHub. In the served book they point
outside it, so open those pages from the repository instead.

## Chapters

1. [Introduction](./src/introduction.md)
2. [Setup](./src/setup.md): one-time toolchain install
3. [Build](./src/build.md): compile firmware for the ESP32
4. [Flash and monitor](./src/flash.md): put it on the board and read its logs
5. [Debugging](./src/debugging.md): ports, board info, image size, erase, reset
6. [Troubleshooting](./src/troubleshooting.md): common errors and fixes
