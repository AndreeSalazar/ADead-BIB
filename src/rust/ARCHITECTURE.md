# ADead-BIB Compiler Architecture v4.0

## Inspirado en los Mejores Compiladores del Mundo

Este compilador toma lo mejor de:
- **LLVM**: Sistema de IR y passes de optimización
- **GCC**: Optimizaciones agresivas y soporte multi-target
- **MSVC**: Integración Windows y ABI perfecta
- **FASM**: Generación directa de bytes sin ensamblador externo
- **Rust**: Seguridad de tipos y sistema de módulos

## Arquitectura de 3 Capas

```
┌─────────────────────────────────────────────────────────────────┐
│                        FRONTEND                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   C Lexer   │  │  C++ Lexer  │  │ Preprocessor│              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                     │
│  ┌──────▼──────┐  ┌──────▼──────┐         │                     │
│  │  C Parser   │  │ C++ Parser  │◄────────┘                     │
│  └──────┬──────┘  └──────┬──────┘                               │
│         │                │                                      │
│  ┌──────▼────────────────▼──────┐                               │
│  │         Unified AST          │                               │
│  └──────────────┬───────────────┘                               │
└─────────────────┼───────────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────────┐
│                      MIDDLE-END (IR)                            │
│  ┌──────────────────────────────┐                               │
│  │      AST → IR Lowering       │  (Similar a LLVM IR)          │
│  └──────────────┬───────────────┘                               │
│                 │                                               │
│  ┌──────────────▼───────────────┐                               │
│  │    Optimization Pipeline     │                               │
│  │  ┌─────────────────────────┐ │                               │
│  │  │ Pass 1: Constant Fold   │ │  (GCC -O1)                    │
│  │  │ Pass 2: Dead Code Elim  │ │  (GCC -O1)                    │
│  │  │ Pass 3: Inlining        │ │  (GCC -O2)                    │
│  │  │ Pass 4: Loop Unroll     │ │  (GCC -O3)                    │
│  │  │ Pass 5: Vectorization   │ │  (GCC -O3 + SIMD)             │
│  │  │ Pass 6: Reg Allocation  │ │  (Linear Scan / Graph Color)  │
│  │  └─────────────────────────┘ │                               │
│  └──────────────┬───────────────┘                               │
└─────────────────┼───────────────────────────────────────────────┘
                  │
┌─────────────────▼───────────────────────────────────────────────┐
│                        BACKEND                                  │
│  ┌──────────────────────────────┐                               │
│  │    Target Selection Layer    │                               │
│  │  ┌────────┐ ┌────────┐ ┌───┐ │                               │
│  │  │x86-64  │ │ ARM64  │ │GPU│ │                               │
│  │  └───┬────┘ └───┬────┘ └─┬─┘ │                               │
│  └──────┼──────────┼────────┼───┘                               │
│         │          │        │                                   │
│  ┌──────▼──────────▼────────▼───┐                               │
│  │   FASM-Style Byte Encoder    │  (Bytes directos, sin ASM)    │
│  │  ┌─────────────────────────┐ │                               │
│  │  │ Instruction Tables      │ │  (Como FASM TABLES.INC)       │
│  │  │ ModR/M + SIB Encoding   │ │                               │
│  │  │ REX Prefix Generation   │ │                               │
│  │  │ Multi-pass Optimization │ │  (Short jumps, alignment)     │
│  │  └─────────────────────────┘ │                               │
│  └──────────────┬───────────────┘                               │
│                 │                                               │
│  ┌──────────────▼───────────────┐                               │
│  │     Binary Format Writers    │                               │
│  │  ┌────┐ ┌────┐ ┌─────┐ ┌───┐ │                               │
│  │  │ PE │ │ELF │ │Mach-O│ │Raw│ │                              │
│  │  └────┘ └────┘ └─────┘ └───┘ │                               │
│  └──────────────────────────────┘                               │
└─────────────────────────────────────────────────────────────────┘
```

## Estructura de Directorios

