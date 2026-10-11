# Troubleshooting

## Build

| Symptom | Fix |
| ------- | --- |
| ``linker `xtensa-esp32-elf-gcc` not found`` | Run `source ~/export-esp.sh` in this terminal ([setup](./setup.md#every-new-terminal)) |
| `custom toolchain 'esp' specified in override file ... is not installed` | Run `espup install` ([setup](./setup.md#2-the-esp-toolchain)) |
| `current package believes it's in a workspace when it's not` | The firmware directory must be under `demos/firmware/`, which the root `Cargo.toml` excludes |
| Firmware build compiles SDL2 | A dependency has `features = ["sim"]`; firmware must not enable `sim` |

## Flash

| Symptom | Fix |
| ------- | --- |
| No serial ports found | Check the USB cable (some are charge-only), run `ls /dev/cu.*`, pass `--port` |
| `/dev/cu.SLAB_USBtoUART` never appears | Install the [CP210x driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers) |
| Hangs at "Connecting..." | Hold **BOOT**, tap **EN**, release **BOOT**, then flash again |
| Board keeps resetting | `espflash monitor` to read the panic ([debugging](./debugging.md#read-a-panic)) |

## Display

| Symptom | Fix |
| ------- | --- |
| OLED stays dark | Check wiring: SDA = GPIO21, SCL = GPIO22, VCC = 3V3, GND = GND ([docs/hardware.md](../../docs/hardware.md)) |
| Still dark with correct wiring | The panel may use I²C address `0x3D`: use `I2CDisplayInterface::new_alternate_address` |
| Garbled image | Lower the I²C speed in `src/main.rs`: `Rate::from_khz(400)` → `Rate::from_khz(100)` |
| `display flush failed` in the monitor | Same checks as above: wiring, address, I²C speed |

## LCD

| Symptom | Fix |
| ------- | --- |
| White screen, nothing drawn | Check wiring: CS = GPIO5, DC = GPIO16, RESET = GPIO17, MOSI = GPIO23, SCK = GPIO18 ([docs/hardware-lcd.md](../../docs/hardware-lcd.md#wiring)) |
| Screen stays black | The backlight pin LED must go to 3V3 |
| Red and blue swapped | Flip `COLOR_ORDER` in `src/main.rs` ([colors](../../docs/hardware-lcd.md#colors)) |
| Colors look like a negative | Flip `COLOR_INVERSION` in `src/main.rs` |
| Garbled or shifted image | Lower `SPI_FREQUENCY` in `src/main.rs`, e.g. to 20 MHz |
| Still wrong with the checks above | The panel may be an ST7789, not an ILI9341 ([controller](../../docs/hardware-lcd.md#controller)) |
