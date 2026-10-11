# Hardware: monochrome OLED

For the color LCD, see [Hardware: color TFT LCD](./hardware-lcd.md).

Target setup: nodemcu-32s (Xtensa ESP32), 128×64 SSD1306 OLED over I²C,
[`esp-hal`](https://github.com/esp-rs/esp-hal) 1.1, `ssd1306` 0.9.

`demos/firmware/blob` builds, links, and converts to a valid ESP32 flash image. It
has not been run on a board yet.

## Wiring

| OLED pin | ESP32  |
| -------- | ------ |
| VCC      | 3V3    |
| GND      | GND    |
| SDA      | GPIO21 |
| SCL      | GPIO22 |

## I²C address

`ssd1306`'s `I2CDisplayInterface::new` uses address `0x3C`, the usual one for
these panels. If the panel stays dark, it may be wired for `0x3D`. Use
`I2CDisplayInterface::new_alternate_address` instead.

## Firmware main loop

The full program is
[`demos/firmware/blob/src/main.rs`](../demos/firmware/blob/src/main.rs). After setting up
I²C and the display, it runs:

```rust
let mut blob = Blob::new(display.bounding_box().size); // size from the driver
loop {
    let start = Instant::now();
    blob.frame(&mut display).unwrap();          // update, clear, draw into the buffer
    if let Err(e) = display.flush() {           // send the buffer over I²C
        log::warn!("display flush failed: {e:?}");
    }
    while start.elapsed() < FRAME_TIME {}       // hold ~30 fps
}
```

`display` is the `ssd1306` driver in buffered graphics mode. It implements
`DrawTarget<Color = BinaryColor>`, so it can be passed straight to
`Scene::frame`.

## Other panel sizes

The panel size is set in one place in the firmware, the `DisplaySize` type
passed to `Ssd1306::new`:

```rust
let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
    .into_buffered_graphics_mode();
```

`ssd1306` provides `DisplaySize128x64`, `DisplaySize128x32`,
`DisplaySize96x16`, `DisplaySize72x40`, `DisplaySize64x48` and
`DisplaySize64x32`. The scene gets its size from
`display.bounding_box().size`, which also follows rotation: a 128×64 panel
with `Rotate90` gives 64×128. Set the same size in the desktop preview (see
[simulator](./simulator.md#panel-size)).

## Frame rate

`flush()` sends the frame over I²C. A full 128×64 frame is 1024 bytes, and
each byte takes 9 clock cycles on the bus (8 data bits plus an acknowledge),
so at 400 kHz one frame needs roughly 23 ms of bus time. This is an estimate,
not a measurement. It puts 30 fps (33 ms per frame) close to the limit. If
the board can't keep up, raise `FRAME_TIME` in the firmware and lower the
preview's fps to match.

## Using the library in your own firmware

Add the crates to your firmware's `Cargo.toml`. Paths are relative to the
firmware crate; these are for one in `demos/firmware/<name>/`:

```toml
[dependencies]
oled-frames = { path = "../../../oled-frames" } # no `sim` feature on firmware
blob = { path = "../../scenes/blob" }            # or your own scene crate
embedded-graphics = "0.8"
ssd1306 = "0.9"
```

For the rest of the project setup (toolchain, target, linker script), copy
`demos/firmware/blob` or follow the
[Rust ESP32 book](../rust-esp32-book/README.md).
