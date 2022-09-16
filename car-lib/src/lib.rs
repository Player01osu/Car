#![no_std]
#![no_main]

//use arduino_hal::atmega_hal::Pins;


extern crate arduino_hal;

pub mod collections;
pub mod hardware;

const STATIC_MEMORY_SIZE: u8 = 200u8;

pub enum Pin {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
}

pub struct GlobalMemory {
    ptr: *const u8,
    size: usize,
    used: usize,
    available: usize,
}

impl GlobalMemory {
    /// Initialize new global memory.
    pub fn new() -> Self {
        Self {
            ptr: &[0u8; STATIC_MEMORY_SIZE as usize] as *const u8,
            size: STATIC_MEMORY_SIZE as usize,
            used: 0,
            available: STATIC_MEMORY_SIZE as usize,
        }
    }

    fn allocate(&mut self, size: usize) -> *const u8 {
        self.check(size);
        self.available -= size;
        self.used += size;

        unsafe { self.ptr.add(size) }
    }

    fn check(&self, size: usize) {
        if size >= self.available {
            panic!()
        }
    }

    fn size(&self) -> usize {
        self.size
    }
}
