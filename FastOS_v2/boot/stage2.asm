; ============================================================
; FastOS_v2 Stage2 - REAL HARDWARE 64-bit Transition
; Designed for REAL PCs, not QEMU
; Debug output at EVERY step to diagnose failures
; ============================================================

format binary
org 0x0000
use16

; ============================================================
; Constants
; ============================================================
STAGE2_SEG      equ 0x1000
STAGE2_LIN      equ 0x10000
STACK16         equ 0x7000
STACK32         equ 0x90000
STACK64         equ 0x90000
KERNEL_ADDR     equ 0x100000

; Page tables
PML4            equ 0x70000
PDPT            equ 0x71000
PD0             equ 0x72000

; VGA text buffer
VGA             equ 0xB8000

; ============================================================
; 16-BIT REAL MODE ENTRY
; ============================================================
entry16:
    ; Save boot drive
    mov  [boot_drv], dl

    ; Setup segments
    cli
    mov  ax, STAGE2_SEG
    mov  ds, ax
    mov  es, ax
    xor  ax, ax
    mov  ss, ax
    mov  sp, STACK16
    sti

    ; === DEBUG 1: Print "S2" via BIOS ===
    mov  al, 'S'
    call putc16
    mov  al, '2'
    call putc16
    mov  al, ' '
    call putc16

    ; === Enable A20 ===
    mov  ax, 0x2401
    int  0x15
    jnc  .a20_ok
    ; Fast A20
    in   al, 0x92
    or   al, 2
    and  al, 0xFE
    out  0x92, al
.a20_ok:
    mov  al, 'A'
    call putc16

    ; === E820 Memory Detection ===
    call detect_e820
    mov  al, 'M'
    call putc16

    ; === Disable VESA, use VGA text ===
    mov  ax, 0x0500
    mov  es, ax
    xor  eax, eax
    mov  [es:0], eax          ; No framebuffer = VGA text mode
    mov  ax, STAGE2_SEG
    mov  es, ax
    mov  al, 'V'
    call putc16

    ; === Prepare for Protected Mode ===
    mov  al, '>'
    call putc16
    mov  al, '3'
    call putc16
    mov  al, '2'
    call putc16

    cli                        ; CRITICAL

    ; Load GDT
    lgdt [gdt32_ptr]

    ; Enable Protected Mode
    mov  eax, cr0
    or   al, 1
    mov  cr0, eax

    ; Far jump to 32-bit code
    db 0x66, 0xEA              ; far jmp opcode (32-bit in 16-bit mode)
    dd STAGE2_LIN + entry32    ; offset
    dw 0x08                    ; selector

; ─── 16-bit Helpers ────────────────────────────────────────
putc16:
    push bx
    mov  ah, 0x0E
    mov  bx, 0x0007
    int  0x10
    pop  bx
    ret

detect_e820:
    push es
    mov  ax, 0x2000
    mov  es, ax
    xor  di, di
    xor  ebx, ebx
    mov  word [es:di], 0
.loop:
    add  di, 2
    mov  eax, 0xE820
    mov  edx, 0x534D4150
    mov  ecx, 24
    int  0x15
    jc   .done
    cmp  eax, 0x534D4150
    jne  .done
    sub  di, 2
    inc  word [es:di]
    add  di, 26
    test ebx, ebx
    jz   .done
    cmp  di, 250
    jb   .loop
.done:
    pop  es
    ret

; ─── 16-bit Data ───────────────────────────────────────────
boot_drv: db 0

; ─── GDT 32-bit ────────────────────────────────────────────
align 8
gdt32:
    dq 0                      ; Null
    ; Code 32-bit (0x08)
    dw 0xFFFF, 0x0000
    db 0x00, 0x9A, 0xCF, 0x00
    ; Data 32-bit (0x10)
    dw 0xFFFF, 0x0000
    db 0x00, 0x92, 0xCF, 0x00
gdt32_end:

gdt32_ptr:
    dw gdt32_end - gdt32 - 1
    dd STAGE2_LIN + gdt32

; ============================================================
; 32-BIT PROTECTED MODE
; ============================================================
align 16
use32
entry32:
    ; Setup segments
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  esp, STACK32

    ; === DEBUG: Write "32" to VGA row 2 ===
    mov  edi, VGA + 160*2
    mov  ax, 0x0A33           ; '3' green
    stosw
    mov  ax, 0x0A32           ; '2'
    stosw
    mov  ax, 0x0A4F           ; 'O'
    stosw
    mov  ax, 0x0A4B           ; 'K'
    stosw

    ; === Setup Paging ===
    ; Clear page tables
    mov  edi, PML4
    xor  eax, eax
    mov  ecx, 0x3000/4        ; 12KB
    rep  stosd

    ; PML4[0] -> PDPT
    mov  dword [PML4], PDPT + 3

    ; PDPT[0] -> PD0
    mov  dword [PDPT], PD0 + 3

    ; PD0: Identity map first 1GB with 2MB pages
    mov  edi, PD0
    mov  eax, 0x83            ; Present + RW + PS (2MB)
    mov  ecx, 512
