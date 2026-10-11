# Build

All commands run from the firmware directory.

```sh
cd demos/firmware/blob
source ~/export-esp.sh
```

## Release build

```sh
cargo build --release
```

This is the build you flash. Output:

```
demos/firmware/blob/target/xtensa-esp32-none-elf/release/blob-firmware
```

The file is an ELF of about 2 MB, most of it debug symbols. The part that goes
into flash is about 110 KB (see [image size](./debugging.md#image-size)).

| Firmware directory | Binary | Display | Flash size |
| ------------------ | ------ | ------- | ---------- |
| `demos/firmware/blob` | `blob-firmware` | SSD1306 OLED | ~110 KB |
| `demos/firmware/blob-lcd` | `blob-lcd-firmware` | ILI9341 TFT LCD | ~94 KB |

The LCD firmware also uses 150 KB of RAM for its frame buffer
([why](../../docs/hardware-lcd.md#why-the-firmware-has-a-frame-buffer)).

## Debug build

```sh
cargo build
```

Output: `target/xtensa-esp32-none-elf/debug/blob-firmware`. The debug profile
still uses `opt-level = "s"`, because unoptimized code is too slow and too big
for the ESP32.

## Faster checks

```sh
cargo check                          # type-check only, no linking
cargo clippy --release -- -D warnings
cargo fmt --check
```

`cargo check` and `cargo clippy` don't link, so they work without
`source ~/export-esp.sh`.

## What the build does

1. `rust-toolchain.toml` selects the `esp` toolchain.
2. `.cargo/config.toml` sets the target to `xtensa-esp32-none-elf` and
   rebuilds `core` for it (`build-std = ["core"]`).
3. `build.rs` adds the `linkall.x` linker script from `esp-hal`.
4. `xtensa-esp32-elf-gcc` (from `~/export-esp.sh`) links the ELF.

## Libraries for the ESP32

The libraries in the root workspace build for the desktop by default. To
check that one still compiles for the board, run from the repository root:

```sh
cargo +esp build -p oled-frames --target xtensa-esp32-none-elf -Z build-std=core --release
cargo +esp build -p blob --target xtensa-esp32-none-elf -Z build-std=core --release
```

## Firmware for another scene

Copy `demos/firmware/blob`, then change the package name and the scene dependency.
Step-by-step: [docs/writing-scenes.md](../../docs/writing-scenes.md#4-run-on-the-board).

## Editor support

rust-analyzer started at the repository root only sees the workspace, not
`demos/firmware/`. To get completions in firmware code, open the firmware directory
(for example `demos/firmware/blob`) as its own folder in your editor.
