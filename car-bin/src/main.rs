#![no_std]
#![no_main]
extern crate arduino_hal;
extern crate car_lib;

use core::panic::PanicInfo;
use car_lib::{GlobalMemory, collections::string::String};

use arduino_hal::hal::pins;
use arduino_hal::Peripherals;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.pb5.into_output();
    let mut global_mem = GlobalMemory::new();
    //let new_string = String::new(&global_mem as *const u8);

    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}
