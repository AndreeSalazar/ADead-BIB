# ADead-BIB — Análisis de Madurez del Compilador

> **Objetivo:** Diagnóstico honesto del estado actual de `src/rust/`, identificando los problemas reales que impiden que ADead-BIB madure como compilador serio, y el plan concreto para resolverlos en orden de prioridad.
>
> **Foco:** Machine Code generation + Sistema de Tipos estilo C.

---

## Resumen Ejecutivo

ADead-BIB tiene una base sólida: un pipeline `AST → ISA (ADeadOp) → Encoder → bytes` que funciona, genera PEs/ELFs válidos, y ya bootea en QEMU. **Pero tiene 5 problemas estructurales que le impiden madurar:**

| # | Problema | Severidad | Impacto |
|---|---------|-----------|---------|
| 1 | **Dos sistemas de tipos desconectados** | 🔴 Crítico | El compilador no sabe qué tamaño tiene un dato |
| 2 | **Todo se compila como 64-bit ciego** | 🔴 Crítico | Machine code incorrecto para tipos < 64-bit |
| 3 | **Tres codegen duplicados** | 🟡 Alto | Código muerto, confusión, bugs divergentes |
| 4 | **Sin register allocator** | 🟡 Alto | Todo pasa por RAX → código lento e inflado |
| 5 | **Encoder con huecos silenciosos** | 🟠 Medio | Combinaciones de registros emiten NOP en vez de error |

---

## Problema 1: Dos Sistemas de Tipos Desconectados

### Diagnóstico

Existen **dos enums `Type` completamente separados** que no se hablan entre sí:

**`frontend/ast.rs` (línea 6):**
```rust
pub enum Type {
    Int, Long, Short, Char, Float, Double, Bool, Void,
    Pointer(Box<Type>), Reference(Box<Type>),
    Array(Box<Type>, Option<usize>),
    Named(String), Auto,
}
```

**`frontend/types.rs` (línea 1):**
```rust
pub enum Type {
    Int,        // 64-bit signed integer
    Float,      // 64-bit float (double)
    Bool, String, Void,
    Array(Box<Type>), FixedArray(Box<Type>, usize),
    Vec4, Vec8, Vec16,
    Class(String),
    Unknown,
}
```

### Problemas Concretos

1. **`ast::Type` tiene `Pointer`, `Short`, `Long`, `Char`, `Double`** → pero `types::Type` no.
2. **`types::Type` tiene `Vec4/Vec8/Vec16`, `Unknown`** → pero `ast::Type` no.
3. **`type_checker.rs` usa `types::Type`** pero recibe nodos del AST que usan `ast::Type` → la información de tipos se pierde en la frontera.
4. **`Param` tiene redundancia:**
   ```rust
   pub struct Param {
       pub type_name: Option<String>,    // "int" como texto
       pub param_type: Option<Type>,     // ast::Type parseado
       pub is_pointer: bool,             // ¿duplica Pointer()?
       pub is_reference: bool,           // ¿duplica Reference()?
   }
   ```
   Hay 4 formas de decir "este parámetro es un puntero a int". Ninguna llega al codegen.

5. **`StructField` solo tiene `type_name: Option<String>`** → el compilador no sabe el tamaño de los campos de un struct.

6. **Ningún tipo llega al codegen.** El `isa_compiler.rs` trata TODO como `i64` (8 bytes, registros de 64-bit). Si declaras `char x = 'A'`, genera `mov rax, 0x41` (10 bytes de machine code) en vez de `mov al, 0x41` (2 bytes).

### Por Qué Importa

Sin tipos reales en el codegen, ADead-BIB no puede:
- Generar `mov al, valor` para `char` (8-bit) → siempre genera `mov rax, valor` (64-bit)
- Calcular `sizeof(struct)` correctamente
- Hacer arithmetic de punteros: `ptr + 1` no sabe si sumar 1, 4, u 8 bytes
- Generar SPIR-V correcto (GPU necesita tipos explícitos)
- Implementar arrays con stride correcto

### Solución Propuesta

**Unificar en un solo `Type` con tamaños explícitos estilo C:**

