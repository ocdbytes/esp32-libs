//! A small round blob with two eyes that hops around and blinks, previewed in
//! a desktop window.
//!
//! Run: `cargo run --example blob --features sim`

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Ellipse, PrimitiveStyle},
};
use oled_frames::{Scene, sim};

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

/// A filled ball whose eyes are unlit holes in the body.
struct Blob {
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

impl Blob {
    /// Creates a blob standing in the bottom-left corner of a `size` screen.
    fn new(size: Size) -> Self {
        Self {
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

impl Scene for Blob {
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
        // Turn around at the side walls.
        let right = self.size.width as f32 - RADIUS;
        if !(RADIUS..=right).contains(&self.x) {
            self.x = self.x.clamp(RADIUS, right);
            self.vx = -self.vx;
        }
    }

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        Circle::with_center(self.center(), BODY_DIAMETER)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(target)?;

        // A blink skips the eyes, so their pixels stay lit like the body.
        if self.tick < BLINK_EVERY - BLINK_LEN {
            let eye = PrimitiveStyle::with_fill(BinaryColor::Off);
            for p in self.eyes() {
                Ellipse::with_center(p, EYE_SIZE)
                    .into_styled(eye)
                    .draw(target)?;
            }
        }
        Ok(())
    }
}

fn main() {
    sim::run(Blob::new(sim::SIZE), 30);
}

#[cfg(test)]
mod tests {
    use embedded_graphics::primitives::Rectangle;
    use embedded_graphics_simulator::SimulatorDisplay;

    use super::*;

    #[test]
    fn stays_on_screen() {
        let mut blob = Blob::new(sim::SIZE);
        let screen = Rectangle::new(Point::zero(), sim::SIZE);
        for _ in 0..10_000 {
            blob.update();
            let body = Circle::with_center(blob.center(), BODY_DIAMETER).bounding_box();
            assert_eq!(screen.intersection(&body), body, "body left the screen");
        }
    }

    #[test]
    fn eyes_light_up_while_blinking() {
        let mut blob = Blob::new(sim::SIZE);
        let mut display = SimulatorDisplay::<BinaryColor>::new(sim::SIZE);

        let Ok(()) = blob.frame(&mut display);
        assert_eq!(
            display.get_pixel(blob.eyes()[0]),
            BinaryColor::Off,
            "eye open"
        );

        for _ in 0..BLINK_EVERY - BLINK_LEN {
            let Ok(()) = blob.frame(&mut display);
        }
        assert_eq!(
            display.get_pixel(blob.eyes()[0]),
            BinaryColor::On,
            "eye shut"
        );
    }
}
