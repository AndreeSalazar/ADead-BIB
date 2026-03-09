# ADead-BIB v7.0 💀🦈

**No necesitas ser genio para ser un gran programador, con tan solo Aprendiendo la base de C y C++
Ya eres capaz de construir aplicaciones, cualquier aplicaciones generales o videojuegos**

**Compilador Nativo: C99 · C++17 → Machine Code Puro**

> Zero Overhead · Zero Bloat · Zero Dead Code  
> Sin NASM · Sin LLVM · Sin GCC · Sin Clang  
> Sin libc externa · Sin linker · 100% Autosuficiente  
> FASM-style: bytes directos al CPU  
> `#include <header_main.h>` = TODO disponible

```mermaid
graph TD
    A[Tu Código .c/.cpp] --> B[ADead-BIB Compiler (adb)]
    B --> C[CPreprocessor/C++Preprocessor]
    C --> D[CLexer/CppLexer]
    D --> E[CParser/CppParser]
    E --> F[CAST/CppAST]
    F --> G[CToIR/CppToIR]
    G --> H[Program (IR)]
    H --> I[IsaCompiler]
    I --> J[ADeadOp stream]
    J --> K[Machine Code]
    K --> L[Archivo ejecutable]
```

┌──────────────────────────────────────────────────────────────────────┐
│  Tu Código (.c / .cpp)                                               │
│                    ↓                                                 │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │              ADead-BIB Compiler (adb)                          │  │
│  │                                                                │  │
│  │   .c   ──→ CPreprocessor → CLexer → CParser → CAST ──┐         │  │
│  │   .cpp ──→ CppPreprocessor → CppLexer → CppParser ───┤         │  │
│  │                                                       ↓        │  │
│  │                                              CToIR / CppToIR   │  │
│  │                                                       ↓        │  │
│  │                                              Program (IR)      │  │
│  │                                                       ↓        │  │
│  │                                              IsaCompiler       │  │
│  │                                              (ADeadOp stream)  │  │
│  │                                                       ↓        │  │
│  │                                              Optimizer         │  │
│  │                                              (DCE, Fold,       │  │
│  │                                               Inline, Peep)    │  │
│  │                                                       ↓        │  │
│  │                                              Encoder           │  │
│  │                                              (FASM-style,      │  │
│  │                                               x86-64 bytes)    │  │
│  │                                                       ↓        │  │
│  │                                              PE / ELF / Raw    │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                    ↓                                                 │
│             .exe / .elf / .bin                                       │
│           (Machine Code Puro)                                        │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Filosofía: ¿Por Qué Existe ADead-BIB?

### El Problema Real de MSVC, GCC y LLVM

Los compiladores industriales (MSVC, GCC, Clang/LLVM) son **referencias técnicas invaluables** — definieron cómo se compila C y C++ a machine code durante décadas. ADead-BIB los estudia, los respeta, y toma sus decisiones de ABI y calling convention como referencia:

| Referencia | Lo que ADead-BIB toma | Lo que ADead-BIB rechaza |
|------------|----------------------|--------------------------|
| **MSVC** | Windows x64 ABI (RCX,RDX,R8,R9), shadow space 32 bytes, PE format | Runtime de >100 KB, CRT implícito, excepciones SEH |
| **GCC** | System V AMD64 ABI (RDI,RSI,RDX,RCX), ELF format, optimizaciones agresivas | Múltiples backends indirectos, código generado inflado |
| **LLVM** | Concepto de IR intermedio, passes de optimización, instruction selection | IR genérico que no llega a bytes directos, overhead de abstracción |
| **FASM** | **Generación directa de bytes sin ensamblador externo** — el encoder de ADead-BIB es FASM-style | — (FASM es la referencia que ADead-BIB sigue fielmente) |

**El resultado:** ADead-BIB genera binarios de **2-10 KB** donde GCC genera **50+ KB** y MSVC genera **100+ KB** para el mismo programa.

### Canon: C99 y C++98 — Las Intenciones Claras

ADead-BIB compila **C99** y **C++98** como estándares canónicos porque representan las intenciones más claras y absolutas de estos lenguajes:

**C99 — El Canon de C:**
- Lo que C99 especifica, el programador lo quiere tal cual
- `int` = 32 bits, `char` = 8 bits, `long long` = 64 bits — tamaños exactos
- Punteros = direcciones reales de memoria, no abstracciones
- `malloc/free` = control manual, sin garbage collector
- `struct` = layout en memoria exacto, sin padding oculto en `@packed`
- Arrays = acceso directo a memoria contigua, `arr[i]` = `*(arr + i * sizeof(element))`
- **Intención C99:** El programador sabe exactamente qué bytes genera cada línea

