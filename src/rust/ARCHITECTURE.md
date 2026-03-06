# ADead-BIB Compiler Architecture v5.0

> Grace Hopper: 'la maquina sirve al humano'
> Dennis Ritchie: 'small is beautiful'
> Ken Thompson: 'trust only code you created'
> Bjarne Stroustrup: 'within C++ a smaller cleaner language struggles to get out'
> ADead-BIB 2026: cumple los 5 💀🦈 🇵🇪

## Pipeline Completo

```
C99/C++98 codigo fuente
        │
        ▼
[ PREPROCESSOR ]  ←── preprocessor/
  header_main.h resolution
  fastos.bib cache (CACHE HIT = nanosegundos)
  symbol deduplication
  C++11-C++17 → C++98 expansion
        │
        ▼
[ PARSER / AST ]  ←── frontend/c/ + frontend/cpp/
  C99 parser separado
  C++98 parser separado
  tipos resueltos
        │
        ▼
[ IR — ADeadOp ]  ←── middle/ir/
  AST → operaciones abstractas
  tipos explicitos, flujo claro
        │
        ▼
[ UB_DETECTOR ]  ←── middle/ub_detector/  (UNICO EN EL MUNDO)
  Analiza IR completo ANTES de codegen
  13 tipos de UB detectados
  Modo Estricto (default) → SE DETIENE
  --warn-ub → avisa y continua
        │
        ▼
[ OPTIMIZER ]  ←── optimizer/
  Dead code elimination, Constant folding
  Constant propagation, Redundant ops removal
  Inline expansion — SIN explotar UB
        │
        ▼
[ REGISTER ALLOCATOR ]  ←── isa/reg_alloc.rs
  IR variables → registros fisicos x86-64
  Linear Scan (v1), Graph Coloring (v2 futuro)
        │
        ▼
[ ISA COMPILER ]  ←── isa/
  c_isa.rs  → C99 layout/sizeof/alignment
  cpp_isa.rs → C++98 vtable/this/constructors
  encoder.rs → bytes x86-64 directos
        │
        ▼
[ OUTPUT ]  ←── output/
  --target fastos  → .po  (24 bytes header)
  --target windows → .exe (PE)
  --target linux   → .elf (ELF)
```

## Estructura de Directorios (Real)

