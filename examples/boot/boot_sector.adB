// ============================================================
// ADead-BIB — Boot Sector Example (v3.1-OS Phase 6)
// ============================================================
// Genera un boot sector válido de 512 bytes que se ejecuta
// directamente en el arranque del CPU (modo real 16-bit).
//
// Compilar: adB boot boot_sector.adB -o boot.bin
// Probar:   qemu-system-x86_64 -drive format=raw,file=boot.bin
//
// Características demostradas:
//   - org directive (dirección de carga)
//   - cli/sti (control de interrupciones)
//   - reg assignments (acceso directo a registros)
//   - raw blocks (bytes de máquina inline)
//   - int_call (interrupciones BIOS)
//   - port_out (I/O ports)
//   - hlt (detener CPU)
// ============================================================

// Establecer origen en 0x7C00 (donde BIOS carga el boot sector)
org 0x7C00

// Desabilitar interrupciones durante inicialización
cli

// Limpiar segmentos: xor ax, ax → mov ds/es/ss, ax
raw { 0x31, 0xC0, 0x8E, 0xD8, 0x8E, 0xC0, 0x8E, 0xD0 }

// Configurar stack pointer en 0x7C00 (crece hacia abajo)
raw { 0xBC, 0x00, 0x7C }

// Habilitar interrupciones
sti

// ---- Imprimir "ADeadBIB" usando BIOS INT 0x10 ----

// Modo teletype: AH = 0x0E
raw { 0xB4, 0x0E }

// 'A'
raw { 0xB0, 0x41 }
int_call(0x10)

// 'D'
raw { 0xB0, 0x44 }
int_call(0x10)

// 'e'
raw { 0xB0, 0x65 }
int_call(0x10)

// 'a'
raw { 0xB0, 0x61 }
int_call(0x10)

// 'd'
raw { 0xB0, 0x64 }
int_call(0x10)

// '-'
raw { 0xB0, 0x2D }
int_call(0x10)

// 'B'
raw { 0xB0, 0x42 }
int_call(0x10)

// 'I'
raw { 0xB0, 0x49 }
int_call(0x10)

// 'B'
raw { 0xB0, 0x42 }
int_call(0x10)

// ---- Infinite loop (halt) ----
hlt
// JMP $ (infinite loop as safety net)
raw { 0xEB, 0xFE }
