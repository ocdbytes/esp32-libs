# Blob demo

<p align="center">
  <img src="./assets/blob.gif" alt="The blob hopping and blinking in the simulator" width="511">
</p>

`demos/scenes/blob` is a 15 px ball with two oval eyes. It walks sideways, hops
under gravity, turns around at the screen edges, and blinks. It runs in
monochrome on an OLED and in color on a TFT LCD.

| | Monochrome OLED | Color LCD |
| - | --------------- | --------- |
| Constructor | `Blob::new(size)` | `Blob::with_colors(size, body, eyes)` |
| Desktop | `cargo run -p blob --example preview --features sim` | `cargo run -p blob --example preview_lcd --features sim` |
| Board | `demos/firmware/blob` | `demos/firmware/blob-lcd` (hot pink body, black eyes) |

Building and flashing: [Rust ESP32 book](../rust-esp32-book/src/flash.md).

## How it works

- **Body:** a filled circle in the body color (`BinaryColor::On` in
  monochrome).
- **Eyes:** two ovals in the eye color drawn on top of the body. In
  monochrome they are `BinaryColor::Off`, so they are unlit holes. They shift
  1 px toward the direction of travel.
- **Blink:** for a few ticks per cycle the eyes are not drawn, so the body
  color shows where they were.
- **Color type:** `Blob<C>` works with any `embedded-graphics` color. `C`
  defaults to `BinaryColor`.
- **Motion:** gravity pulls the blob down each tick. When it reaches the floor
  it takes off again at `HOP_SPEED`. On short screens it bumps the ceiling and
  falls back. Sideways speed flips at the walls.
- **Size:** the constructor takes the screen size. It is tested on every
  panel size `ssd1306` supports, down to 96×16, and on a 240×320 LCD in both
  orientations. The body is always 15 px, so it is small on the LCD.

## Tuning

Constants at the top of `demos/scenes/blob/src/lib.rs`:

| Constant | Default | Effect |
| -------- | ------- | ------ |
| `BODY_DIAMETER` | `15` | Body size in pixels (keep it odd) |
| `EYE_SIZE` | `3 × 5` | Eye width × height (odd keeps eyes symmetric) |
| `EYE_GAP` | `3` | Distance from body center to each eye |
| `EYE_RISE` | `-2` | Eye height relative to body center (negative is up) |
| `BLINK_EVERY` | `90` | Ticks between blinks (3 s at 30 fps) |
| `BLINK_LEN` | `4` | Ticks the eyes stay shut (~130 ms at 30 fps) |
| `GRAVITY` | `0.25` | Downward acceleration, px/tick² |
| `HOP_SPEED` | `4.0` | Take-off speed, px/tick (hop height ≈ 30 px) |
| `WALK_SPEED` | `1.0` | Sideways speed, px/tick |

## Tests

```sh
cargo test -p blob
```

- `stays_on_screen`: over 10,000 ticks the body never leaves the screen, for
  every panel size `ssd1306` supports (128×64, 128×32, 96×16, 72×40, 64×48,
  64×32) and for 240×320 and 320×240.
- `eyes_take_body_color_while_blinking`: an eye pixel shows the eye color when
  open and the body color during a blink, in monochrome and in `Rgb565`.
