# Architecture

## Layers

The display driver and the drawing primitives come from existing crates. This
repository adds the frame loop, the simulator, and the scenes.

| Layer | Crate | Job |
| ----- | ----- | --- |
| Driver (OLED) | [`ssd1306`](https://crates.io/crates/ssd1306) | Talks I²C to a monochrome OLED and holds its 1 KB frame buffer |
| Driver (LCD) | [`mipidsi`](https://crates.io/crates/mipidsi) | Talks SPI to a color TFT LCD (ILI9341, ST7789, ...) |
| Drawing | [`embedded-graphics`](https://crates.io/crates/embedded-graphics) | Circles, lines, rectangles, text, bitmaps |
| Frame loop + preview | [`oled-frames`](../oled-frames) | The `Scene` trait and the desktop simulator |
| Scenes | [`demos/scenes/*`](../demos/scenes) | What is drawn, e.g. the blob |
| Firmware | [`demos/firmware/*`](../demos/firmware) | ESP32 binaries that run a scene on the panel |

## Crate graph

```
demos/firmware/blob (ESP32, OLED)      demos/firmware/blob-lcd (ESP32, LCD)
├── blob        (scene)                ├── blob        (scene)
│   └── oled-frames                    │   └── oled-frames
├── oled-frames                        ├── oled-frames
├── ssd1306     (driver)               ├── mipidsi     (driver)
└── esp-hal     (hardware access)      └── esp-hal     (hardware access)

demos/scenes/blob/examples/preview.rs, preview_lcd.rs (desktop binaries)
└── blob + oled-frames with the `sim` feature
```

`oled-frames` and every scene crate are `no_std`, so the same code compiles
for the desktop and for the ESP32. Only the `sim` feature pulls in `std` and
SDL2, and firmware never enables it.

## The `Scene` trait

```rust
pub trait Scene {
    /// Pixel color: `BinaryColor` for monochrome panels, `Rgb565` for most
    /// color LCDs.
    type Color: PixelColor + Default;

    /// Advances the state by one tick.
    fn update(&mut self);

    /// Draws the current state onto `target`, which has already been cleared.
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;

    /// update(), clear the target to `Color::default()`, draw(). Provided for you.
    fn frame<D>(&mut self, target: &mut D) -> Result<(), D::Error> { ... }
}
```

A scene holds its own state, changes it in `update`, and draws it in `draw`.
`update` takes no time argument: one call is one tick, and the runner's frame
rate decides how long a tick lasts.

`Color` is the scene's pixel type, the same idea as `DrawTarget::Color` in
`embedded-graphics`. Each frame starts cleared to `Color::default()`: off for
`BinaryColor`, black for RGB colors. A scene can be generic over its color, as
`Blob<C>` is, to run on both kinds of panel.

## The frame loop

Every frame the runner calls `scene.frame(&mut target)`. Only the target
differs between the desktop and the board:

```
                           scene.frame(&mut target)
                          update  →  clear  →  draw
                                       │
         ┌─────────────────────────────┼─────────────────────────────┐
         ▼                             ▼                             ▼
  OLED: ssd1306 display      LCD: Framebuffer in RAM        Desktop: SimulatorDisplay
  display.flush() sends      one Image draw sends it        shown in an SDL window
  it over I²C                to mipidsi over SPI
```

All targets implement `embedded-graphics`'s `DrawTarget` for the scene's
color, so scene code doesn't change between them. The LCD firmware keeps its
own frame buffer because `mipidsi` writes straight to the panel; see
[Hardware: color TFT LCD](./hardware-lcd.md#why-the-firmware-has-a-frame-buffer).

## Why firmware is outside the workspace

The workspace (`oled-frames`, `demos/scenes/*`) builds with the normal Rust
toolchain for your computer. Firmware needs the Xtensa `esp` toolchain, the
`xtensa-esp32-none-elf` target, `build-std`, and a linker script. Each
`demos/firmware/<name>/` directory sets these in its own `rust-toolchain.toml`,
`.cargo/config.toml` and `build.rs`. The root `Cargo.toml` excludes
`demos/firmware/` so the two setups don't mix.
