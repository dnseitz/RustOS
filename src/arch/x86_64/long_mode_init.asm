global long_mode_start

section .text
bits 64
 
global int_handler
int_handler:
  mov dword [0xb8000], 0x2f542f53
  mov dword [0xb8004], 0x2f562f45
  mov dword [0xb8008], 0x2f4e2f45
  mov word  [0xb800c], 0x2f21
  hlt

long_mode_start:
  
  ; call the rust main
  extern rust_main
  call rust_main

  ; print 'OS returned!' to screen
  mov rax, 0x4f724f204f534f4f
  mov [0xb8000], rax
  mov rax, 0x4f724f754f744f65
  mov [0xb8008], rax
  mov rax, 0x4f214f644f654f6e
  mov [0xb8010], rax
  hlt
  
;global test_int
;test_int:
;  extern idt_ptr
;  lidt [idt_ptr]
;  mov rax, int_handler
;  mov [idt_ptr+2], rcx ; idt pointer
;  mov [rcx+49*16], ax
;  mov word [rcx+49*16+2], 0x08
;  mov word [rcx+49*16+4], 0x8E00
;  shr rax, 16
;  mov [rcx+49*16+6], ax
;  shr rax, 16
;  mov [rcx+49*16+8], eax
;  mov dword [rcx+49*16+12], 0x00000000
;  int 49
