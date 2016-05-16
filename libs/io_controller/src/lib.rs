#![feature(asm)]
#![feature(const_fn)]
#![no_std]

extern crate spin;

mod asmio;
mod kbd;

pub use kbd::KBDUS;
use core::marker::PhantomData;
use spin::Mutex;
use kbd::Keyboard;

pub static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());

pub trait InOut {
    unsafe fn read_in(port: u16) -> Self;
    unsafe fn write_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn read_in(port: u16) -> u8 {
        asmio::inb(port)
    }
    
    unsafe fn write_out(port: u16, value: u8) {
        asmio::outb(port, value);
    }
}

impl InOut for u16 {
    unsafe fn read_in(port: u16) -> u16 {
        asmio::inw(port)
    }

    unsafe fn write_out(port: u16, value: u16) {
        asmio::outw(port, value);
    }
}

impl InOut for u32 {
    unsafe fn read_in(port: u16) -> u32 {
        asmio::inl(port)
    }

    unsafe fn write_out(port: u16, value: u32) {
        asmio::outl(port, value);
    }
}

pub struct Port<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    pub const fn new(port: u16) -> Port<T> {
        Port { port: port, phantom: PhantomData }
    }

    pub fn read(&self) -> T {
        unsafe { T::read_in(self.port) }
    }

    pub fn write(&self, value: T) {
        unsafe { T::write_out(self.port, value); }
    }
}

pub struct UnsafePort<T: InOut> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T: InOut> UnsafePort<T> {
    pub const unsafe fn new(port: u16) -> UnsafePort<T> {
        UnsafePort { port: port, phantom: PhantomData }
    }

    pub unsafe fn read(&self) -> T {
        T::read_in(self.port)
    }

    pub unsafe fn write(&self, value: T) {
        T::write_out(self.port, value);
    }
}
