# ADead-BIB ISA Layer Documentation

**Versión:** v3.2  
**Arquitectura:** x86-64

---

## Visión General

La ISA Layer de ADead-BIB proporciona una abstracción tipada sobre instrucciones x86-64.
En lugar de emitir bytes directamente, construimos una IR tipada que luego se codifica.

```
AST → IsaCompiler → Vec<ADeadOp> → Encoder → Vec<u8>
```

**Ventajas:**
- Validación de instrucciones en tiempo de compilación
- Optimizaciones sobre la IR antes de emitir bytes
- Multi-target sin reescribir codegen completo
- Debugging legible (print de instrucciones, no hex)

---

## Registros (Reg enum)

### Registros de 64-bit

| Registro | Código | Uso Principal | Caller/Callee Saved |
|----------|--------|---------------|---------------------|
| RAX | 0 | Acumulador, valor de retorno | Caller |
| RCX | 1 | Arg 1 (Windows), contador | Caller |
| RDX | 2 | Arg 2 (Windows), I/O | Caller |
| RBX | 3 | Base, general | Callee |
| RSP | 4 | Stack pointer | - |
| RBP | 5 | Frame pointer | Callee |
| RSI | 6 | Arg 2 (Linux), source | Caller |
| RDI | 7 | Arg 1 (Linux), destination | Caller |
| R8 | 8 | Arg 3 (Windows), Arg 5 (Linux) | Caller |
| R9 | 9 | Arg 4 (Windows), Arg 6 (Linux) | Caller |
| R10 | 10 | Scratch | Caller |
| R11 | 11 | Scratch | Caller |
| R12 | 12 | General | Callee |
| R13 | 13 | General | Callee |
| R14 | 14 | General | Callee |
| R15 | 15 | General | Callee |

### Registros de 32-bit

| Registro | Equivalente 64-bit |
|----------|-------------------|
| EAX | RAX (bits 0-31) |
| ECX | RCX (bits 0-31) |
| EDX | RDX (bits 0-31) |
| EBX | RBX (bits 0-31) |
| ESP | RSP (bits 0-31) |
| EBP | RBP (bits 0-31) |
| ESI | RSI (bits 0-31) |
| EDI | RDI (bits 0-31) |
| R8D-R15D | R8-R15 (bits 0-31) |

### Registros de 8-bit

| Registro | Equivalente 64-bit |
|----------|-------------------|
| AL | RAX (bits 0-7) |
| CL | RCX (bits 0-7) |
| DL | RDX (bits 0-7) |
| BL | RBX (bits 0-7) |
| AH | RAX (bits 8-15) |
| CH | RCX (bits 8-15) |
| DH | RDX (bits 8-15) |
| BH | RBX (bits 8-15) |

### Registros SSE (128-bit)

| Registro | Uso |
|----------|-----|
| XMM0-XMM7 | Argumentos flotantes, retorno |
| XMM8-XMM15 | Scratch |

---

## Operandos (Operand enum)

### Variantes

```rust
pub enum Operand {
    Reg(Reg),                    // Registro
    Imm8(i8),                    // Inmediato 8-bit
    Imm16(i16),                  // Inmediato 16-bit
    Imm32(i32),                  // Inmediato 32-bit
    Imm64(u64),                  // Inmediato 64-bit
    Mem { base: Reg, disp: i32 }, // Memoria [base + disp]
    MemSIB {                     // Memoria con SIB
        base: Reg,
        index: Reg,
        scale: u8,               // 1, 2, 4, u 8
        disp: i32
    },
    RipRel(i32),                 // RIP-relative [RIP + disp]
}
```

### Ejemplos

| Operando | Sintaxis Assembly | Uso |
|----------|-------------------|-----|
| `Reg(RAX)` | `rax` | Registro directo |
| `Imm8(42)` | `42` | Constante pequeña |
| `Imm32(100000)` | `100000` | Constante 32-bit |
| `Imm64(0x123456789)` | `0x123456789` | Constante 64-bit |
| `Mem { base: RBP, disp: -8 }` | `[rbp-8]` | Variable local |
| `MemSIB { base: RAX, index: RBX, scale: 4, disp: 0 }` | `[rax+rbx*4]` | Array access |
| `RipRel(0x100)` | `[rip+0x100]` | Dato global |

