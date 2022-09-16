
# Car

## Build Instructions

avrdude
gcc-avr
cargo
rustup

```sh
cargo build
```

```sh
cp ./target/avr-atmega328p/debug/car-arduino.elf ./
```

```sh
avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:car-bin.elf
```

## Helpful links

[https://github.com/Rahix/avr-hal](https://github.com/Rahix/avr-hal)\
[https://components101.com/microcontrollers/atmega328p-pinout-features-datasheet](https://components101.com/microcontrollers/atmega328p-pinout-features-datasheet)\
[https://deepbluembedded.com/stm32-pwm-example-timer-pwm-mode-tutorial/](https://deepbluembedded.com/stm32-pwm-example-timer-pwm-mode-tutorial/)\
[https://en.wikipedia.org/wiki/Duty_cycle](https://en.wikipedia.org/wiki/Duty_cycle)\
[https://components101.com/motors/servo-motor-basics-pinout-datasheet](https://components101.com/motors/servo-motor-basics-pinout-datasheet)\
[https://upload.wikimedia.org/wikipedia/commons/c/c9/Pinout_of_ARDUINO_Board_and_ATMega328PU.svg](https://upload.wikimedia.org/wikipedia/commons/c/c9/Pinout_of_ARDUINO_Board_and_ATMega328PU.svg)
