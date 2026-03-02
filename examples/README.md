# ADead-BIB — Ejemplos de Compilación
## Backend Universal • Nano Bytes • Sin Overhead 💀🦈
### Hecho en Perú 🇵🇪 • 2025

ADead-BIB genera binarios directamente — sin ensamblador intermedio, sin LLVM, sin GCC.

---

## Modos de Compilación

| Modo | Lenguaje | Output | Comando |
|------|----------|--------|---------|
| **Modo 1** | ADead-BIB (raw) | `.bin` flat binary | `adB flat` / `adB boot` |
| **Modo 2a** | ADead-BIB (typed) | `.exe` PE / ELF | `adB build` |
| **Modo 2b** | C99 nativo | `.exe` PE / ELF | `adB cc` |
| **Modo 2c** | C++11/14/17/20 | `.exe` PE / ELF | `adB cxx` |
| **Modo 3** | GPU compute | `.spv` SPIR-V | `adB gpu` |

---

## Modo 1: Raw Machine Code (Boot/OS)

Flat binary sin headers. Cada byte cuenta.

| Archivo | Descripción | Comando |
|---------|-------------|---------|
| `MODE1_boot_minimal.adB` | Boot sector 512b con `0x55AA` | `adB boot ... -o boot.bin` |
| `MODE1_kernel_driver.adB` | GDT/IDT, port I/O, VGA, serial | `adB flat ... -o kernel.bin` |
| `boot_sector.adB` | Boot con labels y `raw { }` | `adB boot ... -o boot.bin` |
| `boot_labels_test.adB` | Forward/backward label refs | `adB flat ... -o test.bin` |
| `os_kernel_setup.adB` | Kernel config, VGA, serial | `adB flat ... -o kernel.bin` |

---

## Modo 2a: ADead-BIB Typed + OOP

| Archivo | Descripción | Comando |
|---------|-------------|---------|
| `MODE2_app_typed.adB` | Tipos, punteros, arrays, structs | `adB build ... -o app.exe` |
| `MODE2_oop_classes.adB` | Herencia, virtual, interfaces | `adB build ... -o oop.exe` |
| `test_large_program.adB` | Stress test del compilador | `adB build ... -o large.exe` |

---

## Modo 2b: C99 Nativo (`adB cc`)

Sin GCC. Sin Clang. 100% ADead-BIB.

Pipeline: `C Source -> CPreprocessor -> CLexer -> CParser -> CAST -> CToIR -> Program -> x86-64 -> PE/ELF`

| Archivo | Descripción | Headers |
|---------|-------------|---------|
| `hello.c` | Hello World C99 | stdlib |
| `c_algorithms.c` | Quicksort, mergesort, binary search | stdlib |
| `c_bitwise.c` | AND/OR/XOR/shifts, bit manipulation | stdlib |
| `c_compression.c` | Run-length encoding, Huffman | stdlib |
| `c_crypto.c` | XOR/Caesar cipher, hash functions | stdlib |
| `c_database.c` | In-memory key-value store, CRUD | stdlib |
| `c_math.c` | Trig, algebra lineal, complejos | math.h |
| `c_memory.c` | malloc/free, memory pools, arena | stdlib |
| `c_network.c` | Sockets, HTTP basics | network |
| `c_structs.c` | Structs, typedefs, unions, nested | stdlib |
| `c_threading.c` | pthread, mutex, thread pool | pthread |
| `c_fastos_base.c` | 75+ system headers compilados | ALL |
| `c_fastos_complete.c` | FastOS completo (fs, mem, proc) | ALL |

```bash
adB cc hello.c -o hello.exe
adB cc c_fastos_base.c -o fastos.exe
```

Tests: 55 tests (42 unit + 13 example files) — todos pasan

---

## Modo 2c: C++11/14/17/20 (`adB cxx`)

Sin GCC. Sin LLVM. Sin Clang. 100% ADead-BIB C++.

Pipeline: `C++ Source -> CppLexer -> CppParser -> CppAST -> CppToIR -> Program -> x86-64 -> PE/ELF`

### Features soportados

- **OOP**: classes, herencia, virtual, override, constructors, destructors
- **Templates**: function templates, class templates, non-type params, defaults
- **Namespaces**: anidados, using declarations, using namespace
- **Modern C++**: auto, constexpr, nullptr, enum class, range-for
- **Lambdas**: captures (by value, by ref, this), params, return type
- **Casts**: static_cast, dynamic_cast, const_cast, reinterpret_cast, C-style
- **Exception handling**: try/catch/throw (eliminados a error codes)
- **Operator overloading**: todos los operadores comunes
- **Smart pointers**: unique_ptr, shared_ptr -> raw pointers (zero overhead)
- **STL types**: string, vector, map, optional, variant (reconocidos)
- **C++20**: spaceship operator, concepts, coroutines (parsed)

### Archivos C++

| Archivo | Descripcion |
|---------|-------------|
| `hello.cpp` | Hello World C++ |
| `cpp_oop.cpp` | Classes, herencia, virtual, override, constructors |
| `cpp_templates.cpp` | Function/class templates, namespaces, recursion |
| `cpp_modern.cpp` | auto, constexpr, nullptr, enum class, type aliases |

```bash
adB cxx hello.cpp -o hello.exe
adB cxx cpp_oop.cpp -o oop.exe
adB cxx cpp_templates.cpp -o templates.exe
adB cxx cpp_modern.cpp -o modern.exe
```

Tests: 24 tests (8 lexer + 6 parser + 6 IR + 4 example files) — todos pasan

---

## Modo 3: GPU Compute (SPIR-V)

AST -> SPIR-V directo, sin IR intermedio.

| Archivo | Descripcion | Comando |
|---------|-------------|---------|
| `MODE3_gpu_compute.adB` | Vector add, matmul | `adB gpu ... -o compute.spv` |

---

## Arquitectura del Compilador

```
Codigo fuente (.adB / .c / .cpp)
         |
    Lexer -> Tokens
         |
    Parser -> AST
         |
    Type Checker / IR Converter
         |
    ISA Compiler -> ADeadOp
         |
    Encoder -> x86-64 Bytes
         |
    PE (Windows) / ELF (Linux)
```

## Test Summary

| Frontend | Tests | Status |
|----------|-------|--------|
| ADead-BIB | 40+ | All pass |
| C99 | 55 | All pass |
| C++ | 24 | All pass |
| Total | 87+ frontend | All pass |

---

Sin NASM. Sin LLVM. Sin linker externo.
100% ADead-BIB. Hecho en Peru.

Autor: Eddi Andree Salazar Matos
Version: ADead-BIB v3.5
Licencia: MIT
