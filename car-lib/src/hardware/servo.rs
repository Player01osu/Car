use arduino_hal::port::Pins;
pub struct Servo;

impl Servo {
    pub fn attach(pins: &mut Pins) -> Self {
        Self
    }

    fn detach(self) {
    }

    pub fn write(&mut self, value: u8) {

    }

    fn read(&self) {
    }

    fn is_attached(&self) -> bool {
        todo!()
    }
}
