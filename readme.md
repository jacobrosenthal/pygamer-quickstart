# PyGamer Quickstart

This crate is the bare minimum needed to start a new project using the [atsamd pygamer board support crate](https://crates.io/crates/pygamer) for the [Adafruit PyGamer
board](https://www.adafruit.com/product/4242). Fork it, rename it and insert your code in the main.rs

## Prerequisites

* Add the cross compile toolchain `rustup target add thumbv7em-none-eabihf`
* Install [cargo-hf2](https://crates.io/crates/cargo-hf2) the hf2 bootloader flasher tool however your platform requires.

## Uploading an example

* Put your device in bootloader mode usually by tapping the reset button twice.
* Build and upload in one step

```bash
$ cargo hf2 --release
    Finished release [optimized + debuginfo] target(s) in 0.19s
    Searching for a connected device with known vid/pid pair.
    Trying  Ok(Some("Adafruit Industries")) Ok(Some("PyGamer"))
    Flashing "/home/jacob/Downloads/pygamer-quickstart/target/thumbv7em-none-eabihf/release/pygamer-quickstart"
    Finished in 0.079s
$
```
