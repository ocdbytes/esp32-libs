//! Runs the blob scene on an ESP32 with an SSD1306 OLED over I2C.
//!
//! Wiring: SDA = GPIO21, SCL = GPIO22, VCC = 3V3, GND = GND.
//! Build and flash: see `rust-esp32-book/`.
#![no_std]
#![no_main]

use blob::Blob;
use embedded_graphics::prelude::*;
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::main;
use esp_hal::time::{Duration, Instant, Rate};
use frames::Scene;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

/// Time per frame: ~30 fps, the same rate the desktop preview uses.
const FRAME_TIME: Duration = Duration::from_millis(33);

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

    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(400)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22);
    let interface = I2CDisplayInterface::new(i2c);
    // Panel size. Other sizes: DisplaySize128x32, 96x16, 72x40, 64x48, 64x32.
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    log::info!("display ready, running blob");

    let mut blob = Blob::new(display.bounding_box().size);
    loop {
        let start = Instant::now();
        blob.frame(&mut display).unwrap();
        if let Err(e) = display.flush() {
            log::warn!("display flush failed: {e:?}");
        }
        while start.elapsed() < FRAME_TIME {}
    }
}
