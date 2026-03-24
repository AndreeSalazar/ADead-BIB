; ============================================================
; FastOS v2.0 — stage2_real.asm (REAL HARDWARE VERSION)
; Designed to work on REAL PCs, not just QEMU
; ============================================================

format binary
org 0x0000
use16

; ============================================================
; Memory Layout Constants
; ============================================================
STAGE2_SEG      equ 0x1000    ; We're loaded at 0x1000:0x0000 = 0x10000
STAGE2_LINEAR   equ 0x10000   ; Linear address where stage2 lives
STACK_16        equ 0x9000    ; 16-bit stack
STACK_32        equ 0x90000   ; 32-bit stack  
STACK_64        equ 0x90000   ; 64-bit stack
KERNEL_ADDR     equ 0x100000  ; Kernel loaded here (1MB)

; Page tables in low memory (safe area)
PAGE_PML4       equ 0x70000
PAGE_PDPT       equ 0x71000
PAGE_PD         equ 0x72000
PAGE_PD1        equ 0x73000
PAGE_PD2        equ 0x74000
PAGE_PD3        equ 0x75000

; ============================================================
; PHASE 1: 16-BIT REAL MODE
; ============================================================
start16:
    ; Save boot drive
    mov  [boot_drive], dl

    ; Setup segments - we're at 0x1000:0x0000
    cli
    mov  ax, STAGE2_SEG
    mov  ds, ax
    mov  es, ax
    xor  ax, ax
    mov  ss, ax
    mov  sp, STACK_16
    sti

    ; Print startup message via BIOS
    mov  si, msg_start
    call print16

; ─── Enable A20 Line ───────────────────────────────────────
enable_a20:
    ; Method 1: BIOS
    mov  ax, 0x2401
    int  0x15
    jnc  .a20_ok

    ; Method 2: Fast A20 (port 0x92)
    in   al, 0x92
    test al, 0x02
    jnz  .a20_ok
    or   al, 0x02
    and  al, 0xFE
    out  0x92, al

.a20_ok:
    mov  si, msg_a20
    call print16

; ─── Detect Memory (E820) ──────────────────────────────────
detect_memory:
    mov  ax, 0x2000
    mov  es, ax
    xor  di, di
    xor  ebx, ebx
    mov  word [es:di], 0

.e820_loop:
    add  di, 2
    mov  eax, 0x0000E820
    mov  edx, 0x534D4150
    mov  ecx, 24
    int  0x15
    jc   .e820_done
    cmp  eax, 0x534D4150
    jne  .e820_done
    test ecx, ecx
    jz   .e820_done
    sub  di, 2
    inc  word [es:di]
    add  di, 2
    add  di, 24
    test ebx, ebx
    jz   .e820_done
    cmp  di, 240
    jb   .e820_loop

.e820_done:
    mov  ax, STAGE2_SEG
    mov  es, ax
    mov  si, msg_mem
    call print16

; ─── Skip VESA for now - use VGA text mode ─────────────────
    ; Clear VESA info (kernel will use VGA text fallback)
    mov  ax, 0x0500
    mov  es, ax
    xor  eax, eax
    mov  [es:0x0000], eax    ; fb_addr = 0 = no VESA
    mov  [es:0x0004], eax
    mov  [es:0x0008], eax
    mov  [es:0x000C], eax
    mov  ax, STAGE2_SEG
    mov  es, ax

    mov  si, msg_vga
    call print16

; ─── Prepare for Protected Mode ────────────────────────────
    mov  si, msg_pm
    call print16

    cli                      ; CRITICAL: No interrupts during mode switch

    ; Load GDT - use CS-relative address
    lgdt [gdt32_ptr]

; ─── Enter 32-bit Protected Mode ───────────────────────────
    mov  eax, cr0
    or   al, 1               ; Set PE bit
    mov  cr0, eax

    ; Far jump to 32-bit code - flush prefetch queue
    jmp  dword 0x08:pm_entry

; ─── 16-bit Print Helper ───────────────────────────────────
print16:
    lodsb
    test al, al
    jz   .done
    mov  ah, 0x0E
    mov  bx, 0x0007
    int  0x10
    jmp  print16
.done:
    ret

; ─── 16-bit Data ───────────────────────────────────────────
boot_drive:  db 0
msg_start:   db "FastOS v2.0 Stage2", 13, 10, 0
msg_a20:     db "A20 OK", 13, 10, 0
msg_mem:     db "E820 OK", 13, 10, 0
msg_vga:     db "VGA text mode", 13, 10, 0
msg_pm:      db "-> 32-bit PM", 13, 10, 0

; ─── GDT for 32-bit Protected Mode ─────────────────────────
align 8
gdt32_start:
    dq 0                     ; Null descriptor

