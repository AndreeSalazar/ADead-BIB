# ADead-BIB - Roadmap v2.0

> **ADead-BIB** = **A**SM **Dead** - **B**inary **I**s **B**inary
> 
> Lenguaje que compila **DIRECTO a BINARIO y HEX** sin pasar por ensamblador.
> Sin ASM intermedio. Sin LLVM. Sin GCC. **Código → Bytes → Ejecutable.**
> 
> 100% Rust. Cero dependencias externas.

---

## 🎯 Filosofía Core: Binary Is Binary

```
COMPILADORES TRADICIONALES (7+ capas):
  Código → Tokens → AST → IR → Optimizer → ASM → Assembler → Linker → Binario

ADead-BIB (2-3 capas):
  Código → AST → BYTES DIRECTOS → Binario/HEX
```

**Principios:**
1. **No ASM intermedio** - Emitimos bytes x86-64 directamente
2. **No linker externo** - Generamos PE/ELF completos en memoria
3. **No runtime pesado** - El binario es autosuficiente
4. **HEX es ciudadano de primera clase** - Puedes escribir bytes literales

---

## 📊 Estado del Proyecto

| Componente | Estado | Descripción |
|------------|--------|-------------|
| **Lexer** | ✅ Completo | Tokenización con tracking de líneas |
| **Parser** | ✅ Funcional | Sintaxis Rust-style + Python-style |
| **Type Checker** | ⚠️ Básico | Inferencia limitada |
| **Binary CodeGen** | ✅ Funcional | Emite bytes x86-64 directamente |
| **PE Generator** | ✅ Funcional | Windows executables sin linker |
| **ELF Generator** | ✅ Funcional | Linux executables sin linker |
| **GPU HEX** | ✅ Funcional | Opcodes GPU directos (Vulkan/CUDA) |
| **Tests** | 61 pasando | ✅ |

---

## ✅ Versiones Completadas

### v0.5.0 ✅ - Fundamentos
- [x] Sintaxis estilo Rust (`fn`, `let`, `const`)
- [x] `print()` y `println()`
- [x] Secuencias de escape (`\n`, `\t`, `\r`)
- [x] Operaciones aritméticas (+, -, *, /, %)
- [x] **Compilación directa a bytes x86-64**
- [x] Generador PE integrado (sin linker)

### v0.6.0 ✅ - Control de Flujo
- [x] `if` / `else` → bytes de salto condicional directos
- [x] `while` / `for` loops → bytes de loop directos
- [x] `break` y `continue`
- [x] Comparaciones: `==`, `!=`, `<`, `>`, `<=`, `>=`

### v0.7.0 ✅ - Funciones
- [x] `fn nombre() { }` → prólogo/epílogo en bytes
- [x] Parámetros y retorno
- [x] Recursión
- [x] Calling convention Windows x64

### v0.8.0 ✅ - Tipos de Datos
- [x] Booleanos (`true`, `false`)
- [x] Enteros i64
- [x] Flotantes f64 (IEEE 754 directo)
- [x] Strings (punteros a data section)

### v0.9.0 ✅ - Entrada de Usuario
- [x] `input()` → llamada a scanf via IAT

### v1.0.0 ✅ - Estabilidad
- [x] Errores con línea y columna
- [x] 61 tests automatizados
- [x] Documentación ES/EN

### v1.1.0 ✅ - Flotantes Reales
- [x] Decimales con precisión (%.2f)
- [x] PI, E como constantes

### v1.2.0 ✅ - OOP y GPU
- [x] `struct` y `impl`
- [x] GPU Backend (Vulkan SPIR-V + CUDA)
- [x] Pipeline CPU↔GPU unificado

### v1.3.0 - v1.6.0 ✅ - Features Avanzados
- [x] Arrays: `[1, 2, 3]`, indexación, `len()`, iteración
- [x] Conversiones: `int()`, `float()`, `bool()`
- [x] Módulos: `import`, `from X import Y`
- [x] Traits: `trait`, `impl Trait for Struct`
- [x] Clases Python-style: `class`, `def`

---

## 🔥 v2.0.0 - HEX-First Architecture (NUEVA VISIÓN)

