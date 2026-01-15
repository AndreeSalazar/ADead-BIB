// ============================================================================
// cpu/mod.adB — Módulo CPU: Instrucciones x86-64 Directas
// ============================================================================
// Funciones que emiten bytes x86-64 directamente.
// Cada función mapea 1:1 a una instrucción de CPU.
//
// Uso:
//   cpu::mov(rax, 42)     // Emite: 48 C7 C0 2A 00 00 00
//   cpu::xor(rax, rax)    // Emite: 48 31 C0
//   cpu::ret()            // Emite: C3
// ============================================================================

#![exports(
    // Registros
    rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp,
    r8, r9, r10, r11, r12, r13, r14, r15,
    // Instrucciones de movimiento
    mov, lea, push, pop,
    // Instrucciones aritméticas
    add, sub, mul, div, inc, dec, neg,
    // Instrucciones lógicas
    and, or, xor, not, shl, shr,
    // Instrucciones de comparación
    cmp, test,
    // Instrucciones de salto
    jmp, je, jne, jl, jle, jg, jge, jz, jnz,
    // Instrucciones de llamada
    call, ret,
    // Instrucciones especiales
    nop, syscall, int3
)]

// ============================================================================
// REGISTROS x86-64 (constantes tipadas)
// ============================================================================

pub const rax: Reg = Reg { id: 0, name: "rax" }
pub const rcx: Reg = Reg { id: 1, name: "rcx" }
pub const rdx: Reg = Reg { id: 2, name: "rdx" }
pub const rbx: Reg = Reg { id: 3, name: "rbx" }
pub const rsp: Reg = Reg { id: 4, name: "rsp" }
pub const rbp: Reg = Reg { id: 5, name: "rbp" }
pub const rsi: Reg = Reg { id: 6, name: "rsi" }
pub const rdi: Reg = Reg { id: 7, name: "rdi" }
pub const r8:  Reg = Reg { id: 8, name: "r8" }
pub const r9:  Reg = Reg { id: 9, name: "r9" }
pub const r10: Reg = Reg { id: 10, name: "r10" }
pub const r11: Reg = Reg { id: 11, name: "r11" }
pub const r12: Reg = Reg { id: 12, name: "r12" }
pub const r13: Reg = Reg { id: 13, name: "r13" }
pub const r14: Reg = Reg { id: 14, name: "r14" }
pub const r15: Reg = Reg { id: 15, name: "r15" }

struct Reg {
    id: u8,
    name: string
}

// ============================================================================
// INSTRUCCIONES DE MOVIMIENTO
// ============================================================================

// mov reg, imm64 — Mover valor inmediato a registro
// Emite: 48 B8+rd [imm64]
pub fn mov(dest: Reg, value: i64) {
    emit![0x48, 0xB8 + dest.id]
    emit_i64(value)
}

// mov reg, reg — Mover registro a registro
// Emite: 48 89 /r
pub fn mov_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x89, 0xC0 + (src.id << 3) + dest.id]
}

// lea reg, [mem] — Load Effective Address
// Emite: 48 8D /r
pub fn lea(dest: Reg, base: Reg, offset: i32) {
    emit![0x48, 0x8D, 0x80 + (dest.id << 3) + base.id]
    emit_i32(offset)
}

// push reg — Guardar registro en stack
// Emite: 50+rd (o 41 50+rd para r8-r15)
pub fn push(reg: Reg) {
    if reg.id >= 8 {
        emit![0x41, 0x50 + (reg.id - 8)]
    } else {
        emit![0x50 + reg.id]
    }
}

// pop reg — Restaurar registro desde stack
// Emite: 58+rd (o 41 58+rd para r8-r15)
pub fn pop(reg: Reg) {
    if reg.id >= 8 {
        emit![0x41, 0x58 + (reg.id - 8)]
    } else {
        emit![0x58 + reg.id]
    }
}

