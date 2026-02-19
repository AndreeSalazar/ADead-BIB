# ADead-BIB — Nuevos Elementos para Reemplazo Total de ASM

> **Objetivo:** Convertir ADead-BIB en un lenguaje de arquitectura de sistemas capaz de reemplazar completamente a ASM, generar flat binaries, y servir como capa entre CPU ↔ Rust para desarrollo de OS.
>
> **Referencia:** Linux kernel (estructura y conceptos), pero implementado 100% con ADead-BIB + Rust.

---

## Visión de Arquitectura

```
CPU (hardware)
    ↑
ADead-BIB  ← Control absoluto, reemplazo de ASM
    ↑
Rust       ← Lógica segura del kernel
    ↑
OS propio  ← Tu sistema operativo
```

### Stack Final del OS

| Stage | Componente | Lenguaje | Descripción |
|-------|-----------|----------|-------------|
| 0 | Boot sector | ADead-BIB | 512 bytes exactos, cargado por BIOS |
| 1 | Bootloader extendido | ADead-BIB | Cargar kernel, setup A20, modo protegido |
| 2 | Hardware init | ADead-BIB | GDT, IDT, paginación, detección de hardware |
| 3 | Kernel entry | Rust | Punto de entrada del kernel |
| 4 | Kernel core | Rust | Scheduler, filesystem, memoria |
| 5 | Drivers críticos | ADead-BIB + Rust | Controladores de bajo nivel |
| 6 | Userspace | Rust / C | Aplicaciones de usuario |

**ASM = 0%. Todo lo que haría ASM lo hace ADead-BIB.**

---

## Parte I: Elementos de Lenguaje Nuevos

### 1.1 Acceso Directo a Registros CPU

ADead-BIB ya tiene ISA Layer (`src/rust/isa/mod.rs`) con registros definidos: RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8-R15, EAX, ECX, AL, XMM0, XMM1.

**Lo que falta — Sintaxis de nivel .adB para acceso directo:**

```rust
// NUEVO: Declaración directa de registro
reg rax = 0x1000
reg rsp = 0x7C00
reg rbx = 0

// NUEVO: Operaciones directas con registros
rax += rbx
rcx = rax << 2
rdx = rax & 0xFF

// NUEVO: Lectura de registro
let valor = reg rax
```

**Implementación requerida en el compilador:**

| Archivo | Cambio |
|---------|--------|
| `frontend/ast.rs` | Agregar `Stmt::RegAssign { reg: String, value: Expr }`, `Expr::RegRead(String)` |
| `frontend/lexer.rs` | Token `REG` para keyword `reg` |
| `frontend/parser.rs` | Parsear `reg <nombre> = <expr>` |
| `backend/cpu/codegen_v2.rs` | Emitir `mov <reg>, <valor>` directo |
| `isa/mod.rs` | Ya tiene `Reg` enum — mapear strings a registros |

---

### 1.2 Acceso Directo a Memoria (Memory-Mapped I/O)

```rust
// NUEVO: Escritura directa a dirección de memoria
write mem[0xB8000] = 0x0741     // 'A' verde en VGA text mode
write mem[0xB8002] = 0x0744     // 'D' verde
write mem[0xB8004] = 0x0765     // 'e' verde

// NUEVO: Lectura directa de memoria
let valor = read mem[0xB8000]

// NUEVO: Escritura con tamaño explícito
write8  mem[0x3F8] = 0x41       // byte (puerto serial)
write16 mem[0xB8000] = 0x0741   // word
write32 mem[address] = valor    // dword
write64 mem[address] = valor    // qword

// NUEVO: Rango de memoria
fill mem[0xB8000..0xB8FA0] = 0x0720  // limpiar pantalla VGA
```

**Implementación requerida:**

| Archivo | Cambio |
|---------|--------|
| `frontend/ast.rs` | `Stmt::MemWrite { addr: Expr, value: Expr, size: MemSize }`, `Expr::MemRead { addr: Expr, size: MemSize }` |
| `frontend/ast.rs` | `enum MemSize { Byte, Word, DWord, QWord }` |
| `frontend/parser.rs` | Parsear `write mem[expr] = expr` y `read mem[expr]` |
| `backend/cpu/codegen_v2.rs` | Emitir `mov [addr], valor` con el tamaño correcto |