### 2.1 Literales Binarios Nativos
```rust
// Literales HEX directos en el código
let opcode = 0x48_89_E5      // mov rbp, rsp
let mask = 0b1111_0000       // Binario literal
let byte = 0xC3              // ret

// Bytes como array
let code: [u8] = [0x55, 0x48, 0x89, 0xE5, 0xC3]
```

### 2.2 Modo Raw Binary
```rust
// Archivo que compila a bytes puros (sin headers PE/ELF)
#![mode(raw)]
#![base(0x1000)]

fn _start() {
    // Genera solo los bytes de código
}
// Output: archivo .bin con bytes puros
```

### 2.3 Inline HEX (Nuevo)
```rust
fn fast_function() {
    // Insertar bytes directamente en el flujo de código
    emit![0x48, 0x31, 0xC0]  // xor rax, rax
    emit![0xC3]              // ret
}
```

### 2.4 Formatos de Salida
| Formato | Extensión | Descripción |
|---------|-----------|-------------|
| PE | `.exe` | Windows executable con headers |
| ELF | (sin ext) | Linux executable con headers |
| Raw | `.bin` | Bytes puros sin headers |
| Intel HEX | `.hex` | Formato Intel HEX |
| ADead Hybrid | `.ahyb` | Binario CPU+GPU combinado |

### 2.5 Operaciones Bit-Level
```rust
let x: u8 = 0b1010_1100
let shifted = x << 4         // Shift left
let masked = x & 0xF0        // AND mask
let bit3 = x.bit(3)          // Extraer bit individual
let packed = pack(a, b, c)   // Empaquetar bytes
```

---

## 🔥 v2.1.0 - CPU Direct Instructions

### Instrucciones x86-64 como Funciones
```rust
// Mapeo 1:1 a instrucciones de CPU
// NO es ASM textual - son funciones que emiten bytes

fn optimized_loop() {
    cpu::mov(rcx, 1000000)   // Emite: 48 B9 [imm64]
    cpu::xor(rax, rax)       // Emite: 48 31 C0
    
    loop {
        cpu::inc(rax)        // Emite: 48 FF C0
        cpu::dec(rcx)        // Emite: 48 FF C9
        if rcx == 0 { break }
    }
}
```

### Registros como Valores
```rust
// Registros disponibles como constantes tipadas
let result = cpu::rax       // Leer registro
cpu::rax = 42               // Escribir registro (emite mov)

// Registros: rax, rbx, rcx, rdx, rsi, rdi, r8-r15
// XMM: xmm0-xmm15 (para SIMD)
```

---

## 🔥 v2.2.0 - GPU HEX Unificado

### Opcodes GPU Directos
```rust
// Código GPU como bytes directos
gpu::init()                          // 0xC0DA0001
gpu::alloc(4096, reg0)               // 0xC0DA0010
gpu::matmul(reg0, reg1, reg2)        // 0xC0DA0020
gpu::sync()                          // 0xC0DA00FF
```

### Formato AHYB (ADead Hybrid Binary)
```
┌─────────────────────────────────┐
│ Header AHYB (8 bytes)           │
│   Magic: "AHYB"                 │
│   Version: u8                   │
│   Flags: u8                     │
│   CPU_size: u16                 │
│   GPU_size: u16                 │
├─────────────────────────────────┤
│ CPU Section (bytes x86-64)      │
├─────────────────────────────────┤
│ GPU Section (opcodes GPU)       │
└─────────────────────────────────┘
```

---

## 🛠️ Arquitectura del Compilador (Nueva)

