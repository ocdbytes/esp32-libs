# Flash and monitor

Plug the board in over USB, then from the firmware directory:

```sh
cd demos/firmware/blob
source ~/export-esp.sh
cargo run --release
```

`cargo run` uses the runner from `.cargo/config.toml`:

```toml
runner = "espflash flash --monitor --chip esp32"
```

So one command builds, flashes, resets the board, and opens the serial
monitor. The blob firmware logs `display ready, running blob` once the OLED is
set up. Press `Ctrl+C` to leave the monitor; the board keeps running.

## Flash without the monitor

```sh
espflash flash --chip esp32 target/xtensa-esp32-none-elf/release/blob-firmware
```

## Open the monitor only

Reads the board's serial output without reflashing:

```sh
espflash monitor
```

## Choose the serial port

If more than one device is connected, or auto-detection fails, name the port:

```sh
espflash flash --chip esp32 --port /dev/cu.SLAB_USBtoUART \
  target/xtensa-esp32-none-elf/release/blob-firmware
espflash monitor --port /dev/cu.SLAB_USBtoUART
```

To make `cargo run` always use that port, add `--port` to the runner in
`.cargo/config.toml`:

```toml
runner = "espflash flash --monitor --chip esp32 --port /dev/cu.SLAB_USBtoUART"
```

## Log level

Firmware logs with the `log` crate (`log::info!`, `log::warn!`,
`log::error!`), and `esp-println` sends the output to the serial monitor. The
level comes from `ESP_LOG`, which `.cargo/config.toml` sets to `info` at build
time. For more detail on one run:

```sh
ESP_LOG=debug cargo run --release
```

## If flashing hangs at "Connecting..."

Put the board into download mode by hand: hold **BOOT**, tap **EN** (reset),
then release **BOOT**. Then run the flash command again.
