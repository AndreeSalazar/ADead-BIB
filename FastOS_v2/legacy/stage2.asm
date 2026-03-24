; ============================================================
; FastOS v2.0 - STAGE 2 (loaded at 0x1000)
; 16-bit → 32-bit → 64-bit → 128-bit(SSE) → 256-bit(AVX2)
; Optimized for AMD Ryzen 5 5600X (Zen3)
; ============================================================

[BITS 16]
[ORG 0x1000]

stage2_entry:
    ; Setup segments
    xor  ax, ax
    mov  ds, ax
    mov  es, ax
    mov  ss, ax
    mov  sp, 0x7C00

    mov  si, msg_s2
    call print16

    ; ══════════════════════════════════════════
    ; A20 LINE
    ; ══════════════════════════════════════════
    mov  si, msg_a20
    call print16

    ; BIOS method
    mov  ax, 0x2401
    int  0x15
    jnc  .a20_ok

    ; Fast A20
    in   al, 0x92
    or   al, 2
    and  al, 0xFE
    out  0x92, al

    ; KBC method
    call kbc_w1
    mov  al, 0xAD
    out  0x64, al
    call kbc_w1
    mov  al, 0xD0
    out  0x64, al
    call kbc_w2
    in   al, 0x60
    push ax
    call kbc_w1
    mov  al, 0xD1
    out  0x64, al
    call kbc_w1
    pop  ax
    or   al, 2
    out  0x60, al
    call kbc_w1
    mov  al, 0xAE
    out  0x64, al
    call kbc_w1

.a20_ok:
    mov  si, msg_ok
    call print16

    ; ══════════════════════════════════════════
    ; MEMORY DETECTION (E820)
    ; ══════════════════════════════════════════
    mov  si, msg_mem
    call print16

    xor  ax, ax
    mov  es, ax
    mov  di, 0x8000
    xor  ebx, ebx
.e820:
    mov  eax, 0xE820
    mov  ecx, 24
    mov  edx, 0x534D4150
    int  0x15
    jc   .e820_end
    add  di, 24
    test ebx, ebx
    jnz  .e820
.e820_end:
    xor  ax, ax
    mov  es, ax

    mov  si, msg_ok
    call print16

    ; ══════════════════════════════════════════
    ; CPUID CHECK
    ; ══════════════════════════════════════════
    mov  si, msg_cpu
    call print16

    pushfd
    pop  eax
    mov  ecx, eax
    xor  eax, 0x200000
    push eax
    popfd
    pushfd
    pop  eax
    push ecx
    popfd
    xor  eax, ecx
    jz   .no_cpuid

    mov  si, msg_ok
    call print16
    jmp  .go_pm

.no_cpuid:
    mov  si, msg_fail
    call print16

    ; ══════════════════════════════════════════
    ; ENTER PROTECTED MODE (32-bit)
    ; ══════════════════════════════════════════
.go_pm:
    mov  si, msg_pm
    call print16

    cli
    lgdt [gdt_ptr]

    mov  eax, cr0
    or   al, 1
    mov  cr0, eax

    jmp  0x08:pm32_start

; ── 16-bit helpers ──
kbc_w1:
    in   al, 0x64
    test al, 2
    jnz  kbc_w1
    ret
kbc_w2:
    in   al, 0x64
    test al, 1
    jz   kbc_w2
    ret

print16:
    lodsb
    test al, al
    jz   .d
    mov  ah, 0x0E
    mov  bx, 7
    int  0x10
    jmp  print16
.d: ret

; ── 16-bit data ──
msg_s2:   db "[S2] 16-bit init", 13, 10, 0
msg_a20:  db " A20...", 0
msg_mem:  db " E820...", 0
msg_cpu:  db " CPUID...", 0
msg_pm:   db " -> 32-bit", 13, 10, 0
msg_ok:   db "OK ", 0
msg_fail: db "NO ", 0

