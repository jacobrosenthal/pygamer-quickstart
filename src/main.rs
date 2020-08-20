//! Bare bones example for you to fill out
//! `cargo run --release`

#![no_std]
#![no_main]

use pygamer as hal;
use pygamer_panic_led as _; // light D13, back red led on panic

#[hal::entry]
fn main() -> ! {
    let mut peripherals = hal::pac::Peripherals::take().unwrap();
    let _core_peripherals = hal::pac::CorePeripherals::take().unwrap();

    let mut _clocks = hal::clock::GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    loop {
        panic!("bang!"); // remove this and put your code here
    }
}