**C++98 — El Canon de C++:**
- Classes = structs con métodos, vtable solo cuando hay `virtual`
- Templates = monomorphización — solo se genera código para instancias usadas
- Namespaces = organización, cero costo en runtime
- Constructores/Destructores = RAII, pero sin overhead de excepciones
- **Intención C++98:** Zero overhead principle — lo que no usas, no pagas

### Rust como Guardián — Por Qué ADead-BIB Está Escrito en Rust

ADead-BIB está escrito en **Rust** por una razón técnica precisa: Rust es el **guardián** que detecta los problemas que C y C++ no pueden ver en sí mismos:

| Problema en C/C++ | Rust avisa porque... | ADead-BIB lo usa para... |
|-------------------|---------------------|--------------------------|
| Buffer overflow | Ownership + bounds checking | Que el compilador mismo nunca crashee |
| Use-after-free | Borrow checker | Gestión segura del AST y IR en memoria |
| Data races | Send + Sync traits | Compilación paralela sin bugs |
| Null pointer | Option\<T\> obligatorio | Cada nodo del AST tiene tipo garantizado |
| Memory leaks | RAII + Drop | El compilador limpia toda memoria automáticamente |

**Rust no es el lenguaje que ADead-BIB compila — es el lenguaje que garantiza que ADead-BIB compile correctamente.** El resultado final es machine code puro de C/C++, pero construido por un compilador que nunca tiene undefined behavior.

### Eliminación Absoluta: Overhead, Bloat, Dead Code

ADead-BIB tiene una política de **eliminación total** — todo lo que no contribuye a la ejecución final se elimina:

```
ANTES (lo que el programador escribe):
  - Exceptions try/catch/throw
  - RTTI (typeid, dynamic_cast runtime)
  - Smart pointers (unique_ptr, shared_ptr)
  - STL containers overhead (allocator, iterator wrappers)
  - Funciones no llamadas
  - Variables no leídas
  - Branches inalcanzables

DESPUÉS (lo que ADead-BIB genera):
  - Exceptions → error codes (cero stack unwinding)
  - RTTI → eliminado (si no se usa, no existe)
  - Smart pointers → raw pointers (mismo layout, cero reference counting)
  - STL → inlined (solo las operaciones realmente usadas)
  - Dead functions → eliminadas por DCE
  - Dead variables → eliminadas por DCE
  - Dead branches → eliminadas por constant folding
```

**Machine Code Puro** = solo las instrucciones x86-64 que el CPU necesita ejecutar. Nada más.

---

## Instalación

```bash
# 1. Clonar y compilar
git clone https://github.com/AndreeSalazar/ADead-BIB.git
cd ADead-BIB
cargo build --release

# 2. Agregar adb al PATH
#    Windows (PowerShell):
$env:Path += ";C:\ruta\a\ADead-BIB\target\release"
#    Para hacerlo permanente (Admin):
[Environment]::SetEnvironmentVariable('Path', $env:Path + ';C:\ruta\a\ADead-BIB\target\release', 'User')

#    Linux / macOS:
export PATH="$PATH:$HOME/ADead-BIB/target/release"
#    Para hacerlo permanente:
echo 'export PATH="$PATH:$HOME/ADead-BIB/target/release"' >> ~/.bashrc

#    FastOS: No necesita PATH — adb es nativo del sistema

# 3. Instalar headers globales
adb install

# 4. Verificar
adb --version
```

> **Nota:** `adb --version` muestra la ruta exacta y las instrucciones de PATH para tu sistema.

---

## Inicio Rápido

```bash
# ── Crear proyecto (como cargo new) ───────────────
adb create hola              # Proyecto C
adb create mundo --cpp       # Proyecto C++

# ── Compilar y ejecutar proyecto ──────────────────
cd hola
adb run                      # Compila src/main.c → bin/hola.exe y ejecuta
# → "Hola desde hola"

# ── Compilar archivos sueltos ─────────────────────
adb cc hello.c -o hello.exe
adb cxx app.cpp -o app.exe
adb run test.c               # Compila y ejecuta

# ── GPU (SPIR-V directo) ─────────────────────────
adb gpu

# ── Step Compiler (ver cada paso) ────────────────
adb step main.c              # Visualiza el pipeline completo
```

### Estructura de un Proyecto (`adb create`)