// ============================================================================
// INSTRUCCIONES ARITMÉTICAS
// ============================================================================

// add reg, imm32 — Sumar inmediato a registro
// Emite: 48 81 C0+rd [imm32]
pub fn add(dest: Reg, value: i32) {
    emit![0x48, 0x81, 0xC0 + dest.id]
    emit_i32(value)
}

// add reg, reg — Sumar registro a registro
// Emite: 48 01 /r
pub fn add_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x01, 0xC0 + (src.id << 3) + dest.id]
}

// sub reg, imm32 — Restar inmediato de registro
// Emite: 48 81 E8+rd [imm32]
pub fn sub(dest: Reg, value: i32) {
    emit![0x48, 0x81, 0xE8 + dest.id]
    emit_i32(value)
}

// sub reg, reg — Restar registro de registro
// Emite: 48 29 /r
pub fn sub_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x29, 0xC0 + (src.id << 3) + dest.id]
}

// mul reg — Multiplicar RAX por registro (resultado en RDX:RAX)
// Emite: 48 F7 E0+rd
pub fn mul(src: Reg) {
    emit![0x48, 0xF7, 0xE0 + src.id]
}

// div reg — Dividir RDX:RAX por registro (cociente en RAX, resto en RDX)
// Emite: 48 F7 F0+rd
pub fn div(src: Reg) {
    emit![0x48, 0xF7, 0xF0 + src.id]
}

// inc reg — Incrementar registro
// Emite: 48 FF C0+rd
pub fn inc(reg: Reg) {
    emit![0x48, 0xFF, 0xC0 + reg.id]
}

// dec reg — Decrementar registro
// Emite: 48 FF C8+rd
pub fn dec(reg: Reg) {
    emit![0x48, 0xFF, 0xC8 + reg.id]
}

// neg reg — Negar registro (complemento a dos)
// Emite: 48 F7 D8+rd
pub fn neg(reg: Reg) {
    emit![0x48, 0xF7, 0xD8 + reg.id]
}

// ============================================================================
// INSTRUCCIONES LÓGICAS
// ============================================================================

// and reg, imm32 — AND lógico con inmediato
// Emite: 48 81 E0+rd [imm32]
pub fn and(dest: Reg, value: i32) {
    emit![0x48, 0x81, 0xE0 + dest.id]
    emit_i32(value)
}

// and reg, reg — AND lógico entre registros
// Emite: 48 21 /r
pub fn and_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x21, 0xC0 + (src.id << 3) + dest.id]
}

// or reg, imm32 — OR lógico con inmediato
// Emite: 48 81 C8+rd [imm32]
pub fn or(dest: Reg, value: i32) {
    emit![0x48, 0x81, 0xC8 + dest.id]
    emit_i32(value)
}

// or reg, reg — OR lógico entre registros
// Emite: 48 09 /r
pub fn or_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x09, 0xC0 + (src.id << 3) + dest.id]
}

// xor reg, reg — XOR lógico entre registros (útil para limpiar registro)
// Emite: 48 31 /r
pub fn xor(dest: Reg, src: Reg) {
    emit![0x48, 0x31, 0xC0 + (src.id << 3) + dest.id]
}

// not reg — NOT lógico (invertir bits)
// Emite: 48 F7 D0+rd
pub fn not(reg: Reg) {
    emit![0x48, 0xF7, 0xD0 + reg.id]
}

// shl reg, imm8 — Shift left
// Emite: 48 C1 E0+rd [imm8]
pub fn shl(reg: Reg, count: u8) {
    emit![0x48, 0xC1, 0xE0 + reg.id, count]
}

// shr reg, imm8 — Shift right
// Emite: 48 C1 E8+rd [imm8]
pub fn shr(reg: Reg, count: u8) {
    emit![0x48, 0xC1, 0xE8 + reg.id, count]
}

// ============================================================================
// INSTRUCCIONES DE COMPARACIÓN
// ============================================================================

