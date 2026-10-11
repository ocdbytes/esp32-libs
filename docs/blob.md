# Blob demo

<p align="center">
  <img src="./assets/blob.gif" alt="The blob hopping and blinking in the simulator" width="511">
</p>

`demos/scenes/blob` is a 15 px ball with two oval eyes. It walks sideways, hops
under gravity, turns around at the screen edges, and blinks.

- **Desktop:** `cargo run -p blob --example preview --features sim`
- **Board:** `demos/firmware/blob`. See the
  [Rust ESP32 book](../rust-esp32-book/src/flash.md).

## How it works

- **Body:** a filled circle drawn with `BinaryColor::On`.
- **Eyes:** two `BinaryColor::Off` ovals drawn on top of the body, so they are
  unlit holes. They shift 1 px toward the direction of travel.
- **Blink:** for a few ticks per cycle the eyes are not drawn, so their pixels
  stay lit like the rest of the body.
- **Motion:** gravity pulls the blob down each tick. When it reaches the floor
  it takes off again at `HOP_SPEED`. On short screens it bumps the ceiling and
  falls back. Sideways speed flips at the walls.
- **Size:** `Blob::new(size)` takes the screen size. It is tested on every
  panel size `ssd1306` supports, down to 96×16.

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
  64×32).
- `eyes_light_up_while_blinking`: an eye pixel is unlit when the eyes are open
  and lit during a blink.
