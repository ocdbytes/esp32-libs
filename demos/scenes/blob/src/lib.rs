//! A small round blob with two eyes that hops around and blinks.
//!
//! [`Blob`] is an [`oled_frames::Scene`], so the same code runs in the desktop
//! previews (`examples/`) and on the ESP32: monochrome on an OLED
//! (`demos/firmware/blob`), in color on a TFT LCD (`demos/firmware/blob-lcd`).
#![no_std]
#![warn(missing_docs)]

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Ellipse, PrimitiveStyle},
};
use oled_frames::Scene;

/// Body diameter in pixels. Odd, so the circle has a center pixel and reaches
/// equally far on every side; an even one pokes 1 px further right and down.
const BODY_DIAMETER: u32 = 15;
/// Body radius, the closest the center gets to a screen edge.
const RADIUS: f32 = BODY_DIAMETER as f32 / 2.0;
/// Size of each eye in pixels. Odd sizes keep the two eyes mirror images.
const EYE_SIZE: Size = Size::new(3, 5);
/// Horizontal distance from the body center to each eye center.
const EYE_GAP: i32 = 3;
/// Vertical offset of the eyes from the body center (negative is up).
const EYE_RISE: i32 = -2;
/// Ticks from one blink to the next (3 s at 30 fps).
const BLINK_EVERY: u32 = 90;
/// Ticks the eyes stay shut (about 130 ms at 30 fps).
const BLINK_LEN: u32 = 4;
/// Downward acceleration in pixels per tick squared.
const GRAVITY: f32 = 0.25;
/// Upward speed at take-off in pixels per tick; sets the hop height.
const HOP_SPEED: f32 = 4.0;
/// Sideways speed in pixels per tick.
const WALK_SPEED: f32 = 1.0;

/// A filled ball with two eyes, drawn in colors of type `C`.
///
/// On a monochrome panel the body is lit and the eyes are unlit holes
/// ([`Blob::new`]). On a color panel pick both colors ([`Blob::with_colors`]).
#[derive(Debug)]
pub struct Blob<C = BinaryColor> {
    /// Body color; also what shows through the eyes during a blink.
    body: C,
    /// Eye color.
    eyes: C,
    /// Screen size, for the floor and walls.
    size: Size,
    /// Body center in pixels.
    x: f32,
    y: f32,
    /// Velocity in pixels per tick.
    vx: f32,
    vy: f32,
    /// Position in the blink cycle, `0..BLINK_EVERY`.
    tick: u32,
}

impl Blob<BinaryColor> {
    /// Creates a monochrome blob, lit body with unlit eyes, standing in the
    /// bottom-left corner of a `size` screen.
    pub fn new(size: Size) -> Self {
        Self::with_colors(size, BinaryColor::On, BinaryColor::Off)
    }
}

impl<C> Blob<C> {
    /// Creates a blob with the given colors, standing in the bottom-left
    /// corner of a `size` screen.
    pub fn with_colors(size: Size, body: C, eyes: C) -> Self {
        Self {
            body,
            eyes,
            size,
            x: RADIUS,
            y: size.height as f32 - RADIUS,
            vx: WALK_SPEED,
            vy: 0.0,
            tick: 0,
        }
    }

    /// Returns the body center in whole pixels.
    fn center(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }

    /// Returns the eye centers, nudged one pixel toward the walking direction.
    fn eyes(&self) -> [Point; 2] {
        let look = if self.vx > 0.0 { 1 } else { -1 };
        [-EYE_GAP, EYE_GAP].map(|dx| self.center() + Point::new(dx + look, EYE_RISE))
    }
}

impl<C: PixelColor + Default> Scene for Blob<C> {
    type Color = C;

    fn update(&mut self) {
        self.tick = (self.tick + 1) % BLINK_EVERY;

        self.vy += GRAVITY;
        self.x += self.vx;
        self.y += self.vy;

        // Hop again on landing.
        let floor = self.size.height as f32 - RADIUS;
        if self.y >= floor {
            self.y = floor;
            self.vy = -HOP_SPEED;
        }
        // Bump the ceiling and fall back down; matters on short screens.
        if self.y < RADIUS {
            self.y = RADIUS;
            self.vy = 0.0;
        }
        // Turn around at the side walls.
        let right = self.size.width as f32 - RADIUS;
        if !(RADIUS..=right).contains(&self.x) {
            self.x = self.x.clamp(RADIUS, right);
            self.vx = -self.vx;
        }
    }

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        Circle::with_center(self.center(), BODY_DIAMETER)
            .into_styled(PrimitiveStyle::with_fill(self.body))
            .draw(target)?;

        // A blink skips the eyes, so the body color shows where they were.
        if self.tick < BLINK_EVERY - BLINK_LEN {
            let eye = PrimitiveStyle::with_fill(self.eyes);
            for p in self.eyes() {
                Ellipse::with_center(p, EYE_SIZE)
                    .into_styled(eye)
                    .draw(target)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565, primitives::Rectangle};

    use super::*;

    #[test]
    fn stays_on_screen() {
        // Every panel size the `ssd1306` crate supports, plus a 240x320 LCD
        // in portrait and landscape.
        let sizes = [
            (128, 64),
            (128, 32),
            (96, 16),
            (72, 40),
            (64, 48),
            (64, 32),
            (240, 320),
            (320, 240),
        ];
        for (w, h) in sizes {
            let size = Size::new(w, h);
            let screen = Rectangle::new(Point::zero(), size);
            let mut blob = Blob::new(size);
            for _ in 0..10_000 {
                blob.update();
                let body = Circle::with_center(blob.center(), BODY_DIAMETER).bounding_box();
                assert_eq!(
                    screen.intersection(&body),
                    body,
                    "body left a {w}x{h} screen"
                );
            }
        }
    }

    /// Checks that an eye pixel shows `eyes` while open and `body` mid-blink.
    fn assert_blinks<C: PixelColor + Default + core::fmt::Debug>(body: C, eyes: C) {
        // MockDisplay is fixed at 64x64, so the blob gets a 64x64 screen.
        let mut blob = Blob::with_colors(Size::new(64, 64), body, eyes);
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        let Ok(()) = blob.frame(&mut display);
        let eye = blob.eyes()[0];
        assert_eq!(display.get_pixel(eye), Some(eyes), "eye open");

        for _ in 0..BLINK_EVERY - BLINK_LEN {
            let Ok(()) = blob.frame(&mut display);
        }
        let eye = blob.eyes()[0];
        assert_eq!(display.get_pixel(eye), Some(body), "eye shut");
    }

    #[test]
    fn eyes_take_body_color_while_blinking() {
        assert_blinks(BinaryColor::On, BinaryColor::Off);
        assert_blinks(Rgb565::RED, Rgb565::WHITE);
    }
}
