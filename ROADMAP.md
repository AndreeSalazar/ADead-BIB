# ADead-BIB - Roadmap v2.0

> **ADead-BIB** = **A**SM **Dead** - **B**inary **I**s **B**inary
> 
> Lenguaje que compila **DIRECTO a BINARIO (CPU) y HEX (GPU)**.
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

### Principios Fundamentales

1. **No ASM intermedio** — Emitimos bytes x86-64 directamente
2. **No linker externo** — Generamos PE/ELF completos en memoria
3. **No runtime pesado** — El binario es autosuficiente
4. **HEX es ciudadano de primera clase** — Puedes escribir bytes literales
5. **CPU y GPU trabajan por separado** — Contratos directos para cada uno

---

## 🔵 CPU Backend (Binario) - Contratos Directos

### Arquitectura CPU
```
┌────────────────────────────────────────────────────────────┐
│                    CPU Backend                             │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  Código ADead-BIB                                          │
│       ↓                                                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              BINARY EMITTER                         │   │
│  │                                                     │   │
│  │  codegen_v2.rs  →  Genera bytes x86-64              │   │
│  │  binary_raw.rs  →  Emisor de bytes directos         │   │
│  │  pe_tiny.rs     →  PE ultra-compacto (<500 bytes)   │   │
│  │  pe.rs          →  Windows PE estándar              │   │
│  │  elf.rs         →  Linux ELF                        │   │
│  └─────────────────────────────────────────────────────┘   │
│       ↓                                                    │
│  .exe / .elf (Binario Nativo)                              │
└────────────────────────────────────────────────────────────┘
```

### Literales Binarios Implementados ✅
```rust
// Literales binarios (0b...)
let mask = 0b11110000          // 240
let bits = 0b1010_1010         // 170 (con separadores)

// Literales HEX para opcodes CPU
let push_rbp = 0x55            // push rbp
let ret = 0xC3                 // ret
let call = 0xE8                // call rel32

// Literales octales (bonus)
let perms = 0o755              // 493
```

### Tabla de Opcodes x86-64
| Instrucción | Bytes | Descripción |
|-------------|-------|-------------|
| `push rbp` | `0x55` | Guardar base pointer |
| `mov rbp, rsp` | `0x48 0x89 0xE5` | Setup stack frame |
| `pop rbp` | `0x5D` | Restaurar base pointer |
| `ret` | `0xC3` | Retornar |
| `xor rax, rax` | `0x48 0x31 0xC0` | Limpiar rax |
| `call rel32` | `0xE8 [4 bytes]` | Llamar función |
| `jmp rel32` | `0xE9 [4 bytes]` | Salto incondicional |

### Calling Convention Windows x64
```
Parámetros: RCX, RDX, R8, R9 (primeros 4)
            Stack (resto)
Retorno:    RAX
Preservar:  RBX, RBP, RDI, RSI, R12-R15
Alineación: Stack a 16 bytes antes de call
```

---

## 🟢 GPU Backend (HEX) - Contratos Directos

### Arquitectura GPU (Dos Niveles)
```
┌─────────────────────────────────────────────────────────────┐
│                    GPU Backend                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Nivel 1: Opcodes ADead-BIB (0xC0DA...)                     │
│    - Tu contrato                                            │
│    - Tu formato                                             │
│    - Portable                                               │
│    - Documentado                                            │
│                                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Nivel 2: Backend por target                                │
│    ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│    │    SPIR-V    │  │     CUDA     │  │    Vulkan    │     │
│    │  (Todas GPU) │  │   (NVIDIA)   │  │   (Runtime)  │     │
│    └──────────────┘  └──────────────┘  └──────────────┘     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Tabla de Opcodes GPU
| Opcode | HEX | Descripción |
|--------|-----|-------------|
| `GPU_INIT` | `0xC0DA0001` | Inicializar contexto |
| `GPU_SHUTDOWN` | `0xC0DA0002` | Cerrar contexto |
| `GPU_ALLOC` | `0xC0DA0010` | Reservar memoria |
| `GPU_FREE` | `0xC0DA0011` | Liberar memoria |
| `GPU_COPY_H2D` | `0xC0DA0012` | Host → Device |
| `GPU_COPY_D2H` | `0xC0DA0013` | Device → Host |
| `GPU_MATMUL` | `0xC0DA0020` | Multiplicación matrices |
| `GPU_ADD` | `0xC0DA0021` | Suma tensores |
| `GPU_MUL` | `0xC0DA0023` | Multiplicación elemento |
| `GPU_SYNC` | `0xC0DA00F0` | Sincronizar |
| `GPU_END` | `0xC0DAFFFF` | Fin programa |

### Relación CPU ↔ GPU
```
CPU prepara → GPU ejecuta → CPU recibe

