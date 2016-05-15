
use io_controller::{UnsafePort, Port};

const CMD_END_OF_INTERRUPT: u8 = 0x20;
const CMD_INIT: u8 = 0x11;

const MODE_8086: u8 = 0x01;

const PIC_READ_IRR: u8 = 0x0a;
const PIC_READ_ISR: u8 = 0x0b;

struct Pic {
    offset: u8,
    command: UnsafePort<u8>,
    data: UnsafePort<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: UnsafePort::new(0x20),
                    data: UnsafePort::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: UnsafePort::new(0xa0),
                    data: UnsafePort::new(0xa1),
                },
            ],
        }
    }

    pub unsafe fn initialize(&mut self) {
        let wait_port: Port<u8> = Port::new(0x80);
        let wait = || { wait_port.write(0) };

        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Send command: Begin 3 byte initialization sequence
        self.pics[0].command.write(CMD_INIT);
        wait();
        self.pics[1].command.write(CMD_INIT);
        wait();

        // Send data 1: set interrupt offset.
        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        // Send data 2: configure chaining.
        self.pics[0].data.write(4);
        wait();
        self.pics[1].data.write(2);
        wait();

        // Send data 3: set mode
        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
        wait();

        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }

    pub unsafe fn set_mask(&self, mut irq_line: u8) {
        let port = if irq_line < 8 {
            // Master PIC handles this
            0
        } else {
            irq_line -= 8;
            1
        };
        let value = self.pics[port].data.read() | (1 << irq_line);
        self.pics[port].data.write(value);
    }

    pub unsafe fn clear_mask(&self, mut irq_line: u8) {
        let port = if irq_line < 8 {
            // Master PIC handles this
            0
        } else {
            irq_line -= 8;
            1
        };
        let value = self.pics[port].data.read() & !(1 << irq_line);
        self.pics[port].data.write(value);
    }

    /// Returns the contents of the PICs Interrupt Request Register (IRR)
    /// which shows interrupts have been raised but not yet sent to the CPU
    pub unsafe fn get_irr(&self) -> u16 {
        self.interrupt_status(PIC_READ_IRR)
    }

    /// Returns the contents of the PICs In-Service Register (ISR) which
    /// shows us which interrupts are being serviced by the CPU
    pub unsafe fn get_isr(&self) -> u16 {
        self.interrupt_status(PIC_READ_ISR)
    }
    
    /// Write the ocw3 command word to the PICs command port then read from
    /// the command port to get the status of interrupts on the PICs 
    ///     0x0a to get IRR or 0x0b to get ISR
    unsafe fn interrupt_status(&self, ocw3: u8) -> u16 {
        self.pics[0].command.write(ocw3);
        self.pics[1].command.write(ocw3);
        (self.pics[0].command.read() as u16)
            | ((self.pics[1].command.read() as u16) << 8)
    }
}
