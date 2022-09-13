
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
avrdude -p m328p -c arduino -p /dev/ttyacm0 -b 115200 -u flash:w:car-arduino.elf
```

## Helpful links

https://components101.com/microcontrollers/atmega328p-pinout-features-datasheet