```rust
// Un solo Type para todo el compilador
pub enum Type {
    // Enteros con tamaño explícito (como C)
    I8,             // char / int8_t
    I16,            // short / int16_t
    I32,            // int / int32_t
    I64,            // long long / int64_t
    U8,             // unsigned char / uint8_t
    U16,            // unsigned short / uint16_t
    U32,            // unsigned int / uint32_t
    U64,            // unsigned long long / uint64_t

    // Flotantes
    F32,            // float
    F64,            // double

    // Otros primitivos
    Bool,           // bool (1 byte)
    Void,           // void (0 bytes)

    // Compuestos
    Pointer(Box<Type>),                  // T*
    Array(Box<Type>, Option<usize>),     // T[N] o T[]
    Struct(String),                      // struct Name
    Function(Vec<Type>, Box<Type>),      // (args) -> ret

    // SIMD
    Vec4,           // 4×f32 (128-bit SSE)
    Vec8,           // 8×f32 (256-bit AVX)

    // Inferencia
    Auto,           // el compilador deduce
}

impl Type {
    /// Tamaño en bytes — ESENCIAL para codegen correcto
    pub fn size_bytes(&self) -> usize {
        match self {
            Type::I8 | Type::U8 | Type::Bool => 1,
            Type::I16 | Type::U16 => 2,
            Type::I32 | Type::U32 | Type::F32 => 4,
            Type::I64 | Type::U64 | Type::F64 | Type::Pointer(_) => 8,
            Type::Vec4 => 16,
            Type::Vec8 => 32,
            Type::Void => 0,
            Type::Array(t, Some(n)) => t.size_bytes() * n,
            Type::Array(_, None) => 8, // puntero
            Type::Struct(_) => 8, // lookup en tabla de structs
            Type::Function(_, _) => 8, // puntero a función
            Type::Auto => 8, // default
        }
    }

    /// ¿Qué tamaño de registro usar? 
    pub fn reg_size(&self) -> RegSize {
        match self.size_bytes() {
            1 => RegSize::Byte,    // AL, BL, CL...
            2 => RegSize::Word,    // AX, BX, CX...
            4 => RegSize::DWord,   // EAX, EBX, ECX...
            _ => RegSize::QWord,   // RAX, RBX, RCX...
        }
    }

    /// Mapeo C → ADead
    pub fn from_c_name(name: &str) -> Self {
        match name {
            "char" => Type::I8,
            "short" => Type::I16,
            "int" => Type::I32,
            "long" => Type::I64,
            "float" => Type::F32,
            "double" => Type::F64,
            "void" => Type::Void,
            "bool" => Type::Bool,
            _ => Type::Struct(name.to_string()),
        }
    }
}
```

**Impacto en el pipeline:**

```
Antes:  Código → AST (tipos como strings) → ISA (todo 64-bit) → bytes (ciego)
Ahora:  Código → AST (Type real) → ISA (tipo-aware) → bytes (tamaño correcto)
```

---

## Problema 2: Machine Code — Todo Ciego a 64-bit

### Diagnóstico

En `isa_compiler.rs`, la función `emit_expression()` (línea ~1100) SIEMPRE emite resultados en `RAX` (64-bit), sin importar el tipo:

```rust
// Línea ~1130 - Un Number siempre va a RAX con mov imm64
Expr::Number(n) => {
    self.ir.emit(ADeadOp::Mov {
        dst: Operand::Reg(Reg::RAX),     // SIEMPRE 64-bit
        src: Operand::Imm64(*n as u64),  // SIEMPRE 10 bytes
    });
}
```

**Resultado real:** `let x: char = 65` genera:
```
48 B8 41 00 00 00 00 00 00 00    ; mov rax, 65  (10 bytes)
```

**Debería generar:** 
```
B0 41                             ; mov al, 65   (2 bytes)
```

### Otros Problemas de Machine Code

| Ubicación | Problema | Consecuencia |
|-----------|----------|--------------|
| `isa_compiler.rs:1310-1311` | `LeftShift`/`RightShift` siempre usa `amount: 1` hardcoded | `x << 4` genera `x << 1` — **BUG** |
| `isa_compiler.rs:1317` | `BitwiseNot` usa `RawBytes(vec![0x48, 0xF7, 0xD0])` | Debería ser `ADeadOp` propio, no bytes crudos |
| `isa_compiler.rs:1353-1355` | `_ => xor rax, rax` como fallback general | Expresiones no soportadas producen 0 silenciosamente |
| `isa_compiler.rs:1362` | Calling convention hardcoded Windows (RCX,RDX,R8,R9) | En Linux debería ser RDI,RSI,RDX,RCX |
| `encoder.rs:124-127` | `CvtSi2Sd` ignora `dst`/`src`, hardcodea xmm0/rax | Si usas otro registro, genera código incorrecto |
| `encoder.rs:222` | `Mov reg64, imm64` con registros no comunes → `NOP` | `mov r10, 42` produce un NOP silencioso |
| `isa_compiler.rs:405-409` | Stack allocation fija de 128 bytes por función | Funciones con 1 variable desperdician 120 bytes |
| `isa_compiler.rs:781-784` | `ShlAssign`/`ShrAssign` ignoran el valor, usan `amount: 1` | `x <<= 4` genera `x <<= 1` — **BUG** |
| `isa_compiler.rs:725` | `emit_mem_write` usa `RawBytes` para `mov [rbx], rax` | Debería ser `ADeadOp::Mov` con `Operand::Mem` |

