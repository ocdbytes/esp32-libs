# frames

Frame-based drawing for small displays, monochrome OLEDs and color LCDs,
`no_std`.

Implement `Scene` (`type Color` picks the pixel type, `update` advances one
tick, `draw` paints onto an `embedded-graphics` target), then call
`scene.frame(&mut target)` once per frame:

- **Monochrome OLED:** the target is the `ssd1306` display in buffered
  graphics mode; call `display.flush()` after each frame.
- **Color LCD:** the target is a frame buffer in RAM; draw it to the
  `mipidsi` display after each frame. See
  [Hardware: color TFT LCD](../docs/hardware-lcd.md).
- **Desktop:** enable the `sim` feature and call
  `frames::sim::run(scene, size, fps)` to open a window that looks like
  the panel.

```toml
[dependencies]
frames = { path = "../frames" }                     # firmware
frames = { path = "../frames", features = ["sim"] } # desktop preview
```

## Documentation

- [Architecture](../docs/architecture.md)
- [Writing scenes](../docs/writing-scenes.md)
- [Simulator](../docs/simulator.md)
- [Hardware: OLED](../docs/hardware.md)
- [Hardware: LCD](../docs/hardware-lcd.md)
- [Building and flashing firmware](../rust-esp32-book/README.md)