.fill_pd:
    mov  [edi], eax
    add  eax, 0x200000
    add  edi, 8
    loop .fill_pd

    ; === DEBUG: "PG" ===
    mov  edi, VGA + 160*2 + 10
    mov  ax, 0x0E50           ; 'P' yellow
    stosw
    mov  ax, 0x0E47           ; 'G'
    stosw

    ; Load CR3
    mov  eax, PML4
    mov  cr3, eax

    ; Enable PAE
    mov  eax, cr4
    or   eax, 0x20
    mov  cr4, eax

    ; === DEBUG: "PA" ===
    mov  edi, VGA + 160*2 + 16
    mov  ax, 0x0E50           ; 'P'
    stosw
    mov  ax, 0x0E41           ; 'A'
    stosw

    ; Enable Long Mode (EFER.LME)
    mov  ecx, 0xC0000080
    rdmsr
    or   eax, 0x100
    wrmsr

    ; === DEBUG: "LM" ===
    mov  edi, VGA + 160*2 + 22
    mov  ax, 0x0E4C           ; 'L'
    stosw
    mov  ax, 0x0E4D           ; 'M'
    stosw

    ; Load 64-bit GDT
    lgdt [STAGE2_LIN + gdt64_ptr]

    ; Enable Paging (activates Long Mode)
    mov  eax, cr0
    or   eax, 0x80000000
    mov  cr0, eax

    ; Far jump to 64-bit
    jmp  0x08:(STAGE2_LIN + entry64)

; ─── GDT 64-bit ────────────────────────────────────────────
align 8
gdt64:
    dq 0                      ; Null
    ; Code 64-bit (0x08)
    dw 0, 0
    db 0, 0x9A, 0x20, 0       ; L=1
    ; Data 64-bit (0x10)
    dw 0, 0
    db 0, 0x92, 0, 0
gdt64_end:

gdt64_ptr:
    dw gdt64_end - gdt64 - 1
    dq STAGE2_LIN + gdt64

; ============================================================
; 64-BIT LONG MODE
; ============================================================
align 16
use64
entry64:
    ; Setup segments
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  rsp, STACK64

    ; === DEBUG: "64" on row 3 ===
    mov  rdi, VGA + 160*3
    mov  ax, 0x0B36           ; '6' cyan
    stosw
    mov  ax, 0x0B34           ; '4'
    stosw
    mov  ax, 0x0B4F           ; 'O'
    stosw
    mov  ax, 0x0B4B           ; 'K'
    stosw

    ; === Enable SSE ===
    mov  rax, cr0
    and  ax, 0xFFFB           ; Clear EM
    or   ax, 2                ; Set MP
    mov  cr0, rax
    mov  rax, cr4
    or   ax, 0x600            ; OSFXSR + OSXMMEXCPT
    mov  cr4, rax

    ; === DEBUG: "SS" ===
    mov  rdi, VGA + 160*3 + 10
    mov  ax, 0x0A53           ; 'S' green
    stosw
    stosw

    ; === Copy kernel from 0x14000 to 0x100000 ===
    mov  rsi, 0x14000         ; Kernel after stage2 (16KB)
    mov  rdi, KERNEL_ADDR
    mov  rcx, 8192            ; 64KB / 8
    cld
    rep  movsq

    ; === DEBUG: "CP" (copy done) ===
    mov  rdi, VGA + 160*3 + 16
    mov  ax, 0x0E43           ; 'C' yellow
    stosw
    mov  ax, 0x0E50           ; 'P'
    stosw

    ; === Verify kernel exists ===
    mov  rax, KERNEL_ADDR
    mov  eax, [rax]
    test eax, eax
    jz   .no_kernel

    ; === DEBUG: "JMP" ===
    mov  rdi, VGA + 160*3 + 22
    mov  ax, 0x0A4A           ; 'J' green
    stosw
    mov  ax, 0x0A4D           ; 'M'
    stosw
    mov  ax, 0x0A50           ; 'P'
    stosw

    ; Mask all IRQs
    cli
    mov  al, 0xFF
    out  0x21, al
    out  0xA1, al

    ; Clear registers
    xor  rax, rax
    xor  rbx, rbx
    xor  rcx, rcx
    xor  rdx, rdx
    xor  rsi, rsi
    xor  rdi, rdi
    xor  rbp, rbp
    xor  r8, r8
    xor  r9, r9
    xor  r10, r10
    xor  r11, r11
    xor  r12, r12
    xor  r13, r13
    xor  r14, r14
    xor  r15, r15

    ; === CALL KERNEL ===
    mov  rax, KERNEL_ADDR
    call rax

    jmp  .halt

.no_kernel:
    ; === DEBUG: "NO" (no kernel) ===
    mov  rdi, VGA + 160*4
    mov  ax, 0x0C4E           ; 'N' red
    stosw
    mov  ax, 0x0C4F           ; 'O'
    stosw
    mov  ax, 0x0C4B           ; 'K'
    stosw

.halt:
    cli
    hlt
    jmp  .halt

; ─── Padding to 16KB ───────────────────────────────────────
times 16384 - ($ - $$) db 0
