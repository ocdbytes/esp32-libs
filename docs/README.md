# Documentation

Guides for the `oled-frames` library and the scenes built on it. For installing
the ESP32 toolchain, building firmware and flashing, see the
[Rust ESP32 book](../rust-esp32-book/README.md).

| Page | Read it to |
| ---- | ---------- |
| [Architecture](./architecture.md) | Understand the crates, the `Scene` trait and the frame loop |
| [Writing scenes](./writing-scenes.md) | Make your own scene and run it on the desktop and the board |
| [Simulator](./simulator.md) | Use and configure the desktop preview window |
| [Hardware: OLED](./hardware.md) | Wire the SSD1306 OLED and understand its firmware main loop |
| [Hardware: LCD](./hardware-lcd.md) | Wire the ILI9341 TFT LCD, fix colors and orientation, frame buffer and frame rate |
| [Blob demo](./blob.md) | See how the blob works and tune it |
| [Testing](./testing.md) | Run tests, lints, and ESP32 build checks |
