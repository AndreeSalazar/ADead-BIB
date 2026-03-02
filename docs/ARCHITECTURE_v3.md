# ADead-BIB v3.0 — C/C++ Native Compiler

> **Sin GCC. Sin LLVM. Sin Clang. 100% ADead-BIB.**

---

## Visión

ADead-BIB v3.0 es un compilador nativo de C/C++ que genera binarios x86-64 directamente, sin dependencias externas.

```
C/C++ Source → Lexer → Parser → AST → IR → ISA Compiler → x86-64 → PE/ELF
```

---

## Estructura del Proyecto

```
src/rust/
├── main.rs              # CLI: adB cc, adB cxx, adB build, adB run
├── builder.rs           # Orchestrator: C/C++ → Binary
├── lib.rs               # Library exports
│
├── frontend/
│   ├── c/               # C99/C11 Frontend
│   │   ├── c_lexer.rs       # Tokenizer
│   │   ├── c_parser.rs      # Recursive descent parser
│   │   ├── c_ast.rs         # C AST types
│   │   ├── c_to_ir.rs       # C AST → ADead-BIB IR
│   │   ├── c_stdlib.rs      # 50+ built-in headers
│   │   └── c_preprocessor.rs
│   │
│   ├── cpp/             # C++11/14/17/20 Frontend
│   │   ├── cpp_lexer.rs     # Tokenizer (80+ keywords)
│   │   ├── cpp_parser.rs    # Classes, templates, lambdas
│   │   ├── cpp_ast.rs       # C++ AST types
│   │   ├── cpp_to_ir.rs     # C++ AST → ADead-BIB IR
│   │   └── cpp_stdlib.rs
│   │
│   ├── ast.rs           # ADead-BIB IR (Program, Function, Stmt, Expr)
│   ├── types.rs         # Type system
│   └── type_checker.rs  # Static analysis
│
├── isa/                 # ISA Layer (x86-64 codegen)
│   ├── mod.rs           # ADeadOp enum (100+ instructions)
│   ├── isa_compiler.rs  # IR → ADeadOp
│   ├── encoder.rs       # ADeadOp → x86-64 bytes (FASM-inspired)
│   ├── optimizer.rs     # Peephole, DCE, constant folding
│   └── reg_alloc.rs     # Register allocation
│
├── backend/
│   ├── cpu/
│   │   ├── pe.rs            # Windows PE generator
│   │   ├── elf.rs           # Linux ELF generator
│   │   ├── flat_binary.rs   # Raw binary (bootloaders)
│   │   └── pe_tiny.rs       # Minimal PE (<500 bytes)
│   │
│   └── gpu/
│       ├── vulkan.rs        # SPIR-V generator
│       ├── cuda.rs          # CUDA code generator
│       └── unified_pipeline.rs
│
└── optimizer/
    ├── const_fold.rs    # Constant folding
    ├── branch_detector.rs
    └── binary_optimizer.rs
```

---

## Comandos CLI

### Compilación Principal

```bash
# C Compiler
adB cc hello.c                    # → hello.exe
adB cc main.c -o app.exe          # → app.exe

# C++ Compiler
adB cxx main.cpp                  # → main.exe
adB cxx app.cpp -o myapp.exe      # → myapp.exe

# Auto-detect by extension
adB build program.c               # C
adB build program.cpp             # C++
adB program.c                     # Direct compilation

# Build and run
adB run test.c                    # Compile + execute
```

### Binarios Mínimos

```bash
adB nano output.exe 0             # Smallest x64 PE (~1KB)
adB micro output.exe 0            # Sub-256 byte x86 PE
adB vm program.adb 0              # MicroVM bytecode
```

### GPU

```bash
adB gpu                           # Detect GPU, generate shader
adB spirv matmul 1024             # SPIR-V compute shader
adB vulkan                        # Initialize Vulkan runtime
adB cuda matmul 1024              # Generate CUDA code
adB unified vectoradd 1000000     # Auto CPU↔GPU decision
```

---

## Pipeline de Compilación

### 1. Frontend (C/C++ → IR)

```
Source Code
    ↓
Preprocessor (#include, #define)
    ↓
Lexer (tokens)
    ↓
Parser (AST)
    ↓
IR Converter (Program/Function/Stmt/Expr)
```

### 2. Middle-end (Optimización)

```
IR
    ↓
Type Checker
    ↓
Constant Folding
    ↓
Branch Optimization
    ↓
Optimized IR
```

### 3. Backend (IR → Binary)

```
Optimized IR
    ↓
ISA Compiler (ADeadOp stream)
    ↓
ISA Optimizer (peephole, DCE)
    ↓
Encoder (x86-64 bytes)
    ↓
Binary Optimizer
    ↓
PE/ELF Generator
    ↓
Executable
```

---

## Características Soportadas

### C (C99/C11)

- Structs, unions, enums
- Pointers, arrays
- Functions, recursion
- Control flow (if, for, while, switch)
- printf, malloc, free
- 50+ built-in headers

### C++ (C++11/14/17/20)

- Classes with inheritance
- Templates (monomorphized)
- Namespaces
- Lambdas
- Smart pointers → raw pointers
- Exceptions → error codes
- RTTI → eliminated
- STL containers recognized

---

## Formatos de Salida

| Target | Format | Size |
|--------|--------|------|
| Windows | PE (.exe) | ~1.5KB+ |
| Linux | ELF | ~1KB+ |
| Raw | Flat binary | Variable |
| Nano | Minimal PE | ~1KB |
| Micro | x86 PE | <256 bytes |

---

## Tests

```bash
cargo test --lib    # 255 tests
```

---

## Filosofía

> **ADead-BIB no abstrae la máquina, la domestica.**

- **Sin dependencias**: No GCC, no LLVM, no Clang
- **Bytes directos**: El compilador emite x86-64 directamente
- **C/C++ nativo**: Lenguajes probados, sin sintaxis experimental
- **Optimización agresiva**: Constant folding, peephole, DCE

---

**Autor:** Eddi Andreé Salazar Matos  
**Versión:** 3.0 — C/C++ Native Compiler  
**Tests:** 255 passing
