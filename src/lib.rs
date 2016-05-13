#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack (64B) and no guard page
    
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    loop{}

    // Testing global WRITER
    use core::fmt::Write;
    let mut writer = vga_buffer::WRITER.lock();
    writer.write_str("Hello from the global Writer!\n");
    write!(writer, "Here's some formatted numbers: {} {}", 42, 1.337);

    // Testing manual printing
    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write 'Hello World!' to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    // Testing module printing
    vga_buffer::print_something();
}



#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
