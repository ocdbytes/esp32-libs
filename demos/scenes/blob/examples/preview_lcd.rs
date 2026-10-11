//! Shows the blob in color on a simulated 240x320 TFT LCD.
//!
//! Run from the workspace root:
//! `cargo run -p blob --example preview_lcd --features sim`

use blob::Blob;
use embedded_graphics::{
    pixelcolor::{Rgb565, WebColors},
    prelude::*,
};
use frames::sim;

/// Panel size in pixels: 240x320 portrait. Use 320x240 for landscape.
const SIZE: Size = Size::new(240, 320);

fn main() {
    let blob = Blob::with_colors(SIZE, Rgb565::CSS_HOT_PINK, Rgb565::BLACK);
    sim::run(blob, SIZE, 30);
}