```
hola/
├── adb.toml           # Configuración del proyecto
│     [project]
│     name = "hola"
│     version = "0.1.0"
│     lang = "c"       # o "cpp"
│     standard = "c99" # o "cpp17"
│
│     [build]
│     src = "src/"
│     include = "include/"
│     output = "bin/"
│
├── include/
│   └── header_main.h   ← todo disponible
├── src/
│   └── main.c          ← tu código
└── bin/                 ← output de compilación
```

### Resolución de Headers (sin flags -I)

Cuando escribes `#include <header.h>`, ADead-BIB busca en este orden:

1. **Carpeta `include/` del proyecto** — headers locales
2. **`~/.adead/include/`** — headers globales (`adb install`)
3. **stdlib interna** — C99/C++ completa (fallback)

= Sin `-I flags`, sin CMake, sin Makefile. Encuentra solo.

---

## Step Compiler — Ve Exactamente Qué Hace el Compilador

```bash
adb step main.c
```

Muestra **cada fase del pipeline**, paso por paso — como leer el compilador en tiempo real:

```
[SOURCE]   12 lines, 245 bytes

--- Phase 1: PREPROCESSOR ---
[PREPROC]  165 lines after preprocessing
[PREPROC]  #include <stdio.h> -> resolved internally

--- Phase 2: LEXER ---
[LEXER]    78 tokens generated
[LEXER]       1:0    Int                                        OK
[LEXER]       1:1    Identifier("main")                         OK
[LEXER]       1:2    LParen                                     OK

--- Phase 3: PARSER ---
[PARSER]   function 'main' (0 params, 3 stmts) OK
[PARSER]   Total: 1 functions, 0 structs, 28 typedefs

--- Phase 4: IR (Intermediate Representation) ---
[IR]       function 'main' -> 5 IR statements OK
[IR]         VarDecl { var_type: I32, name: "x", value: Some(Number(42)) }
[IR]         Println(String("Hello"))

--- Phase 5: UB DETECTOR ---
[UB]       No undefined behavior detected OK

--- Phase 6: CODEGEN (x86-64) ---
[CODEGEN]  127 bytes of machine code generated
[CODEGEN]  First 16 bytes:
[CODEGEN]    E9 00 00 00 00 55 48 89 E5 53 41 54 56 57 48 81
[CODEGEN]  Data section strings:
[CODEGEN]    "Hello"

--- Phase 7: OUTPUT ---
[OUTPUT]   Target: Windows PE x86-64
[OUTPUT]   Code:   127 bytes
[OUTPUT]   Data:   32 bytes
[OUTPUT]   Est. binary: ~1183 bytes
```

**7 fases visibles:** Source → Preprocessor → Lexer → Parser → IR → UB → Codegen → Output

Funciona con C y C++: `adb step archivo.c` o `adb step archivo.cpp`

---

## v7.0 — Autosuficiencia Total

ADead-BIB v7.0 es **100% autosuficiente**:

- **Sin libc externa**: Toda la biblioteca estándar C/C++ está implementada internamente (`stdlib/c/`, `stdlib/cpp/`)
- **Sin linker**: Unity build — todo se compila a un solo IR, un solo binario
- **`#include <header_main.h>`**: Un solo include que da acceso a TODA la stdlib (C y C++)
- **Tree shaking**: Solo las funciones que usas llegan al binario final
- **fastos_*.h**: Headers individuales (`fastos_stdio.h`, `fastos_math.h`, etc.) para control granular

```c
// Un solo include. Todo disponible. Sin linker.
#include <header_main.h>

int main() {
    printf("Hello from ADead-BIB v7.0!\n");
    double s = sin(3.14);
    void *p = malloc(1024);
    free(p);
    return 0;
}
```

---

## Frontends: C99 y C++17

### C99 Frontend — Canon de C

| Característica C99 | Estado | Intención |
|---------------------|--------|-----------|
| Variables y tipos (`int`, `char`, `short`, `long`, `float`, `double`) | ✅ | Tamaños exactos → registros correctos |
| Punteros y aritmética de punteros | ✅ | Direcciones reales, stride por `sizeof` |
| Arrays y acceso directo | ✅ | Memoria contigua, `arr[i]` = `*(arr + i * size)` |
| Structs, unions, typedefs | ✅ | Layout en memoria explícito |
| Enums | ✅ | Constantes enteras |
| Function pointers | ✅ | `call reg` directo |
| Preprocesador (`#include`, `#define`, `#ifdef`) | ✅ | 75+ headers de sistema inyectados |
| Control de flujo (`if`, `for`, `while`, `do-while`, `switch`) | ✅ | Branch → `jcc` directo |
| Recursión | ✅ | Stack frame real por llamada |
| `malloc/free` | ✅ | Enlace directo con Win32 API / Linux syscall |
| Bitwise (`&`, `|`, `^`, `<<`, `>>`, `~`) | ✅ | Instrucciones x86-64 directas |
| Operadores compuestos (`+=`, `-=`, `*=`, `/=`, `<<=`, `>>=`) | ✅ | In-place, sin temporales |

