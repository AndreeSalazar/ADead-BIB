; ============================================================
; FastOS_v2 Stage1 - MBR Bootloader for REAL HARDWARE
; 512 bytes, loads Stage2 from disk
; ============================================================

format binary
org 0x7C00
use16

STAGE2_SEGMENTS equ 32        ; 32 sectors = 16KB
STAGE2_ADDR     equ 0x1000    ; Load stage2 at segment 0x1000

start:
    ; Disable interrupts during setup
    cli
    
    ; Setup segments - BIOS loads us at 0x0000:0x7C00
    xor  ax, ax
    mov  ds, ax
    mov  es, ax
    mov  ss, ax
    mov  sp, 0x7C00           ; Stack below bootloader
    
    ; Save boot drive from BIOS
    mov  [boot_drive], dl
    
    sti

    ; Clear screen
    mov  ax, 0x0003
    int  0x10

    ; Print banner
    mov  si, msg_banner
    call print

    ; Load Stage2 using INT 13h
    mov  si, msg_loading
    call print

    ; Reset disk system
    xor  ax, ax
    mov  dl, [boot_drive]
    int  0x13
    jc   disk_error

    ; Load Stage2: 32 sectors starting at sector 2
    mov  ax, STAGE2_ADDR
    mov  es, ax
    xor  bx, bx               ; ES:BX = 0x1000:0x0000

    mov  ah, 0x02             ; BIOS read sectors
    mov  al, STAGE2_SEGMENTS  ; Number of sectors
    mov  ch, 0                ; Cylinder 0
    mov  cl, 2                ; Start at sector 2 (sector 1 = MBR)
    mov  dh, 0                ; Head 0
    mov  dl, [boot_drive]
    int  0x13
    jc   disk_error

    ; Verify we read correct number of sectors
    cmp  al, STAGE2_SEGMENTS
    jne  disk_error

    mov  si, msg_ok
    call print

    ; Jump to Stage2
    mov  dl, [boot_drive]     ; Pass boot drive to stage2
    jmp  STAGE2_ADDR:0x0000

disk_error:
    mov  si, msg_error
    call print
    jmp  halt

halt:
    cli
    hlt
    jmp  halt

; ─── Print string (SI = string pointer) ────────────────────
print:
    lodsb
    test al, al
    jz   .done
    mov  ah, 0x0E
    mov  bx, 0x0007
    int  0x10
    jmp  print
.done:
    ret

; ─── Data ──────────────────────────────────────────────────
boot_drive:  db 0
msg_banner:  db "FastOS v2.0 Stage1", 13, 10, 0
msg_loading: db "Loading Stage2...", 0
msg_ok:      db "OK", 13, 10, 0
msg_error:   db "DISK ERROR", 13, 10, 0

; ─── Padding and boot signature ────────────────────────────
times 510 - ($ - $$) db 0
dw 0xAA55
