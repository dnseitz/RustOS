use vga_buffer::print_error;
use spin::Mutex;
use io;
use io::{ALT, CONTROL, SHIFT, CAPSLOCK, NUMLOCK, SCROLLLOCK, KBDUS};

mod idt;
mod pic;

macro_rules! save_scratch_registers {
    () => {
        asm!("push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11"
              :::: "intel", "volatile");
    }
}

macro_rules! restore_scratch_registers {
    () => {
        asm!("pop r11
              pop r10
              pop r9
              pop r8
              pop rdi
              pop rsi
              pop rdx
              pop rcx
              pop rax"
              :::: "intel", "volatile");
    }
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                save_scratch_registers!();
                asm!("mov rdi, rsp
                      add rdi, 9*8 // calculate exception stack frame pointer
                      call $0"
                      :: "i"($name as extern "C" fn(*const ExceptionStackFrame))
                      : "rdi" : "intel", "volatile");
                restore_scratch_registers!();
                asm!("iretq"
                      :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                save_scratch_registers!();
                asm!("mov rsi, [rsp + 9*8] // load error code into rsi
                      mov rdi, rsp
                      add rdi, 10*8 // calculate exception stack frame pointer
                      sub rsp, 8 // align the stack pointer
                      call $0
                      add rsp, 8 // undo stack pointer alignment"
                      :: "i"($name as extern "C" fn(*const ExceptionStackFrame,u64))
                      : "rdi","rsi" : "intel");
                restore_scratch_registers!();
                asm!("add rsp, 8 // pop error code
                      iretq"
                      :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

lazy_static! {
    static ref IDT: idt::Idt = {
        PICS.lock().initialize();

        let mut idt = idt::Idt::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(3, handler!(breakpoint_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        idt.set_handler(32, handler!(timer_handler));
        idt.set_handler(33, handler!(keyboard_handler));

        idt
    };
}

pub static PICS: Mutex<pic::ChainedPics> = Mutex::new(pic::ChainedPics::new(0x20, 0x28));

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub fn init() {
    IDT.load();
}

extern "C" fn divide_by_zero_handler(stack_frame: *const ExceptionStackFrame) {
    unsafe {
        print_error(format_args!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", *stack_frame));
    }
    loop {}
}

extern "C" fn invalid_opcode_handler(stack_frame: *const ExceptionStackFrame) {
    unsafe {
        print_error(format_args!("EXCEPTION: INVALID OPCODE at {:#x}\n{:#?}", 
                                 (*stack_frame).instruction_pointer, *stack_frame));
    }
    loop {}
}

bitflags! {
    flags PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0,
        const CAUSED_BY_WRITE = 1 << 1,
        const USER_MODE = 1 << 2,
        const MALFORMED_TABLE = 1 << 3,
        const INSTRUCTION_FETCH = 1 << 4,
    }
}

extern "C" fn page_fault_handler(stack_frame: *const ExceptionStackFrame, error_code: u64) {
    use x86::controlregs;
    unsafe {
        print_error(format_args!("EXCEPTION: PAGE FAULT while accessing {:#x}\n\
                                  error code: {:?}\n{:#?}",
                                  controlregs::cr2(),
                                  PageFaultErrorCode::from_bits(error_code).unwrap(),
                                  *stack_frame));
    }

    unsafe {
        if controlregs::cr2() == 0xdeadbeaf {
            let stack_frame = &mut *(stack_frame as *mut ExceptionStackFrame);
            stack_frame.instruction_pointer += 7;
            return;
        }
    }
    loop {}
}

extern "C" fn breakpoint_handler(stack_frame: *const ExceptionStackFrame) {
    unsafe {
        print_error(format_args!("EXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
                                 (*stack_frame).instruction_pointer,
                                 *stack_frame));
    }
}

extern "C" fn keyboard_handler(stack_frame: *const ExceptionStackFrame) {
    /*
    unsafe {
        print_error(format_args!("KEYBOARD INTERRUPT\n"));
    }
    */
    let convert = |ch| {
        match ch {
            b'0' => b')', // '0' -> ')'
            b'1' => b'!', // '1' -> '!'
            b'2' => b'@', // '2' -> '@'
            b'3' => b'#', // '3' -> '#'
            b'4' => b'$', // '4' -> '$'
            b'5' => b'%', // '5' -> '%'
            b'6' => b'^', // '6' -> '^'
            b'7' => b'&', // '7' -> '&'
            b'8' => b'*', // '8' -> '*'
            b'9' => b'(', // '9' -> '('
            b'-' => b'_',
            b'=' => b'+',
            b'[' => b'{',
            b']' => b'}',
            b'\\' => b'|',
            b',' => b'<',
            b'.' => b'>',
            b'/' => b'?',
            b'`' => b'~',
            b';' => b':',
            b'\'' => b'"',
            8 => 8,       // backspace
            _ => (ch - (b'a' - b'A')),
        }
    };

    let mut kbd = io::KEYBOARD.lock();
    let scancode = kbd.read_key();
    if (scancode & 0x7f) > 88 {
        // Undefined character
        return;
    }
    let value = KBDUS[(scancode & 0x7f) as usize];
    if (scancode & 0x80) != 0 {
        // The key was released
        match value {
            64 => kbd.release(CONTROL),
            65 | 66 => kbd.release(SHIFT),
            67 => kbd.release(ALT),
            _ => {},
        }
    }
    else {
        match value {
            64 => kbd.press(CONTROL),
            65 | 66 => kbd.press(SHIFT),
            67 => kbd.press(ALT),
            68 => kbd.toggle(CAPSLOCK),
            69 => kbd.toggle(NUMLOCK),
            70 => kbd.toggle(SCROLLLOCK),
            _ => {
                if kbd.is_set(CAPSLOCK) {
                    if kbd.is_set(SHIFT) {
                        print!("{}", value as char);
                    }
                    else {
                        print!("{}", convert(value) as char);
                    }
                }
                else {
                    if kbd.is_set(SHIFT) {
                        print!("{}", convert(value) as char);
                    }
                    else {
                        print!("{}", value as char);
                    }
                }
            }
        }
    }

    PICS.lock().notify_end_of_interrupt(33);
}

extern "C" fn timer_handler(stack_frame: *const ExceptionStackFrame) {
    /*
    unsafe {
        print_error(format_args!("TIMER INTERRUPT\n"));
    }
    */
    PICS.lock().notify_end_of_interrupt(32);
}
