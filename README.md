<p align="center">
  <img src="./docs/assets/blob.gif" alt="A round blob with two eyes hopping across a simulated blue OLED and blinking" width="511">
</p>

<h1 align="center">esp32-libs</h1>
<p align="center"><em>Rust libraries and demos for ESP32 boards with monochrome OLED displays.</em></p>

Draw on an SSD1306 OLED (128×64, 128×32 and other sizes) one frame at a time.
Write a scene once, preview it in a desktop window that looks like the panel,
then flash the same code to an ESP32.

## What's here

| Path | What it is | Builds for |
| ---- | ---------- | ---------- |
| [`oled-frames/`](./oled-frames) | Library: the `Scene` trait and the desktop simulator | Desktop, ESP32 |
| [`demos/`](./demos) | Everything built on the library; start with its README | |
| [`demos/scenes/blob/`](./demos/scenes/blob) | Demo scene: a blob that hops and blinks | Desktop, ESP32 |
| [`demos/firmware/blob/`](./demos/firmware/blob) | ESP32 firmware that shows the blob on the OLED | ESP32 |
| [`docs/`](./docs) | How the library works and how to write scenes | |
| [`rust-esp32-book/`](./rust-esp32-book) | Toolchain setup, building, flashing, debugging | |

## Quick start

Preview the blob on your computer (needs Rust 1.85+ and CMake):

```sh
cargo run -p blob --example preview --features sim
```

Run it on the board (one-time toolchain setup:
[rust-esp32-book/src/setup.md](./rust-esp32-book/src/setup.md)):

```sh
cd demos/firmware/blob
source ~/export-esp.sh
cargo run --release     # build, flash, and open the serial monitor
```

## Documentation

- **[docs/](./docs/README.md)**: library guide covering architecture, writing
  scenes, the simulator, hardware wiring, the blob demo, and testing.
- **[rust-esp32-book/](./rust-esp32-book/README.md)**: installing the ESP32
  toolchain, building firmware, flashing, monitoring, and troubleshooting.

## Repository layout

```
esp32-libs/
├── Cargo.toml              Workspace: oled-frames + demos/scenes/*
├── .cargo/config.toml      Desktop build settings (CMake 4 fix for SDL2)
├── oled-frames/            The library
├── demos/                  Everything built on the library
│   ├── scenes/blob/        Scene crate + desktop preview example
│   └── firmware/blob/      ESP32 binary, outside the workspace (esp toolchain)
├── docs/                   Library documentation
└── rust-esp32-book/        Build and flash guide (mdBook)
```

Firmware crates are excluded from the workspace because they use the Xtensa
`esp` toolchain and their own target. Everything else builds with normal Rust
on your computer.

## Development

From the repository root:

```sh
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --all --check
```

Firmware checks run from `demos/firmware/<name>/`. See
[docs/testing.md](./docs/testing.md).
