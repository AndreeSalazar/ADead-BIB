; ============================================================
; FastOS v2.0 — Context Switch + Syscall Stubs
; 64-bit Long Mode
;
; switch_context(uint64_t *old_rsp, uint64_t new_rsp)
;   - Saves callee-saved registers on current stack
;   - Stores RSP to *old_rsp
;   - Loads new_rsp
;   - Restores callee-saved registers from new stack
;   - Returns (which jumps to the new process's saved RIP)
; ============================================================

[BITS 64]

section .text

; ── Context Switch ──
; System V ABI: RDI = &old_rsp, RSI = new_rsp
; Callee-saved: RBX, RBP, R12, R13, R14, R15

global switch_context
switch_context:
    ; Save callee-saved registers on current stack
    push r15
    push r14
    push r13
    push r12
    push rbx
    push rbp

    ; Save current RSP to *old_rsp
    mov  [rdi], rsp

    ; Load new stack pointer
    mov  rsp, rsi

    ; Restore callee-saved registers from new stack
    pop  rbp
    pop  rbx
    pop  r12
    pop  r13
    pop  r14
    pop  r15

    ; Return — pops saved RIP from new stack, resuming the other process
    ret
