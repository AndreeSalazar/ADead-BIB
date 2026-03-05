; ============================================================
; FastOS v2.0 — MBR for 64-bit (Stage 1)
; Loads loader (32 sectors) + kernel (32 sectors) separately
; ============================================================

format binary as 'bin'
org 0x7C00

use16

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
    
    ; Load 96 sectors (loader 16KB + embedded kernel 32KB) to 0x1000:0x0000
    mov ax, 0x1000
    mov es, ax
    xor bx, bx
    mov ah, 0x02
    mov al, 96        ; Read 96 sectors (48KB total)
    mov ch, 0
    mov cl, 2
    mov dh, 0
    mov dl, [boot_drv]
    int 0x13
    jc disk_err
    
    mov dl, [boot_drv]
    jmp 0x1000:0x0000

disk_err:
    mov si, msg_err
    call print
    cli
    hlt
    jmp $

print:
    lodsb
    test al, al
    jz .d
    mov ah, 0x0E
    int 0x10
    jmp print
.d: ret

boot_drv: db 0
msg: db "FastOS", 13, 10, 0
msg_err: db "Err", 0

times 510 - ($ - $$) db 0
dw 0xAA55