```
src/rust/
├── lib.rs                    # Exports públicos
├── main.rs                   # CLI driver
│
├── frontend/                 # FRONTEND - Parsing
│   ├── mod.rs
│   ├── common/               # Compartido entre C y C++
│   │   ├── mod.rs
│   │   ├── source.rs         # Source location tracking
│   │   ├── diagnostics.rs    # Error/warning reporting (como Clang)
│   │   └── preprocessor.rs   # Unified C/C++ preprocessor
│   │
│   ├── c/                    # C Frontend (C99/C11/C17)
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   ├── sema.rs           # Semantic analysis
│   │   └── stdlib.rs         # C standard library stubs
│   │
│   ├── cpp/                  # C++ Frontend (C++11/14/17/20)
│   │   ├── mod.rs
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   ├── sema.rs           # Semantic analysis + templates
│   │   ├── mangler.rs        # Name mangling (Itanium ABI)
│   │   └── stdlib.rs         # C++ standard library stubs
│   │
│   └── ast/                  # Unified AST
│       ├── mod.rs
│       ├── types.rs          # Type system
│       ├── expr.rs           # Expressions
│       ├── stmt.rs           # Statements
│       └── decl.rs           # Declarations
│
├── middle/                   # MIDDLE-END - IR & Optimization
│   ├── mod.rs
│   │
│   ├── ir/                   # Intermediate Representation
│   │   ├── mod.rs
│   │   ├── module.rs         # IR Module (like LLVM Module)
│   │   ├── function.rs       # IR Function
│   │   ├── basicblock.rs     # Basic blocks (CFG)
│   │   ├── instruction.rs    # IR Instructions
│   │   ├── types.rs          # IR Type system
│   │   ├── value.rs          # SSA Values
│   │   └── builder.rs        # IR Builder (like LLVM IRBuilder)
│   │
│   ├── lowering/             # AST → IR
│   │   ├── mod.rs
│   │   ├── c_lower.rs        # C AST → IR
│   │   └── cpp_lower.rs      # C++ AST → IR
│   │
│   ├── analysis/             # Program Analysis
│   │   ├── mod.rs
│   │   ├── cfg.rs            # Control Flow Graph
│   │   ├── domtree.rs        # Dominator Tree
│   │   ├── loops.rs          # Loop detection
│   │   ├── alias.rs          # Alias analysis
│   │   └── liveness.rs       # Liveness analysis
│   │
│   └── passes/               # Optimization Passes (LLVM-style)
│       ├── mod.rs
│       ├── pass_manager.rs   # Pass scheduling
│       │
│       ├── transform/        # Transformation passes
│       │   ├── mod.rs
│       │   ├── dce.rs        # Dead Code Elimination
│       │   ├── constfold.rs  # Constant Folding
│       │   ├── inline.rs     # Function Inlining
│       │   ├── mem2reg.rs    # Memory to Register (SSA)
│       │   ├── gvn.rs        # Global Value Numbering
│       │   ├── licm.rs       # Loop Invariant Code Motion
│       │   ├── unroll.rs     # Loop Unrolling
│       │   ├── vectorize.rs  # Auto-vectorization
│       │   └── tailcall.rs   # Tail Call Optimization
│       │
│       └── codegen/          # Codegen preparation
│           ├── mod.rs
│           ├── legalize.rs   # Legalize for target
│           ├── isel.rs       # Instruction Selection
│           └── regalloc.rs   # Register Allocation
│
├── backend/                  # BACKEND - Code Generation
│   ├── mod.rs
│   │
│   ├── target/               # Target Descriptions
│   │   ├── mod.rs
│   │   ├── x86_64/           # x86-64 Target
│   │   │   ├── mod.rs
│   │   │   ├── abi.rs        # Calling conventions (Win64/SysV)
│   │   │   ├── registers.rs  # Register definitions
│   │   │   └── features.rs   # CPU features (SSE, AVX, etc.)
│   │   │
│   │   ├── arm64/            # ARM64 Target (future)
│   │   │   └── mod.rs
│   │   │
│   │   └── gpu/              # GPU Targets
│   │       ├── mod.rs
│   │       ├── spirv.rs      # SPIR-V generation
│   │       └── cuda.rs       # CUDA/PTX generation
│   │
│   ├── encoder/              # FASM-Style Instruction Encoder
│   │   ├── mod.rs
│   │   ├── x86_64/           # x86-64 Encoder
│   │   │   ├── mod.rs
│   │   │   ├── tables.rs     # Instruction tables (FASM TABLES.INC)
│   │   │   ├── modrm.rs      # ModR/M + SIB encoding
│   │   │   ├── rex.rs        # REX prefix generation
│   │   │   ├── vex.rs        # VEX/EVEX for AVX
│   │   │   ├── encode.rs     # Main encoder
│   │   │   └── reloc.rs      # Relocations
│   │   │
│   │   └── multipass.rs      # Multi-pass optimization (short jumps)
│   │
│   └── format/               # Binary Format Writers
│       ├── mod.rs
│       ├── pe.rs             # Windows PE/COFF
│       ├── elf.rs            # Linux ELF
│       ├── macho.rs          # macOS Mach-O (future)
│       └── raw.rs            # Raw binary (bootloaders)
│
├── driver/                   # Compilation Driver
│   ├── mod.rs
│   ├── session.rs            # Compilation session
│   ├── config.rs             # Compiler configuration
│   └── pipeline.rs           # Full compilation pipeline
│
└── support/                  # Support Libraries
    ├── mod.rs
    ├── arena.rs              # Memory arena allocator
    ├── interner.rs           # String interning
    └── timing.rs             # Compilation timing
```