### El Bug de Shift Más Grave

```rust
// isa_compiler.rs línea 1310-1311
BitwiseOp::LeftShift  => self.ir.emit(ADeadOp::Shl { dst: Reg::RAX, amount: 1 }),
BitwiseOp::RightShift => self.ir.emit(ADeadOp::Shr { dst: Reg::RAX, amount: 1 }),
```

El valor de `right` (la expresión que indica cuántos bits shiftear) **ya fue evaluada** y está en `RBX`, pero se ignora completamente. `x << n` siempre genera `x << 1`.

**Fix necesario:**
- Si `right` es constante → `Shl { dst, amount: n }`  
- Si `right` es variable → `mov cl, bl; shl rax, cl` (shift by CL register)

### Solución Propuesta

El codegen necesita **Type-Aware Emission** — saber el tipo para elegir el tamaño correcto:

```rust
// Nuevo concepto: emit_typed_expression
fn emit_expression_typed(&mut self, expr: &Expr, expected: &Type) {
    match (expr, expected) {
        (Expr::Number(n), Type::I8) => {
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(Reg::AL),
                src: Operand::Imm8(*n as i8),
            });
        }
        (Expr::Number(n), Type::I32) => {
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(Reg::EAX),
                src: Operand::Imm32(*n as i32),
            });
        }
        // etc...
    }
}
```

Y el shift necesita manejar el amount correctamente:

```rust
BitwiseOp::LeftShift => {
    // right ya está en RBX (evaluado antes)
    match right.as_ref() {
        Expr::Number(n) => {
            self.ir.emit(ADeadOp::Shl { dst: Reg::RAX, amount: *n as u8 });
        }
        _ => {
            // Variable shift: usar CL
            self.ir.emit(ADeadOp::Mov {
                dst: Operand::Reg(Reg::RCX),
                src: Operand::Reg(Reg::RBX),
            });
            // shl rax, cl → necesita nuevo ADeadOp::ShlCl o RawBytes correcto
            self.ir.emit(ADeadOp::RawBytes(vec![0x48, 0xD3, 0xE0])); // shl rax, cl
        }
    }
}
```

---

## Problema 3: Tres CodeGen Duplicados

### Diagnóstico

Existen **3 generadores de código** para CPU:

| Archivo | Estado | Uso Real |
|---------|--------|----------|
| `codegen.rs` | 🔴 Legacy, casi no funciona | `if` dice "TODO", `print` dice "implementación pendiente" |
| `codegen_v2.rs` | 🟡 Funcional pero obsoleto | Emite bytes directos (sin ISA Layer) |
| `isa_compiler.rs` | 🟢 El principal | Usa `ADeadOp` → Encoder. Es el correcto. |

**`codegen.rs`** tiene literalmente:
```rust
Stmt::If { .. } => {
    // TODO: Implementar control flow
    eprintln!("⚠️  If statement not implemented in legacy codegen");
}
```

**`codegen_v2.rs`** duplica toda la lógica de `isa_compiler.rs` pero emitiendo bytes crudos directamente (1471 líneas de código redundante).

### Impacto

- Confusión: ¿cuál usar? Los tres se exportan en `mod.rs` y `lib.rs`.
- Bugs divergentes: un fix en `isa_compiler.rs` no se refleja en `codegen_v2.rs`.
- 2000+ líneas de código muerto que nadie mantiene.

### Solución Propuesta

1. **Eliminar `codegen.rs`** — completamente muerto.
2. **Marcar `codegen_v2.rs` como `#[deprecated]`** — mantener solo para referencia temporal.
3. **`isa_compiler.rs` es EL codegen** — todo flujo debe pasar por ahí.
4. Actualizar `mod.rs` y `lib.rs` para reflejar esto.