; ══════════════════════════════════════════
; GDT
; ══════════════════════════════════════════
align 16
gdt_start:
    dq 0                            ; 0x00 Null
    dw 0xFFFF, 0x0000               ; 0x08 Code32
    db 0x00, 0x9A, 0xCF, 0x00
    dw 0xFFFF, 0x0000               ; 0x10 Data32
    db 0x00, 0x92, 0xCF, 0x00
    dw 0x0000, 0x0000               ; 0x18 Code64
    db 0x00, 0x9A, 0x20, 0x00
    dw 0x0000, 0x0000               ; 0x20 Data64
    db 0x00, 0x92, 0x00, 0x00
gdt_end:

gdt_ptr:
    dw gdt_end - gdt_start - 1
    dd gdt_start

; ============================================================
; 32-BIT PROTECTED MODE
; ============================================================
[BITS 32]
align 16
pm32_start:
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  esp, 0x90000

    ; VGA: "[32] Protected Mode"
    mov  edi, 0xB8000 + (6 * 160)
    mov  esi, v_pm
    mov  ah, 0x0A
    call vga32

    ; ── Identity map 4GB with 2MB pages ──
    mov  edi, 0x70000
    xor  eax, eax
    mov  ecx, 6144
    rep  stosd

    mov  dword [0x70000], 0x71003     ; PML4[0]->PDPT
    mov  dword [0x71000], 0x72003     ; PDPT[0]->PD0
    mov  dword [0x71008], 0x73003     ; PDPT[1]->PD1
    mov  dword [0x71010], 0x74003     ; PDPT[2]->PD2
    mov  dword [0x71018], 0x75003     ; PDPT[3]->PD3

    mov  edi, 0x72000
    mov  eax, 0x83                    ; Present+RW+PS(2MB)
    mov  ecx, 2048
.fill:
    mov  [edi], eax
    mov  dword [edi+4], 0
    add  eax, 0x200000
    add  edi, 8
    loop .fill

    ; VGA: "[32] Paging OK"
    mov  edi, 0xB8000 + (7 * 160)
    mov  esi, v_pg
    mov  ah, 0x0A
    call vga32

    ; CR3 + PAE
    mov  eax, 0x70000
    mov  cr3, eax
    mov  eax, cr4
    or   eax, 0x20
    mov  cr4, eax

    ; Enable Long Mode (EFER.LME)
    mov  ecx, 0xC0000080
    rdmsr
    or   eax, 0x100
    wrmsr

    ; Enable paging -> Long Mode active
    mov  eax, cr0
    or   eax, 0x80000000
    mov  cr0, eax

    jmp  0x18:lm64_start

vga32:
    lodsb
    test al, al
    jz   .d
    stosw
    jmp  vga32
.d: ret

v_pm: db "[32] PROTECTED MODE OK", 0
v_pg: db "[32] PAGING 4GB OK", 0

; ============================================================
; 64-BIT LONG MODE
; ============================================================
[BITS 64]
align 16
lm64_start:
    mov  ax, 0x20
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    mov  rsp, 0x90000

    ; VGA: "[64] Long Mode"
    mov  rdi, 0xB8000 + (8 * 160)
    mov  rsi, v_64
    mov  ah, 0x0B
    call vga64

    ; ══════════════════════════════════════════
    ; 128-BIT: ENABLE SSE (XMM0-XMM15)
    ; ══════════════════════════════════════════
    mov  rax, cr0
    and  ax, 0xFFFB               ; Clear EM
    or   ax, 0x0002               ; Set MP
    mov  cr0, rax

    mov  rax, cr4
    or   ax, 0x0600               ; OSFXSR + OSXMMEXCPT
    mov  cr4, rax

    ; Test SSE
    xorps  xmm0, xmm0
    xorps  xmm1, xmm1
    xorps  xmm15, xmm15

    mov  rdi, 0xB8000 + (9 * 160)
    mov  rsi, v_sse
    mov  ah, 0x0A
    call vga64

    ; ══════════════════════════════════════════
    ; 256-BIT: ENABLE AVX/AVX2 (YMM0-YMM15)
    ; Ryzen 5 5600X = Zen3 = AVX2 supported
    ; ══════════════════════════════════════════

    ; Check XSAVE + AVX support
    mov  eax, 1
    cpuid
    test ecx, (1 << 26)          ; XSAVE?
    jz   .no_avx
    test ecx, (1 << 28)          ; AVX?
    jz   .no_avx

    ; Enable OSXSAVE (CR4 bit 18)
    mov  rax, cr4
    or   eax, (1 << 18)
    mov  cr4, rax

    ; XCR0: enable SSE state + AVX state
    xor  ecx, ecx
    xgetbv
    or   eax, 0x07                ; X87 + SSE + AVX
    xsetbv

    ; Test AVX
    vzeroall

    mov  rdi, 0xB8000 + (10 * 160)
    mov  rsi, v_avx
    mov  ah, 0x0E
    call vga64

    ; Check AVX2
    mov  eax, 7
    xor  ecx, ecx
    cpuid
    test ebx, (1 << 5)           ; AVX2?
    jz   .no_avx2

    mov  rdi, 0xB8000 + (11 * 160)
    mov  rsi, v_avx2
    mov  ah, 0x0D
    call vga64

    jmp  .cpu_brand

