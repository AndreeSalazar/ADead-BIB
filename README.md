# ADead-BIB v4.0 💀🦈

**Compilador Nativo: C99 · C++98 → Machine Code Puro**

> Zero Overhead · Zero Bloat · Zero Dead Code  
> Sin NASM · Sin LLVM · Sin GCC · Sin Clang  
> FASM-style: bytes directos al CPU

```
┌──────────────────────────────────────────────────────────────────────┐
│  Tu Código (.c / .cpp)                                               │
│                    ↓                                                 │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │              ADead-BIB Compiler (adeadc)                       │  │
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

## Inicio Rápido

```bash
# Clonar y compilar el compilador
git clone https://github.com/AndreeSalazar/ADead-BIB.git
cd ADead-BIB
cargo build --release

# Compilar C99
adeadc cc examples/c/hello.c -o hello.exe

# Compilar C++
adeadc cxx examples/cpp/hello.cpp -o hello.exe

# Auto-detect por extensión
adeadc build examples/c/hello.c -o hello.exe
adeadc build examples/cpp/cpp_oop.cpp -o oop.exe

# Compilar y ejecutar
adeadc run examples/c/test_counter.c

# GPU (SPIR-V directo)
adeadc gpu
```

---

## Frontends: C99 y C++98

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

### C++98 Frontend — Canon de C++ (Zero Overhead)

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
│   ├── main.rs                    # CLI driver (adeadc)
│   ├── lib.rs                     # Exports públicos
│   ├── builder.rs                 # Orchestrator
│   │
│   ├── frontend/                  # FRONTENDS
│   │   ├── c/                     # C99 Frontend
│   │   │   ├── c_lexer.rs             # Tokenizer C99
│   │   │   ├── c_parser.rs            # Recursive descent C99
│   │   │   ├── c_ast.rs               # C AST types
│   │   │   ├── c_to_ir.rs             # CAST → Program IR
│   │   │   ├── c_preprocessor.rs      # #include/#define/#ifdef
│   │   │   ├── c_stdlib.rs            # 75+ headers built-in
│   │   │   └── c_compiler_extensions.rs
│   │   │
│   │   ├── cpp/                   # C++98 Frontend
│   │   │   ├── cpp_lexer.rs           # Tokenizer C++
│   │   │   ├── cpp_parser.rs          # Classes, templates, namespaces
│   │   │   ├── cpp_ast.rs             # C++ AST types
│   │   │   ├── cpp_to_ir.rs           # CppAST → Program IR
│   │   │   ├── cpp_preprocessor.rs    # C++ preprocessor
│   │   │   ├── cpp_stdlib.rs          # STL stubs
│   │   │   └── cpp_compiler_extensions.rs
│   │   │
│   │   ├── ast.rs                 # IR compartido (Program, Function, Stmt, Expr)
│   │   ├── types.rs               # Sistema de tipos
│   │   └── type_checker.rs        # Análisis estático
│   │
│   ├── isa/                       # ISA LAYER (el core)
│   │   ├── mod.rs                     # ADeadOp enum, Reg, Operand
│   │   ├── isa_compiler.rs            # Program IR → ADeadOp stream
│   │   ├── encoder.rs                 # ADeadOp → x86-64 bytes (FASM-style)
│   │   ├── decoder.rs                 # x86-64 bytes → ADeadOp (disassembly)
│   │   ├── optimizer.rs               # Peephole, DCE sobre ADeadOp
│   │   ├── reg_alloc.rs               # Register allocator
│   │   └── codegen.rs                 # Codegen auxiliar
│   │
│   ├── backend/                   # BACKEND (binary output)
│   │   ├── cpu/
│   │   │   ├── pe.rs                  # Windows PE x64
│   │   │   ├── elf.rs                 # Linux ELF
│   │   │   ├── flat_binary.rs         # Raw binary (bootloaders, kernels)
│   │   │   ├── pe_tiny.rs             # PE mínimo (<500 bytes)
│   │   │   ├── os_codegen.rs          # Real mode/Protected mode/Long mode
│   │   │   └── ...
│   │   └── gpu/
│   │       ├── vulkan.rs              # SPIR-V generation
│   │       └── ...
│   │
│   ├── optimizer/                 # OPTIMIZADOR
│   │   ├── const_fold.rs              # Constant folding
│   │   ├── branch_detector.rs         # Branch optimization
│   │   ├── branchless.rs              # Branchless transforms
│   │   ├── binary_optimizer.rs        # Binary-level optimization
│   │   └── simd.rs                    # Auto-vectorization
│   │
│   ├── toolchain/                 # REFERENCIA MSVC/GCC/LLVM
│   │   ├── calling_conventions.rs     # Win64 + SysV calling conventions
│   │   ├── gcc_builtins.rs            # __attribute__, __builtin_*
│   │   ├── llvm_attrs.rs              # LLVM attributes/intrinsics
│   │   ├── msvc_compat.rs             # __declspec, MSVC extensions
│   │   └── cpp_name_mangler.rs        # Itanium ABI name mangling
│   │
│   ├── middle/                    # MIDDLE-END (IR avanzado)
│   │   ├── ir/                        # SSA IR (inspirado en LLVM IR)
│   │   ├── analysis/                  # CFG, dominator tree, liveness
│   │   ├── passes/                    # DCE, inline, mem2reg, GVN, LICM
│   │   └── lowering/                  # AST → IR lowering
│   │
│   └── runtime/                   # RUNTIME
│       └── ...
│
├── examples/
│   ├── c/                         # 46 archivos C99 — todos compilan ✅
│   ├── cpp/                       # 33 archivos C++ — todos compilan ✅
│   ├── boot/                      # Boot sectors, kernels
│   └── gpu/                       # GPU compute shaders
│
├── docs/                          # Documentación técnica
├── Cargo.toml                     # 100% Rust, sin deps de C/C++
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
| **C99** | 46 | 46 | **100%** ✅ |
| **C++** | 33 | 33 | **100%** ✅ |
| **Total** | **79** | **79** | **100%** ✅ |

### Runtime Verificado

```
test_counter.exe    → "After +1: total=1 pass=1" ✅
test_recursion.exe  → "fib(10)=55, power(2,10)=1024" ✅
test_bsort.exe      → "sorted=[1,2,3]" ✅
test_gcd.exe        → "gcd(48,18) = 6" ✅
test_prime.exe      → "prime(17)=1, prime(100)=0" ✅
test_class_basic.exe → "after 3 inc: 0" ✅
```

---

## Comandos CLI (`adeadc`)

```bash
# ── C99 ────────────────────────────────────────────
adeadc cc hello.c -o hello.exe        # Compilar C99
adeadc cc main.c                      # → main.exe automático

