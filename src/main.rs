//! Bare bones example for you to fill out
//! `cargo hf2 --release`

#![no_std]
#![no_main]

use pygamer as hal;

// light D13, the red led on the back of the Pygamer, on panic
use pygamer_panic_led as _;

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