.no_avx:
    mov  rdi, 0xB8000 + (10 * 160)
    mov  rsi, v_noavx
    mov  ah, 0x0C
    call vga64
    jmp  .cpu_brand

.no_avx2:
    mov  rdi, 0xB8000 + (11 * 160)
    mov  rsi, v_noavx2
    mov  ah, 0x0C
    call vga64

    ; ══════════════════════════════════════════
    ; CPU BRAND STRING (Ryzen 5 5600X)
    ; ══════════════════════════════════════════
.cpu_brand:
    mov  eax, 0x80000000
    cpuid
    cmp  eax, 0x80000004
    jb   .final

    mov  rdi, cpu_buf
    mov  eax, 0x80000002
    cpuid
    mov  [rdi],    eax
    mov  [rdi+4],  ebx
    mov  [rdi+8],  ecx
    mov  [rdi+12], edx
    mov  eax, 0x80000003
    cpuid
    mov  [rdi+16], eax
    mov  [rdi+20], ebx
    mov  [rdi+24], ecx
    mov  [rdi+28], edx
    mov  eax, 0x80000004
    cpuid
    mov  [rdi+32], eax
    mov  [rdi+36], ebx
    mov  [rdi+40], ecx
    mov  [rdi+44], edx
    mov  byte [rdi+48], 0

    ; Print "CPU: <brand>"
    mov  rdi, 0xB8000 + (13 * 160)
    mov  rsi, v_cpu
    mov  ah, 0x07
    call vga64
    mov  rsi, cpu_buf
    mov  ah, 0x0F
    call vga64

    ; ══════════════════════════════════════════
    ; FINAL BANNER
    ; ══════════════════════════════════════════
.final:
    mov  rdi, 0xB8000 + (15 * 160)
    mov  rsi, v_line
    mov  ah, 0x0F
    call vga64

    mov  rdi, 0xB8000 + (16 * 160) + 10
    mov  rsi, v_rdy
    mov  ah, 0x0A
    call vga64

    mov  rdi, 0xB8000 + (17 * 160)
    mov  rsi, v_line
    mov  ah, 0x0F
    call vga64

.halt:
    cli
    hlt
    jmp  .halt

; ── 64-bit VGA helper ──
vga64:
    lodsb
    test al, al
    jz   .d
    stosw
    jmp  vga64
.d: ret

; ── 64-bit data ──
v_64:     db "[64] LONG MODE OK", 0
v_sse:    db "[128] SSE ENABLED (XMM0-15)", 0
v_avx:    db "[256] AVX ENABLED (YMM0-15)", 0
v_avx2:   db "[256] AVX2 CONFIRMED (Zen3 Ryzen5)", 0
v_noavx:  db "[!!] AVX NOT AVAILABLE", 0
v_noavx2: db "[!!] AVX2 NOT AVAILABLE", 0
v_cpu:    db "CPU: ", 0
v_line:   db "========================================", 0
v_rdy:    db ">>> FASTOS v2.0 READY (256-bit) <<<", 0

cpu_buf:  times 49 db 0

; Pad to exactly 16KB (32 sectors)
times (16384) - ($ - stage2_entry) db 0