---

## Instrucciones (ADeadOp enum)

### Movimiento de Datos

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Mov { dst, src }` | Mover datos | `89/8B/B8+r` |
| `Movzx { dst, src }` | Mover con zero-extend | `0F B6/B7` |
| `Movsx { dst, src }` | Mover con sign-extend | `0F BE/BF` |
| `Lea { dst, src }` | Load effective address | `8D` |
| `Push { src }` | Push a stack | `50+r/FF /6` |
| `Pop { dst }` | Pop de stack | `58+r/8F /0` |
| `Xchg { a, b }` | Intercambiar | `87` |

### Aritmética

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Add { dst, src }` | Suma | `01/03/81 /0` |
| `Sub { dst, src }` | Resta | `29/2B/81 /5` |
| `Mul { src }` | Multiplicación sin signo | `F7 /4` |
| `Imul { dst, src }` | Multiplicación con signo | `0F AF` |
| `Div { src }` | División sin signo | `F7 /6` |
| `Idiv { src }` | División con signo | `F7 /7` |
| `Neg { dst }` | Negación | `F7 /3` |
| `Inc { dst }` | Incrementar | `FF /0` |
| `Dec { dst }` | Decrementar | `FF /1` |

### Lógica y Bitwise

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `And { dst, src }` | AND bitwise | `21/23/81 /4` |
| `Or { dst, src }` | OR bitwise | `09/0B/81 /1` |
| `Xor { dst, src }` | XOR bitwise | `31/33/81 /6` |
| `Not { dst }` | NOT bitwise | `F7 /2` |
| `Shl { dst, count }` | Shift left | `D3 /4` |
| `Shr { dst, count }` | Shift right (unsigned) | `D3 /5` |
| `Sar { dst, count }` | Shift right (signed) | `D3 /7` |
| `ShlCl { dst }` | Shift left por CL | `D3 /4` |
| `ShrCl { dst }` | Shift right por CL | `D3 /5` |

### Comparación y Saltos

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Cmp { left, right }` | Comparar | `39/3B/81 /7` |
| `Test { left, right }` | Test (AND sin guardar) | `85` |
| `Jmp { target }` | Salto incondicional | `EB/E9` |
| `Je { target }` | Jump if equal (ZF=1) | `74/0F 84` |
| `Jne { target }` | Jump if not equal (ZF=0) | `75/0F 85` |
| `Jl { target }` | Jump if less (SF≠OF) | `7C/0F 8C` |
| `Jle { target }` | Jump if less or equal | `7E/0F 8E` |
| `Jg { target }` | Jump if greater | `7F/0F 8F` |
| `Jge { target }` | Jump if greater or equal | `7D/0F 8D` |
| `Jb { target }` | Jump if below (unsigned) | `72/0F 82` |
| `Ja { target }` | Jump if above (unsigned) | `77/0F 87` |

### Llamadas y Retorno

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Call { target }` | Llamar función | `E8` |
| `CallReg { reg }` | Llamar registro | `FF /2` |
| `Ret` | Retornar | `C3` |
| `RetN { bytes }` | Retornar y limpiar stack | `C2` |

### Sistema

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Syscall` | System call (Linux) | `0F 05` |
| `Int { vector }` | Interrupción | `CD` |
| `Nop` | No operation | `90` |
| `Hlt` | Halt CPU | `F4` |
| `Cli` | Clear interrupts | `FA` |
| `Sti` | Set interrupts | `FB` |

### Punto Flotante (SSE)

| Instrucción | Descripción | Encoding |
|-------------|-------------|----------|
| `Movsd { dst, src }` | Mover double | `F2 0F 10/11` |
| `Movss { dst, src }` | Mover float | `F3 0F 10/11` |
| `Addsd { dst, src }` | Sumar double | `F2 0F 58` |
| `Subsd { dst, src }` | Restar double | `F2 0F 5C` |
| `Mulsd { dst, src }` | Multiplicar double | `F2 0F 59` |
| `Divsd { dst, src }` | Dividir double | `F2 0F 5E` |
| `CvtSi2Sd { dst, src }` | Int → Double | `F2 0F 2A` |
| `CvtSd2Si { dst, src }` | Double → Int | `F2 0F 2D` |

---

## Calling Conventions

### Windows x64

| Argumento | Registro (int) | Registro (float) |
|-----------|----------------|------------------|
| 1 | RCX | XMM0 |
| 2 | RDX | XMM1 |
| 3 | R8 | XMM2 |
| 4 | R9 | XMM3 |
| 5+ | Stack | Stack |

**Shadow space:** 32 bytes reservados en stack

### Linux x64 (System V)

| Argumento | Registro (int) | Registro (float) |
|-----------|----------------|------------------|
| 1 | RDI | XMM0 |
| 2 | RSI | XMM1 |
| 3 | RDX | XMM2 |
| 4 | RCX | XMM3 |
| 5 | R8 | XMM4 |
| 6 | R9 | XMM5 |
| 7+ | Stack | Stack |

**Red zone:** 128 bytes debajo de RSP (leaf functions)

---

## Register Allocator

### TempAllocator

Asigna registros temporales para expresiones.

```rust
let mut alloc = TempAllocator::new();

