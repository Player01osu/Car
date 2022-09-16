#![no_std]
#![no_main]
extern crate arduino_hal;
extern crate car_lib;

use core::panic::PanicInfo;
use arduino_hal::{Peripherals, hal::pins, simple_pwm::{IntoPwmPin, Timer1Pwm}, pac::TC1, delay_ms};
use car_lib::{GlobalMemory, collections::string::String, hardware::servo::Servo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let mut global_mem = GlobalMemory::new();

    let peripherals = Peripherals::take().unwrap();
    let pins = pins!(peripherals);

    let mut led = pins.pb5.into_output();
    //let mut dc_motor = pins.pd5.into_output();

    let mut timer1 = Timer1Pwm::new(peripherals.TC1, arduino_hal::simple_pwm::Prescaler::Prescale256);
    let mut servo = pins.pb1.into_output().into_pwm(&mut timer1);

        servo.enable();
    loop {
        led.toggle();
        //dc_motor.set_low();
        for i in 21..162u8 {
            servo.set_duty(i);
            delay_ms(50);
        }

        //dc_motor.set_high();
        arduino_hal::delay_ms(100);
    }
}
