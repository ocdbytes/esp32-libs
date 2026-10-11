# Architecture

## Layers

The display driver and the drawing primitives come from existing crates. This
repository adds the frame loop, the simulator, and the scenes.

| Layer | Crate | Job |
| ----- | ----- | --- |
| Driver | [`ssd1306`](https://crates.io/crates/ssd1306) | Talks I²C to the panel and holds the 1 KB frame buffer |
| Drawing | [`embedded-graphics`](https://crates.io/crates/embedded-graphics) | Circles, lines, rectangles, text, bitmaps |
| Frame loop + preview | [`oled-frames`](../oled-frames) | The `Scene` trait and the desktop simulator |
| Scenes | [`demos/scenes/*`](../demos/scenes) | What is drawn, e.g. the blob |
| Firmware | [`demos/firmware/*`](../demos/firmware) | ESP32 binaries that run a scene on the panel |

## Crate graph

```
demos/firmware/blob (ESP32 binary)
├── blob        (scene)
│   └── oled-frames
├── oled-frames
├── ssd1306     (driver)
└── esp-hal     (ESP32 hardware access)

demos/scenes/blob/examples/preview.rs (desktop binary)
└── blob + oled-frames with the `sim` feature
```

`oled-frames` and every scene crate are `no_std`, so the same code compiles
for the desktop and for the ESP32. Only the `sim` feature pulls in `std` and
SDL2, and firmware never enables it.

## The `Scene` trait

```rust
pub trait Scene {
    /// Advances the state by one tick.
    fn update(&mut self);

    /// Draws the current state onto `target`, which has already been cleared.
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;

    /// update(), clear the target, draw(). Provided for you.
    fn frame<D>(&mut self, target: &mut D) -> Result<(), D::Error> { ... }
}
```

A scene holds its own state, changes it in `update`, and draws it in `draw`.
`update` takes no time argument: one call is one tick, and the runner's frame
rate decides how long a tick lasts.

## The frame loop

Every frame the runner calls `scene.frame(&mut target)`. Only the target
differs between the desktop and the board:

```
                      scene.frame(&mut target)
                     update  →  clear  →  draw
                                  │
            ┌─────────────────────┴─────────────────────┐
            ▼                                           ▼
  ESP32: ssd1306 display                     Desktop: SimulatorDisplay
  then display.flush() sends it over I²C     shown in an SDL window
```

Both targets implement `embedded-graphics`'s
`DrawTarget<Color = BinaryColor>`, so scene code doesn't change between them.

## Why firmware is outside the workspace

The workspace (`oled-frames`, `demos/scenes/*`) builds with the normal Rust
toolchain for your computer. Firmware needs the Xtensa `esp` toolchain, the
`xtensa-esp32-none-elf` target, `build-std`, and a linker script. Each
`demos/firmware/<name>/` directory sets these in its own `rust-toolchain.toml`,
`.cargo/config.toml` and `build.rs`. The root `Cargo.toml` excludes
`demos/firmware/` so the two setups don't mix.
