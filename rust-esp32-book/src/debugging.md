# Debugging

Run from the firmware directory with the board plugged in, unless noted.

## Find the board

```sh
espflash list-ports       # serial ports espflash can use
espflash board-info       # chip type, revision, MAC address, flash size
ls /dev/cu.*              # macOS: all serial devices
```

## Image size

How much flash the firmware uses (no board needed):

```sh
espflash save-image --chip esp32 target/xtensa-esp32-none-elf/release/blob-firmware /tmp/fw.bin
```

It prints the app size, for example
`App/part. size: 113,968/4,128,768 bytes, 2.76%` for the blob firmware.

## Read a panic

The panic handler in `src/main.rs` logs the panic message. Open the monitor to
see it:

```sh
espflash monitor
```

A board that keeps resetting usually prints the same panic over and over
here.

## Reset the board

```sh
espflash reset
```

## Erase the flash

Wipes everything, including the bootloader and partition table, which
recovers a board in a bad state. The next `cargo run --release` writes them
back.

```sh
espflash erase-flash --chip esp32
```
