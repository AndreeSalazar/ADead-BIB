; ============================================================
; FastOS v2.0 - CSM LEGACY BOOTLOADER (Ryzen 5 5600X)
; Gradual: 16→32→64→128(SSE)→256(AVX/AVX2)
; ============================================================

; ====== MBR: SECTOR 0 (512 bytes) ======
[BITS 16]
[ORG 0x7C00]

mbr_start:
    cli
    xor  ax, ax
    mov  ds, ax
    mov  es, ax
    mov  ss, ax
    mov  sp, 0x7C00
    sti

    mov  [boot_drv], dl

    ; Clear screen
    mov  ax, 0x0003
    int  0x10

    ; Banner
    mov  si, m_ban
    call pr
    mov  si, m_csm
    call pr

    ; Reset disk
    xor  ax, ax
    mov  dl, [boot_drv]
    int  0x13

    ; Check LBA
    mov  ah, 0x41
    mov  bx, 0x55AA
    mov  dl, [boot_drv]
    int  0x13
    jc   .chs
    cmp  bx, 0xAA55
    jne  .chs

    ; LBA read with retry
    mov  byte [retries], 3
.lba:
    mov  si, dap
    mov  byte [si],   0x10
    mov  byte [si+1], 0
    mov  word [si+2], 32       ; 16KB
    mov  word [si+4], 0x1000
    mov  word [si+6], 0
    mov  dword [si+8], 1
    mov  dword [si+12], 0
    mov  ah, 0x42
    mov  dl, [boot_drv]
    int  0x13
    jnc  .ok
    dec  byte [retries]
    jz   .chs
    xor  ax, ax
    mov  dl, [boot_drv]
    int  0x13
    jmp  .lba

.chs:
    mov  byte [retries], 3
.chs_r:
    mov  ah, 0x02
    mov  al, 32
    mov  ch, 0
    mov  cl, 2
    mov  dh, 0
    mov  dl, [boot_drv]
    xor  bx, bx
    mov  es, bx
    mov  bx, 0x1000
    int  0x13
    jnc  .ok
    dec  byte [retries]
    jz   .err
    pusha
    xor  ax, ax
    mov  dl, [boot_drv]
    int  0x13
    popa
    jmp  .chs_r

.ok:
    mov  si, m_ok
    call pr
    mov  dl, [boot_drv]
    jmp  0x0000:0x1000

.err:
    mov  si, m_de
    call pr
    mov  al, ah
    call phex
.hlt:
    cli
    hlt
    jmp  .hlt

pr:
    lodsb
    test al, al
    jz   .d
    mov  ah, 0x0E
    mov  bx, 7
    int  0x10
    jmp  pr
.d: ret

phex:
    push ax
    shr  al, 4
    call .n
    pop  ax
    and  al, 0xF
    call .n
    ret
.n:
    add  al, '0'
    cmp  al, '9'
    jbe  .e
    add  al, 7
.e: mov  ah, 0x0E
    int  0x10
    ret

boot_drv:  db 0
retries:   db 0
align 4
dap:       times 16 db 0

m_ban: db "FastOS v2.0 Ryzen5", 13, 10, 0
m_csm: db "CSM Legacy Boot", 13, 10, 0
m_ok:  db "Stage2 loaded", 13, 10, 0
m_de:  db "DISK ERR:", 0

times 510 - ($ - mbr_start) db 0
dw 0xAA55