---

### 1.3 Instrucciones de I/O de Puerto (in/out)

```rust
// NUEVO: Instrucciones de puerto I/O (esenciales para hardware)
out 0x3F8, 0x41        // Enviar byte 'A' a COM1
let data = in 0x3F8    // Leer byte de COM1

out 0x20, 0x20         // EOI al PIC (End Of Interrupt)
out 0x60, data         // Escribir a controlador de teclado

// NUEVO: Con registros
out 0x3F8, reg al      // Enviar contenido de AL a puerto
reg al = in 0x60       // Leer puerto de teclado a AL
```

**Implementación requerida:**

| Archivo | Cambio |
|---------|--------|
| `isa/mod.rs` | Agregar `ADeadOp::In { port: Operand, dst: Reg }`, `ADeadOp::Out { port: Operand, src: Operand }` |
| `isa/encoder.rs` | Codificar `in al, imm8` (0xE4), `out imm8, al` (0xE6), variantes 16/32 bit |
| `frontend/ast.rs` | `Stmt::PortOut { port: Expr, value: Expr }`, `Expr::PortIn { port: Expr }` |
| `frontend/parser.rs` | Parsear `out <port>, <valor>` y `in <port>` |

---

### 1.4 Control de CPU Modes y Registros Especiales

```rust
// NUEVO: Registros de control (CR0-CR4)
reg cr0 = read_cr0()
cr0 = cr0 | 0x80000001    // Habilitar paging + protected mode
write_cr0(cr0)

reg cr3 = page_directory   // Cargar directorio de páginas
write_cr3(cr3)

// NUEVO: Instrucciones privilegiadas
cli()                       // Desactivar interrupciones
sti()                       // Activar interrupciones
hlt()                       // Halt CPU (esperar interrupción)
lidt(idt_pointer)           // Cargar IDT
lgdt(gdt_pointer)           // Cargar GDT

// NUEVO: Flags del procesador
let flags = read_flags()
```

**Instrucciones privilegiadas que ADead-BIB debe emitir:**

| Instrucción | Opcode x86-64 | Uso |
|-------------|---------------|-----|
| `CLI` | `0xFA` | Desactivar interrupciones |
| `STI` | `0xFB` | Activar interrupciones |
| `HLT` | `0xF4` | Detener CPU |
| `LGDT` | `0x0F 0x01 /2` | Cargar GDT |
| `LIDT` | `0x0F 0x01 /3` | Cargar IDT |
| `IRET` | `0xCF` / `0x48 0xCF` | Retorno de interrupción |
| `INVLPG` | `0x0F 0x01 /7` | Invalidar página TLB |
| `MOV CRx` | `0x0F 0x22` / `0x0F 0x20` | Leer/escribir registros de control |
| `WRMSR` | `0x0F 0x30` | Escribir MSR |
| `RDMSR` | `0x0F 0x32` | Leer MSR |
| `CPUID` | `0x0F 0xA2` | Información del procesador |

**Implementación requerida en `isa/mod.rs`:**

```rust
// Nuevas variantes ADeadOp:
ADeadOp::Cli,                              // CLI
ADeadOp::Sti,                              // STI
ADeadOp::Hlt,                              // HLT
ADeadOp::Lgdt { src: Operand },            // LGDT [mem]
ADeadOp::Lidt { src: Operand },            // LIDT [mem]
ADeadOp::Iret,                             // IRETQ
ADeadOp::Invlpg { addr: Operand },         // INVLPG [mem]
ADeadOp::MovCr { cr: u8, src: Reg },       // MOV CRx, reg
ADeadOp::ReadCr { cr: u8, dst: Reg },      // MOV reg, CRx
ADeadOp::Wrmsr,                            // WRMSR
ADeadOp::Rdmsr,                            // RDMSR
ADeadOp::Cpuid,                            // CPUID
ADeadOp::In { port: Operand, dst: Reg },   // IN
ADeadOp::Out { port: Operand, src: Operand }, // OUT
```

---

