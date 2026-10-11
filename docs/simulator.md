# Simulator

`oled_frames::sim::run(scene, size, fps)` opens a window that looks like the
panel and runs the scene until you close the window or press **Esc**:

- **Monochrome scenes** (`BinaryColor`) look like a blue OLED, with a dark gap
  between pixels.
- **Color scenes** (e.g. `Rgb565`) show their true colors with no gaps, like an
  LCD.
It is built on
[`embedded-graphics-simulator`](https://crates.io/crates/embedded-graphics-simulator).

```sh
cargo run -p blob --example preview --features sim       # 128x64 OLED
cargo run -p blob --example preview_lcd --features sim   # 240x320 color LCD
```

## Requirements

- Rust 1.85 or newer
- CMake and a C compiler. On macOS: `brew install cmake` and
  `xcode-select --install`.

SDL2 is compiled from source and linked statically (`sdl2` crate features
`bundled` and `static-link`), so you do not need SDL2 installed. The first
build takes about 30 seconds while SDL2 compiles.

## Settings

| Setting | Value | Where |
| ------- | ----- | ----- |
| Panel size | `size` argument to `run` (blob preview: `SIZE = 128×64`) | the preview's `main` |
| Colors | Monochrome: `OledBlue` theme (cyan on dark blue). Color: true colors | `oled-frames/src/lib.rs` |
| Pixel size | `SCALE = 2` window pixels per panel pixel | `oled-frames/src/lib.rs` |
| Pixel gap | Monochrome: `OLED_PIXEL_SPACING = 1` window pixel. Color: none | `oled-frames/src/lib.rs` |
| Frame rate | `fps` argument to `run` | the preview's `main` |

## The `sim` feature

`oled-frames` has a `sim` feature that adds `std`, the simulator, and SDL2.
Scene crates forward it (`sim = ["oled-frames/sim"]`), and their preview
example requires it:

```toml
[[example]]
name = "preview"
required-features = ["sim"]
```

Because of `required-features`, `cargo test` without `--features sim` skips the
preview and doesn't build SDL2. Firmware never enables `sim`.

## Troubleshooting

**`CMake Error ... Compatibility with CMake < 3.5 has been removed from CMake`**
SDL2's bundled build file predates CMake 4. The root `.cargo/config.toml`
sets `CMAKE_POLICY_VERSION_MINIMUM=3.5`, which fixes this. If you build
outside this repository, set that environment variable yourself.

**`failed to run custom build command for sdl2-sys`**
Usually CMake or a C compiler is missing. Install them as described in
[Requirements](#requirements).

**`unresolved import sim` when building a scene crate**
The example needs the `sim` feature. Run it with `--features sim`, and keep the
`[[example]] required-features = ["sim"]` entry in the crate's `Cargo.toml`.

**The firmware build tries to compile SDL2**
A firmware dependency has `features = ["sim"]`. Remove it.

## Panel size

Pass your panel's size to `run`, and to the scene if it needs it:

```rust
const SIZE: Size = Size::new(128, 32);

fn main() {
    sim::run(Blob::new(SIZE), SIZE, 30);
}
```

The window grows or shrinks to match: a 240×320 LCD opens a 480×640 window.
Any size works; the `ssd1306` crate supports 128×64, 128×32, 96×16, 72×40,
64×48 and 64×32 panels.

## Limitations

- Only tested on macOS.
