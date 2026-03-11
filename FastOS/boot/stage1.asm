; ============================================================
; FastOS v2.0 — stage1.asm (MBR — Stage 1)
; Compilar: fasm boot/stage1.asm boot/stage1.bin
;
; Filosofia: "Solo despierta el CPU, carga stage2."
; Su unico trabajo: cargar stage2 y saltar.
; No hace transiciones de modo.
; No maneja hardware complejo.
; 512 bytes exactos. No mas. No menos.
; ============================================================

format binary
org 0x7C00
use16

; ─── Segmentos y Stack ────────────────────────────────────
_start:
    cli
    xor  ax, ax
    mov  ds, ax
    mov  es, ax
    mov  ss, ax
    mov  sp, 0x7C00         ; Stack justo debajo del MBR
    sti

    mov  [boot_drv], dl     ; Guardar numero de disco BIOS

    ; Modo de video texto 80x25 — limpia pantalla
    mov  ax, 0x0003
    int  0x10

    ; Mensaje inicial
    mov  si, msg_stage1
    call bios_print

; ─── Cargar stage2 desde disco ───────────────────────────
; INT 0x13 AH=0x02: Read Sectors
;   AL  = sectores a leer (32 = 16KB de stage2)
;   CH  = cilindro 0
;   CL  = sector inicial 2 (sector 1 es el MBR)
;   DH  = cabeza 0
;   DL  = drive (ya guardado)
;   ES:BX = destino en memoria

load_stage2:
    mov  ah, 0x42           ; LBA Extended Read
    mov  dl, [boot_drv]     ; Boot drive
    mov  si, dap            ; Disk Address Packet
    int  0x13
    jc   disk_error

    mov  si, msg_ok
    call bios_print

    ; Pasar DL (boot drive) a stage2 y saltar
    mov  dl, [boot_drv]
    jmp  0x1000:0x0000      ; Saltar a stage2

; ─── Error de disco ──────────────────────────────────────
disk_error:
    mov  si, msg_err
    call bios_print
    cli
    hlt
    jmp  $                  ; Bucle de seguridad

; ─── Imprimir cadena con BIOS (TTY) ─────────────────────
; SI = puntero a cadena terminada en 0
bios_print:
    lodsb
    test al, al
    jz   .done
    mov  ah, 0x0E           ; BIOS TTY write
    mov  bx, 0x0007         ; BH=pagina 0, BL=atributo gris
    int  0x10
    jmp  bios_print
.done:
    ret

; ─── Datos ───────────────────────────────────────────────
boot_drv:    db 0
msg_stage1:  db "FastOS v2.0 Stage1", 13, 10, 0
msg_ok:      db "Stage2 OK", 13, 10, 0
msg_err:     db "Disk Error - Halt", 13, 10, 0

; ─── Disk Address Packet (DAP) para LBA ──────────────────
align 4
dap:
    db 0x10         ; DAP Size (16 bytes)
    db 0x00         ; Zero
    dw 100          ; Sectores a leer: 100 = 50KB (Loader 16KB + Kernel 32KB)
    dw 0x0000       ; Offset destino
    dw 0x1000       ; Segmento destino (0x1000:0x0000)
    dq 1            ; LBA Start (comenzar en sector 1, tras el MBR)

; ─── Padding y firma MBR ─────────────────────────────────
times 510 - ($ - $$) db 0  ; Rellenar hasta byte 510
dw 0xAA55                  ; Firma de boot valida (sector booteable)