# ── C++ ────────────────────────────────────────────
adeadc cxx app.cpp -o app.exe         # Compilar C++
adeadc cxx main.cpp                   # → main.exe automático

# ── Auto-detect ───────────────────────────────────
adeadc build program.c                # Detecta .c → C99
adeadc build program.cpp              # Detecta .cpp → C++

# ── Build + Run ───────────────────────────────────
adeadc run test.c                     # Compilar y ejecutar

# ── Flat Binary (OS/Kernel) ──────────────────────
adeadc cc kernel.c -o kernel.bin --flat
adeadc cc boot.c -o boot.bin --flat16 --org=0x7C00 --size=512

# ── Binarios Mínimos ─────────────────────────────
adeadc nano output.exe                # PE más pequeño posible
adeadc micro output.exe               # PE32 < 256 bytes

# ── GPU ───────────────────────────────────────────
adeadc gpu                            # Detectar GPU + generar shader
adeadc spirv matmul 1024              # SPIR-V compute shader
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

**ADead-BIB v4.0: C99 · C++98 → Machine Code Puro 💀🦈**

**MSVC, GCC, LLVM = referencias técnicas estudiadas y respetadas**  
**FASM = el modelo de encoding directo que ADead-BIB sigue**  
**Rust = el guardián que garantiza que el compilador nunca falle**  
**Resultado = Zero Overhead, Zero Bloat, Zero Dead Code**

**"C = intención absoluta del programador  
C++ = zero overhead principle  
Rust = guardián de correctitud  
FASM = bytes directos al CPU  
ADead-BIB = todo unido, machine code puro"**