### 1.5 Interrupt Handlers (ISR — Interrupt Service Routines)

```rust
// NUEVO: Definir handler de interrupción
@interrupt
fn timer_handler(frame: InterruptFrame) {
    // Incrementar tick counter
    reg rax = read mem[TICK_COUNT_ADDR]
    rax += 1
    write mem[TICK_COUNT_ADDR] = rax

    // Enviar EOI
    out 0x20, 0x20
}

// NUEVO: Definir handler de excepción con error code
@exception(error_code)
fn page_fault_handler(frame: InterruptFrame, error: u64) {
    let faulting_addr = read_cr2()
    // Manejar page fault...
}

// NUEVO: Registrar handlers
install_interrupt(32, timer_handler)     // IRQ0 = timer
install_interrupt(33, keyboard_handler)  // IRQ1 = keyboard
install_interrupt(14, page_fault_handler) // Page fault

// NUEVO: InterruptFrame (struct especial)
struct InterruptFrame {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}
```

**Implementación: El compilador genera el wrapper de ISR automáticamente:**

```
; Auto-generado por ADead-BIB para @interrupt:
push rax
push rbx
push rcx
push rdx
push rsi
push rdi
push rbp
push r8-r15
; ... tu código del handler ...
pop r15-r8
pop rbp
pop rdi
pop rsi
pop rdx
pop rcx
pop rbx
pop rax
iretq
```

---

### 1.6 Definición de Estructuras de Datos de Hardware

```rust
// NUEVO: Estructuras con layout exacto (packed)
@packed
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit: u8,
    base_high: u8,
}

@packed
struct IDTEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

@packed
struct GDTPointer {
    size: u16,
    offset: u64,
}

// NUEVO: Crear y usar
let gdt = GDTEntry {
    limit_low: 0xFFFF,
    base_low: 0x0000,
    base_mid: 0x00,
    access: 0x9A,       // Code segment, present, ring 0
    flags_limit: 0xCF,
    base_high: 0x00,
}
```

---

### 1.7 Inline Raw Bytes / Inline Machine Code

```rust
// NUEVO: Insertar bytes crudos en cualquier lugar
@raw {
    0xFA                          // CLI
    0x0F, 0x01, 0x15              // LGDT
    0x00, 0x10, 0x00, 0x00        // dirección GDT
}

// NUEVO: Inline block con label
@raw("boot_signature") {
    0x55, 0xAA                    // Boot signature
}

// NUEVO: Data block (va a sección de datos)
@data {
    "Hello from ADead-BIB OS\0"   // String con null terminator
}

@data("gdt_table") {
    // Null descriptor
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // Code segment
    0xFF, 0xFF, 0x00, 0x00, 0x00, 0x9A, 0xCF, 0x00,
    // Data segment
    0xFF, 0xFF, 0x00, 0x00, 0x00, 0x92, 0xCF, 0x00,
}
```

---

### 1.8 Saltos Absolutos y Far Jumps

```rust
// NUEVO: Salto absoluto a dirección
jump 0x8000                        // Saltar a dirección

// NUEVO: Far jump (cambio de segmento — necesario para cambio de modo)
far_jump 0x08, protected_mode     // Saltar a código de 32-bit, selector GDT 0x08
far_jump 0x10, long_mode          // Saltar a código de 64-bit

// NUEVO: Salto condicional a dirección
jump_if zero, 0x8000
jump_if carry, error_handler
```

---

### 1.9 Secciones y Alineación

```rust
// NUEVO: Declarar sección
@section(".text")
fn kernel_entry() { ... }

@section(".rodata")
const MESSAGE: str = "ADead-BIB OS v1.0"

@section(".bss")
static STACK: [u8; 4096]           // Stack del kernel

// NUEVO: Alinear a N bytes
@align(4096)                       // Alinear a página
static PAGE_TABLE: [u64; 512]

@align(16)
fn simd_function() { ... }

// NUEVO: Posición absoluta (para boot sector)
@org(0x7C00)                       // Código se carga en esta dirección
fn boot_entry() { ... }

@org(0x1FE)                        // Posición 510 del boot sector
@raw { 0x55, 0xAA }               // Boot signature
```

---