gdt32_code:                  ; 0x08: 32-bit code
    dw 0xFFFF                ; Limit low
    dw 0x0000                ; Base low
    db 0x00                  ; Base mid
    db 0x9A                  ; Access: Present, Ring 0, Code, Readable
    db 0xCF                  ; Flags: 4K granularity, 32-bit
    db 0x00                  ; Base high

gdt32_data:                  ; 0x10: 32-bit data
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0x92                  ; Access: Present, Ring 0, Data, Writable
    db 0xCF
    db 0x00
gdt32_end:

gdt32_ptr:
    dw gdt32_end - gdt32_start - 1
    dd STAGE2_LINEAR + gdt32_start

; ============================================================
; PHASE 2: 32-BIT PROTECTED MODE
; ============================================================
align 16
use32
pm_entry:
    ; Setup 32-bit segments
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  esp, STACK_32

    ; === DEBUG: Write directly to VGA to prove we're in 32-bit ===
    mov  edi, 0xB8000 + (3 * 160)
    mov  eax, 0x0A5B         ; '[' in green
    stosd
    mov  eax, 0x0A33         ; '3'
    stosd
    mov  eax, 0x0A32         ; '2'
    stosd
    mov  eax, 0x0A5D         ; ']'
    stosd
    mov  eax, 0x0A20         ; ' '
    stosd
    mov  eax, 0x0A4F         ; 'O'
    stosd
    mov  eax, 0x0A4B         ; 'K'
    stosd

; ─── Setup 4-Level Paging ──────────────────────────────────
setup_paging:
    ; Clear page table area (24KB)
    mov  edi, PAGE_PML4
    xor  eax, eax
    mov  ecx, 6144           ; 24KB / 4
    rep  stosd

    ; PML4[0] -> PDPT
    mov  dword [PAGE_PML4], PAGE_PDPT + 0x03

    ; PDPT entries -> PD tables (4GB identity map)
    mov  dword [PAGE_PDPT + 0x00], PAGE_PD  + 0x03
    mov  dword [PAGE_PDPT + 0x08], PAGE_PD1 + 0x03
    mov  dword [PAGE_PDPT + 0x10], PAGE_PD2 + 0x03
    mov  dword [PAGE_PDPT + 0x18], PAGE_PD3 + 0x03

    ; Fill PD0: 0x00000000 - 0x3FFFFFFF (2MB pages)
    mov  edi, PAGE_PD
    mov  eax, 0x000083       ; Present + RW + PS (2MB page)
    mov  ecx, 512
.fill_pd0:
    mov  [edi], eax
    mov  dword [edi+4], 0
    add  eax, 0x200000
    add  edi, 8
    dec  ecx
    jnz  .fill_pd0

    ; Fill PD1: 0x40000000 - 0x7FFFFFFF
    mov  edi, PAGE_PD1
    mov  eax, 0x40000083
    mov  ecx, 512
.fill_pd1:
    mov  [edi], eax
    mov  dword [edi+4], 0
    add  eax, 0x200000
    add  edi, 8
    dec  ecx
    jnz  .fill_pd1

    ; Fill PD2: 0x80000000 - 0xBFFFFFFF
    mov  edi, PAGE_PD2
    mov  eax, 0x80000083
    mov  ecx, 512
.fill_pd2:
    mov  [edi], eax
    mov  dword [edi+4], 0
    add  eax, 0x200000
    add  edi, 8
    dec  ecx
    jnz  .fill_pd2

    ; Fill PD3: 0xC0000000 - 0xFFFFFFFF
    mov  edi, PAGE_PD3
    mov  eax, 0xC0000083
    mov  ecx, 512
.fill_pd3:
    mov  [edi], eax
    mov  dword [edi+4], 0
    add  eax, 0x200000
    add  edi, 8
    dec  ecx
    jnz  .fill_pd3

    ; DEBUG: Paging setup done
    mov  edi, 0xB8000 + (4 * 160)
    mov  eax, 0x0E50         ; 'P' yellow
    stosd
    mov  eax, 0x0E41         ; 'A'
    stosd
    mov  eax, 0x0E47         ; 'G'
    stosd
    mov  eax, 0x0E45         ; 'E'
    stosd

    ; Load CR3 with PML4 address
    mov  eax, PAGE_PML4
    mov  cr3, eax

    ; Enable PAE (required for long mode)
    mov  eax, cr4
    or   eax, 0x20           ; PAE bit
    mov  cr4, eax

    ; Enable Long Mode in EFER MSR
    mov  ecx, 0xC0000080     ; EFER MSR
    rdmsr
    or   eax, 0x100          ; LME bit
    wrmsr

    ; Load 64-bit GDT
    lgdt [STAGE2_LINEAR + gdt64_ptr]

    ; Enable paging -> activates long mode
    mov  eax, cr0
    or   eax, 0x80000000     ; PG bit
    mov  cr0, eax

    ; Far jump to 64-bit code
    jmp  0x08:lm_entry

