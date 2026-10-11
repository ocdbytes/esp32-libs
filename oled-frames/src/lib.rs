//! Frame-based drawing for monochrome OLED displays.
//!
//! Write a [`Scene`] once and run it on anything that accepts embedded-graphics
//! [`BinaryColor`] pixels: on the panel through a driver crate such as
//! `ssd1306` (buffered graphics mode), or in a desktop window with the `sim`
//! feature.
//!
//! The I2C driver and the drawing primitives (lines, circles, text, images)
//! come from those crates. This crate only adds the per-frame loop on top.
#![no_std]
#![warn(missing_docs)]

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};

/// Drawable state that advances by one fixed step per frame.
pub trait Scene {
    /// Advances the state by one tick.
    ///
    /// One tick is one frame period of the runner, so speeds are in pixels per
    /// tick.
    fn update(&mut self);

    /// Draws the current state onto `target`, which has already been cleared.
    ///
    /// # Errors
    ///
    /// Returns the target's error if a draw call fails.
    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;

    /// Renders the next frame: [`update`](Self::update), clear, then
    /// [`draw`](Self::draw).
    ///
    /// The caller still sends the frame to the panel, e.g. `display.flush()`.
    ///
    /// # Errors
    ///
    /// Returns the target's error if clearing or drawing fails.
    fn frame<D>(&mut self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.update();
        target.clear(BinaryColor::Off)?;
        self.draw(target)
    }
}

/// Desktop window that previews scenes as a 128x64 blue OLED.
#[cfg(feature = "sim")]
pub mod sim {
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
    };

    use crate::Scene;

    /// Simulated panel size in pixels.
    pub const SIZE: Size = Size::new(128, 64);

    /// Window pixels per panel pixel. Raise it if the window is too small.
    const SCALE: u32 = 2;
    /// Dark gap between panel pixels, in window pixels.
    const PIXEL_SPACING: u32 = 1;

    /// Opens a window and runs `scene` at up to `fps` frames per second.
    ///
    /// Returns when the window is closed or Escape is pressed. Use the same
    /// `fps` as the firmware loop so motion matches the panel.
    pub fn run<S: Scene>(mut scene: S, fps: u32) {
        let mut display = SimulatorDisplay::<BinaryColor>::new(SIZE);
        let settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .scale(SCALE)
            .pixel_spacing(PIXEL_SPACING)
            .build();
        let mut window = Window::new("oled-frames", &settings);
        window.set_max_fps(fps);
        loop {
            let Ok(()) = scene.frame(&mut display);
            window.update(&display);
            if window.events().any(|e| e == SimulatorEvent::Quit) {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use embedded_graphics::mock_display::MockDisplay;

    use super::*;

    /// Lights one pixel that moves right by one each tick.
    struct Dot(i32);

    impl Scene for Dot {
        fn update(&mut self) {
            self.0 += 1;
        }

        fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
        where
            D: DrawTarget<Color = BinaryColor>,
        {
            Pixel(Point::new(self.0, 0), BinaryColor::On).draw(target)
        }
    }

    #[test]
    fn frame_clears_previous_frame() {
        let mut dot = Dot(-1);
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);
        let Ok(()) = dot.frame(&mut display);
        let Ok(()) = dot.frame(&mut display);
        assert_eq!(display.get_pixel(Point::new(0, 0)), Some(BinaryColor::Off));
        assert_eq!(display.get_pixel(Point::new(1, 0)), Some(BinaryColor::On));
    }
}
