# ADead-BIB — Ejemplos de Compilación

## Backend Universal · Nano Bytes · Sin Overhead 💀🦈

### Hecho en Perú 🇵🇪 · v3.5 · 2025

ADead-BIB genera binarios directamente — sin ensamblador intermedio, sin LLVM, sin GCC.
Auto-detecta `.c` y `.cpp` por extensión: `adeadc hello.cpp -o hello.exe` funciona directo.

---

## Modos de Compilación

| Modo | Lenguaje | Output | Comando |
|------|----------|--------|---------|
| **Modo 1** | ADead-BIB (raw) | `.bin` flat binary | `adeadc flat` / `adeadc boot` |
| **Modo 2a** | ADead-BIB (typed) | `.exe` PE / ELF | `adeadc build` |
| **Modo 2b** | C99 nativo | `.exe` PE / ELF | `adeadc cc` o auto `.c` |
| **Modo 2c** | C++11/14/17/20 | `.exe` PE / ELF | `adeadc cxx` o auto `.cpp` |
| **Modo 3** | GPU compute | `.spv` SPIR-V | `adeadc gpu` |

---

## Estructura de Carpetas

```
examples/
├── boot/               # Modo 1: Boot sectors, kernels, drivers
│   ├── MODE1_boot_minimal.adB
│   ├── MODE1_kernel_driver.adB
│   ├── boot_sector.adB
│   ├── boot_labels_test.adB
│   ├── os_kernel_setup.adB
│   └── boot.bin
├── adB/                # Modo 2a: ADead-BIB typed + OOP + guías
│   ├── MODE2_app_typed.adB
│   ├── MODE2_oop_classes.adB
│   ├── test_large_program.adB
│   ├── 01_hello.adB ... 11_pointers_real.adB
│   └── (guías de aprendizaje)
├── c/                  # Modo 2b: C99 nativo (13 archivos)
│   ├── hello.c
│   ├── c_algorithms.c
│   ├── c_bitwise.c
│   ├── c_compression.c
│   ├── c_crypto.c
│   ├── c_database.c
│   ├── c_fastos_base.c
│   ├── c_fastos_complete.c
│   ├── c_math.c
│   ├── c_memory.c
│   ├── c_network.c
│   ├── c_structs.c
│   └── c_threading.c
├── cpp/                # Modo 2c: C++11/14/17/20 (4 archivos)
│   ├── hello.cpp
│   ├── cpp_oop.cpp
│   ├── cpp_templates.cpp
│   └── cpp_modern.cpp
├── gpu/                # Modo 3: GPU compute SPIR-V
│   └── MODE3_gpu_compute.adB
└── README.md
```

---

## Modo 1: Boot/OS — `boot/`

Flat binary sin headers. Cada byte cuenta.

```bash
adeadc boot boot/MODE1_boot_minimal.adB -o boot.bin
adeadc flat boot/MODE1_kernel_driver.adB -o kernel.bin
adeadc flat boot/os_kernel_setup.adB -o kernel.bin
```

---

## Modo 2a: ADead-BIB Typed — `adB/`

```bash
adeadc build adB/MODE2_app_typed.adB -o app.exe
adeadc build adB/MODE2_oop_classes.adB -o oop.exe
adeadc build adB/test_large_program.adB -o large.exe
```

---

## Modo 2b: C99 — `c/`

Sin GCC. Sin Clang. 100% ADead-BIB.

Pipeline: `C Source → CLexer → CParser → CAST → CToIR → Program → x86-64 → PE/ELF`

```bash
# Explícito
adeadc cc c/hello.c -o hello.exe

# Auto-detección por extensión .c
adeadc c/hello.c -o hello.exe
adeadc c/c_fastos_base.c -o fastos.exe
adeadc c/c_algorithms.c -o algorithms.exe
```

| Archivo | Descripción |
|---------|-------------|
| `hello.c` | Hello World C99 |
| `c_algorithms.c` | Quicksort, mergesort, binary search, linked list |
| `c_bitwise.c` | AND/OR/XOR/shifts, bit manipulation |
| `c_compression.c` | Run-length encoding, Huffman |
| `c_crypto.c` | XOR/Caesar cipher, hash functions |
| `c_database.c` | In-memory key-value store, CRUD |
| `c_math.c` | Trig, álgebra lineal, complejos |
| `c_memory.c` | malloc/free, memory pools, arena |
| `c_network.c` | Sockets, HTTP basics |
| `c_structs.c` | Structs, typedefs, unions, nested |
| `c_threading.c` | pthread, mutex, thread pool |
| `c_fastos_base.c` | 75+ system headers compilados |
| `c_fastos_complete.c` | FastOS completo (fs, mem, proc) |

Tests: 55 (42 unit + 13 example files) — todos pasan ✅

---

## Modo 2c: C++ — `cpp/`

Sin GCC. Sin LLVM. Sin Clang. 100% ADead-BIB C++.

Pipeline: `C++ Source → CppLexer → CppParser → CppAST → CppToIR → Program → x86-64 → PE/ELF`

```bash
# Explícito
adeadc cxx cpp/hello.cpp -o hello.exe

# Auto-detección por extensión .cpp
adeadc cpp/hello.cpp -o hello.exe
adeadc cpp/cpp_oop.cpp -o oop.exe
adeadc cpp/cpp_templates.cpp -o templates.exe
adeadc cpp/cpp_modern.cpp -o modern.exe
```

| Archivo | Descripción |
|---------|-------------|
| `hello.cpp` | Hello World C++ |
| `cpp_oop.cpp` | Classes, herencia, virtual, override, constructors |
| `cpp_templates.cpp` | Function/class templates, namespaces, recursion |
| `cpp_modern.cpp` | auto, constexpr, nullptr, enum class, type aliases |

**Features soportados:**
- **OOP**: classes, herencia, virtual, override, constructors, destructors
- **Templates**: function/class templates, non-type params, defaults
- **Namespaces**: anidados, using declarations
- **Modern C++**: auto, constexpr, nullptr, enum class, range-for
- **Lambdas**: captures, params, return type
- **Casts**: static_cast, dynamic_cast, const_cast, reinterpret_cast
- **Exceptions**: try/catch/throw (eliminados a error codes)
- **Smart pointers**: unique_ptr, shared_ptr → raw pointers
- **C++20**: spaceship operator, concepts, coroutines (parsed)

Tests: 24 (8 lexer + 6 parser + 6 IR + 4 example files) — todos pasan ✅

---

## Modo 3: GPU — `gpu/`

AST → SPIR-V directo, sin IR intermedio.

```bash
adeadc gpu gpu/MODE3_gpu_compute.adB -o compute.spv
```

---

## Arquitectura del Compilador

```
Código fuente (.adB / .c / .cpp)
         │
    Lexer → Tokens
         │
    Parser → AST
         │
    Type Checker / IR Converter
         │
    ISA Compiler → ADeadOp
         │
    Encoder → x86-64 Bytes
         │
    PE (Windows) / ELF (Linux)
```

## Test Summary

| Frontend | Tests | Status |
|----------|-------|--------|
| ADead-BIB | 40+ | ✅ |
| C99 | 55 | ✅ |
| C++ | 24 | ✅ |
| **Total** | **87+ frontend** | ✅ |

---

**Sin NASM. Sin LLVM. Sin linker externo.**
**100% ADead-BIB. Hecho en Perú 🇵🇪 💀🦈**

Autor: Eddi Andreé Salazar Matos
Versión: ADead-BIB v3.5
Licencia: MIT
