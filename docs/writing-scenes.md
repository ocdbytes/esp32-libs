# Writing scenes

A scene is a `no_std` crate in `demos/scenes/<name>/` with a desktop preview
example. To run it on the board, you add a matching `demos/firmware/<name>/`.
`demos/scenes/blob` and `demos/firmware/blob` are the templates.

## 1. Create the scene crate

From the repository root:

```sh
cp -R demos/scenes/blob demos/scenes/slider
```

In `demos/scenes/slider/Cargo.toml`, rename the package:

```toml
name = "slider"
```

The workspace includes every `demos/scenes/*` directory, so no other change is
needed.

## 2. Write the scene

Replace `demos/scenes/slider/src/lib.rs`:

```rust
//! A ball that slides across the screen.
#![no_std]

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use oled_frames::Scene;

/// A ball that slides left to right and wraps around.
#[derive(Default)]
pub struct Slider {
    /// Ticks since the start.
    x: u32,
}

impl Scene for Slider {
    fn update(&mut self) {
        self.x = self.x.wrapping_add(1);
    }

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        // Read the size from the target so the scene fits any panel.
        let size = target.bounding_box().size;
        let center = Point::new((self.x % size.width) as i32, size.height as i32 / 2);
        Circle::with_center(center, 9)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(target)
    }
}
```

And `demos/scenes/slider/examples/preview.rs`:

```rust
use embedded_graphics::prelude::*;
use oled_frames::sim;
use slider::Slider;

fn main() {
    sim::run(Slider::default(), Size::new(128, 64), 30);
}
```

Use your panel's size, e.g. `Size::new(128, 32)`.

## 3. Preview on the desktop

```sh
cargo run -p slider --example preview --features sim
```

## 4. Run on the board

```sh
cp -R demos/firmware/blob demos/firmware/slider
rm -rf demos/firmware/slider/target demos/firmware/slider/Cargo.lock
```

In `demos/firmware/slider/Cargo.toml`, rename the package and point at the new
scene:

```toml
name = "slider-firmware"
# ...
slider = { path = "../../scenes/slider" }   # replaces the `blob` line
```

In `demos/firmware/slider/src/main.rs`, replace `use blob::Blob;` and
`Blob::new(...)` with your scene type and its constructor: `use slider::Slider;`
and `Slider::default()`. The slider doesn't take the screen size, so also
remove the now-unused `use embedded_graphics::prelude::*;`. Then build and
flash as described in the
[Rust ESP32 book](../rust-esp32-book/src/flash.md).

## Guidelines

- **Match the hardware.** Use the same panel size and frame rate in the
  preview (`sim::run(scene, size, 30)`) as in the firmware (`DisplaySize…`,
  `FRAME_TIME`), so the preview looks like the board. Speeds are in pixels per
  tick.
- **Redraw everything in `draw`.** The target is cleared before each `draw`.
- **Unlit pixels are shapes too.** Drawing with `BinaryColor::Off` on top of a
  filled shape cuts a hole in it. The blob's eyes work this way.
- **Don't hard-code the screen size.** In `draw`, use
  `target.bounding_box().size`, as the slider does. If `update` needs the size
  (walls, a floor), pass it to the constructor, as `Blob::new(size)` does.
  Test the scene on more than one size: `demos/scenes/blob` checks every panel size
  `ssd1306` supports.
- **Use odd diameters for round things.** `Circle::with_center` with an even
  diameter reaches 1 px further right and down than left and up. An odd
  diameter has a true center pixel and is symmetric.
- **Stay `no_std`.** Firmware has no heap or `std`, so avoid `Vec`, `String`
  and `println!` in scene code. `f32` arithmetic (`+`, `*`, `clamp`) works and
  the ESP32 has a hardware FPU. Functions like `sqrt`, `sin` and `cos` are not
  available in `no_std`; use the [`libm`](https://crates.io/crates/libm) crate.
- **Test the logic on the desktop.** Scene crates are plain `no_std` libraries,
  so `cargo test -p <name>` runs their tests on your computer.
  `embedded_graphics::mock_display::MockDisplay` (64×64) works as a draw
  target in tests without the simulator. See `demos/scenes/blob/src/lib.rs`.