---

## Problema 4: Sin Register Allocator

### Diagnóstico

Toda expresión se evalúa en `RAX`. Para operaciones binarias:

```rust
// isa_compiler.rs - patrón repetido
self.emit_expression(left);                                    // resultado en RAX
self.ir.emit(ADeadOp::Push { src: Operand::Reg(Reg::RAX) }); // push al stack
self.emit_expression(right);                                   // resultado en RAX
self.ir.emit(ADeadOp::Mov { dst: RBX, src: RAX });           // mover a RBX
self.ir.emit(ADeadOp::Pop { dst: Reg::RAX });                // recuperar left
self.ir.emit(ADeadOp::Add { dst: RAX, src: RBX });           // operar
```

**Para `a + b + c + d`, esto genera 4 pushes y 4 pops.** Un register allocator simple asignaría registros sin tocar el stack.

### Impacto en Tamaño de Binario

Cada `push/pop` innecesario son 2 bytes mínimo. En un programa con 100 operaciones aritméticas, son ~400 bytes de más.

### Solución Propuesta (Incremental)

No necesitas un register allocator completo (eso es un proyecto de meses). Un **allocator de temporales simple** ya mejoraría enormemente:

```rust
struct TempAllocator {
    // Registros disponibles para temporales (no RAX/RSP/RBP)
    available: Vec<Reg>,  // [RBX, RCX, RDX, RSI, RDI, R8..R15]
    in_use: Vec<Reg>,
}

impl TempAllocator {
    fn alloc(&mut self) -> Reg {
        self.available.pop().unwrap_or_else(|| {
            // Si no hay registros, spill al stack (fallback actual)
            Reg::RAX // señal de "usa push/pop"
        })
    }
    fn free(&mut self, reg: Reg) {
        self.in_use.retain(|r| r != &reg);
        self.available.push(reg);
    }
}
```

---

## Problema 5: Encoder con Huecos Silenciosos

### Diagnóstico

En `encoder.rs`, muchas combinaciones de operandos caen a un fallback `NOP`:

```rust
// encoder.rs línea 222 - mov reg64, imm64
_ => self.emit(&[0x90]), // fallback nop
```

Si intentas `mov r10, 42` o `mov r12, 100`, el encoder emite `NOP` (0x90) en silencio. **No hay error, no hay warning.** Tu programa simplemente no funciona.

### Otros Huecos

| Patrón | Lo que hace | Lo que debería hacer |
|--------|-------------|---------------------|
| `mov r10-r15, imm64` | NOP | Emitir con REX.B prefix |
| `mov reg32 no-EAX, imm32` | Fallback a RAX encoding | Usar encoding correcto para cada reg |
| `CvtSi2Sd xmm1, rbx` | Emite `cvtsi2sd xmm0, rax` | Respetar dst/src |
| `add mem, imm` | No implementado | Necesario para `x += 5` optimizado |

### Solución Propuesta

1. **Cambiar todos los fallback NOP por `panic!` o `Result::Err`** — fallar ruidosamente, no silenciosamente.
2. **Implementar la tabla completa de MOV** — usando el patrón REX:
   ```rust
   fn encode_mov_reg64_imm64(&mut self, reg: &Reg, val: u64) {
       let (idx, needs_ext) = reg_index(reg);
       let rex = 0x48 | if needs_ext { 0x01 } else { 0x00 };
       self.emit(&[rex, 0xB8 + idx]);
       self.emit_u64(val);
   }
   ```
3. **Agregar tests para CADA registro** — no solo RAX/RCX/RBX.

---

## Problema Adicional: Calling Convention Rota en Linux

### Diagnóstico

`isa_compiler.rs` línea 1360-1373 — `emit_call()` **siempre** usa Windows x64 calling convention:

```rust
let dst = match i {
    0 => Reg::RCX,  // ← Windows
    1 => Reg::RDX,
    2 => Reg::R8,
    3 => Reg::R9,
    _ => unreachable!(),
};
```

En Linux (System V AMD64 ABI), debería ser:
```rust
let dst = match i {
    0 => Reg::RDI,  // ← Linux
    1 => Reg::RSI,
    2 => Reg::RDX,
    3 => Reg::RCX,
    _ => unreachable!(),
};
```

