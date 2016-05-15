#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]
#![feature(alloc, collections)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate x86;
extern crate hole_list_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate once;
extern crate interrupts;

#[macro_use]
mod vga_buffer;
mod memory;

use alloc::boxed::Box;
use interrupts::{PICS, PIT, setup_idt, Registers};
use x86::irq::{enable, disable};

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Hello, World!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();

    memory::init(boot_info);

    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test2 = Box::new("hello");
    println!("{:?} {:?}", heap_test, heap_test2);

    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;
    for i in &vec_test {
        print!("{} ", i);
    }

    println!("It didn't crash!");

    for i in 0..10000 {
        format!("Some String");
    }

    println!("Deallocation worked!");

    setup_idt();
    
    println!("IDT setup!");

    PIT.lock().set_rate(100);

    unsafe { enable(); }
    
    loop{}
}

#[no_mangle]
pub extern "C" fn rust_int_handler(registers: usize) {
    let r = unsafe { Registers::load(registers) };
    if r.int_no != 32 {
        println!("    Exception! Int code: {}", r.int_no);
    }
    if r.int_no > 31 || r.int_no <= 47 {
        irq_handler(&r);
    }
}

fn irq_handler(registers: &Registers) {
    // Handle different types of irqs here
    
    if registers.int_no == 32 {
        timer_int(registers);
    }

    unsafe { PICS.lock().notify_end_of_interrupt(registers.int_no as u8); }
}

fn timer_int(registers: &Registers) {
    let mut pit = PIT.lock();
    pit.tick();
    if pit.get_ticks() % pit.get_rate() as u64 == 0 {
        println!("One second has passed");
    }
}

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};

    let wp_bit = 1 << 16;
    unsafe { cr0_write(cr0() | wp_bit) };
}



#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] 
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
