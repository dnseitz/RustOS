#![feature(const_fn)]

extern crate spin;

mod asmio;

use core::marker::PhantomData;
use spin::Mutex;

static KEYBOARD: Mutex<Port<u8>> = Mutex::new(unsafe {
    Port::new(0x60)
});

pub trait InOut {
    unsafe fn in(port: u16) -> Self;
    unsafe fn out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn in(port: u16) -> u8 {
        asmio::inb(port)
    }
    
    unsafe fn out(port: u16, value: u8) {
        asmio::outb(port, value);
    }
}

impl InOut for u16 {
    unsafe fn in(port: u16) -> u16 {
        asmio::inw(port)
    }

    unsafe fn out(port: u16, value: u16) {
        asmio::outw(port, value);
    }
}

impl InOut for u32 {
    unsafe fn in(port: u16) -> u32 {
        asmio::inl(port)
    }

    unsafe fn out(port: u16, value: u32) {
        asmio::outl(port, value);
    }
}

struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> where T: InOut {
    pub const unsafe fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&self) -> T {
        unsafe { T::in(self.port) }
    }

    pub fn write(&self, value: T) {
        unsafe { T::out(self.port, value); }
    }
}