// cmp reg, imm32 — Comparar registro con inmediato
// Emite: 48 81 F8+rd [imm32]
pub fn cmp(reg: Reg, value: i32) {
    emit![0x48, 0x81, 0xF8 + reg.id]
    emit_i32(value)
}

// cmp reg, reg — Comparar registros
// Emite: 48 39 /r
pub fn cmp_reg(dest: Reg, src: Reg) {
    emit![0x48, 0x39, 0xC0 + (src.id << 3) + dest.id]
}

// test reg, reg — Test lógico (AND sin guardar resultado)
// Emite: 48 85 /r
pub fn test(dest: Reg, src: Reg) {
    emit![0x48, 0x85, 0xC0 + (src.id << 3) + dest.id]
}

// ============================================================================
// INSTRUCCIONES DE SALTO
// ============================================================================

// jmp rel32 — Salto incondicional
// Emite: E9 [rel32]
pub fn jmp(offset: i32) {
    emit![0xE9]
    emit_i32(offset)
}

// je rel32 — Saltar si igual (ZF=1)
// Emite: 0F 84 [rel32]
pub fn je(offset: i32) {
    emit![0x0F, 0x84]
    emit_i32(offset)
}

// jne rel32 — Saltar si no igual (ZF=0)
// Emite: 0F 85 [rel32]
pub fn jne(offset: i32) {
    emit![0x0F, 0x85]
    emit_i32(offset)
}

// jl rel32 — Saltar si menor (SF≠OF)
// Emite: 0F 8C [rel32]
pub fn jl(offset: i32) {
    emit![0x0F, 0x8C]
    emit_i32(offset)
}

// jle rel32 — Saltar si menor o igual (ZF=1 o SF≠OF)
// Emite: 0F 8E [rel32]
pub fn jle(offset: i32) {
    emit![0x0F, 0x8E]
    emit_i32(offset)
}

// jg rel32 — Saltar si mayor (ZF=0 y SF=OF)
// Emite: 0F 8F [rel32]
pub fn jg(offset: i32) {
    emit![0x0F, 0x8F]
    emit_i32(offset)
}

// jge rel32 — Saltar si mayor o igual (SF=OF)
// Emite: 0F 8D [rel32]
pub fn jge(offset: i32) {
    emit![0x0F, 0x8D]
    emit_i32(offset)
}

// jz rel32 — Saltar si cero (alias de je)
// Emite: 0F 84 [rel32]
pub fn jz(offset: i32) {
    je(offset)
}

// jnz rel32 — Saltar si no cero (alias de jne)
// Emite: 0F 85 [rel32]
pub fn jnz(offset: i32) {
    jne(offset)
}

// ============================================================================
// INSTRUCCIONES DE LLAMADA
// ============================================================================

// call rel32 — Llamar función
// Emite: E8 [rel32]
pub fn call(offset: i32) {
    emit![0xE8]
    emit_i32(offset)
}

// ret — Retornar de función
// Emite: C3
pub fn ret() {
    emit![0xC3]
}

// ============================================================================
// INSTRUCCIONES ESPECIALES
// ============================================================================

// nop — No operation
// Emite: 90
pub fn nop() {
    emit![0x90]
}

// syscall — Llamada al sistema (Linux)
// Emite: 0F 05
pub fn syscall() {
    emit![0x0F, 0x05]
}

// int3 — Breakpoint
// Emite: CC
pub fn int3() {
    emit![0xCC]
}

// ============================================================================
// HELPERS INTERNOS
// ============================================================================

fn emit_i32(value: i32) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8
    ]
}

fn emit_i64(value: i64) {
    emit![
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
        ((value >> 32) & 0xFF) as u8,
        ((value >> 40) & 0xFF) as u8,
        ((value >> 48) & 0xFF) as u8,
        ((value >> 56) & 0xFF) as u8
    ]
}