CPU:
  1. Escribe datos en memoria
  2. Escribe comandos GPU
  3. Dispara ejecución
  4. Se aparta

GPU:
  1. Lee comandos
  2. Ejecuta kernels
  3. Escribe resultados
  4. Sin volver a preguntar
```

**La CPU NO mira cada iteración.**
**La GPU NO pide permiso.**

---

## 📊 Estado del Proyecto

| Componente | Estado | Descripción |
|------------|--------|-------------|
| **Lexer** | ✅ Completo | HEX/BIN/OCT literals |
| **Parser** | ✅ Funcional | Sintaxis Rust + Python |
| **Type Checker** | ⚠️ Básico | Inferencia limitada |
| **CPU Binary** | ✅ Funcional | Bytes x86-64 directos |
| **GPU HEX** | ✅ Funcional | Opcodes 0xC0DA... |
| **SPIR-V Backend** | ✅ Funcional | Todas las GPUs Vulkan |
| **CUDA Backend** | ✅ Funcional | NVIDIA PTX |
| **PE Generator** | ✅ Funcional | Windows sin linker |
| **ELF Generator** | ✅ Funcional | Linux sin linker |
| **Tests** | ✅ Pasando | CPU + GPU + v2.0 |

---

## ✅ Versiones Completadas

### v0.5.0 - v1.6.0 ✅ (Fundamentos)
- [x] Sintaxis Rust/Python
- [x] Compilación directa a bytes x86-64
- [x] Control de flujo (if, while, for)
- [x] Funciones con calling convention
- [x] OOP (structs, classes, traits)
- [x] Arrays, módulos, input()
- [x] GPU básico (Vulkan/CUDA)

### v2.0.0 ✅ (HEX-First Architecture)
- [x] **Literales HEX**: `0xFF`, `0x1234`, `0xFF_FF`
- [x] **Literales Binarios**: `0b11110000`, `0b1111_0000`
- [x] **Literales Octales**: `0o755`, `0o777`
- [x] **Separadores estilo Rust**: `0xFF_FF`, `0b1111_0000`
- [x] **Tests CPU organizados**: binario/, opcodes/, contratos/
- [x] **Tests GPU organizados**: hex/, opcodes/, contratos/
- [x] **Documentación de estructura**: docs/ESTRUCTURA.md

---

## 🔥 v2.1.0 - CPU Direct Instructions (PRÓXIMO)

### Objetivo
Funciones que emiten instrucciones x86-64 directamente.

### Sintaxis Propuesta
```rust
fn optimized_loop() {
    // Mapeo 1:1 a instrucciones de CPU
    // NO es ASM textual - son funciones que emiten bytes
    
    cpu::mov(rcx, 1000000)   // Emite: 48 B9 [imm64]
    cpu::xor(rax, rax)       // Emite: 48 31 C0
    
    loop {
        cpu::inc(rax)        // Emite: 48 FF C0
        cpu::dec(rcx)        // Emite: 48 FF C9
        if rcx == 0 { break }
    }
}
```

### Tareas
- [ ] Implementar módulo `cpu::` con funciones de instrucciones
- [ ] Registros como constantes tipadas (rax, rbx, rcx, etc.)
- [ ] Validación de operandos en tiempo de compilación
- [ ] Tests para cada instrucción

---

## 🔥 v2.2.0 - GPU Direct Functions

### Objetivo
Funciones que emiten opcodes GPU directamente.

### Sintaxis Propuesta
```rust
fn gpu_matmul() {
    gpu::init()                          // 0xC0DA0001
    gpu::alloc(4096, reg0)               // 0xC0DA0010
    gpu::matmul(reg0, reg1, reg2)        // 0xC0DA0020
    gpu::sync()                          // 0xC0DA00F0
}
```

### Tareas
- [ ] Implementar módulo `gpu::` con funciones de opcodes
- [ ] Registros GPU como constantes
- [ ] Generación automática de command buffer
- [ ] Tests para cada opcode

---

## 🔥 v2.3.0 - emit![] Macro

### Objetivo
Insertar bytes directamente en el flujo de código.

### Sintaxis Propuesta
```rust
fn fast_function() {
    // Insertar bytes directamente
    emit![0x48, 0x31, 0xC0]  // xor rax, rax
    emit![0xC3]              // ret
}
```

### Tareas
- [ ] Implementar macro `emit![]` en el parser
- [ ] Validación de bytes en tiempo de compilación
- [ ] Integración con el flujo de código existente
- [ ] Tests

---

## 🔥 v2.4.0 - Modo Raw Binary

### Objetivo
Compilar a bytes puros sin headers PE/ELF.

### Sintaxis Propuesta
```rust
#![mode(raw)]
#![base(0x1000)]

