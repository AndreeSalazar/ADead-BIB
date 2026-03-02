# ADead-BIB v3.5 💀🦈

**Compilador Multi-Lenguaje: ADead-BIB · C99 · C++ → Binario Nativo**

> CPU = IR Completo | GPU = SPIR-V Directo | Sin NASM, Sin LLVM, Sin GCC

```
┌────────────────────────────────────────────────────────────────────┐
│          Tu Código (.adB / .c / .cpp)                              │
│                         ↓                                          │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                ADead-BIB Compiler v3.5                       │  │
│  │                                                              │  │
│  │   .adB ──→ Lexer/Parser ──→ AST ──┐                          │  │
│  │   .c   ──→ CLexer/CParser ──→ IR ─┤──→ ISA ──→ x86-64        │  │
│  │   .cpp ──→ CppLexer/CppParser ──→─┘      │                   │  │
│  │                                           ↓                  │  │
│  │   .adB ──→ AST ──→ SPIR-V bytes (GPU directo)                │  │
│  └──────────────────────────────────────────────────────────────┘  │
│                    ↓                          ↓                    │
│              .exe / .elf                  .spv (GPU)               │
│           (Binario Nativo)             (Compute Shader)            │
└────────────────────────────────────────────────────────────────────┘
```

---

## Inicio Rápido

```bash
# Clonar y compilar
git clone https://github.com/AndreeSalazar/ADead-BIB.git
cd ADead-BIB
cargo build --release

# ADead-BIB nativo
cargo run -- build examples/adB/MODE2_app_typed.adB -o app.exe

# C99 (auto-detecta .c)
cargo run -- examples/c/hello.c -o hello.exe

# C++ (auto-detecta .cpp)
cargo run -- examples/cpp/hello.cpp -o hello.exe

# O explícito:
cargo run -- cc examples/c/hello.c -o hello.exe
cargo run -- cxx examples/cpp/cpp_oop.cpp -o oop.exe

# GPU
cargo run -- gpu
```

---

## Frontends Soportados

| Frontend | Extensión | Comando | Auto-detect |
|----------|-----------|---------|-------------|
| **ADead-BIB** | `.adB` | `adeadc build` | ✅ |
| **C99** | `.c` | `adeadc cc` | ✅ por `.c` |
| **C++11/14/17/20** | `.cpp` `.cxx` `.cc` | `adeadc cxx` | ✅ por `.cpp` |
| **GPU** | `.adB` | `adeadc gpu` | — |

### C Frontend (v3.4)