```
┌──────────────────────────────────────────────────────────────────┐
│                    ADead-BIB Compiler v2.0                        │
│                    "Binary Is Binary"                             │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  Source (.adB)                                                    │
│       │                                                           │
│       ▼                                                           │
│  ┌─────────┐     ┌────────┐     ┌─────────────┐                  │
│  │  Lexer  │ ──▶ │ Parser │ ──▶ │ Type Check  │                  │
│  └─────────┘     └────────┘     └─────────────┘                  │
│                                        │                          │
│                                        ▼                          │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │              BINARY EMITTER (No ASM!)                       │  │
│  │                                                             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │  │
│  │  │ CPU x86-64   │  │ GPU Vulkan   │  │ GPU CUDA     │      │  │
│  │  │ Bytes Direct │  │ SPIR-V Direct│  │ PTX Direct   │      │  │
│  │  └──────────────┘  └──────────────┘  └──────────────┘      │  │
│  │         │                  │                 │              │  │
│  │         ▼                  ▼                 ▼              │  │
│  │  ┌──────────────────────────────────────────────────┐      │  │
│  │  │              BYTE STREAM                          │      │  │
│  │  │  [0x55, 0x48, 0x89, 0xE5, ...]                   │      │  │
│  │  └──────────────────────────────────────────────────┘      │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                        │                          │
│                                        ▼                          │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │              FORMAT GENERATOR                               │  │
│  │                                                             │  │
│  │  ┌────────┐  ┌────────┐  ┌────────┐  ┌────────┐  ┌──────┐ │  │
│  │  │  PE    │  │  ELF   │  │  RAW   │  │  HEX   │  │ AHYB │ │  │
│  │  │ .exe   │  │ binary │  │  .bin  │  │  .hex  │  │.ahyb │ │  │
│  │  └────────┘  └────────┘  └────────┘  └────────┘  └──────┘ │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                   │
└──────────────────────────────────────────────────────────────────┘
```

---

## 📁 Estructura del Código Fuente

```
src/rust/
├── frontend/           # Análisis de código fuente
│   ├── lexer.rs       # Tokenización
│   ├── parser.rs      # Parsing a AST
│   ├── ast.rs         # Definición del AST
│   └── type_checker.rs
│
├── backend/
│   ├── cpu/           # Generación de bytes CPU
│   │   ├── binary_emitter.rs   # 🔥 Core: emite bytes x86-64
│   │   ├── x86_opcodes.rs      # Tabla de opcodes x86-64
│   │   ├── pe_generator.rs     # Genera PE sin linker
│   │   ├── elf_generator.rs    # Genera ELF sin linker
│   │   └── raw_binary.rs       # Output bytes puros
│   │
│   └── gpu/           # Generación de bytes GPU
│       ├── hex_emitter.rs      # 🔥 Core: emite opcodes GPU
│       ├── spirv_direct.rs     # SPIR-V sin GLSL
│       ├── cuda_direct.rs      # PTX directo
│       └── ahyb_format.rs      # Formato híbrido
│
├── optimizer/         # Optimizaciones a nivel de bytes
│   ├── peephole.rs    # Optimización de secuencias de bytes
│   └── simd.rs        # Auto-vectorización
│
└── main.rs            # CLI
```

---

## 🔢 Tabla de Bytes x86-64 (Referencia Interna)

| Instrucción | Bytes | Descripción |
|-------------|-------|-------------|
| `push rbp` | `55` | Guardar base pointer |
| `mov rbp, rsp` | `48 89 E5` | Setup stack frame |
| `pop rbp` | `5D` | Restaurar base pointer |
| `ret` | `C3` | Retornar |
| `xor rax, rax` | `48 31 C0` | Limpiar rax (return 0) |
| `mov rax, imm64` | `48 B8 [8 bytes]` | Cargar inmediato 64-bit |
| `inc rcx` | `48 FF C1` | Incrementar rcx |
| `dec rcx` | `48 FF C9` | Decrementar rcx |
| `jmp rel8` | `EB [1 byte]` | Salto corto |
| `jmp rel32` | `E9 [4 bytes]` | Salto largo |
| `call rel32` | `E8 [4 bytes]` | Llamar función |

---

## 🎮 Opcodes GPU (Referencia Interna)

