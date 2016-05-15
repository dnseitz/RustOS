%macro pushaq 0
  push rax
  push rcx
  push rdx
  push rbx
  push rbp
  push rsi
  push rdi
%endmacro

%macro popaq 0
  pop rdi
  pop rsi
  pop rbp
  pop rbx
  pop rdx
  pop rcx
  pop rax
%endmacro

; Macro defining interrupt service routine for
; interrupts that push an error code onto the stack
; for us already,
; arg = int #
%macro isr_err 1 
  global isr%1
  isr%1:
    cli
    push byte %1
    jmp rust_handler
%endmacro

; Macro defining interrupt service routine for
; interrupts that don't push an error code onto the
; stack, so we push a dummy 0 code onto the stack 
; to keep things consistent
; arg = int #
%macro isr_nerr 1
  global isr%1
  isr%1:
    cli
    push byte 0
    push byte %1
    jmp rust_handler
%endmacro

isr_nerr 0
isr_nerr 1
isr_nerr 2
isr_nerr 3
isr_nerr 4
isr_nerr 5
isr_nerr 6
isr_nerr 7
isr_err  8
isr_nerr 9
isr_err  10
isr_err  11
isr_err  12
isr_err  13
isr_err  14
isr_nerr 15
isr_nerr 16
isr_nerr 17
isr_nerr 18
isr_nerr 19
isr_nerr 20
isr_nerr 21
isr_nerr 22
isr_nerr 23
isr_nerr 24
isr_nerr 25
isr_nerr 26
isr_nerr 27
isr_nerr 28
isr_nerr 29
isr_nerr 30
isr_nerr 31

; This is the entry point to the rust handler that
; every service routine jumps to. We need to let the
; assembler know that our fault_handler exists in 
; another file
extern rust_int_handler
rust_handler:
  pushaq
  mov rdi, rsp
  call rust_int_handler
  popaq
  add rsp, 16  ; Cleans up the pushed error code and pushed ISR number
  iretq