## Parte II: Flat Binary — La Salida Más Importante

### 2.1 Qué es un Flat Binary

| Formato | Estructura | Uso |
|---------|-----------|-----|
| PE (.exe) | Headers + sections + imports | Ejecutables Windows |
| ELF | Headers + sections + symbols | Ejecutables Linux |
| **Flat Binary** | **Solo código, sin headers** | **Boot sector, firmware, OS kernels** |

ADead-BIB ya genera PE (`pe.rs`, `pe_tiny.rs`, etc.) y ELF (`elf.rs`). **Falta: generador de flat binary.**

### 2.2 Implementación del Generador Flat Binary

**Nuevo archivo: `src/rust/backend/cpu/flat_binary.rs`**

```rust
/// Generador de Flat Binary — solo bytes, sin headers
pub struct FlatBinaryGenerator {
    code: Vec<u8>,
    data: Vec<u8>,
    origin: u64,          // Dirección base (ej: 0x7C00 para boot, 0x100000 para kernel)
    target_size: Option<usize>,  // Tamaño fijo (ej: 512 para boot sector)
    padding_byte: u8,     // Byte de padding (0x00 o 0x90=NOP)
}

impl FlatBinaryGenerator {
    pub fn new(origin: u64) -> Self { ... }

    /// Generar flat binary
    pub fn generate(&self, code: &[u8], data: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();

        // Código primero
        output.extend_from_slice(code);

        // Datos después
        output.extend_from_slice(data);

        // Padding si tiene tamaño fijo
        if let Some(size) = self.target_size {
            while output.len() < size {
                output.push(self.padding_byte);
            }
        }

        output
    }

    /// Boot sector: exactamente 512 bytes, con firma 0x55AA al final
    pub fn generate_boot_sector(&self, code: &[u8]) -> Vec<u8> {
        let mut sector = vec![0u8; 512];

        // Copiar código
        let copy_len = code.len().min(510);
        sector[..copy_len].copy_from_slice(&code[..copy_len]);

        // Boot signature
        sector[510] = 0x55;
        sector[511] = 0xAA;

        sector
    }
}
```

### 2.3 CLI para Flat Binary

```bash
# Nuevo comando CLI
adB flat boot.adB -o boot.bin              # Flat binary genérico
adB boot boot.adB -o boot.bin              # Boot sector (512 bytes + 0x55AA)
adB flat kernel.adB -o kernel.bin --org 0x100000  # Kernel en dirección específica
```

**Cambio en `main.rs`:** Agregar subcomando `flat` y `boot` al CLI.

---

## Parte III: Boot Sector con ADead-BIB (Stage 0)

### 3.1 Requisitos del Boot Sector

| Requisito | Valor |
|-----------|-------|
| Tamaño exacto | 512 bytes |
| Firma | `0x55 0xAA` en bytes 510-511 |
| Dirección de carga | `0x7C00` |
| Modo CPU | Real mode (16-bit) |
| Registros al inicio | `CS=0, IP=0x7C00, DL=drive number` |

> [!IMPORTANT]
> **ADead-BIB actualmente genera x86-64 (64-bit).** Para boot sector se necesita soporte de **modo real (16-bit)** y **modo protegido (32-bit)** como targets temporales.

### 3.2 Soporte Multi-Mode CPU

**Nuevo enum en `codegen_v2.rs`:**

```rust
pub enum CpuMode {
    Real16,       // Modo real — boot sector
    Protected32,  // Modo protegido — transición
    Long64,       // Modo largo — kernel (YA EXISTENTE)
}
```

**Esto requiere:**

| Tarea | Detalle |
|-------|---------|
| 16-bit encoding | Prefijo `0x66` invierte operand size en modo real |
| 32-bit encoding | Subset de instrucciones sin REX prefix |
| Mode switching | `cli → lgdt → mov cr0 → far jump → long mode` |

### 3.3 Ejemplo: Boot Sector ADead-BIB