**Pipeline:** `C source → CPreprocessor → CLexer → CParser → CAST → CToIR → Program → IsaCompiler → Encoder → x86-64 → PE/ELF`

### C++17 Frontend — Canon de C++ (Zero Overhead)

| Característica C++ | Estado | Cómo ADead-BIB lo compila |
|---------------------|--------|---------------------------|
| Classes (campos, métodos, constructores, destructores) | ✅ | `struct` + funciones con `this` pointer |
| Herencia (single, multiple) | ✅ | Campos concatenados + vtable si hay `virtual` |
| Virtual functions | ✅ | vtable devirtualizada cuando es posible |
| Templates (function, class) | ✅ | Monomorphización — solo instancias usadas |
| Namespaces (anidados, `using`) | ✅ | Prefijo de nombres, cero costo runtime |
| Operator overloading | ✅ | Inline a instrucciones directas |
| `auto` type deduction | ✅ | Resuelto en compilación |
| `constexpr` | ✅ | Evaluado en compilación → constante |
| `nullptr` | ✅ | `0x0` literal |
| `enum class` | ✅ | Entero con scope |
| Range-for | ✅ | Loop con índice, sin iterador runtime |
| Lambdas | ✅ | Closure inline, captures resueltos |
| Casts (`static_cast`, `dynamic_cast`, `const_cast`, `reinterpret_cast`) | ✅ | Resueltos en compilación o eliminados |
| **Exceptions (try/catch/throw)** | ✅ → eliminados | Convertidos a error codes, cero stack unwinding |
| **Smart pointers (unique_ptr, shared_ptr)** | ✅ → eliminados | Convertidos a raw pointers, cero reference counting |
| **RTTI (typeid, dynamic_cast runtime)** | ✅ → eliminado | Si no se usa, no existe en el binario |

**Pipeline:** `C++ source → CppPreprocessor → CppLexer → CppParser → CppAST → CppToIR → Program → IsaCompiler → Encoder → x86-64 → PE/ELF`

---

## Referencia de Compiladores: ADead-BIB vs MSVC/GCC/LLVM

### Calling Conventions (Referencia directa de MSVC y GCC)

```
Windows x64 (referencia MSVC):
  Args:  RCX, RDX, R8, R9, stack
  Ret:   RAX (int), XMM0 (float)
  Shadow space: 32 bytes
  Callee-saved: RBX, RBP, RDI, RSI, R12-R15

System V AMD64 (referencia GCC):
  Args:  RDI, RSI, RDX, RCX, R8, R9, stack
  Ret:   RAX (int), XMM0 (float)
  Red zone: 128 bytes
  Callee-saved: RBX, RBP, R12-R15
```

ADead-BIB detecta el target automáticamente y usa la convención correcta.

### Encoding: FASM-Style (Bytes Directos)

ADead-BIB genera bytes x86-64 **directamente**, igual que FASM — sin pasar por un ensamblador externo:

```
Instrucción          Bytes generados        Encoding
─────────────────────────────────────────────────────
mov rax, rbx         48 89 D8               REX.W + MOV r/m64, r64
add rax, 42          48 83 C0 2A            REX.W + ADD r/m64, imm8
call printf          E8 xx xx xx xx         CALL rel32
ret                  C3                     RET
push rbp             55                     PUSH r64
sub rsp, 32          48 83 EC 20            SUB r/m64, imm8
xor eax, eax         31 C0                  XOR r32, r32
```

### Optimizaciones (Inspiradas en GCC -O2/-O3 y LLVM Passes)

| Optimización | Referencia | Qué hace ADead-BIB |
|-------------|------------|---------------------|
| Dead Code Elimination | GCC -O1, LLVM `dce` | Elimina funciones, variables y branches no usados |
| Constant Folding | GCC -O1, LLVM `constprop` | `2 + 3 * 4` → `14` en compilación, 0 instrucciones runtime |
| Inlining | GCC -O2, LLVM `inline` | Funciones pequeñas expandidas en el caller |
| Peephole | GCC -O2 | `add reg, 1` → `inc reg`, `mov reg, reg` eliminado |
| Register Allocation | GCC/LLVM `regalloc` | Temporales en R10-R15, minimiza push/pop al stack |
| Strength Reduction | GCC -O2 | `x * 0` → `0`, `x + 0` → `x`, `x * 2` → `shl x, 1` |
| Dynamic Stack Frame | — (ADead-BIB propio) | Stack frame calculado exacto, no 128 bytes fijos |

