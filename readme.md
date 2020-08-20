# PyGamer Quickstart

This crate is the bare minimum needed to start a new project using the [atsamd pygamer board support crate](https://crates.io/crates/pygamer) for the [Adafruit PyGamer
board](https://www.adafruit.com/product/4242). Fork it, rename it and insert your code in the main.rs

## Prerequisites

* Add the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [hf2-cli](https://crates.io/crates/hf2-cli) the hf2 bootloader flasher tool however your platform requires.

## Uploading an example

* Put your device in bootloader mode usually by tapping the reset button twice.
* Build and upload in one step

```bash
$ cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyGamer"))
    Flashing "/home/jacob/Downloads/pygamer-quickstart/target/thumbv7em-none-eabihf/release/pygamer-quickstart"
    Finished in 0.079s
$
```

## What happend

In your ./cargo/config hf2 is set as your 'runner' so when the build is done, it calls hf2 with your elf file. If hf2 finds a usb device connected and in bootloader mode, it uploads to it. You could do this manually by calling something like `hf2 elf target/thumbv7em-none-eabihf/release/pygamer-quickstart`
