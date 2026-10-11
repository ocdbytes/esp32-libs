# Demos

Things built on the [`oled-frames`](../oled-frames) library. Each demo has two
halves with the same name:

| Folder | What it is | Built with |
| ------ | ---------- | ---------- |
| [`scenes/<name>/`](./scenes) | The drawing logic: a `no_std` crate implementing `Scene`, its tests, and a desktop preview | Normal Rust, from the repository root |
| [`firmware/<name>/`](./firmware) | A small ESP32 program that runs the scene on a display | The `esp` toolchain, from inside its own folder |

Put logic in the scene and keep the firmware thin. You can then work on the
scene with the desktop preview and its tests, and only flash the board to
check the result.

## Blob

Two firmwares share the one scene:

| Firmware | Display |
| -------- | ------- |
| `firmware/blob` | 128×64 SSD1306 OLED over I²C, monochrome |
| `firmware/blob-lcd` | 2.8" 240×320 ILI9341 TFT LCD over SPI, color |

```sh
# Desktop previews (from the repository root)
cargo run -p blob --example preview --features sim
cargo run -p blob --example preview_lcd --features sim

# On the board
cd demos/firmware/blob        # or demos/firmware/blob-lcd
source ~/export-esp.sh
cargo run --release
```

- How the blob works: [docs/blob.md](../docs/blob.md)
- Adding a new demo: [docs/writing-scenes.md](../docs/writing-scenes.md)
- Building and flashing: [rust-esp32-book](../rust-esp32-book/README.md)