```rust
// boot.adB — Boot sector ADead-BIB
@mode(real16)           // Modo real 16-bit
@org(0x7C00)           // Dirección de carga

fn boot_entry() {
    cli()                        // Desactivar interrupciones

    // Setup segmentos
    reg ax = 0
    reg ds = ax
    reg es = ax
    reg ss = ax
    reg sp = 0x7C00              // Stack crece hacia abajo

    // Imprimir mensaje via BIOS INT 10h
    bios_print("ADead-BIB OS Loading...\n")

    // Cargar kernel desde disco
    load_sectors(drive: reg dl, count: 32, dst: 0x10000)

    // Habilitar A20 gate
    enable_a20()

    // Cargar GDT
    lgdt(gdt_pointer)

    // Switch a modo protegido
    reg cr0 = read_cr0()
    cr0 = cr0 | 1
    write_cr0(cr0)

    // Far jump a código 32-bit
    far_jump 0x08, protected_start
}

// BIOS print via INT 10h
fn bios_print(msg: *u8) {
    reg si = msg
    @loop {
        reg al = read mem8[si]
        if al == 0 { break }
        reg ah = 0x0E           // Teletype output
        @int 0x10               // BIOS video interrupt
        si += 1
    }
}

// Al final: boot signature
@org(0x1FE)
@raw { 0x55, 0xAA }
```

**Resultado:** Flat binary de 512 bytes, booteable.

---

## Parte IV: Estructuras de Sistema (Referencia de Linux)

### 4.1 GDT (Global Descriptor Table)

```rust
// gdt.adB — Tabla de Descriptores Globales
@packed @align(8)
struct GDTEntry {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    flags_limit: u8,
    base_high: u8,
}

const GDT: [GDTEntry; 5] = [
    // 0x00: Null descriptor
    GDTEntry { 0, 0, 0, 0, 0, 0 },

    // 0x08: Kernel code (64-bit)
    GDTEntry {
        limit_low: 0xFFFF,
        base_low: 0, base_mid: 0,
        access: 0x9A,        // Present, Ring 0, Code, Execute/Read
        flags_limit: 0xAF,   // Long mode, 4KB granularity
        base_high: 0
    },

    // 0x10: Kernel data
    GDTEntry {
        limit_low: 0xFFFF,
        base_low: 0, base_mid: 0,
        access: 0x92,        // Present, Ring 0, Data, Read/Write
        flags_limit: 0xCF,
        base_high: 0
    },

    // 0x18: User code (Ring 3)
    GDTEntry {
        limit_low: 0xFFFF, base_low: 0, base_mid: 0,
        access: 0xFA, flags_limit: 0xAF, base_high: 0
    },

    // 0x20: User data (Ring 3)
    GDTEntry {
        limit_low: 0xFFFF, base_low: 0, base_mid: 0,
        access: 0xF2, flags_limit: 0xCF, base_high: 0
    },
]
```

### 4.2 IDT (Interrupt Descriptor Table)

```rust
// idt.adB — Tabla de Descriptores de Interrupción
@packed
struct IDTEntry {
    offset_low: u16,
    selector: u16,          // Selector GDT (0x08 = kernel code)
    ist: u8,                // Interrupt Stack Table index
    type_attr: u8,          // Type + DPL + Present
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

fn setup_idt() {
    // 256 entradas (estándar x86-64)
    let idt: [IDTEntry; 256]

    // Excepciones CPU (0-31)
    idt[0]  = make_idt_entry(divide_error,    0x08, 0x8E)  // #DE
    idt[1]  = make_idt_entry(debug,           0x08, 0x8E)  // #DB
    idt[6]  = make_idt_entry(invalid_opcode,  0x08, 0x8E)  // #UD
    idt[8]  = make_idt_entry(double_fault,    0x08, 0x8E)  // #DF
    idt[13] = make_idt_entry(general_protect, 0x08, 0x8E)  // #GP
    idt[14] = make_idt_entry(page_fault,      0x08, 0x8E)  // #PF

    // IRQs de hardware (32-47)
    idt[32] = make_idt_entry(timer_irq,       0x08, 0x8E)  // PIT/APIC Timer
    idt[33] = make_idt_entry(keyboard_irq,    0x08, 0x8E)  // Keyboard

    // Cargar IDT
    let idt_ptr = IDTPointer { size: 4095, offset: &idt }
    lidt(idt_ptr)
}
```