| Opcode | HEX | Descripción |
|--------|-----|-------------|
| GPU_INIT | `0xC0DA0001` | Inicializar contexto |
| GPU_ALLOC | `0xC0DA0010` | Reservar memoria |
| GPU_FREE | `0xC0DA0011` | Liberar memoria |
| GPU_COPY_H2D | `0xC0DA0012` | Host → Device |
| GPU_COPY_D2H | `0xC0DA0013` | Device → Host |
| GPU_MATMUL | `0xC0DA0020` | Multiplicación matrices |
| GPU_ADD | `0xC0DA0021` | Suma tensores |
| GPU_RELU | `0xC0DA0030` | Activación ReLU |
| GPU_SOFTMAX | `0xC0DA0033` | Softmax |
| GPU_SYNC | `0xC0DA00F0` | Sincronizar |
| GPU_END | `0xC0DAFFFF` | Fin programa |

---

## 📋 Prioridades de Desarrollo

| Prioridad | Feature | Versión |
|-----------|---------|---------|
| 🔴 **CRÍTICO** | `emit![]` macro para inline HEX | v2.0.0 |
| 🔴 **CRÍTICO** | Modo `#![mode(raw)]` | v2.0.0 |
| 🔴 **CRÍTICO** | Output `.bin` y `.hex` | v2.0.0 |
| 🟡 **ALTO** | Funciones `cpu::*` | v2.1.0 |
| 🟡 **ALTO** | GPU HEX unificado | v2.2.0 |
| 🟢 **MEDIO** | Formato AHYB | v2.2.0 |
| 🟢 **MEDIO** | Optimizador peephole | v2.3.0 |

---

## 🧹 v2.3.0 - Post-Procesamiento (Eliminar Ruido)

### Filosofía: Binario Limpio
El post-procesamiento es **OBLIGATORIO** para eliminar todo el ruido del binario final.

```
ANTES del post-procesamiento:
  [headers][padding][código][padding][datos][padding][metadatos]
  
DESPUÉS del post-procesamiento:
  [headers mínimos][código optimizado][datos compactos]
```

### Optimizaciones de Limpieza
| Optimización | Descripción | Ahorro |
|--------------|-------------|--------|
| **Strip padding** | Eliminar bytes de relleno innecesarios | ~20% |
| **Dead code removal** | Eliminar código no alcanzable | ~10% |
| **Constant folding** | `2 + 3` → `5` en tiempo de compilación | ~5% |
| **String dedup** | Strings duplicados → una sola copia | ~5% |
| **NOP elimination** | Eliminar NOPs de alineación innecesarios | ~3% |

### Modos de Limpieza
```rust
// Modo normal (default)
#![clean(normal)]

// Modo agresivo (binario más pequeño)
#![clean(aggressive)]

// Modo debug (sin limpieza, para debugging)
#![clean(none)]
```

### Resultado Esperado
| Programa | Sin limpiar | Limpio | Reducción |
|----------|-------------|--------|-----------|
| Hello World | 2048 bytes | 512 bytes | **75%** |
| Loop simple | 2560 bytes | 768 bytes | **70%** |
| Con funciones | 3072 bytes | 1024 bytes | **67%** |

---

## 📝 Sintaxis Humana (Principio Core)

### Filosofía: Simple para Humanos, Directo a Bytes
La sintaxis de ADead-BIB está diseñada para ser **legible por humanos** mientras compila **directamente a bytes**.

### Sintaxis Básica (Mantenida Simple)
```rust
// Variables - como escribirías en papel
let x = 42
let nombre = "Hola"
let activo = true

// Funciones - clara y directa
fn saludar(nombre) {
    println("Hola, " + nombre)
}

// Control de flujo - sin sorpresas
if x > 10 {
    println("Grande")
} else {
    println("Pequeño")
}

// Loops - intuitivos
for i in 0..10 {
    println(i)
}

while activo {
    // hacer algo
}
```

### Mapeo Sintaxis → Bytes
| Sintaxis Humana | Bytes Generados | Descripción |
|-----------------|-----------------|-------------|
| `let x = 42` | `48 C7 45 F8 2A 00 00 00` | mov [rbp-8], 42 |
| `x + y` | `48 03 C1` | add rax, rcx |
| `if x > 0` | `48 83 F8 00` `7E xx` | cmp rax, 0; jle |
| `fn foo()` | `55 48 89 E5` | push rbp; mov rbp, rsp |
| `return` | `5D C3` | pop rbp; ret |
| `println(x)` | `E8 xx xx xx xx` | call printf |

