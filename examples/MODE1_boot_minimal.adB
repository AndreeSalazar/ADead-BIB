// ============================================================
// ADead-BIB — MODO 1: Boot/OS (Machine Code Directo)
// ============================================================
// Este modo genera código máquina puro sin headers.
// Ideal para: bootloaders, kernels, drivers, firmware.
//
// Compilar: adeadc raw MODE1_boot_minimal.adB -o boot.bin
// Probar:   qemu-system-x86_64 -drive format=raw,file=boot.bin
//
// Características:
//   - org: dirección de carga
//   - raw {}: bytes de máquina inline
//   - cli/sti/hlt: instrucciones privilegiadas
//   - int_call(): interrupciones BIOS/sistema
//   - port_out(): I/O ports directos
//   - labels y jumps
// ============================================================

// ---- BOOT SECTOR: 512 bytes, carga en 0x7C00 ----
org 0x7C00

// Desabilitar interrupciones
cli

// Limpiar segmentos (xor ax,ax; mov ds/es/ss,ax)
raw { 0x31, 0xC0 }  // xor ax, ax
raw { 0x8E, 0xD8 }  // mov ds, ax
raw { 0x8E, 0xC0 }  // mov es, ax
raw { 0x8E, 0xD0 }  // mov ss, ax

// Stack en 0x7C00
raw { 0xBC, 0x00, 0x7C }  // mov sp, 0x7C00

// Habilitar interrupciones
sti

// ---- Imprimir mensaje usando BIOS ----
// INT 0x10, AH=0x0E (teletype output)

raw { 0xB4, 0x0E }  // mov ah, 0x0E

// Imprimir "OS"
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

// ---- Halt ----
hlt
raw { 0xEB, 0xFE }  // jmp $ (infinite loop)

// ---- Padding hasta byte 510 ----
// El compilador ADead-BIB añade padding automático para boot sectors

// ---- Boot signature (0x55AA) ----
// Se añade automáticamente con: adeadc raw --boot