// Allocate
let r1 = alloc.alloc(); // Some(RBX)
let r2 = alloc.alloc(); // Some(RCX)

// Free
alloc.free(r1.unwrap());

// Reallocate
let r3 = alloc.alloc(); // Some(RBX) - reutilizado
```

**Registros disponibles:** RBX, RCX, RDX, RSI, RDI, R8-R12

### StackFrame

Calcula espacio de stack para variables locales.

```rust
let mut frame = StackFrame::new();

let off_x = frame.alloc_local("x".to_string(), 8); // -8
let off_y = frame.alloc_local("y".to_string(), 4); // -12
let off_z = frame.alloc_local("z".to_string(), 1); // -13

let total = frame.total_size(); // 16 (aligned)
```

---

## Encoder

El encoder convierte `ADeadOp` a bytes x86-64.

### Ejemplo

```rust
let ops = vec![
    ADeadOp::Push { src: Operand::Reg(Reg::RBP) },
    ADeadOp::Mov { dst: Operand::Reg(Reg::RBP), src: Operand::Reg(Reg::RSP) },
    ADeadOp::Sub { dst: Operand::Reg(Reg::RSP), src: Operand::Imm32(32) },
    // ... código ...
    ADeadOp::Mov { dst: Operand::Reg(Reg::RSP), src: Operand::Reg(Reg::RBP) },
    ADeadOp::Pop { dst: Operand::Reg(Reg::RBP) },
    ADeadOp::Ret,
];

let bytes = encode_ops(&ops);
// [0x55, 0x48, 0x89, 0xE5, 0x48, 0x83, 0xEC, 0x20, ..., 0x48, 0x89, 0xEC, 0x5D, 0xC3]
```

---

## Portabilidad

### Portar a ARM64

Para portar a ARM64, crear:

1. `src/rust/isa/reg_arm64.rs` - Registros ARM64
2. `src/rust/isa/encoder_arm64.rs` - Encoder ARM64

**Mapeo de registros:**

| x86-64 | ARM64 | Uso |
|--------|-------|-----|
| RAX | X0 | Retorno |
| RCX | X1 | Arg 2 |
| RDX | X2 | Arg 3 |
| RBX | X19 | Callee-saved |
| RSP | SP | Stack |
| RBP | X29 | Frame |
| RDI | X0 | Arg 1 |
| RSI | X1 | Arg 2 |

**Ejemplo de encoding:**

```rust
// x86-64: MOV RAX, 42
// → 48 B8 2A 00 00 00 00 00 00 00

// ARM64: MOV X0, #42
// → D2 80 05 40 (MOVZ X0, #42)
```

---

## Archivos

| Archivo | Descripción |
|---------|-------------|
| `src/rust/isa/mod.rs` | Definiciones de Reg, Operand, ADeadOp |
| `src/rust/isa/encoder.rs` | Encoder x86-64 |
| `src/rust/isa/decoder.rs` | Decoder x86-64 |
| `src/rust/isa/isa_compiler.rs` | AST → ADeadOp |
| `src/rust/isa/optimizer.rs` | Optimizaciones IR |
| `src/rust/isa/reg_alloc.rs` | Register allocator |

---

**Autor:** Eddi Andreé Salazar Matos  
**Licencia:** MIT