---

## ISA Layer: ADeadOp → x86-64

El corazón de ADead-BIB es la capa ISA que convierte operaciones abstractas (`ADeadOp`) a bytes x86-64 reales:

| ADeadOp | Descripción | x86-64 Encoding |
|---------|-------------|------------------|
| `Mov { dst, src }` | Mover datos | `89/8B` + ModR/M |
| `Add { dst, src }` | Suma | `01/03` + ModR/M |
| `Sub { dst, src }` | Resta | `29/2B` + ModR/M |
| `Mul { src }` | Multiplicación | `F7 /4` |
| `Div { src }` | División | `F7 /6` |
| `Shl { dst, amount }` | Shift left | `C1 /4 imm8` |
| `Shr { dst, amount }` | Shift right | `C1 /5 imm8` |
| `Cmp { left, right }` | Comparar | `39/3B` |
| `Jmp { target }` | Salto | `EB/E9` |
| `Je/Jne/Jl/Jg` | Saltos condicionales | `74/75/7C/7F` |
| `Call { target }` | Llamar función | `E8 rel32` |
| `Ret` | Retornar | `C3` |
| `Push { src }` | Push stack | `50+r` |
| `Pop { dst }` | Pop stack | `58+r` |
| `Cli` | Desactivar interrupciones | `FA` |
| `Sti` | Activar interrupciones | `FB` |
| `Hlt` | Halt CPU | `F4` |
| `In { port, dst }` | Leer puerto I/O | `E4/EC` |
| `Out { port, src }` | Escribir puerto I/O | `E6/EE` |

---

## Estructura del Proyecto

