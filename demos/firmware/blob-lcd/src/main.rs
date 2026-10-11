//! Runs the blob scene in color on an ESP32 with a 2.8" 240x320 ILI9341 TFT
//! LCD over SPI.
//!
//! Wiring (LCD -> ESP32): VCC -> 3V3, GND -> GND, CS -> GPIO5,
//! RESET -> GPIO17, DC -> GPIO16, SDI (MOSI) -> GPIO23, SCK -> GPIO18,
//! LED -> 3V3. SDO (MISO) and the touch pins stay unconnected.
//! Build and flash: see `rust-esp32-book/`.
#![no_std]
#![no_main]

use blob::Blob;
use embedded_graphics::{
    framebuffer::{Framebuffer, buffer_size},
    image::Image,
    pixelcolor::{
        Rgb565, WebColors,
        raw::{BigEndian, RawU16},
    },
    prelude::*,
};
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::time::{Duration, Instant, Rate};
use frames::Scene;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::{ColorInversion, ColorOrder, Orientation};
use static_cell::ConstStaticCell;

/// Frame width in pixels. For landscape, swap with `HEIGHT` and rotate below.
const WIDTH: usize = 240;
/// Frame height in pixels.
const HEIGHT: usize = 320;
/// Panel orientation. Landscape: `Orientation::new().rotate(Rotation::Deg90)`.
const ORIENTATION: Orientation = Orientation::new();
/// Time per frame: ~30 fps, the same rate the desktop preview uses.
const FRAME_TIME: Duration = Duration::from_millis(33);
/// SPI clock. Most modules work at 40 MHz; lower it if the image is garbled.
const SPI_FREQUENCY: Rate = Rate::from_mhz(40);
/// Subpixel order. Flip to `Rgb` if red and blue are swapped.
const COLOR_ORDER: ColorOrder = ColorOrder::Bgr;
/// Color inversion. Flip to `Inverted` if black shows as white.
const COLOR_INVERSION: ColorInversion = ColorInversion::Normal;

/// Off-screen frame the scene draws into; sent to the panel in one write.
type Frame =
    Framebuffer<Rgb565, RawU16, BigEndian, WIDTH, HEIGHT, { buffer_size::<Rgb565>(WIDTH, HEIGHT) }>;

/// The frame is 150 KB, far too big for the stack, so it lives in static RAM.
static FRAME: ConstStaticCell<Frame> = ConstStaticCell::new(Frame::new());

/// Busy-wait delay for the display's reset sequence. `esp_hal::delay` would
/// need esp-hal's `unstable` feature.
struct BusyDelay;

impl DelayNs for BusyDelay {
    fn delay_ns(&mut self, ns: u32) {
        let wait = Duration::from_micros(u64::from(ns.div_ceil(1_000)));
        let start = Instant::now();
        while start.elapsed() < wait {}
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::error!("{info}");
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default().with_frequency(SPI_FREQUENCY),
    )
    .unwrap()
    .with_sck(peripherals.GPIO18)
    .with_mosi(peripherals.GPIO23);
    let cs = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO17, Level::High, OutputConfig::default());
    let spi = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut buffer = [0u8; 512];
    let interface = SpiInterface::new(spi, dc, &mut buffer);
    let mut display = Builder::new(ILI9341Rgb565, interface)
        .reset_pin(rst)
        .orientation(ORIENTATION)
        .color_order(COLOR_ORDER)
        .invert_colors(COLOR_INVERSION)
        .init(&mut BusyDelay)
        .unwrap();
    log::info!("display ready, running blob");

    let frame = FRAME.take();
    let size = frame.bounding_box().size;
    let mut blob = Blob::with_colors(size, Rgb565::CSS_HOT_PINK, Rgb565::BLACK);
    loop {
        let start = Instant::now();
        let Ok(()) = blob.frame(frame);
        // One window write of the whole frame: no flicker from clear-then-draw.
        if let Err(e) = Image::new(&frame.as_image(), Point::zero()).draw(&mut display) {
            log::warn!("display write failed: {e:?}");
        }
        while start.elapsed() < FRAME_TIME {}
    }
}
