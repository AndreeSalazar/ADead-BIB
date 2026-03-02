// ============================================================
// ADead-BIB — MODO 1: Boot/OS (Machine Code Directo)
// ============================================================
// Genera código máquina puro sin headers.
// Ideal para: bootloaders, kernels, drivers, firmware.
//
// Compilar: adeadc raw MODE1_boot_minimal.adB -o boot.bin --boot
// Probar:   qemu-system-x86_64 -drive format=raw,file=boot.bin
//
// Modo 1 usa:
//   org     — dirección de carga
//   raw {}  — bytes de máquina inline
//   cli/sti/hlt — instrucciones privilegiadas
//   int_call()  — interrupciones BIOS
//   port_out()  — I/O ports directos
//   @naked      — sin prologue/epilogue
//   @interrupt  — handler con iretq automático
// ============================================================

// Dirección de carga del BIOS
org 0x7C00

// Deshabilitar interrupciones durante setup
cli

// Limpiar segmentos de datos
raw { 0x31, 0xC0 }  // xor ax, ax
raw { 0x8E, 0xD8 }  // mov ds, ax
raw { 0x8E, 0xC0 }  // mov es, ax
raw { 0x8E, 0xD0 }  // mov ss, ax

// Stack debajo del boot sector
raw { 0xBC, 0x00, 0x7C }  // mov sp, 0x7C00

// Habilitar interrupciones
sti

// ---- Imprimir "OS OK" usando BIOS INT 0x10 (teletype) ----
raw { 0xB4, 0x0E }  // mov ah, 0x0E (BIOS teletype)

raw { 0xB0, 0x4F }  // mov al, 'O'
int_call(0x10)
raw { 0xB0, 0x53 }  // mov al, 'S'
int_call(0x10)
raw { 0xB0, 0x20 }  // mov al, ' '
int_call(0x10)
raw { 0xB0, 0x4F }  // mov al, 'O'
int_call(0x10)
raw { 0xB0, 0x4B }  // mov al, 'K'
int_call(0x10)
raw { 0xB0, 0x0D }  // mov al, '\r'
int_call(0x10)
raw { 0xB0, 0x0A }  // mov al, '\n'
int_call(0x10)

// ---- Halt infinito ----
hlt
raw { 0xEB, 0xFE }  // jmp $ (infinite loop safety)

// Boot signature 0x55AA añadida automáticamente por: adeadc raw --boot