fn _start() {
    // Genera solo los bytes de código
}
// Output: archivo .bin con bytes puros
```

### Tareas
- [ ] Implementar atributo `#![mode(raw)]`
- [ ] Implementar atributo `#![base(addr)]`
- [ ] Generador de .bin sin headers
- [ ] Tests

---

## 🔥 v2.5.0 - Formato AHYB (ADead Hybrid Binary)

### Objetivo
Binario que contiene código CPU + GPU en un solo archivo.

### Formato
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
│ GPU Section (opcodes HEX)       │
└─────────────────────────────────┘
```

### Tareas
- [ ] Definir especificación AHYB
- [ ] Generador de archivos .ahyb
- [ ] Loader de archivos .ahyb
- [ ] Tests

---

## 🧹 v2.6.0 - Post-Procesamiento

### Objetivo
Eliminar ruido del binario final.

### Optimizaciones
| Optimización | Descripción | Ahorro |
|--------------|-------------|--------|
| **Strip padding** | Eliminar bytes de relleno | ~20% |
| **Dead code removal** | Eliminar código no alcanzable | ~10% |
| **Constant folding** | `2 + 3` → `5` en compilación | ~5% |
| **String dedup** | Strings duplicados → una copia | ~5% |
| **NOP elimination** | Eliminar NOPs innecesarios | ~3% |

### Modos
```rust
#![clean(normal)]      // Default
#![clean(aggressive)]  // Binario más pequeño
#![clean(none)]        // Sin limpieza (debug)
```

---

## 📝 Sintaxis Humana (Principio Core)

### Filosofía
**Simple para humanos, directo a bytes.**

```rust
// Lo que escribes (humano)
let x = 0xFF
let y = 0b11110000
let sum = x + y

// Lo que genera (bytes)
48 C7 C0 FF 00 00 00    ; mov rax, 255
48 C7 C1 F0 00 00 00    ; mov rcx, 240
48 01 C8                ; add rax, rcx
```

### Sintaxis Soportada
```rust
// Variables
let x = 42
let hex = 0xFF
let bin = 0b1010

// Funciones
fn add(a, b) {
    return a + b
}

// Control de flujo
if x == 0xFF {
    println("Max byte!")
}

for i in 0..10 {
    println(i)
}

// OOP
struct Point { x, y }
impl Point {
    fn new(x, y) { return Point { x, y } }
}
```

---

## 🧪 Tests Organizados

### Estructura
```
TESTEO/
├── CPU/                     # Tests CPU (Binario)
│   ├── binario/             # Literales 0b...
│   ├── opcodes/             # Opcodes x86-64
│   └── contratos/           # Calling conventions
│
├── GPU/                     # Tests GPU (HEX)
│   ├── hex/                 # Literales 0x...
│   ├── opcodes/             # Opcodes GPU
│   └── contratos/           # Command buffers
│
└── v2/                      # Tests v2.0.0
    ├── hex/
    ├── raw/
    ├── cpu/
    ├── gpu/
    └── integrados/
```

### Comandos
```bash
# Tests CPU
cargo run --bin adeadc -- run TESTEO/CPU/binario/test_binary_literals.adB
cargo run --bin adeadc -- run TESTEO/CPU/opcodes/test_x86_opcodes.adB

# Tests GPU
cargo run --bin adeadc -- run TESTEO/GPU/hex/test_hex_literals.adB
cargo run --bin adeadc -- run TESTEO/GPU/opcodes/test_gpu_opcodes.adB

# Test integrado v2.0
cargo run --bin adeadc -- run TESTEO/v2/integrados/test_v2_0_0_hex_first.adB
```

---

## 📅 Timeline Estimado

| Versión | Objetivo | Estimado |
|---------|----------|----------|
| v2.0.0 | HEX-First Architecture | ✅ Completado |
| v2.1.0 | CPU Direct Instructions | Q1 2025 |
| v2.2.0 | GPU Direct Functions | Q1 2025 |
| v2.3.0 | emit![] Macro | Q2 2025 |
| v2.4.0 | Modo Raw Binary | Q2 2025 |
| v2.5.0 | Formato AHYB | Q3 2025 |
| v2.6.0 | Post-Procesamiento | Q3 2025 |

---

## 🔗 Documentación

- [README.md](README.md) — Documentación principal (inglés)
- [GUIA_ES.md](GUIA_ES.md) — Guía en español
- [docs/ESTRUCTURA.md](docs/ESTRUCTURA.md) — Estructura del proyecto
- [TESTEO/README.md](TESTEO/README.md) — Guía de tests

---

**ADead-BIB: Código → Bytes → Binario**
**CPU (Binario) + GPU (HEX) = Contratos Directos**
**Sin ASM. Sin LLVM. Sin mentiras.**