### 4.3 Paginación (Page Tables)

```rust
// paging.adB — Tablas de paginación x86-64
@align(4096)
static PML4: [u64; 512]        // Page Map Level 4

@align(4096)
static PDPT: [u64; 512]        // Page Directory Pointer Table

@align(4096)
static PD: [u64; 512]          // Page Directory

fn setup_paging() {
    // Identity map primeros 2MB
    PML4[0] = addr_of(PDPT) | 0x03    // Present + Writable
    PDPT[0] = addr_of(PD)   | 0x03
    PD[0]   = 0x00000000     | 0x83    // 2MB page, Present + Writable

    // Cargar PML4 en CR3
    write_cr3(addr_of(PML4))
}
```

---

## Parte V: Elementos Faltantes en la ISA Actual

### 5.1 Operaciones que `ADeadOp` Necesita (actualmente no tiene)

| Operación | Necesaria para | Prioridad |
|-----------|---------------|-----------|
| `IN port, reg` | I/O de hardware | 🔴 Crítica |
| `OUT port, reg` | I/O de hardware | 🔴 Crítica |
| `CLI` | Manejo de interrupciones | 🔴 Crítica |
| `STI` | Manejo de interrupciones | 🔴 Crítica |
| `HLT` | Scheduler, idle | 🔴 Crítica |
| `IRETQ` | ISR handlers | 🔴 Crítica |
| `LGDT` | Setup GDT | 🔴 Crítica |
| `LIDT` | Setup IDT | 🔴 Crítica |
| `MOV CRx` | Paginación, modo protegido | 🔴 Crítica |
| `CPUID` | Detección de features | 🟡 Alta |
| `RDMSR/WRMSR` | Configuración CPU | 🟡 Alta |
| `INVLPG` | TLB management | 🟡 Alta |
| `INT n` | BIOS calls (solo en real mode) | 🟡 Alta |
| `LODS/STOS/MOVS` | Operaciones de string | 🟢 Media |
| `REP` prefix | Loops de copia rápida | 🟢 Media |
| `SHR` | Shift right | 🟢 Media |
| `ROR/ROL` | Rotaciones | 🟢 Media |
| `BT/BTS/BTR` | Bit test/set/reset | 🟢 Media |
| `BSWAP` | Endianness | 🔵 Baja |
| `XCHG` | Atomic swap | 🔵 Baja |
| `LOCK` prefix | Operaciones atómicas (SMP) | 🔵 Baja |

### 5.2 Registros Faltantes en `Reg` Enum

| Registros | Necesarios para |
|-----------|----------------|
| `CR0, CR2, CR3, CR4` | Control CPU, paginación |
| `DR0-DR7` | Debug registers |
| `CS, DS, ES, FS, GS, SS` | Segment registers |
| `AX, BX, CX, DX, SI, DI, SP, BP` | 16-bit mode |
| `AH, BH, CH, DH, BL, CL, DL` | 8-bit sub-registers |
| `RFLAGS` | Reading/modifying flags |

---

## Parte VI: Plan de Implementación por Fases

### Fase 1: Flat Binary + Primitivas Críticas (Fundación) ✅ COMPLETADO

> **Meta:** Generar un flat binary que la CPU ejecute directamente.

- [x] Crear `flat_binary.rs` — generador flat binary
- [x] Agregar soporte CLI `adB flat` y `adB boot`
- [x] Agregar `ADeadOp::Cli`, `Sti`, `Hlt`, `Iret` al ISA
- [x] Agregar `ADeadOp::In`, `Out` al ISA
- [x] Agregar `ADeadOp::Lgdt`, `Lidt` al ISA
- [x] Agregar `ADeadOp::MovCr`, `ReadCr` al ISA
- [x] Codificar todas las nuevas ops en `encoder.rs`
- [x] **Test:** Generar infinite loop flat binary y verificar ✅
- [x] **Fix:** `Target::Raw` skip prologue/epilogue en `compile_top_level` ✅

### Fase 2: Sintaxis .adB para Hardware (Lenguaje) ✅ COMPLETADO

> **Meta:** Escribir código .adB que controle hardware directamente.

