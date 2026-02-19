// ============================================================
// ADead-BIB — OS Kernel Setup Example (Phase 6)
// ============================================================
// Demuestra las capacidades OS-level de ADead-BIB:
//   - Privileged instructions (cli, sti, hlt, cpuid)
//   - Register access (reg rax = value)
//   - Memory-mapped I/O (write_mem, read_mem)
//   - Port I/O (port_out, port_in)
//   - Interrupt handlers (@interrupt attribute)
//   - Far jumps (mode switching)
//   - Raw machine code blocks
//
// Este ejemplo muestra cómo ADead-BIB reemplaza ASM
// completamente para desarrollo de OS.
//
// Compilar: adB flat os_kernel_setup.adB -o kernel.bin
// ============================================================

// ---- Fase 1: Inicialización del kernel ----

// Desactivar interrupciones durante setup
cli

// Limpiar registros de propósito general
reg rax = 0
reg rbx = 0
reg rcx = 0
reg rdx = 0

// ---- Fase 2: Setup de stack ----

// Configurar stack pointer en 0x90000
reg rsp = 0x90000
reg rbp = 0x90000

// ---- Fase 3: Detección de CPU ----

// CPUID para identificar el procesador
cpuid

// ---- Fase 4: Configurar puerto serial (COM1 = 0x3F8) ----

// Desactivar interrupciones del UART
port_out(0x3F9, 0x00)

// Activar DLAB (Divisor Latch Access Bit)
port_out(0x3FB, 0x80)

// Set divisor a 3 (38400 baud)
port_out(0x3F8, 0x03)
port_out(0x3F9, 0x00)

// 8 bits, no parity, 1 stop bit
port_out(0x3FB, 0x03)

// Activar FIFO
port_out(0x3FA, 0xC7)

// Activar IRQs, RTS/DSR set
port_out(0x3FC, 0x0B)

// ---- Fase 5: Enviar 'OK' por serial ----

// Enviar 'O'
port_out(0x3F8, 0x4F)

// Enviar 'K'
port_out(0x3F8, 0x4B)

// ---- Fase 6: Escribir a VGA text buffer (0xB8000) ----

// Escribir 'A' en blanco sobre negro en posición (0,0)
// VGA: byte 0 = character, byte 1 = attribute (0x0F = white on black)
write_mem(0xB8000, 0x0F41)

// Escribir 'D' en posición (0,1)
write_mem(0xB8002, 0x0F44)

// Escribir 'B' en posición (0,2)
write_mem(0xB8004, 0x0F42)

// ---- Fase 7: Habilitar interrupciones y halt ----

sti
hlt

// Safety net: infinite loop
raw { 0xEB, 0xFE }
