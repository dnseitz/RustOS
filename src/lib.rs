#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc, collections)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86;
extern crate hole_list_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate once;
extern crate bit_field;

#[macro_use]
mod vga_buffer;
mod memory;
mod interrupts;

use alloc::boxed::Box;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Hello, World!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();

    memory::init(boot_info);

    interrupts::init();

//    divide_by_zero();
//    unsafe { asm!("ud2") };
    unsafe{ *(0xdeadbeaf as *mut u64) = 42 };

    println!("It did not crash!");

    loop{}
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

// MARK - Divide by zero

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}