## IR Design (Inspirado en LLVM)

```rust
// Ejemplo de IR generado
fn @main() -> i32 {
entry:
    %0 = alloca i32              ; int x
    store i32 5, %0              ; x = 5
    %1 = load i32, %0            ; load x
    %2 = add i32 %1, 10          ; x + 10
    ret i32 %2
}
```

## Optimization Levels (Como GCC)

- **-O0**: Sin optimización (debug)
- **-O1**: Optimizaciones básicas (DCE, const fold)
- **-O2**: Optimizaciones estándar (inline, GVN, LICM)
- **-O3**: Optimizaciones agresivas (unroll, vectorize)
- **-Os**: Optimizar para tamaño
- **-Ofast**: Máxima velocidad (puede romper IEEE float)

## FASM-Style Encoding

El encoder genera bytes directamente sin ensamblador externo:

```rust
// Ejemplo: MOV RAX, RBX
fn encode_mov_rr(dst: Reg, src: Reg) -> Vec<u8> {
    let rex = rex_wrxb(true, src.needs_rex(), false, dst.needs_rex());
    let modrm = modrm_rr(src.code(), dst.code());
    vec![rex, 0x89, modrm]  // REX.W + MOV r/m64, r64
}
```

## Calling Conventions

### Windows x64 (MSVC ABI)
- Args: RCX, RDX, R8, R9, stack
- Return: RAX (int), XMM0 (float)
- Shadow space: 32 bytes
- Callee-saved: RBX, RBP, RDI, RSI, R12-R15

### System V AMD64 (Linux/macOS)
- Args: RDI, RSI, RDX, RCX, R8, R9, stack
- Return: RAX (int), XMM0 (float)
- Red zone: 128 bytes
- Callee-saved: RBX, RBP, R12-R15

## Herencia de Rust

El compilador está escrito en Rust y hereda:
- **Ownership**: Gestión de memoria sin GC
- **Pattern Matching**: Para AST y IR
- **Enums**: Para representar instrucciones
- **Traits**: Para polimorfismo de targets
- **Modules**: Para organización clara

## Roadmap

1. **v4.0**: Reorganización completa (actual)
2. **v4.1**: IR completo con SSA
3. **v4.2**: Passes de optimización
4. **v4.3**: ARM64 backend
5. **v5.0**: Self-hosting (compilar ADead-BIB con ADead-BIB)