**Fix:**
```rust
fn arg_reg(&self, index: usize) -> Reg {
    match self.target {
        Target::Windows => [Reg::RCX, Reg::RDX, Reg::R8, Reg::R9][index],
        Target::Linux   => [Reg::RDI, Reg::RSI, Reg::RDX, Reg::RCX][index],
        Target::Raw     => [Reg::RDI, Reg::RSI, Reg::RDX, Reg::RCX][index],
    }
}
```

---

## Problema Adicional: Operand::Mem Limitado

### Diagnóstico

El `Operand::Mem` actual solo soporta `base + displacement`:

```rust
Mem { base: Reg, disp: i32 }  // Solo [rbp - 8], [rax + 0]
```

No soporta el addressing mode completo de x86-64 (`base + index * scale + displacement`):

```
[rax + rbx * 4 + 16]  ← arrays: base + index * sizeof(element) + offset
```

Esto es **esencial** para:
- Acceso a arrays: `arr[i]` → `[base + i * element_size]`
- Structs con campos: `s.field` → `[base + field_offset]`
- Tablas de función virtual: `vtable[n]` → `[base + n * 8]`

### Solución

```rust
pub enum Operand {
    Reg(Reg),
    Imm8(i8),
    Imm16(i16),    // NUEVO — necesario para modo 16-bit
    Imm32(i32),
    Imm64(u64),
    Mem {
        base: Reg,
        index: Option<Reg>,   // NUEVO
        scale: u8,             // NUEVO (1, 2, 4, 8)
        disp: i32,
    },
}
```

---

## Plan de Maduración — Orden de Ejecución

### Fase A: Corrección de Bugs Críticos (1-2 días)

> **Meta:** Que el código que genera HOY sea correcto.

| # | Tarea | Archivo | Esfuerzo |
|---|-------|---------|----------|
| A1 | Fix shift bug (amount hardcoded a 1) | `isa_compiler.rs:1310-1311, 781-784` | 30 min |
| A2 | Fix encoder fallback NOP → panic | `encoder.rs:222` y similares | 1 hora |
| A3 | Fix calling convention Linux | `isa_compiler.rs:1360-1373` | 30 min |
| A4 | Fix `CvtSi2Sd` que ignora dst/src | `encoder.rs:124-127` | 30 min |
| A5 | Reemplazar `BitwiseNot` RawBytes → `ADeadOp` propio | `isa_compiler.rs:1317` | 30 min |
| A6 | Reemplazar `emit_mem_write` RawBytes → `ADeadOp::Mov` | `isa_compiler.rs:725` | 30 min |

### Fase B: Unificación de Tipos (3-5 días)

> **Meta:** Un solo sistema de tipos que fluya del parser al codegen.

| # | Tarea | Archivo | Esfuerzo |
|---|-------|---------|----------|
| B1 | Crear `Type` unificado con tamaños (I8/I16/I32/I64/U8...) | `frontend/types.rs` | 2 horas |
| B2 | Eliminar `ast::Type`, usar el nuevo en todo el AST | `frontend/ast.rs` | 3 horas |
| B3 | Agregar tipo a `StructField` | `frontend/ast.rs` | 30 min |
| B4 | Simplificar `Param` (eliminar redundancia) | `frontend/ast.rs` | 1 hora |
| B5 | Actualizar parser para producir tipos reales | `frontend/parser.rs` | 4 horas |
| B6 | Reescribir `type_checker.rs` usando el tipo unificado | `frontend/type_checker.rs` | 4 horas |
| B7 | Propagar tipos al `isa_compiler.rs` | `isa/isa_compiler.rs` | 4 horas |

### Fase C: Machine Code Correcto por Tipo (3-5 días)

> **Meta:** `char x = 65` genera `mov al, 65`, no `mov rax, 65`.

| # | Tarea | Archivo | Esfuerzo |
|---|-------|---------|----------|
| C1 | Agregar `Operand::Imm16` | `isa/mod.rs` | 30 min |
| C2 | Extender `Operand::Mem` con index+scale | `isa/mod.rs` | 2 horas |
| C3 | Implementar `encode_mov` completo (todos los registros) | `isa/encoder.rs` | 4 horas |
| C4 | `emit_expression_typed()` que use tamaño correcto | `isa/isa_compiler.rs` | 4 horas |
| C5 | Agregar `ADeadOp::BitwiseNot` y `ADeadOp::ShlCl` | `isa/mod.rs` | 1 hora |
| C6 | Tests: verificar bytes para cada tamaño de tipo | `isa/encoder.rs` tests | 3 horas |

