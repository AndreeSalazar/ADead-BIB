; ============================================================
; FastOS v2.0 — ISR/IRQ Stubs (64-bit Long Mode)
; CPU exception handlers (0-31) and hardware IRQ handlers (32-47)
;
; Each stub pushes the interrupt number and (if needed) a dummy
; error code, then jumps to a common handler that saves all
; registers and calls the C function isr_handler().
; ============================================================

[BITS 64]

section .text

extern isr_handler

; ── Common ISR stub ──
; At this point the stack has: SS, RSP, RFLAGS, CS, RIP, [err_code], int_no
; We save all general-purpose registers and call the C handler.

isr_common:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    ; Pass pointer to saved frame as first argument (RDI = RSP)
    mov  rdi, rsp
    call isr_handler

    pop  r15
    pop  r14
    pop  r13
    pop  r12
    pop  r11
    pop  r10
    pop  r9
    pop  r8
    pop  rbp
    pop  rdi
    pop  rsi
    pop  rdx
    pop  rcx
    pop  rbx
    pop  rax

    ; Remove int_no and err_code from stack
    add  rsp, 16
    iretq

; ── ISR stubs for CPU exceptions (vectors 0-31) ──
; Exceptions 8, 10-14, 17, 21, 29, 30 push an error code.
; All others need a dummy 0 pushed.

%macro ISR_NOERR 1
global isr%1
isr%1:
    push qword 0          ; Dummy error code
    push qword %1         ; Interrupt number
    jmp  isr_common
%endmacro

%macro ISR_ERR 1
global isr%1
isr%1:
    ; CPU already pushed error code
    push qword %1         ; Interrupt number
    jmp  isr_common
%endmacro

ISR_NOERR 0    ; #DE  Division Error
ISR_NOERR 1    ; #DB  Debug
ISR_NOERR 2    ; NMI
ISR_NOERR 3    ; #BP  Breakpoint
ISR_NOERR 4    ; #OF  Overflow
ISR_NOERR 5    ; #BR  Bound Range
ISR_NOERR 6    ; #UD  Invalid Opcode
ISR_NOERR 7    ; #NM  Device Not Available
ISR_ERR   8    ; #DF  Double Fault
ISR_NOERR 9    ; Coprocessor Segment Overrun (legacy)
ISR_ERR   10   ; #TS  Invalid TSS
ISR_ERR   11   ; #NP  Segment Not Present
ISR_ERR   12   ; #SS  Stack-Segment Fault
ISR_ERR   13   ; #GP  General Protection Fault
ISR_ERR   14   ; #PF  Page Fault
ISR_NOERR 15   ; Reserved
ISR_NOERR 16   ; #MF  x87 FPU Error
ISR_ERR   17   ; #AC  Alignment Check
ISR_NOERR 18   ; #MC  Machine Check
ISR_NOERR 19   ; #XM  SIMD FP Exception
ISR_NOERR 20   ; #VE  Virtualization Exception
ISR_ERR   21   ; #CP  Control Protection Exception
ISR_NOERR 22
ISR_NOERR 23
ISR_NOERR 24
ISR_NOERR 25
ISR_NOERR 26
ISR_NOERR 27
ISR_NOERR 28
ISR_ERR   29   ; #VC  VMM Communication Exception
ISR_ERR   30   ; #SX  Security Exception
ISR_NOERR 31

; ── IRQ stubs (vectors 32-47, mapped from PIC) ──

%macro IRQ 2
global irq%1
irq%1:
    push qword 0          ; Dummy error code
    push qword %2         ; Interrupt vector number
    jmp  isr_common
%endmacro

IRQ 0,  32   ; PIT Timer
IRQ 1,  33   ; Keyboard
IRQ 2,  34   ; Cascade
IRQ 3,  35   ; COM2
IRQ 4,  36   ; COM1
IRQ 5,  37   ; LPT2
IRQ 6,  38   ; Floppy
IRQ 7,  39   ; LPT1 / Spurious
IRQ 8,  40   ; RTC
IRQ 9,  41   ; Free
IRQ 10, 42   ; Free
IRQ 11, 43   ; Free
IRQ 12, 44   ; PS/2 Mouse
IRQ 13, 45   ; FPU
IRQ 14, 46   ; Primary ATA
IRQ 15, 47   ; Secondary ATA
