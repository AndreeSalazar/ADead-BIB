; ============================================================
; FastOS v2.0 — Kernel Entry Point
; Loaded at 0x100000 (1MB) by stage2.asm
; Sets up 64-bit environment and calls kernel_main()
;
; At this point stage2 has already:
;   - Entered 64-bit Long Mode
;   - Enabled SSE/AVX/AVX2
;   - Set up identity-mapped paging (4GB, 2MB pages)
;   - RSP = 0x90000
;   - RDI = pointer to boot info (0x500)
; ============================================================

[BITS 64]

section .text.entry

global kernel_entry
extern kernel_main
extern __bss_start
extern __bss_end

kernel_entry:
    ; ── Reload 64-bit data segments ──
    ; Stage2's GDT has Data64 at selector 0x20
    mov  ax, 0x20
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax

    ; ── Set up a clean kernel stack ──
    ; Use 0x90000 (576KB, well below our kernel at 1MB)
    mov  rsp, 0x90000
    xor  rbp, rbp           ; Frame pointer = NULL (stack base)

    ; ── Save boot info pointer (RDI) ──
    ; kernel_main expects RDI as first argument (System V ABI)
    ; RDI already set by stage2

    ; ── Zero BSS section ──
    ; The linker script defines __bss_start and __bss_end
    mov  rdi, __bss_start
    mov  rcx, __bss_end
    sub  rcx, rdi
    shr  rcx, 3             ; Convert bytes to qwords
    xor  rax, rax
    rep  stosq

    ; ── Restore boot info pointer and call kernel_main ──
    mov  rdi, 0x500          ; Boot info struct address
    call kernel_main

    ; ── If kernel_main returns, panic ──
    ; This should NEVER happen. Display error on VGA.
    mov  rdi, 0xB8000 + (24 * 160)  ; Last row
    mov  rsi, .panic_msg
    mov  ah, 0x4F                    ; White on red
.panic_loop:
    lodsb
    test al, al
    jz   .panic_halt
    stosw
    jmp  .panic_loop

.panic_halt:
    cli
    hlt
    jmp  .panic_halt

.panic_msg: db "!!! KERNEL PANIC: kernel_main() returned !!!", 0
