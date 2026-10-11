# Testing

## Workspace (desktop)

From the repository root:

```sh
cargo test --workspace                                                # all tests
cargo clippy --workspace --all-targets --all-features -- -D warnings  # lints, incl. previews
cargo fmt --all --check                                               # formatting
```

`cargo test` without features doesn't build SDL2. `--all-features` turns on
`sim`, so clippy also checks the preview examples (and builds SDL2 the first
time).

| Crate | Test | Checks |
| ----- | ---- | ------ |
| `oled-frames` | `frame_clears_previous_frame` | `frame()` clears the last frame before drawing |
| `blob` | `stays_on_screen` | The body stays fully on screen for 10,000 ticks, on every `ssd1306` panel size |
| `blob` | `eyes_light_up_while_blinking` | Eyes are unlit when open and lit during a blink |

## Libraries on the ESP32 target

Checks that a library still compiles for the board. Needs the `esp`
toolchain (see [setup](../rust-esp32-book/src/setup.md)):

```sh
cargo +esp build -p oled-frames --target xtensa-esp32-none-elf -Z build-std=core --release
cargo +esp build -p blob --target xtensa-esp32-none-elf -Z build-std=core --release
```

## Firmware

From `demos/firmware/<name>/`, with `source ~/export-esp.sh` run first:

```sh
cargo build --release
cargo clippy --release -- -D warnings
cargo fmt --check
```

Firmware has no on-device tests. Put logic in the scene crate and test it
there.
