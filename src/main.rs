#![no_std]
#![no_main]
extern crate arduino_hal;

use core::panic::PanicInfo;

use arduino_hal::Peripherals;
use arduino_hal::hal::pins;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

#[arduino_hal::entry]
fn main() -> !{
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.pb5.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
