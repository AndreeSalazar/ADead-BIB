; ============================================================
; FastOS v2.0 — MBR for 64-bit (Stage 1)
; Minimal 512-byte MBR that loads stage2_64
; ============================================================

format binary as 'bin'
org 0x7C00

use16

STAGE2_SEG      equ 0x1000
STAGE2_SECTORS  equ 32

_start:
    cli
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
    mov sp, 0x7C00
    mov [boot_drv], dl
    sti
    
    mov ax, 0x0003
    int 0x10
    
    mov si, msg
    call print
    
    mov ax, STAGE2_SEG
    mov es, ax
    xor bx, bx
    mov ah, 0x02
    mov al, STAGE2_SECTORS
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drv]
    int 0x13
    jc disk_err
    
    jmp STAGE2_SEG:0x0000

disk_err:
    mov si, msg_err
    call print
halt_loop:
    cli
    hlt
    jmp halt_loop

print:
    lodsb
    test al, al
    jz .d
    mov ah, 0x0E
    int 0x10
    jmp print
.d: ret

boot_drv: db 0
msg:      db "FastOS 64-bit MBR", 13, 10, 0
msg_err:  db "Disk err", 0

times 510 - ($ - $$) db 0
dw 0xAA55
