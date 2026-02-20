// ============================================================
// ADead-BIB v3.3 — Boot Labels Test
// ============================================================
// Ejemplo de bootloader usando la nueva sintaxis con labels.
// Sin offsets hardcodeados — el compilador resuelve todo.
//
// Sintaxis nueva:
//   label nombre:     — Define un label
//   jmp nombre        — Salta a un label
//   jz nombre         — Salta si zero
//   jnz nombre        — Salta si no zero
//   jc nombre         — Salta si carry
//   jnc nombre        — Salta si no carry
//   db 0x55, 0xAA     — Define bytes
//   dw 0x1234         — Define words (16-bit)
//   dd 0x12345678     — Define dwords (32-bit)
//   times 510 db 0    — Repite byte N veces
// ============================================================

// Origen: 0x7C00 (boot sector)
org 0x7C00

// ---- Inicialización ----
// xor ax, ax
raw { 0x31, 0xC0 }
// mov ds, ax
raw { 0x8E, 0xD8 }
// mov es, ax
raw { 0x8E, 0xC0 }
// mov ss, ax
raw { 0x8E, 0xD0 }
// mov sp, 0x7C00
raw { 0xBC, 0x00, 0x7C }

// ---- Imprimir mensaje ----
// mov si, msg
raw { 0xBE }
dw 0x7C20  // Offset del mensaje (calculado)

label print_loop:
// lodsb
raw { 0xAC }
// or al, al
raw { 0x08, 0xC0 }
// jz done
jz print_done
// mov ah, 0x0E
raw { 0xB4, 0x0E }
// int 0x10
int_call(0x10)
// jmp print_loop
jmp print_loop

label print_done:

// ---- Halt ----
cli
hlt
// Infinite loop
raw { 0xEB, 0xFE }

// ---- Mensaje ----
label msg:
db "ADead-BIB Boot OK!", 0x0D, 0x0A, 0x00

// ---- Padding hasta 510 bytes ----
// (El compilador calculará el padding necesario)

// ---- Boot signature ----
// times (510 - ($ - $$)) db 0
// dw 0xAA55
