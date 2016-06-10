#![feature(const_fn)]
#![feature(asm)]
#![no_std]

extern crate io_controller;
extern crate spin;
#[macro_use]
extern crate x86;

mod pic;
mod pit;

use pic::ChainedPics;
use pit::Pit;
use spin::Mutex;
use core::mem::size_of;
use x86::dtables::{lidt, DescriptorTablePointer};
use x86::irq::IdtEntry;

pub static PICS: Mutex<ChainedPics> = Mutex::new(
                                        unsafe { ChainedPics::new(0x20, 0x28) });
pub static PIT: Mutex<Pit> = Mutex::new(unsafe { Pit::new() });
static mut IDT: IDTable = IDTable::init();

macro_rules! setup_isr {
    ($i:ident, $n:expr) => ({
        extern {
            fn $i();
        }
        let ptr = $i as *const u8;
        unsafe { IDT.add_gate($n, ptr, 0x08) }
    });
}

pub struct IDTable {
    entries: [IdtEntry; 256],
}

impl IDTable {
    const fn init() -> IDTable {
        IDTable {
            entries: [IdtEntry::missing(); 256],
        }
    }

    pub fn add_gate(&mut self, num: usize, base: *const u8, selector: u16) {
        self.entries[num] = IdtEntry::interrupt_gate(selector, base);
    }
}

pub struct Registers {
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    pub int_no: u64,
    pub err_code: u64,
    pub rip: u64,
    pub cs:  u64,
    pub eflags: u64,
    pub useresp: u64,
    pub ss: u64,
}

impl Registers {
    pub unsafe fn load(address: usize) -> &'static Registers {
        &*(address as *const Registers)
    }
}

pub fn setup_idt() {
    unsafe { PICS.lock().initialize(); }

    // Setup the descriptor table pointer
    let addr = unsafe { &IDT as *const IDTable as u64 };
    let tbl_ptr = DescriptorTablePointer {
        limit: ((size_of::<IdtEntry>() * 256) - 1) as u16,
        base: addr,
    };
    
    // assign the descriptors here... there's a lot but i'm not sure how to 
    // do this better.
    setup_isr!(isr0, 0);
    setup_isr!(isr1, 1);
    setup_isr!(isr2, 2);
    setup_isr!(isr3, 3);
    setup_isr!(isr4, 4);
    setup_isr!(isr5, 5);
    setup_isr!(isr6, 6);
    setup_isr!(isr7, 7);
    setup_isr!(isr8, 8);
    setup_isr!(isr9, 9);
    setup_isr!(isr10, 10);
    setup_isr!(isr11, 11);
    setup_isr!(isr12, 12);
    setup_isr!(isr13, 13);
    setup_isr!(isr14, 14);
    setup_isr!(isr15, 15);
    setup_isr!(isr16, 16);
    setup_isr!(isr17, 17);
    setup_isr!(isr18, 18);
    setup_isr!(isr19, 19);
    setup_isr!(isr20, 20);
    setup_isr!(isr21, 21);
    setup_isr!(isr22, 22);
    setup_isr!(isr23, 23);
    setup_isr!(isr24, 24);
    setup_isr!(isr25, 25);
    setup_isr!(isr26, 26);
    setup_isr!(isr27, 27);
    setup_isr!(isr28, 28);
    setup_isr!(isr29, 29);
    setup_isr!(isr30, 30);
    setup_isr!(isr31, 31);
    setup_isr!(isr32, 32);
    setup_isr!(isr33, 33);
    setup_isr!(isr34, 34);
    setup_isr!(isr35, 35);
    setup_isr!(isr36, 36);
    setup_isr!(isr37, 37);
    setup_isr!(isr38, 38);
    setup_isr!(isr39, 39);
    setup_isr!(isr40, 40);
    setup_isr!(isr41, 41);
    setup_isr!(isr42, 42);
    setup_isr!(isr43, 43);
    setup_isr!(isr44, 44);
    setup_isr!(isr45, 45);
    setup_isr!(isr46, 46);
    setup_isr!(isr47, 47);
    
    unsafe { lidt(&tbl_ptr); }
}
