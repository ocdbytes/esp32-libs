//! Shows the blob in a desktop window that looks like the OLED.
//!
//! Run from the workspace root:
//! `cargo run -p blob --example preview --features sim`

use blob::Blob;
use embedded_graphics::prelude::*;
use oled_frames::sim;

/// Panel size in pixels; set it to match your display.
const SIZE: Size = Size::new(128, 64);

fn main() {
    sim::run(Blob::new(SIZE), SIZE, 30);
}