- **Preprocesador** completo (#include, #define, #ifdef)
- **75+ headers** de sistema inyectados (stdio, stdlib, string, math, pthread...)
- **Structs**, typedefs, unions, function pointers
- **Punteros**, arrays, malloc/free, bitwise
- Pipeline: `C → CLexer → CParser → CAST → CToIR → Program → x86-64 → PE/ELF`
- **55 tests** — todos pasan ✅

### C++ Frontend (v3.5) — NUEVO

- **OOP**: classes, herencia, virtual, override, constructors, destructors
- **Templates**: function/class templates, non-type params, defaults
- **Namespaces**: anidados, using declarations
- **Modern C++**: auto, constexpr, nullptr, enum class, range-for
- **Lambdas**: captures (by value, by ref, this), params, return type
- **Casts**: static_cast, dynamic_cast, const_cast, reinterpret_cast
- **Exceptions**: try/catch/throw → eliminados a error codes
- **Smart pointers**: unique_ptr, shared_ptr → raw pointers (zero overhead)
- **C++20**: spaceship operator, concepts, coroutines (parsed)
- Pipeline: `C++ → CppLexer → CppParser → CppAST → CppToIR → Program → x86-64 → PE/ELF`
- **24 tests** — todos pasan ✅

---

## Filosofía: División por Verdad Binaria

| Aspecto | CPU | GPU |
|---------|-----|-----|
| **Representación** | IR (Intermediate Representation) | SPIR-V (bytecode directo) |
| **Optimización** | Completa (DCE, inlining, etc.) | Mínima (driver optimiza) |
| **Ejecución** | Secuencial + SIMD | Masivamente paralela |
| **Memoria** | Stack + Heap | Buffers + Shared Memory |

---

## CPU Backend: IR Completo

### Pipeline CPU

```
Código (.adB/.c/.cpp) → AST → IR → Optimizador → x86-64 bytes → PE/ELF
```

### IR Operations

| IR Op | Descripción | x86-64 |
|-------|-------------|--------|
| `IR_CONST` | Cargar constante | `mov rax, imm` |
| `IR_ADD` | Suma | `add rax, rbx` |
| `IR_MUL` | Multiplicación | `imul rax, rbx` |
| `IR_LOAD` | Cargar de memoria | `mov rax, [rbp+off]` |
| `IR_STORE` | Guardar en memoria | `mov [rbp+off], rax` |
| `IR_CALL` | Llamar función | `call rel32` |
| `IR_RET` | Retornar | `ret` |
| `IR_JMP` | Salto incondicional | `jmp rel32` |
| `IR_JZ` | Salto si cero | `jz rel32` |

### Optimizaciones CPU

- **Dead Code Elimination (DCE)** — Elimina código inalcanzable
- **Constant Folding** — Evalúa constantes en compilación
- **Inlining** — Expande funciones pequeñas
- **Register Allocation** — Minimiza accesos a memoria
- **Peephole** — Optimiza patrones locales

---

## GPU Backend: SPIR-V Directo

```
Código ADead → AST → SPIR-V bytes (directo, sin IR)
```

### ADead GPU Opcodes (4 bits)

| Opcode | Valor | Operación |
|--------|-------|-----------|
| `EXIT` | 0x0 | Terminar kernel |
| `LOAD` | 0x1 | acc = buffer[gid] |
| `STORE` | 0x2 | buffer[gid] = acc |
| `ADD` | 0x4 | acc += buffer[gid] |
| `MUL` | 0x6 | acc *= buffer[gid] |
| `MATMUL` | 0xB | Matrix multiply |
| `SYNC` | 0xC | Barrier |

### FFI GPU (Python)

```python
from FFI_GPU import GPU

gpu = GPU()
A = gpu.buffer(data_a)
B = gpu.buffer(data_b)
C = gpu.buffer(size=N)

kernel = gpu.load_spirv("vecadd.spv")
gpu.dispatch(kernel, A, B, C, groups=(N//256, 1, 1))
result = C.read()
```

---

## Estructura del Proyecto

```
ADead-BIB/
├── src/rust/
│   ├── frontend/
│   │   ├── lexer.rs, parser.rs, ast.rs    # ADead-BIB frontend
│   │   ├── c/                              # C99 frontend
│   │   │   ├── c_lexer.rs, c_parser.rs
│   │   │   ├── c_ast.rs, c_to_ir.rs
│   │   │   └── c_preprocessor.rs, c_stdlib.rs
│   │   └── cpp/                            # C++ frontend (NUEVO)
│   │       ├── cpp_lexer.rs, cpp_parser.rs
│   │       ├── cpp_ast.rs, cpp_to_ir.rs
│   │       └── mod.rs
│   ├── backend/
│   │   ├── cpu/         # IR → x86-64 → PE/ELF
│   │   └── gpu/         # AST → SPIR-V → Vulkan
│   ├── isa/             # ISA Compiler (ADeadOp)
│   ├── optimizer/       # DCE, Constant Folding, Inlining
│   └── runtime/         # GPU dispatcher, misuse detector
│
├── examples/
│   ├── adB/             # ADead-BIB examples + guías
│   ├── c/               # 13 C99 examples
│   ├── cpp/             # 4 C++ examples
│   ├── boot/            # Boot sectors, kernels
│   └── gpu/             # GPU compute
│
├── FFI/                 # Multi-language bindings
├── FFI GPU/             # Python GPU runtime
├── Metal_Dead/          # IA Personal CPU-first
└── Cargo.toml
```

---

## Comandos CLI

```bash
# ADead-BIB
adeadc build main.adB -o app.exe       # Compilar a .exe
adeadc run main.adB                     # Compilar y ejecutar
adeadc opt main.adB                     # Optimización máxima
adeadc check main.adB                   # Verificar sintaxis

# C (explícito o auto-detect por extensión)
adeadc cc hello.c -o hello.exe
adeadc hello.c -o hello.exe             # Auto-detect .c

# C++ (explícito o auto-detect por extensión)
adeadc cxx hello.cpp -o hello.exe
adeadc hello.cpp -o hello.exe           # Auto-detect .cpp

# Boot/OS
adeadc boot boot.adB -o boot.bin        # Boot sector 512b
adeadc flat kernel.adB -o kernel.bin    # Flat binary

# GPU
adeadc gpu                              # Detectar GPU + shader
adeadc spirv matmul 1024                # Generar SPIR-V
adeadc unified matmul 1024              # CPU+GPU unificado

# Binarios mínimos
adeadc tiny main.adB -o tiny.exe        # PE < 500 bytes
adeadc nano output.exe                  # PE más pequeño posible
adeadc micro output.exe                 # PE32 < 256 bytes

# Proyecto
adeadc create mi_proyecto               # Nuevo proyecto
adeadc init                             # Inicializar en directorio actual
```

---

## Metal_Dead: IA con ADead-BIB

Metal_Dead es una IA personal que usa ADead-BIB FFI para cómputo CPU-first.

```python
from Metal_Dead.core import MetalDeadCPU

ai = MetalDeadCPU()
response = ai.chat("Hola, soy el desarrollador")
print(response)
ai.shutdown()
```

- **CPU-First**: Optimizado para CPU con SIMD (AVX2)
- **ADead-BIB FFI**: Integración nativa
- **Transformer**: 2 capas, 128 dim embedding, ~1.2 MB RAM

---

## Tamaños de Binario

| Modo | Tamaño | Descripción |
|------|--------|-------------|
| CPU Normal | ~2 KB | Con IR optimizado |
| CPU Ultra | ~1 KB | Optimización máxima |
| CPU Tiny | <500 bytes | PE mínimo |
| CPU Micro | <256 bytes | PE32 ultra-compacto |
| GPU Shader | ~2 KB | SPIR-V completo |

---

## Test Summary

| Frontend | Tests | Status |
|----------|-------|--------|
| ADead-BIB | 40+ | ✅ |
| C99 | 55 | ✅ |
| C++ | 24 | ✅ |
| **Total** | **87+ frontend** | ✅ |

---

## Autor

**Eddi Andreé Salazar Matos**
eddi.salazar.dev@gmail.com
Hecho en Perú 🇵🇪

## Licencia

**GNU General Public License v2.0**

```
Copyright (C) 2024-2026 Eddi Andreé Salazar Matos
eddi.salazar.dev@gmail.com
```

---

**ADead-BIB v3.5: ADead-BIB · C99 · C++ → Binario Nativo 💀🦈**
**Sin NASM. Sin LLVM. Sin GCC. Sin Clang. Hecho en Perú 🇵🇪**