```
src/rust/
├── lib.rs                    # Exports publicos + re-exports
├── main.rs                   # CLI: adeadc cc/cxx/build/run
├── builder.rs                # Orchestrador del pipeline
│
├── preprocessor/             # Sin CMake, Sin Linker
│   ├── mod.rs
│   ├── resolver.rs           # Header resolution automatica
│   ├── dedup.rs              # Symbol Table deduplication
│   └── expander.rs           # C++11-C++17 → C++98 canon
│
├── frontend/                 # C/C++ Parsing
│   ├── mod.rs
│   ├── ast.rs                # Unified AST (Program, Stmt, Expr)
│   ├── types.rs              # Type system
│   ├── type_checker.rs       # Static analysis
│   ├── c/                    # C99 Frontend
│   │   ├── c_lexer.rs
│   │   ├── c_parser.rs
│   │   ├── c_ast.rs
│   │   ├── c_preprocessor.rs
│   │   ├── c_stdlib.rs
│   │   └── c_to_ir.rs
│   └── cpp/                  # C++98 Frontend
│       ├── cpp_lexer.rs
│       ├── cpp_parser.rs
│       ├── cpp_ast.rs
│       ├── cpp_preprocessor.rs
│       ├── cpp_stdlib.rs
│       └── cpp_to_ir.rs
│
├── middle/                   # IR + UB Detection + Analysis
│   ├── mod.rs
│   ├── ir/                   # ADeadOp IR (SSA-form)
│   │   ├── module.rs
│   │   ├── function.rs
│   │   ├── basicblock.rs
│   │   ├── instruction.rs
│   │   ├── types.rs
│   │   ├── value.rs
│   │   └── builder.rs
│   │
│   ├── ub_detector/          # UB Detection (UNICO EN EL MUNDO)
│   │   ├── mod.rs            # Orquesta todos los detectores
│   │   ├── null_check.rs     # NullPointerDereference
│   │   ├── bounds_check.rs   # ArrayOutOfBounds
│   │   ├── overflow_check.rs # IntegerOverflow/Underflow/DivByZero
│   │   ├── uninit_check.rs   # UninitializedVariable
│   │   ├── useafter_check.rs # UseAfterFree, DanglingPointer
│   │   ├── type_check.rs     # TypeConfusion, InvalidCast
│   │   ├── race_check.rs     # DataRace, StackOverflow
│   │   ├── lifetime.rs       # Lifetime analysis
│   │   ├── report.rs         # UBReport, UBKind (14 tipos)
│   │   ├── cache.rs          # UB results para fastos.bib
│   │   └── analyzer.rs       # Coordinator general
│   │
│   ├── analysis/             # CFG, Dominators, Loops
│   ├── lowering/             # AST → IR
│   └── passes/               # Optimization passes (LLVM-style)
│
├── optimizer/                # AST-level + Binary-level optimizations
│   ├── mod.rs
│   ├── dead_code.rs          # Dead code elimination
│   ├── const_fold.rs         # Constant folding
│   ├── const_prop.rs         # Constant propagation
│   ├── redundant.rs          # Redundant ops removal
│   ├── inline_exp.rs         # Inline expansion
│   ├── binary_optimizer.rs   # Binary-level size optimization
│   ├── branch_detector.rs    # Branch pattern detection
│   ├── branchless.rs         # Branchless transforms
│   └── simd.rs               # SIMD code generation
│
├── isa/                      # ISA Layer — x86-64
│   ├── mod.rs                # Reg, ADeadOp, ADeadIR
│   ├── c_isa.rs              # C99: sizeof, alignment
│   ├── cpp_isa.rs            # C++98: vtable, this, ctors
│   ├── isa_compiler.rs       # Main ISA compiler
│   ├── encoder.rs            # ADeadOp → bytes x86-64
│   ├── decoder.rs            # bytes → ADeadOp
│   ├── reg_alloc.rs          # Register Allocator
│   ├── optimizer.rs          # ISA-level peephole opts
│   └── compiler/             # Modular compilation stages
│
├── cache/                    # fastos.bib System
│   ├── mod.rs                # ADeadCache struct
│   ├── serializer.rs         # Cache → bytes
│   ├── deserializer.rs       # bytes → Cache
│   ├── hasher.rs             # FNV-1a header hashing
│   └── validator.rs          # Cache hit/stale/miss/corrupt
│
├── output/                   # Binary Output Formats
│   ├── mod.rs                # OutputFormat enum
│   ├── pe.rs                 # Windows PE (.exe)
│   ├── elf.rs                # Linux ELF
│   └── po.rs                 # FastOS .Po (24-byte header)
│
├── backend/                  # Low-level code generation
│   ├── cpu/                  # x86-64 backends (PE, ELF, flat)
│   └── gpu/                  # GPU backends (Vulkan, CUDA, HIP)
│
├── bg/                       # Binary Guardian (security)
├── runtime/                  # CPU/GPU detection + dispatch
└── toolchain/                # GCC/LLVM/MSVC compatibility
```

## Los 14 Tipos de UB Detectados

```rust
pub enum UBKind {
    NullPointerDereference,   // ptr usado sin check NULL
    UseAfterFree,             // ptr usado despues de free()
    DoubleFree,               // free() llamado dos veces
    DanglingPointer,          // ptr a stack variable fuera de scope
    ArrayOutOfBounds,         // index >= size
    IntegerOverflow,          // signed int overflow
    IntegerUnderflow,         // signed int underflow
    DivisionByZero,           // division por cero
    ShiftOverflow,            // shift >= sizeof(tipo) * 8
    UninitializedVariable,    // variable usada sin inicializar
    TypeConfusion,            // cast invalido entre tipos
    InvalidCast,              // downcast sin verificar
    DataRace,                 // acceso concurrente sin sync
    StackOverflow,            // recursion infinita
}
```

## Modos de Operacion

- **Modo Estricto** (default): UB encontrado → SE DETIENE
- **--warn-ub**: UB encontrado → AVISA Y CONTINUA (tu responsabilidad)

## Orden Critico del Pipeline

```
IR → UB_Detector → Optimizer → Reg_Allocator → Encoder
         ↑
   ANTES de optimizar = cobertura total garantizada
   Si fuera DESPUES, el optimizer podria eliminar checks
   = exactamente lo que hace GCC ❌
   ADead-BIB: UB primero, optimizacion despues ✓
```
