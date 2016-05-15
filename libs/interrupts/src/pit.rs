use io_controller::UnsafePort;

const SET_RATE: u8 = 0x36;

pub struct Pit {
    channel: [UnsafePort<u8>; 3],
    command: UnsafePort<u8>,
    rate: u32,
    ticks: u64,
}

impl Pit {
    pub const unsafe fn new() -> Pit {
        Pit {
            channel: [
                UnsafePort::new(0x40),
                UnsafePort::new(0x41),
                UnsafePort::new(0x42),
            ],
            command: UnsafePort::new(0x43),
            rate: 18,
            ticks: 0,
        }
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn get_rate(&self) -> u32 {
        self.rate
    }

    pub fn set_rate(&mut self, rate_hz: u32) {
        let divisor = 1193180 / rate_hz;
        unsafe {
            self.command.write(SET_RATE); //Set command byte 
            self.channel[0].write((divisor & 0xf) as u8); // Low byte of divisor
            self.channel[0].write((divisor >> 8) as u8); // High byte of divisor
        }
        self.rate = rate_hz;
    }
}
