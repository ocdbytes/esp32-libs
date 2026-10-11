# oled-frames

Frame-based drawing for monochrome OLED displays, `no_std`.

Implement `Scene` (`update` advances one tick, `draw` paints onto an
`embedded-graphics` target), then call `scene.frame(&mut target)` once per
frame:

- **ESP32:** the target is the `ssd1306` display in buffered graphics mode;
  call `display.flush()` after each frame.
- **Desktop:** enable the `sim` feature and call
  `oled_frames::sim::run(scene, size, fps)` to open a window that looks like a
  blue OLED of that size.

```toml
[dependencies]
oled-frames = { path = "../oled-frames" }                     # firmware
oled-frames = { path = "../oled-frames", features = ["sim"] } # desktop preview
```

## Documentation

- [Architecture](../docs/architecture.md)
- [Writing scenes](../docs/writing-scenes.md)
- [Simulator](../docs/simulator.md)
- [Hardware](../docs/hardware.md)
- [Building and flashing firmware](../rust-esp32-book/README.md)