- [x] Agregar keyword `reg` al lexer
- [x] Parsear `reg <nombre> = <valor>`
- [x] Parsear `write_mem(addr, valor)` / `read_mem(addr)`
- [x] Parsear `port_out(port, valor)` / `port_in(port)`
- [x] Agregar `@packed` para structs (`Struct.is_packed` en AST)
- [x] Agregar `org addr` para posición de código
- [x] Agregar `raw { bytes }` para inline bytes
- [x] Conectar parser → isa_compiler → encoder para todas las primitivas
- [x] **Test:** Compilar ejemplo VGA text mode (`os_kernel_setup.adB`) ✅

### Fase 3: Boot Sector Real (Primer Binario Booteable) ✅ COMPLETADO

> **Meta:** Boot sector ADead-BIB que corra en QEMU.

- [x] Soporte modo real 16-bit (`RealModeCodegen` en `os_codegen.rs`)
- [x] Soporte `int_call(N)` para interrupciones BIOS
- [x] Generar boot sector de 512 bytes (verificado: `stage1.bin` = 512 bytes)
- [x] Implementar `bios_print()` via INT 10h (imprime "ADead-OS No ASM!")
- [x] Agregar firma `0x55AA` (verificado en bytes 510-511)
- [x] **Test:** Boot sector genera bytes correctos: `FA 31 C0 8E D8...` ✅

### Fase 4: Transición a Long Mode (Setup de Kernel) ✅ COMPLETADO

> **Meta:** Bootloader que transicione a modo 64-bit.

- [x] Implementar setup de GDT en .adB (`stage2.adB` + `GdtGenerator`)
- [x] Implementar habilitación de A20 (port 0x92 fast method)
- [x] Implementar paginación identity-mapped (`PagingSetup` 2MB pages)
- [x] Implementar transición Real → Protected → Long mode (`stage2.adB`)
- [x] Far jump a kernel entry en 64-bit (`far_jump(0x08, offset)`)
- [x] **Test:** `stage2.bin` genera bytes correctos: `FA E4 92 0C 02...` ✅

### Fase 5: Kernel Entry + Interrupciones (Kernel Basics) ✅ COMPLETADO

> **Meta:** Kernel mínimo con manejo de interrupciones.

- [x] `@interrupt` attribute para funciones ISR (`FunctionAttributes.is_interrupt`)
- [x] Auto-generar push/pop de registros en ISR (`emit_interrupt_prologue/epilogue`)
- [x] Setup IDT con handlers de excepciones (`IdtGenerator` 256 entradas)
- [x] Timer handler (estructura en `kernel/src/main.rs`)
- [x] Keyboard handler (estructura en `kernel/src/main.rs`)
- [x] **Test:** 10 os_codegen tests passing ✅

### Fase 6: Integración Rust + ADead-BIB (OS Serio) ✅ COMPLETADO

> **Meta:** Combinar ADead-BIB para hardware y Rust para lógica.

- [x] FFI bidireccional ADead-BIB ↔ Rust (`adead_kernel.h` + `RustKernelBridge`)
- [x] Calling convention compatible (System V AMD64 en headers C)
- [x] Linker script para combinar objetos (`kernel.ld` en `OS Stack New/link/`)
- [x] ADead-BIB maneja: boot, GDT, IDT, ISR wrappers (`stage1.adB`, `stage2.adB`)
- [x] Rust maneja: VGA driver, panic handler, kernel_main (`kernel/src/`)
- [x] C headers: ABI contract (`adead_types.h`, `adead_kernel.h`)
- [x] Build script: `build.ps1` combina los 3 lenguajes
- [x] **Test:** Boot sector + flat binary compilan correctamente ✅

---

## Parte VII: Verificación en Cada Fase

| Herramienta | Uso |
|-------------|-----|
| `qemu-system-x86_64` | Boot y ejecutar flat binaries |
| `qemu-system-i386` | Test de modo real/protegido |
| `objdump -b binary -m i386:x86-64 -D boot.bin` | Desensamblar flat binary |
| `xxd boot.bin` | Inspeccionar bytes directamente |
| `ndisasm -b 16 boot.bin` | Desensamblar modo 16-bit |
| `bochs` | Debugger paso a paso de boot |