```
ADead-BIB/
├── src/rust/
│   ├── main.rs                        # CLI driver (adb)
│   ├── lib.rs                         # Exports públicos
│   ├── builder.rs                     # Orchestrator del pipeline
│   ├── errors.json                    # Catálogo de errores
│   │
│   ├── cli/                       # TERMINAL UI
│   │   └── term.rs                    # ANSI colors, phase bars, formatting
│   │
│   ├── frontend/                  # FRONTENDS
│   │   ├── ast.rs                     # IR compartido (Program, Function, Stmt, Expr)
│   │   ├── types.rs                   # Sistema de tipos
│   │   ├── type_checker.rs            # Análisis estático
│   │   ├── lexer.rs                   # Utilidades compartidas de lexer
│   │   ├── parser.rs                  # Utilidades compartidas de parser
│   │   │
│   │   ├── c/                         # C99 Frontend
│   │   │   ├── c_lexer.rs             # Tokenizer C99
│   │   │   ├── c_parser.rs            # Recursive descent C99
│   │   │   ├── c_ast.rs               # C AST types
│   │   │   ├── c_to_ir.rs             # CAST → Program IR
│   │   │   ├── c_preprocessor.rs      # #include/#define/#ifdef
│   │   │   ├── c_stdlib.rs            # 75+ headers built-in
│   │   │   └── c_compiler_extensions.rs # Extensiones de compilador C
│   │   │
│   │   └── cpp/                       # C++17 Frontend
│   │       ├── cpp_lexer.rs           # Tokenizer C++
│   │       ├── cpp_parser.rs          # Classes, templates, namespaces
│   │       ├── cpp_ast.rs             # C++ AST types
│   │       ├── cpp_to_ir.rs           # CppAST → Program IR
│   │       ├── cpp_preprocessor.rs    # C++ preprocessor
│   │       ├── cpp_stdlib.rs          # STL stubs
│   │       └── cpp_compiler_extensions.rs # Extensiones de compilador C++
│   │
│   ├── preprocessor/              # SIN CMAKE, SIN LINKER
│   │   ├── resolver.rs                # Header resolution + linker eliminator
│   │   ├── dedup.rs                   # Symbol deduplication global
│   │   └── expander.rs                # C++17 → C++98 canon (34 features)
│   │
│   ├── stdlib/                    # STANDARD LIBRARY PROPIA
│   │   ├── header_main.rs             # header_main.h — hereda TODO
│   │   ├── canon_tests.rs             # Canon test suite
│   │   ├── fase_tests.rs              # FASE test suite
│   │   ├── integration_tests.rs       # Integration test suite
│   │   ├── c/                         # C99: stdio, stdlib, string, math...
│   │   └── cpp/                       # C++: iostream, vector, map, memory...
│   │
│   ├── middle/                    # MIDDLE-END (IR avanzado)
│   │   ├── ir/                        # SSA IR (module, function, basicblock...)
│   │   │   └── pdp11_heritage.rs      # PDP-11 heritage reference
│   │   ├── ub_detector/               # 21+ tipos de UB detection
│   │   │   ├── null_check.rs          # NullPointerDereference
│   │   │   ├── bounds_check.rs        # ArrayOutOfBounds
│   │   │   ├── overflow_check.rs      # IntegerOverflow/DivByZero/ShiftOverflow
│   │   │   ├── uninit_check.rs        # UninitializedVariable
│   │   │   ├── useafter_check.rs      # UseAfterFree/DanglingPtr
│   │   │   ├── type_check.rs          # TypeConfusion/StrictAliasing
│   │   │   ├── race_check.rs          # StackOverflow/DataRace
│   │   │   ├── unsequenced_check.rs   # UnsequencedModification
│   │   │   ├── lifetime.rs            # DoubleFree/lifetime analysis
│   │   │   ├── format_check.rs        # FormatStringMismatch
│   │   │   ├── cache.rs               # UB results cache
│   │   │   └── report.rs              # UBReport, UBKind
│   │   ├── analysis/                  # CFG, dominator tree, loops
│   │   ├── lowering/                  # AST → IR (c_lower.rs, cpp_lower.rs)
│   │   └── passes/                    # Transform passes (LLVM-style)
│   │       └── transform/             # DCE, GVN, LICM, inline, vectorize...
│   │
│   ├── optimizer/                 # AST-LEVEL OPTIMIZATIONS
│   │   ├── const_fold.rs              # Constant folding
│   │   ├── const_prop.rs              # Constant propagation
│   │   ├── dead_code.rs               # Dead code elimination
│   │   ├── branch_detector.rs         # Branch pattern detection
│   │   ├── branchless.rs              # Branchless transforms
│   │   ├── binary_optimizer.rs        # Binary-level size optimization
│   │   ├── inline_exp.rs              # Inline expansion
│   │   └── simd.rs                    # Auto-vectorization SIMD
│   │
│   ├── isa/                       # ISA LAYER (the core)
│   │   ├── isa_compiler.rs            # Program IR → ADeadOp stream
│   │   ├── c_isa.rs                   # C99 sizeof/alignment rules
│   │   ├── cpp_isa.rs                 # C++98 vtable/this/constructors
│   │   ├── encoder.rs                 # ADeadOp → x86-64 bytes (FASM-style)
│   │   ├── decoder.rs                 # x86-64 bytes → ADeadOp (disassembly)
│   │   ├── optimizer.rs               # Peephole optimization
│   │   ├── reg_alloc.rs               # Register allocator (dual mode)
│   │   ├── codegen.rs                 # Codegen auxiliar
│   │   └── compiler/                  # Modular compilation stages
│   │       ├── expressions.rs         # Expression compilation
│   │       ├── statements.rs          # Statement compilation
│   │       ├── control_flow.rs        # Control flow compilation
│   │       ├── functions.rs           # Function compilation
│   │       └── arrays.rs              # Array compilation
│   │
│   ├── output/                    # BINARY OUTPUT (sin linker)
│   │   ├── pe.rs                      # Windows PE (.exe)
│   │   ├── elf.rs                     # Linux ELF
│   │   └── po.rs                      # FastOS .po (24-byte header)
│   │
│   ├── backend/                   # BACKEND (low-level)
│   │   ├── cpu/                       # x86-64 backends
│   │   │   ├── pe.rs / pe_tiny.rs / pe_compact.rs ...  # Multiple PE generators
│   │   │   ├── elf.rs                 # Linux ELF
│   │   │   ├── flat_binary.rs         # Raw binary (bootloaders, kernels)
│   │   │   ├── os_codegen.rs          # Real/Protected/Long mode
│   │   │   ├── iat_registry.rs        # Import Address Table
│   │   │   ├── microvm.rs             # MicroVM bytecode (4-bit ops)
│   │   │   ├── syscalls.rs            # System calls
│   │   │   └── win32_resolver.rs      # Win32 API resolver
│   │   │
│   │   └── gpu/                       # GPU backends
│   │       ├── vulkan/                # Vulkan backend (ash)
│   │       ├── spirv/                 # SPIR-V bytecode generation
│   │       ├── cuda/                  # CUDA code generation
│   │       ├── hip/                   # HIP (AMD ROCm) support
│   │       ├── hex/                   # Binary GPU hex tools
│   │       ├── unified_pipeline.rs    # CPU↔GPU hybrid auto-dispatch
│   │       ├── vulkan_runtime.rs      # Vulkan runtime
│   │       ├── gpu_detect.rs          # GPU detection
│   │       ├── compute.rs             # GPU compute abstraction
│   │       ├── memory.rs              # GPU memory management
│   │       ├── metrics.rs             # GPU performance metrics
│   │       └── scheduler.rs           # GPU task scheduler
│   │
│   ├── bg/                        # BINARY GUARDIAN (security)
│   │   ├── analyzer.rs                # ISA-level binary analysis
│   │   ├── arch_map.rs                # Architecture capability map
│   │   ├── binary_loader.rs           # Binary loader
│   │   ├── capability.rs              # Capability definitions
│   │   └── policy.rs                  # Security policy engine (APPROVE/DENY)
│   │
│   ├── cache/                     # FASTOS.BIB CACHE v2
│   │   ├── serializer.rs              # Cache → bytes
│   │   ├── deserializer.rs            # bytes → Cache
│   │   ├── hasher.rs                  # FNV-1a hashing
│   │   └── validator.rs               # Cache hit/stale/miss/corrupt
│   │
│   ├── runtime/                   # RUNTIME DETECTION
│   │   ├── cpu_detect.rs              # CPU features (SSE/AVX)
│   │   ├── dispatcher.rs              # CPU compute dispatch
│   │   ├── gpu_dispatcher.rs          # GPU compute dispatch
│   │   └── gpu_misuse_detector.rs     # GPU misuse detection
│   │
│   └── toolchain/                 # TOOLCHAIN COMPATIBILITY
│       ├── calling_conventions.rs     # Win64 + SysV calling conventions
│       ├── gcc_builtins.rs            # __attribute__, __builtin_*
│       ├── gcc_compat.rs              # GCC flag emulation
│       ├── clang_compat.rs            # Clang flag emulation
│       ├── llvm_attrs.rs              # LLVM attributes/intrinsics
│       ├── msvc_compat.rs             # __declspec, MSVC extensions
│       └── cpp_name_mangler.rs        # Itanium ABI name mangling
│
├── examples/
│   ├── c/                         # 34 archivos C99 — todos compilan ✅
│   ├── cpp/                       # 22 archivos C++ — todos compilan ✅
│   ├── boot/                      # Boot sectors, kernels
│   └── gpu/                       # GPU compute shaders
│
├── docs/                          # Documentación técnica
├── BG — Binary Guardian/          # Documentación BG
├── DirectX12/                     # DX12 test projects
├── FastOS/                        # FastOS related
├── FFI GPU/                       # FFI GPU (Python ↔ Vulkan)
├── Metal_Dead/                    # Metal (macOS) backend docs
├── python/                        # Python FFI tools
├── abi_translators/               # ABI translation layer
├── EXTENSION/                     # VS Code extension
├── Test-Canon/                    # Canon verification suite (48 tests)
├── Test-UB-Global/                # Global UB test suite
├── Test SQLite/                   # SQLite compilation tests
├── ub_tests/                      # UB detection tests
├── Cargo.toml                     # 100% Rust, sin deps de C/C++
├── ARCHITECTURE.md                # Arquitectura completa
└── README.md                      # Este archivo
```

