# Hardware: color TFT LCD

Target setup: nodemcu-32s (Xtensa ESP32), 2.8" 240×320 TFT LCD over SPI with
an ILI9341 controller, [`esp-hal`](https://github.com/esp-rs/esp-hal) 1.1,
[`mipidsi`](https://crates.io/crates/mipidsi) 0.10.

`demos/firmware/blob-lcd` builds, links, and converts to a valid ESP32 flash
image. It has not been run on a board yet, so expect to adjust the
[color settings](#colors) on first boot.

## Wiring

| LCD pin | ESP32 | Notes |
| ------- | ----- | ----- |
| VCC | 3V3 | |
| GND | GND | |
| CS | GPIO5 | Chip select |
| RESET | GPIO17 | |
| DC | GPIO16 | Data/command; sometimes labelled D/C or RS |
| SDI (MOSI) | GPIO23 | Data to the panel |
| SCK | GPIO18 | SPI clock |
| LED | 3V3 | Backlight, always on |
| SDO (MISO) | not connected | The firmware never reads from the panel |
| T_CLK, T_CS, T_DIN, T_DO, T_IRQ | not connected | Touch controller, unused |

The pins are set in `demos/firmware/blob-lcd/src/main.rs`.

## Controller

2.8" 240×320 SPI modules almost always use an ILI9341; some use an ST7789.
Check the seller's listing. For an ST7789, change the model in `main.rs`:

```rust
use mipidsi::models::ST7789;
// ...
let mut display = Builder::new(ST7789, interface)
```

then check the colors as below.

## Colors

Panels differ in subpixel order and color inversion, so `main.rs` has two
settings to flip if the colors look wrong:

| What you see | Change |
| ------------ | ------ |
| Red and blue swapped (the blob is blue-ish, not pink) | `COLOR_ORDER`: `Bgr` ↔ `Rgb` |
| Black background shows as white, colors look like a negative | `COLOR_INVERSION`: `Normal` ↔ `Inverted` |

The defaults are `Bgr` and `Normal`. To check them, draw `mipidsi`'s test
image once after `init`:

```rust
mipidsi::TestImage::new().draw(&mut display).unwrap();
```

and compare it with the
[`mipidsi` troubleshooting guide](https://docs.rs/mipidsi/latest/mipidsi/_troubleshooting/index.html).

## Orientation

The default is portrait, 240×320. For landscape, swap the frame size and
rotate the panel in `main.rs`:

```rust
use mipidsi::options::{ColorInversion, ColorOrder, Orientation, Rotation};

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const ORIENTATION: Orientation = Orientation::new().rotate(Rotation::Deg90);
```

The scene gets its size from the frame, so the blob adapts. Set the same size
in the desktop preview (`SIZE` in `examples/preview_lcd.rs`).

## Why the firmware has a frame buffer

The OLED driver (`ssd1306`) keeps the frame in RAM and sends it on `flush()`.
`mipidsi` instead writes every draw call straight to the panel. Clearing and
redrawing directly on the panel each frame would flicker, and send 150 KB of
black every frame before the scene is drawn.

So the firmware draws each frame into its own buffer, an `embedded-graphics`
`Framebuffer` of 240 × 320 × 2 bytes = 150 KB, then sends it to the panel in
one write:

```rust
let Ok(()) = blob.frame(frame);                                    // update, clear, draw in RAM
Image::new(&frame.as_image(), Point::zero()).draw(&mut display)?;  // one write to the panel
```

The buffer is a `static` (via `static_cell::ConstStaticCell`) because it is far
too big for the stack. In the built firmware it takes 153,601 bytes of the
ESP32's 192 KB main data RAM (`dram_seg` in `esp-hal`'s `ld/esp32/memory.x`),
leaving about 35 KB for the stack. That leaves little room for features that
need a heap, such as Wi-Fi. If you need that RAM, render the frame in
horizontal bands instead, at the cost of drawing the scene once per band.

## Frame rate

A full frame is 153,600 bytes. At the 40 MHz SPI clock that is about 31 ms of
bus time (153,600 × 8 / 40,000,000), so 30 fps (33 ms per frame) is at the
limit. This is an estimate, not a measurement.

- **Image garbled:** lower `SPI_FREQUENCY` (e.g. 20 MHz) and raise
  `FRAME_TIME` to match.
- **Motion slower than the preview:** the bus can't keep up; raise
  `FRAME_TIME` and lower the preview's fps to match.
