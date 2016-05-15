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

extern {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();
    // Below are the IRQs
    fn isr32();
    fn isr33();
    fn isr34();
    fn isr35();
    fn isr36();
    fn isr37();
    fn isr38();
    fn isr39();
    fn isr40();
    fn isr41();
    fn isr42();
    fn isr43();
    fn isr44();
    fn isr45();
    fn isr46();
    fn isr47();
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

    let mut ptr: *const u8;
    // assign the descriptors here... there's a lot but i'm not sure how to 
    // do this better.
    ptr = isr0 as *const u8;
    unsafe { IDT.add_gate(0, ptr, 0x08); }
    
    ptr = isr1 as *const u8;
    unsafe { IDT.add_gate(1, ptr, 0x08); }
    
    ptr = isr2 as *const u8;
    unsafe { IDT.add_gate(2, ptr, 0x08); }
    
    ptr = isr3 as *const u8;
    unsafe { IDT.add_gate(3, ptr, 0x08); }
    
    ptr = isr4 as *const u8;
    unsafe { IDT.add_gate(4, ptr, 0x08); }
    
    ptr = isr5 as *const u8;
    unsafe { IDT.add_gate(5, ptr, 0x08); }
    
    ptr = isr6 as *const u8;
    unsafe { IDT.add_gate(6, ptr, 0x08); }
    
    ptr = isr7 as *const u8;
    unsafe { IDT.add_gate(7, ptr, 0x08); }
    
    ptr = isr8 as *const u8;
    unsafe { IDT.add_gate(8, ptr, 0x08); }
    
    ptr = isr9 as *const u8;
    unsafe { IDT.add_gate(9, ptr, 0x08); }

    ptr = isr10 as *const u8;
    unsafe { IDT.add_gate(10, ptr, 0x08); }

    ptr = isr11 as *const u8;
    unsafe { IDT.add_gate(11, ptr, 0x08); }
    
    ptr = isr12 as *const u8;
    unsafe { IDT.add_gate(12, ptr, 0x08); }
    
    ptr = isr13 as *const u8;
    unsafe { IDT.add_gate(13, ptr, 0x08); }
    
    ptr = isr14 as *const u8;
    unsafe { IDT.add_gate(14, ptr, 0x08); }
    
    ptr = isr15 as *const u8;
    unsafe { IDT.add_gate(15, ptr, 0x08); }
    
    ptr = isr16 as *const u8;
    unsafe { IDT.add_gate(16, ptr, 0x08); }
    
    ptr = isr17 as *const u8;
    unsafe { IDT.add_gate(17, ptr, 0x08); }
    
    ptr = isr18 as *const u8;
    unsafe { IDT.add_gate(18, ptr, 0x08); }
    
    ptr = isr19 as *const u8;
    unsafe { IDT.add_gate(19, ptr, 0x08); }
    
    ptr = isr20 as *const u8;
    unsafe { IDT.add_gate(20, ptr, 0x08); }
    
    ptr = isr21 as *const u8;
    unsafe { IDT.add_gate(21, ptr, 0x08); }
    
    ptr = isr22 as *const u8;
    unsafe { IDT.add_gate(22, ptr, 0x08); }
    
    ptr = isr23 as *const u8;
    unsafe { IDT.add_gate(23, ptr, 0x08); }
    
    ptr = isr24 as *const u8;
    unsafe { IDT.add_gate(24, ptr, 0x08); }
    
    ptr = isr25 as *const u8;
    unsafe { IDT.add_gate(25, ptr, 0x08); }
    
    ptr = isr26 as *const u8;
    unsafe { IDT.add_gate(26, ptr, 0x08); }
    
    ptr = isr27 as *const u8;
    unsafe { IDT.add_gate(27, ptr, 0x08); }
    
    ptr = isr28 as *const u8;
    unsafe { IDT.add_gate(28, ptr, 0x08); }
    
    ptr = isr29 as *const u8;
    unsafe { IDT.add_gate(29, ptr, 0x08); }
    
    ptr = isr30 as *const u8;
    unsafe { IDT.add_gate(30, ptr, 0x08); }
    
    ptr = isr31 as *const u8;
    unsafe { IDT.add_gate(31, ptr, 0x08); }
    
    ptr = isr32 as *const u8;
    unsafe { IDT.add_gate(32, ptr, 0x08); }
    
    ptr = isr33 as *const u8;
    unsafe { IDT.add_gate(33, ptr, 0x08); }
    
    ptr = isr34 as *const u8;
    unsafe { IDT.add_gate(34, ptr, 0x08); }
    
    ptr = isr35 as *const u8;
    unsafe { IDT.add_gate(35, ptr, 0x08); }
    
    ptr = isr36 as *const u8;
    unsafe { IDT.add_gate(36, ptr, 0x08); }
    
    ptr = isr37 as *const u8;
    unsafe { IDT.add_gate(37, ptr, 0x08); }
    
    ptr = isr38 as *const u8;
    unsafe { IDT.add_gate(38, ptr, 0x08); }
    
    ptr = isr39 as *const u8;
    unsafe { IDT.add_gate(39, ptr, 0x08); }
    
    ptr = isr40 as *const u8;
    unsafe { IDT.add_gate(40, ptr, 0x08); }
    
    ptr = isr41 as *const u8;
    unsafe { IDT.add_gate(41, ptr, 0x08); }
    
    ptr = isr42 as *const u8;
    unsafe { IDT.add_gate(42, ptr, 0x08); }
    
    ptr = isr43 as *const u8;
    unsafe { IDT.add_gate(43, ptr, 0x08); }
    
    ptr = isr44 as *const u8;
    unsafe { IDT.add_gate(44, ptr, 0x08); }
    
    ptr = isr45 as *const u8;
    unsafe { IDT.add_gate(45, ptr, 0x08); }
    
    ptr = isr46 as *const u8;
    unsafe { IDT.add_gate(46, ptr, 0x08); }
    
    ptr = isr47 as *const u8;
    unsafe { IDT.add_gate(47, ptr, 0x08); }
    
    unsafe { lidt(&tbl_ptr); }
}