---

## Tamaños de Binario (vs MSVC/GCC)

| Programa | ADead-BIB | GCC -Os | MSVC /O1 |
|----------|-----------|---------|----------|
| Hello World | **2.0 KB** | ~50 KB | ~100 KB |
| Counter + printf | **2.0 KB** | ~50 KB | ~100 KB |
| Recursion (fib, power) | **2.5 KB** | ~50 KB | ~100 KB |
| Classes + OOP | **3.0 KB** | ~55 KB | ~110 KB |
| Templates | **3.5 KB** | ~55 KB | ~110 KB |
| Stdlib largo (~100 funcs) | **42 KB** | ~200 KB | ~300 KB |

**¿Por qué?** ADead-BIB no incluye CRT, no incluye exception handling tables, no incluye RTTI tables, no incluye debug info por defecto. Solo machine code puro.

---

## Resultados de Compilación

| Frontend | Archivos | Compilan | Tasa |
|----------|----------|----------|------|
| **C99 examples** | 34 | 34 | **100%** ✅ |
| **C++ examples** | 22 | 22 | **100%** ✅ |
| **C99 Canon** | 18 | 18 | **100%** ✅ |
| **C++98 Canon** | 15 | 15 | **100%** ✅ |
| **Integration tests** | 18 | 18 | **100%** ✅ |
| **FASE tests (C99+C++17+PE)** | 19 | 19 | **100%** ✅ |
| **Total Rust tests** | **539** | **539** | **100%** ✅ |