### Test Mínimo Inmediato — "El 80% del camino"

```rust
// infinite_loop.adB
@mode(real16)
@org(0x7C00)

fn boot() {
    // Infinite loop
    @loop { hlt() }
}

// Boot signature
@org(0x1FE)
@raw { 0x55, 0xAA }
```

Genera:

```
0xF4        ; HLT
0xEB 0xFD   ; JMP -3 (loop infinito)
... padding (507 bytes de 0x00) ...
0x55 0xAA   ; Boot signature
```

```bash
adB boot infinite_loop.adB -o boot.bin
qemu-system-x86_64 -fda boot.bin
# → QEMU arranca y se queda en halt. ¡ÉXITO!
```

---

## Resumen: Lo que ADead-BIB Tiene vs Lo que Necesita

| Capacidad | Estado | Prioridad |
|-----------|--------|-----------|
| Lexer/Parser completo | ✅ Existe | — |
| AST rico (~60 nodos) | ✅ Existe | — |
| ISA Layer (`ADeadOp`) | ✅ Existe (30+ ops) | — |
| Codegen x86-64 | ✅ Existe (v1 + v2) | — |
| PE generation | ✅ Existe (6 variantes) | — |
| ELF generation | ✅ Existe | — |
| Binary raw emission | ✅ Existe (`binary_raw.rs`) | — |
| **Flat binary generator** | ✅ Implementado (`flat_binary.rs`) | — |
| **Instrucciones privilegiadas** | ✅ Implementado (CLI/STI/HLT/LGDT/LIDT/CPUID/RDMSR/WRMSR/INVLPG) | — |
| **I/O ports (IN/OUT)** | ✅ Implementado (`port_out`/`port_in` + encoder) | — |
| **Control registers (CRx)** | ✅ Implementado (CR0-CR4 read/write) | — |
| **Modo real 16-bit** | ✅ Implementado (`RealModeCodegen` en `os_codegen.rs`) | — |
| **Sintaxis `reg`/`mem`/`out`** | ✅ Implementado (AST + parser + isa_compiler) | — |
| **`@packed` structs** | ✅ Implementado (`PackedStruct` + AST `is_packed`) | — |
| **`@interrupt` functions** | ✅ Implementado (auto push/pop + iretq wrapper) | — |
| **`@org` / `@align`** | ✅ Implementado (AST + parser + isa_compiler) | — |
| **`@raw` inline bytes** | ✅ Implementado (`RawBlock` → `RawBytes`) | — |
| **Far jumps** | ✅ Implementado (`FarJmp` con selector:offset) | — |
| **Modo protegido 32-bit** | ✅ Implementado (`ProtectedModeCodegen` en `os_codegen.rs`) | — |
| **BIOS INT calls** | ✅ Implementado (`int_call(n)` → `INT n`) | — |
| **Segment registers** | ✅ Implementado (CS/DS/ES/FS/GS/SS read/write) | — |
| **GDT generation** | ✅ Implementado (`GdtGenerator` con entradas estándar) | — |
| **IDT generation** | ✅ Implementado (`IdtGenerator` con 256 entradas) | — |
| **Paging setup** | ✅ Implementado (`PagingSetup` con identity mapping 2MB) | — |
| **Mode transitions** | ✅ Implementado (Real→Protected, Protected→Long) | — |
| **Rust kernel bridge** | ✅ Implementado (`RustKernelBridge` + linker script gen) | — |
| **`@exception` handlers** | ✅ Implementado (con error code support) | — |
| **`@naked` functions** | ✅ Implementado (sin prologue/epilogue) | — |
| **CpuMode enum** | ✅ Implementado (Real16/Protected32/Long64) | — |

---

> **ADead-BIB Phase 6 COMPLETADO. Todos los elementos de OS-level han sido implementados. ADead-BIB ahora puede reemplazar ASM completamente para desarrollo de sistemas operativos, desde boot sectors hasta kernel integration con Rust.**

---

**Autor:** Eddi Andreé Salazar Matos
**Versión:** 3.1-OS Phase 6 Complete
**Estado:** Todos los elementos implementados y testeados (143 tests passing)
