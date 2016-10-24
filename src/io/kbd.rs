use super::Port;

bitflags! {
    flags KeyFlags: u8 {
        const ALT =         1 << 0,
        const CONTROL =     1 << 1,
        const SHIFT =       1 << 2,
        const CAPSLOCK =    1 << 3,
        const NUMLOCK =     1 << 4,
        const SCROLLLOCK =  1 << 5,
    }
}

pub struct Keyboard {
    data: Port<u8>,
    control: Port<u8>,
    status: KeyFlags,
}

impl Keyboard {
    pub const fn new() -> Keyboard {
        Keyboard {
            data: Port::new(0x60),
            control: Port::new(0x64),
            status: KeyFlags { bits: 0 },
        }
    }

    pub fn press(&mut self, key_mask: KeyFlags) {
        self.status.insert(key_mask);
    }

    pub fn release(&mut self, key_mask: KeyFlags) {
        self.status.remove(key_mask);
    }

    pub fn toggle(&mut self, key_mask: KeyFlags) {
        self.status.toggle(key_mask);
    }

    pub fn is_set(&self, key_mask: KeyFlags) -> bool {
        self.status.contains(key_mask)
    }

    pub fn read_key(&self) -> u8 {
        self.data.read()
    }
}

pub static KBDUS: [u8; 89] = [
    0, 
    27, 
    b'1',
    b'2',
    b'3',
    b'4',
    b'5',
    b'6',
    b'7',
    b'8',
    b'9',
    b'0',
    b'-',
    b'=',
    8,     // 14 - Backspace
    b'\t',
    b'q',
    b'w',
    b'e',
    b'r',
    b't',
    b'y',
    b'u',
    b'i',
    b'o',
    b'p',
    b'[',
    b']',
    b'\n', // 28 - Enter key
    64,     // 29 - Ctrl
    b'a',
    b's',
    b'd',
    b'f',
    b'g',
    b'h',
    b'j',
    b'k',
    b'l',
    b';', // 39
    b'\'',
    b'`',
    65,    // 42 - Left Shift
    b'\\',
    b'z',
    b'x',
    b'c',
    b'v',
    b'b',
    b'n', // 49
    b'm',
    b',',
    b'.',
    b'/',
    66,    // 54 - Right shift
    b'*',
    67,    // 56 - Alt
    b' ',
    68,    // 58 - Caps Lock
    0,    // 59 - F1 key...
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,    // 68 - ...F10 key
    69,    // 69 - Num lock
    70,    // 70 - Scroll Lock
    0,    // 71 - Home key
    0,    // 72 - Up Arrow
    0,    // 73 - Page Up
    b'-',
    0,    // 75 - Left Arrow
    0,
    0,    // 77 - Right Arrow
    b'+',
    0,    // 79 - End key
    0,    // 80 - Down Arrow
    0,    // 81 - Page Down
    0,    // 82 - Insert Key
    0,    // 83 - Delete Key
    0, 0, 0,
    0,    // 87 - F11 key
    0,    // 88 - F12 key
    ];
