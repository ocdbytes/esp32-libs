//! Frame-based drawing for small displays: monochrome OLEDs and color LCDs.
//!
//! Write a [`Scene`] once and run it on anything that accepts embedded-graphics
//! pixels of the scene's color type: on the panel through a driver crate such
//! as `ssd1306` (monochrome OLED) or `mipidsi` (color TFT LCD), or in a
//! desktop window with the `sim` feature.
//!
//! The drivers and the drawing primitives (lines, circles, text, images) come
//! from those crates. This crate only adds the per-frame loop on top.
#![no_std]
#![warn(missing_docs)]

use embedded_graphics::prelude::*;

/// Drawable state that advances by one fixed step per frame.
pub trait Scene {
    /// Pixel color: `BinaryColor` for monochrome panels, `Rgb565` for most
    /// color LCDs.
    ///
    /// Each frame starts cleared to `Color::default()`: off for
    /// `BinaryColor`, black for RGB colors.
    type Color: PixelColor + Default;

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
        D: DrawTarget<Color = Self::Color>;

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
        D: DrawTarget<Color = Self::Color>,
    {
        self.update();
        target.clear(Self::Color::default())?;
        self.draw(target)
    }
}

/// Desktop window that previews scenes like the real panel.
#[cfg(feature = "sim")]
pub mod sim {
    use embedded_graphics::{
        pixelcolor::{Rgb888, raw::RawData},
        prelude::*,
    };
    use embedded_graphics_simulator::{
        BinaryColorTheme, OutputSettings, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
        Window,
    };

    use crate::Scene;

    /// Window pixels per panel pixel. Raise it if the window is too small.
    const SCALE: u32 = 2;
    /// Dark gap between monochrome pixels, in window pixels, like an OLED.
    const OLED_PIXEL_SPACING: u32 = 1;

    /// Opens a window showing a `size` panel and runs `scene` at up to `fps`
    /// frames per second.
    ///
    /// Monochrome scenes look like a blue OLED; color scenes show their true
    /// colors. Returns when the window is closed or Escape is pressed. Use the
    /// panel size and frame rate of the real hardware so the preview matches.
    pub fn run<S>(mut scene: S, size: Size, fps: u32)
    where
        S: Scene,
        S::Color: Into<Rgb888> + From<Rgb888>,
    {
        let mut display = SimulatorDisplay::with_default_color(size, S::Color::default());
        let mut window = Window::new("oled-frames", &settings::<S::Color>());
        window.set_max_fps(fps);
        loop {
            let Ok(()) = scene.frame(&mut display);
            window.update(&display);
            if window.events().any(|e| e == SimulatorEvent::Quit) {
                return;
            }
        }
    }

    /// Returns the window settings for color type `C`.
    ///
    /// The OLED theme only makes sense for 1-bit colors; for anything else it
    /// would distort the output, so color scenes use the default theme.
    fn settings<C: PixelColor>() -> OutputSettings {
        let builder = OutputSettingsBuilder::new().scale(SCALE);
        if C::Raw::BITS_PER_PIXEL == 1 {
            builder
                .theme(BinaryColorTheme::OledBlue)
                .pixel_spacing(OLED_PIXEL_SPACING)
                .build()
        } else {
            builder.build()
        }
    }
}

#[cfg(test)]
mod tests {
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565},
    };

    use super::*;

    /// Lights one pixel that moves right by one each tick.
    struct Dot<C>(i32, C);

    impl<C: PixelColor + Default> Scene for Dot<C> {
        type Color = C;

        fn update(&mut self) {
            self.0 += 1;
        }

        fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
        where
            D: DrawTarget<Color = C>,
        {
            Pixel(Point::new(self.0, 0), self.1).draw(target)
        }
    }

    /// Runs two frames of a dot in `color` and checks only the second is left.
    fn assert_frame_clears<C: PixelColor + Default + core::fmt::Debug>(color: C) {
        let mut dot = Dot(-1, color);
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);
        let Ok(()) = dot.frame(&mut display);
        let Ok(()) = dot.frame(&mut display);
        assert_eq!(display.get_pixel(Point::new(0, 0)), Some(C::default()));
        assert_eq!(display.get_pixel(Point::new(1, 0)), Some(color));
    }

    #[test]
    fn frame_clears_previous_frame() {
        assert_frame_clears(BinaryColor::On);
        assert_frame_clears(Rgb565::RED);
    }
}