; ─── 32-bit Data ───────────────────────────────────────────
align 8
gdt64_start:
    dq 0                     ; Null

gdt64_code:                  ; 0x08: 64-bit code
    dw 0x0000
    dw 0x0000
    db 0x00
    db 0x9A                  ; Present, Ring 0, Code
    db 0x20                  ; L=1 (64-bit), G=0
    db 0x00

gdt64_data:                  ; 0x10: 64-bit data
    dw 0x0000
    dw 0x0000
    db 0x00
    db 0x92                  ; Present, Ring 0, Data
    db 0x00
    db 0x00
gdt64_end:

gdt64_ptr:
    dw gdt64_end - gdt64_start - 1
    dq STAGE2_LINEAR + gdt64_start

; ============================================================
; PHASE 3: 64-BIT LONG MODE
; ============================================================
align 16
use64
lm_entry:
    ; Setup 64-bit segments
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  rsp, STACK_64

    ; DEBUG: Write [64] to VGA
    mov  rdi, 0xB8000 + (5 * 160)
    mov  rax, 0x0B5B         ; '[' cyan
    stosw
    mov  rax, 0x0B36         ; '6'
    stosw
    mov  rax, 0x0B34         ; '4'
    stosw
    mov  rax, 0x0B5D         ; ']'
    stosw
    mov  rax, 0x0B20         ; ' '
    stosw
    mov  rax, 0x0B4F         ; 'O'
    stosw
    mov  rax, 0x0B4B         ; 'K'
    stosw

; ─── Enable SSE ────────────────────────────────────────────
    mov  rax, cr0
    and  ax, 0xFFFB          ; Clear EM
    or   ax, 0x0002          ; Set MP
    mov  cr0, rax

    mov  rax, cr4
    or   ax, 0x0600          ; OSFXSR + OSXMMEXCPT
    mov  cr4, rax

    ; DEBUG: SSE OK
    mov  rdi, 0xB8000 + (6 * 160)
    mov  rax, 0x0A53         ; 'S' green
    stosw
    mov  rax, 0x0A53         ; 'S'
    stosw
    mov  rax, 0x0A45         ; 'E'
    stosw

; ─── Copy Kernel to 0x100000 ───────────────────────────────
    ; Kernel is at 0x14000 (after 16KB stage2)
    mov  rsi, 0x14000
    mov  rdi, KERNEL_ADDR
    mov  rcx, 8192           ; 64KB / 8
    cld
    rep  movsq

    ; DEBUG: Kernel copied
    mov  rdi, 0xB8000 + (7 * 160)
    mov  rax, 0x0E4B         ; 'K' yellow
    stosw
    mov  rax, 0x0E45         ; 'E'
    stosw
    mov  rax, 0x0E52         ; 'R'
    stosw
    mov  rax, 0x0E4E         ; 'N'
    stosw
    mov  rax, 0x0E45         ; 'E'
    stosw
    mov  rax, 0x0E4C         ; 'L'
    stosw

    ; Verify kernel exists
    mov  rax, KERNEL_ADDR
    mov  eax, [rax]
    test eax, eax
    jz   .no_kernel

    ; DEBUG: Jumping to kernel
    mov  rdi, 0xB8000 + (8 * 160)
    mov  rax, 0x0A4A         ; 'J' green
    stosw
    mov  rax, 0x0A55         ; 'U'
    stosw
    mov  rax, 0x0A4D         ; 'M'
    stosw
    mov  rax, 0x0A50         ; 'P'
    stosw

    ; Disable interrupts and mask PIC
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

    ; CALL KERNEL
    mov  rax, KERNEL_ADDR
    call rax

    ; If kernel returns (shouldn't)
    jmp  .halt

.no_kernel:
    ; No kernel found - show error
    mov  rdi, 0xB8000 + (9 * 160)
    mov  rax, 0x0C4E         ; 'N' red
    stosw
    mov  rax, 0x0C4F         ; 'O'
    stosw
    mov  rax, 0x0C20         ; ' '
    stosw
    mov  rax, 0x0C4B         ; 'K'
    stosw
    mov  rax, 0x0C45         ; 'E'
    stosw
    mov  rax, 0x0C52         ; 'R'
    stosw
    mov  rax, 0x0C4E         ; 'N'
    stosw

.halt:
    cli
    hlt
    jmp  .halt

; ─── Padding to 16KB ───────────────────────────────────────
times 16384 - ($ - $$) db 0