### Test-Canon Verificado

```
C99 Canon (18 tests): tipos, punteros, arrays, structs, unions,
  enums, typedef, control, funciones, function pointers,
  preprocesador, bitwise, casting, scope, strings, malloc,
  sizeof, expresiones complejas — ALL PASS ✅

C++98 Canon (15 tests): clases, herencia, virtual/polimorfismo,
  templates función, templates clase, namespaces, operator overload,
  referencias, const correctness, encapsulamiento, constructores,
  static members, punteros objetos, enum class, STL — ALL PASS ✅

Integration (18 tests): header_main.h C/C++, fastos_*.h,
  symbol registries, no-linker verification, resolver content,
  full E2E programs — ALL PASS ✅
```

---

## Comandos CLI (`adb`)

```bash
# ── Proyectos (como cargo) ────────────────────────
adb create hola                       # Nuevo proyecto C
adb create hola --cpp                 # Nuevo proyecto C++
adb build                             # Compilar proyecto (lee adb.toml)
adb run                               # Compilar y ejecutar proyecto

# ── C99 ────────────────────────────────────────────
adb cc hello.c -o hello.exe           # Compilar C99
adb cc main.c                         # → main.exe automático

# ── C++ ────────────────────────────────────────────
adb cxx app.cpp -o app.exe            # Compilar C++
adb cxx main.cpp                      # → main.exe automático

# ── Auto-detect ───────────────────────────────────
adb build program.c                   # Detecta .c → C99
adb build program.cpp                 # Detecta .cpp → C++

# ── Build + Run ───────────────────────────────────
adb run test.c                        # Compilar y ejecutar

# ── Headers globales ──────────────────────────────
adb install                           # Instala headers en ~/.adead/include/
adb include                           # Muestra ruta de headers

# ── Flat Binary (OS/Kernel) ──────────────────────
adb cc kernel.c -o kernel.bin --flat
adb cc boot.c -o boot.bin --flat16 --org=0x7C00 --size=512

# ── Binarios Mínimos ─────────────────────────────
adb nano output.exe                   # PE más pequeño posible
adb micro output.exe                  # PE32 < 256 bytes

# ── GPU ───────────────────────────────────────────
adb gpu                               # Detectar GPU + generar shader
adb spirv matmul 1024                 # SPIR-V compute shader

# ── Step Compiler ────────────────────────────────
adb step program.c                    # Visualizar compilación paso a paso

# ── MicroVM ──────────────────────────────────────
adb vm program.c                      # Compilar a MicroVM bytecode (4-bit ops)

# ── Vulkan Runtime ───────────────────────────────
adb vulkan shader.comp                # Compilar + ejecutar con Vulkan runtime
adb vk shader.comp                    # Alias de vulkan

# ── CUDA ─────────────────────────────────────────
adb cuda kernel.cu                    # CUDA code generation

# ── CPU↔GPU Hybrid Pipeline ─────────────────────
adb unified program.c                 # CPU↔GPU auto-dispatch pipeline
adb uni program.c                     # Alias de unified

# ── Auto-detect por extensión ────────────────────
adb program.c                         # Detecta .c → compila C99
adb program.cpp                       # Detecta .cpp → compila C++
```

---

## GPU Backend: SPIR-V Directo

```
Código ADead → AST → SPIR-V bytes (directo, sin IR intermedio)
```

```python
# FFI GPU (Python)
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

**ADead-BIB v7.0: C99 · C++17 → Machine Code Puro 💀🦈**

**MSVC, GCC, LLVM = referencias técnicas estudiadas y respetadas**  
**FASM = el modelo de encoding directo que ADead-BIB sigue**  
**Rust = el guardián que garantiza que el compilador nunca falle**  
**Resultado = Zero Overhead, Zero Bloat, Zero Dead Code**  
**v7.0 = 100% Autosuficiente — Sin libc, Sin linker, header_main.h = TODO**

```bash
# Tu primer proyecto con ADead-BIB:
adb create hola
cd hola
adb run
# → "Hola desde hola" — 2KB, sin GCC, sin linker
```

**"C = intención absoluta del programador  
C++ = zero overhead principle  
Rust = guardián de correctitud  
FASM = bytes directos al CPU  
header_main.h = un include, todo disponible  
adb create = como cargo new, pero para C/C++  
ADead-BIB = único en el mundo 💀🦈 🇵🇪"**