### Fase D: Limpieza (1-2 días)

> **Meta:** Un solo codegen, zero código muerto.

| # | Tarea | Archivo | Esfuerzo |
|---|-------|---------|----------|
| D1 | Eliminar `codegen.rs` | `backend/cpu/codegen.rs` | 15 min |
| D2 | Deprecar `codegen_v2.rs` | `backend/cpu/codegen_v2.rs` | 15 min |
| D3 | Actualizar `mod.rs` y `lib.rs` | `backend/cpu/mod.rs`, `lib.rs` | 30 min |
| D4 | Limpiar re-exports innecesarios | `backend/mod.rs` | 15 min |

### Fase E: Register Allocator Básico (5-7 días)

> **Meta:** Reducir pushes/pops innecesarios en un 60%+.

| # | Tarea | Archivo | Esfuerzo |
|---|-------|---------|----------|
| E1 | Implementar `TempAllocator` | Nuevo: `isa/reg_alloc.rs` | 4 horas |
| E2 | Integrar en `emit_expression` | `isa/isa_compiler.rs` | 6 horas |
| E3 | Respetar callee-saved registers | `isa/isa_compiler.rs` | 2 horas |
| E4 | Calcular stack frame real (no 128 fijo) | `isa/isa_compiler.rs` | 2 horas |
| E5 | Benchmarks: comparar tamaño antes/después | Tests | 2 horas |

---

## Cómo los Tipos y el Machine Code Trabajan Juntos

Después de las Fases B y C, el flujo sería:

```
Código ADead:        int x = 42          char c = 'A'         int* p = &x
                       ↓                    ↓                    ↓
Tipo resuelto:       Type::I32           Type::I8             Type::Pointer(I32)
                       ↓                    ↓                    ↓
ISA generada:        Mov(EAX, Imm32)     Mov(AL, Imm8)       Lea(RAX, Mem)
                       ↓                    ↓                    ↓
Bytes:               B8 2A 00 00 00      B0 41                48 8D 45 F8
                     (5 bytes)           (2 bytes)            (4 bytes)
```

**Versus hoy:**

```
Código ADead:        int x = 42          char c = 'A'         int* p = &x
                       ↓                    ↓                    ↓
Tipo resuelto:       (ninguno)           (ninguno)            (ninguno)
                       ↓                    ↓                    ↓
ISA generada:        Mov(RAX, Imm64)     Mov(RAX, Imm64)     Xor(RAX, RAX)
                       ↓                    ↓                    ↓
Bytes:               48 B8 2A...00       48 B8 41...00        31 C0
                     (10 bytes)          (10 bytes)           (fallback=0)
```

El tipado C no es solo "estilo": **determina qué bytes de machine code se generan**.

---

## Métricas de Éxito

| Métrica | Hoy | Después de Fase C | Después de Fase E |
|---------|-----|-------------------|-------------------|
| `char x = 65` → bytes | 10 | 2 | 2 |
| `x << 4` correcto | ❌ genera `x<<1` | ✅ | ✅ |
| `mov r10, 42` | NOP (silencioso) | correcto o error | correcto |
| Push/pop por expresión binaria | 2 siempre | 2 siempre | 0-1 |
| Archivos de codegen | 3 | 1 (+1 deprecated) | 1 |
| Tamaño "Hello World" PE | ~1500 bytes | ~1200 bytes | ~900 bytes |
| Linux calling convention | Rota | ✅ | ✅ |

---

## Conclusión

ADead-BIB tiene los cimientos correctos: la ISA Layer (`ADeadOp`), el encoder, el decoder, y el optimizer forman un pipeline real. **Pero falta el nexo entre el sistema de tipos y la generación de machine code.** Ese nexo es exactamente lo que separa un "generador de bytes" de un "compilador maduro".

Las Fases A-C (corrección de bugs + tipos unificados + codegen tipo-aware) son el **cambio de calidad más grande posible con el menor esfuerzo** — transforman ADead-BIB de "compila pero genera código sub-óptimo" a "compila y genera machine code correcto para cada tipo".

---

**Autor del Análisis:** Generado para Eddi Andreé Salazar Matos  
**Fecha:** 2026-02-20  
**Versión Base Analizada:** ADead-BIB v3.1-OS (143 tests)
