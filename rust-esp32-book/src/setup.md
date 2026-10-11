# Setup

Install these once per machine. Details are in the
[Rust on ESP Book](https://docs.esp-rs.org/book/installation/index.html).

## 1. Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. The `esp` toolchain

```sh
cargo install espup --locked
espup install
```

`espup install` installs:

- the `esp` Rust toolchain (named in each firmware's `rust-toolchain.toml`)
- the Xtensa GCC linker and LLVM
- `~/export-esp.sh`, which puts the linker on your `PATH`

## 3. `espflash`

```sh
cargo install espflash --locked
```

## 4. USB serial driver (macOS)

The nodemcu-32s uses a Silicon Labs CP210x USB-to-serial chip and shows up as
`/dev/cu.SLAB_USBtoUART`. If that port never appears when the board is
plugged in, install the
[CP210x VCP driver](https://www.silabs.com/developer-tools/usb-to-uart-bridge-vcp-drivers).

## 5. `esp-generate` (optional)

Only needed to create new projects from scratch. Copying `demos/firmware/blob` is
usually simpler.

```sh
cargo install esp-generate --locked
```

## Verify

```sh
espflash --version
rustup toolchain list | grep esp      # should list `esp`
```

## Every new terminal

Building firmware links with `xtensa-esp32-elf-gcc`, which is only on your
`PATH` after:

```sh
source ~/export-esp.sh
```

Without it the build fails with ``linker `xtensa-esp32-elf-gcc` not found``.
To load it automatically in every terminal:

```sh
echo 'source ~/export-esp.sh' >> ~/.zshrc
```

## Desktop preview (optional)

Previewing scenes on your computer needs CMake and a C compiler:
`brew install cmake` and `xcode-select --install` on macOS. See
[docs/simulator.md](../../docs/simulator.md).