### Principios de Diseño
1. **Legibilidad** - El código debe leerse como pseudocódigo
2. **Predictibilidad** - Cada construcción genera bytes predecibles
3. **Sin magia** - No hay transformaciones ocultas
4. **Directo** - Mínimas capas entre código y binario

---

## 🧪 Estructura de Tests

### Carpeta TESTEO/ (Organización)
```
TESTEO/
├── v1/                     # Tests de versiones 1.x
│   ├── arrays/             # Arrays y colecciones
│   ├── conversiones/       # int(), float(), bool()
│   ├── input/              # input() real
│   ├── len/                # len() function
│   ├── modules/            # Sistema de módulos
│   ├── traits/             # Traits e interfaces
│   └── integrados/         # Tests completos por versión
│
├── v2/                     # Tests de versiones 2.x (NUEVO)
│   ├── hex/                # Literales HEX
│   │   ├── test_hex_literal.adB
│   │   ├── test_binary_literal.adB
│   │   └── test_emit_macro.adB
│   ├── raw/                # Modo raw binary
│   │   ├── test_raw_mode.adB
│   │   └── test_base_address.adB
│   ├── cpu/                # Instrucciones CPU directas
│   │   ├── test_cpu_mov.adB
│   │   ├── test_cpu_loop.adB
│   │   └── test_registers.adB
│   ├── gpu/                # GPU HEX
│   │   ├── test_gpu_init.adB
│   │   ├── test_gpu_matmul.adB
│   │   └── test_ahyb_format.adB
│   ├── clean/              # Post-procesamiento
│   │   ├── test_strip_padding.adB
│   │   ├── test_dead_code.adB
│   │   └── test_size_comparison.adB
│   └── integrados/         # Tests completos v2.x
│       ├── test_v2_0_0_hex_first.adB
│       ├── test_v2_1_0_cpu_direct.adB
│       └── test_v2_2_0_gpu_hex.adB
│
└── README.md               # Documentación de tests
```

### Comandos de Test
```bash
# Ejecutar todos los tests
cargo test

# Test específico v2.0
cargo run --bin adeadc -- run TESTEO/v2/hex/test_hex_literal.adB

# Test de tamaño (post-procesamiento)
cargo run --bin adeadc -- build TESTEO/v2/clean/test_size_comparison.adB --clean aggressive
```

---

## 🐛 Bugs Conocidos

| Prioridad | Bug | Estado |
|-----------|-----|--------|
| 🔴 Alta | Type Checker no infiere retornos | Pendiente |
| 🟡 Media | Parser Python-style sin indentación real | Pendiente |
| 🟢 Baja | Warnings de variables no usadas | Pendiente |

---

## 📜 Historial de Cambios

| Versión | Fecha | Cambios |
|---------|-------|---------|
| v2.0.0 | 2025-01 | 🔥 HEX-First Architecture |
| v1.6.0 | 2024-12 | Traits e interfaces |
| v1.5.0 | 2024-12 | Sistema de módulos |
| v1.4.0 | 2024-12 | Input real (scanf) |
| v1.3.0 | 2024-12 | Arrays y conversiones |
| v1.2.0 | 2024-12 | Structs, impl, GPU |
| v1.1.0 | 2024-12 | Flotantes reales |
| v1.0.0 | 2024-12 | Estabilidad |

---

## 🤝 Contribuir

1. Fork el repositorio
2. `git checkout -b feature/mi-feature`
3. `cargo test`
4. Pull Request

### Áreas de ayuda
- Implementar `emit![]` macro
- Más opcodes x86-64 en tabla
- Testing Linux ELF
- Documentación de bytes

---

## 📚 Recursos

- **Docs**: `GUIA_ES.md`, `GUIDE_EN.md`
- **Ejemplos**: `/examples/*.adB`
- **Tests**: `cargo test`
- **Intel x86-64 Manual**: Referencia de opcodes

---

*ADead-BIB: Donde el código se convierte en bytes, sin intermediarios.*

*Creado por Eddi Andreé Salazar Matos* 🇵🇪
*Última actualización: Enero 2025*
