#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::ehal;
use bsp::hal;
use bsp::pac;

use panic_halt as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

use ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(1000u32);
        red_led.set_high().unwrap();
        delay.delay_ms(1000u32);
        red_led.set_low().unwrap();
    }
}