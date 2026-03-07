# ADead-BIB Compiler Architecture v6.0

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
  C++11/C++14/C++17 completo → C++98 canon (34 features)
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
  21 tipos de UB detectados (UNICO EN EL MUNDO)
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
  IR variables → 13 registros fisicos x86-64 (R0-R15)
  TempAllocator (fast) + LinearScanAllocator (liveness)
  Spill automático con stack alignment 16-byte
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
│   └── expander.rs           # C++11/C++14/C++17 completo → C++98 canon (34 features)
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
│   ├── ub_detector/          # UB Detection — 21 tipos (UNICO EN EL MUNDO)
│   │   ├── mod.rs            # Orquesta 10 sub-analizadores
│   │   ├── null_check.rs     # NullPtr + malloc/calloc/realloc tracking
│   │   ├── bounds_check.rs   # ArrayOutOfBounds + negative index
│   │   ├── overflow_check.rs # Overflow/Underflow/DivByZero/ShiftOverflow
│   │   ├── uninit_check.rs   # UninitializedVariable (flow-sensitive)
│   │   ├── useafter_check.rs # UseAfterFree + DanglingPtr + ReturnLocalAddr
│   │   ├── type_check.rs     # TypeConfusion + StrictAliasing + InvalidCast
│   │   ├── race_check.rs     # StackOverflow (recursion sin base case)
│   │   ├── unsequenced_check.rs # UnsequencedModification (i=i++)
│   │   ├── lifetime.rs       # DoubleFree + lifetime analysis
│   │   ├── report.rs         # UBReport, UBKind (21 tipos)
│   │   ├── cache.rs          # UB results cacheados en fastos.bib
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
│   ├── reg_alloc.rs          # TempAllocator + LinearScanAllocator (13 regs)
│   ├── optimizer.rs          # ISA-level peephole opts
│   └── compiler/             # Modular compilation stages
│
├── cache/                    # fastos.bib System
│   ├── mod.rs                # ADeadCache struct
│   ├── serializer.rs         # Cache → bytes (tipos + simbolos + UB completo)
│   ├── deserializer.rs       # bytes → Cache (roundtrip completo v2)
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

## Los 21 Tipos de UB Detectados (UNICO EN EL MUNDO)

```rust
pub enum UBKind {
    // ── Memoria ──
    NullPointerDereference,     // ptr usado sin check NULL (+ malloc tracking)
    UseAfterFree,               // ptr usado despues de free()
    DoubleFree,                 // free() llamado dos veces
    DanglingPointer,            // ptr a stack variable fuera de scope
    ReturnLocalAddress,         // return &local_var (dangling on return)
    BufferOverflow,             // write past buffer end (memcpy, strcpy)

    // ── Aritmetica ──
    ArrayOutOfBounds,           // index >= size (+ negative index)
    IntegerOverflow,            // signed int overflow [C99 §6.5.5]
    IntegerUnderflow,           // signed int underflow
    DivisionByZero,             // division por cero [C99 §6.5.5]
    ShiftOverflow,              // shift >= sizeof(tipo) * 8 [C99 §6.5.7]
    SignedOverflowPromotion,    // char→int promotion causes overflow

    // ── Tipos ──
    UninitializedVariable,      // variable usada sin inicializar
    TypeConfusion,              // cast invalido entre tipos
    InvalidCast,                // downcast sin verificar
    StrictAliasingViolation,    // type punning via pointer cast [C99 §6.5/7]
    AlignmentViolation,         // misaligned pointer cast

    // ── Concurrencia ──
    DataRace,                   // acceso concurrente sin sync
    UnsequencedModification,    // i = i++ (orden no definido) [C99 §6.5/2]
    StackOverflow,              // recursion infinita

    // ── Formato ──
    FormatStringMismatch,       // printf("%d", float_var)
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

## C++11/C++14/C++17 → C++98 Canon (34 Features)

```
MacroExpander expande syntax moderno a C++98 internamente.
El parser solo necesita entender C++98. Zero overhead.

C++11 (12 features):
  lambda, range-for, auto, nullptr, static_assert,
  enum class, using alias, variadic templates,
  constexpr functions, move semantics,
  initializer_list, delegating constructors

C++14 (6 features):
  generic lambda, [[deprecated]], binary literals,
  digit separators, return type deduction, make_unique

C++17 (14 features):
  structured bindings, if constexpr, std::optional,
  std::variant, std::string_view, std::any,
  fold expressions, [[nodiscard]], [[maybe_unused]],
  [[fallthrough]], nested namespaces, inline variables,
  constexpr if (scoped), type traits check

Total: 34 features expandidas a C++98 canon puro
```

## Register Allocator — Dual Mode

```
1. TempAllocator (v1 — rapido)
   - 13 registros: RBX,RCX,RDX,RSI,RDI,R8-R15
   - 5 callee-saved: RBX,R12,R13,R14,R15
   - Spill a stack cuando se agotan registros
   - Windows x64 args: RCX,RDX,R8,R9
   - Linux x64 args: RDI,RSI,RDX,RCX,R8,R9

2. LinearScanAllocator (v2 — liveness)
   - Intervalos de vida por variable
   - Spill del intervalo que termina mas tarde
   - Stack alignment 16 bytes automatico
   - Metricas: spill_slots_used, spill_stack_size

3. StackFrame calculator
   - Calcula tamaño real de frame (no fijo 128)
   - Alignment natural por tipo (1/2/4/8 bytes)
   - Aligned total a 16 bytes (x64 ABI)
```

## Cache fastos.bib v2 — Serialización Completa

```
Header (28 bytes):
  magic: "ADEAD.BI" (8 bytes)
  version: u32 (4 bytes) — v2
  timestamp: u64 (8 bytes)
  hash: u64 (8 bytes) — FNV-1a del source

Body:
  AST data (length-prefixed blob)
  TypeTable (count + entries con kind/size/alignment)
  SymbolTable (count + entries con kind)
  UB Reports (count + cached results)

Validación: Hit/Stale/Miss/Corrupt
Hash cambia → Stale → recompila → nuevo cache
```
