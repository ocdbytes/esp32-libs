# Introduction

This book covers the commands to turn a firmware crate in `demos/firmware/` into a
program running on an ESP32.

## Board

| | |
| - | - |
| Board | nodemcu-32s (ESP32, Xtensa) |
| Displays | 128×64 SSD1306 OLED over I²C ([wiring](../../docs/hardware.md)), firmware `demos/firmware/blob` |
| | 2.8" 240×320 ILI9341 TFT LCD over SPI ([wiring](../../docs/hardware-lcd.md)), firmware `demos/firmware/blob-lcd` |
| USB serial chip | Silicon Labs CP210x |
| Serial port (macOS) | usually `/dev/cu.SLAB_USBtoUART` |

## Tools

| Tool | Job |
| ---- | --- |
| `rustup` | Installs and switches Rust toolchains |
| `espup` | Installs the `esp` Rust toolchain for Xtensa, plus its GCC linker and LLVM |
| `espflash` | Flashes firmware and opens the serial monitor |
| `esp-generate` | Creates new `esp-hal` projects (optional) |

## Firmware crates

Each directory in `demos/firmware/` is a standalone Cargo project. It is not part
of the root workspace. These files make it build for the ESP32:

| File | Sets |
| ---- | ---- |
| `rust-toolchain.toml` | `channel = "esp"`: the Xtensa-capable Rust toolchain |
| `.cargo/config.toml` | Target `xtensa-esp32-none-elf`, `build-std = ["core"]`, the `espflash` runner, `ESP_LOG` level |
| `build.rs` | Links with `linkall.x` and prints hints for common linker errors |

So all commands in this book run from inside the firmware directory. The
examples use `demos/firmware/blob/`; for the LCD, use
`demos/firmware/blob-lcd/` and the binary name `blob-lcd-firmware`.

## Reference

- [The Rust on ESP Book](https://docs.esp-rs.org/book/): the official esp-rs
  guide
- [`esp-hal`](https://github.com/esp-rs/esp-hal)
- [`espflash`](https://github.com/esp-rs/espflash)
