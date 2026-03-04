# ADead-BIB — Ejemplos de Compilación

## Backend Universal · Nano Bytes · Sin Overhead 💀🦈

### Hecho en Perú 🇵🇪 · v4.0 · 2026

ADead-BIB genera binarios directamente — sin ensamblador intermedio, sin LLVM, sin GCC.
Auto-detecta `.c` y `.cpp` por extensión: `adeadc hello.cpp -o hello.exe` funciona directo.

**ADN del compilador:** Inspirado en GCC (optimizaciones agresivas), MSVC (Windows ABI), LLVM (IR + passes), FASM (generación directa de bytes a machine code).

---

## Modos de Compilación

| Modo | Lenguaje | Output | Comando |
|------|----------|--------|---------|
| **Modo 1** | ADead-BIB (raw) | `.bin` flat binary | `adeadc flat` / `adeadc boot` |
| **Modo 2a** | ADead-BIB (typed) | `.exe` PE / ELF | `adeadc build` |
| **Modo 2b** | C99/C11 nativo | `.exe` PE / ELF | `adeadc cc` o auto `.c` |
| **Modo 2c** | C++11/14/17/20 | `.exe` PE / ELF | `adeadc cxx` o auto `.cpp` |
| **Modo 3** | GPU compute | `.spv` SPIR-V | `adeadc gpu` |

---

## Pipeline: C/C++ Directo a Machine Code (estilo FASM)

```
C/C++ Source
     │
     ├── CPreprocessor → #include resolution, #define skip
     │
     ├── CLexer / CppLexer → Token stream (58+ tokens)
     │
     ├── CParser / CppParser → C AST / C++ AST
     │         (Recursive descent, full operator precedence)
     │
     ├── CToIR / CppToIR → ADead-BIB IR (Program/Function/Stmt/Expr)
     │         (Devirtualization, template monomorphization, vtable elimination)
     │
     ├── ISA Compiler → ADeadIR (Vec<ADeadOp>)
     │         (Type-checked x86-64 instruction IR)
     │
     ├── Encoder → Raw x86-64 bytes (estilo FASM)
     │         (ModR/M, SIB, REX prefix, label resolution, multi-pass)
     │
     └── PE Writer / ELF Writer → Executable
              (Directo a disco, sin linker externo)
```

**Sin NASM. Sin ensamblador. Sin linker. Bytes directos como FASM.**

---

## Modo 2b: C99/C11 — `c/`

Sin GCC. Sin Clang. 100% ADead-BIB. Pipeline completo a machine code.

```bash
# Explícito
adeadc cc c/hello.c -o hello.exe

# Auto-detección por extensión .c
adeadc c/hello.c -o hello.exe
adeadc c/c_algorithms.c -o algorithms.exe
adeadc run c/test_forloop.c
```

### Archivos Showcase (13)

| Archivo | Descripción |
|---------|-------------|
| `hello.c` | Showcase completo: 21 funciones, 4 structs, punteros, bitwise, enums |
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

### Tests de Features C (30 archivos)

| Test | Feature Verificado |
|------|--------------------|
| `test_forloop.c` | For loops con iteradores |
| `test_while.c` / `test_while2.c` / `test_while3.c` | While loops |
| `test_dowhile.c` | Do-while loops |
| `test_switch.c` | Switch/case/default |
| `test_nested_loops.c` | Loops anidados con break/continue |
| `test_bsort.c` | Bubble sort (nested for + swap) |
| `test_pointers.c` | Punteros: &, *, pass-by-reference |
| `test_recursion.c` | Recursión: fibonacci, power |
| `test_enum.c` | Enums con valores explícitos |
| `test_typedef.c` | Typedef con tipos y structs |
| `test_global_vars.c` | Variables globales |
| `test_cast.c` | Type casting: (char), (int*) |
| `test_sizeof.c` | sizeof(type), sizeof(expr) |
| `test_multivar_decl.c` | Declaración múltiple: `int a, b, c;` |
| `test_compound_assign.c` | Todos: +=, -=, *=, /=, %=, &=, \|=, ^=, <<=, >>= |
| `test_string_ops.c` | Strings, strlen, concatenación |
| `test_bitwise.c` | Operaciones bitwise completas |
| `test_struct_nested.c` | Structs anidados: Point, Rect |
| `test_array_init.c` | Arrays: init list, sum, indexing |
| `test_array_minimal.c` / `test_array_simple.c` | Arrays básicos |
| `test_void_func.c` | Funciones void y (void) |
| `test_complex_expr.c` | Precedencia de operadores |
| `test_multi_func.c` | 8+ funciones llamándose entre sí |
| `test_increment.c` | Pre/post ++/-- |
| `test_ternary.c` | Operador ternario |
| `test_threshold.c` | Funciones con múltiples params |
| `test_counter.c` | Asignaciones y printf |
| `test_frame.c` | Stack frames |
| `test_many_funcs.c` | Muchas funciones |
| `c_stdlib_long.c` | 20+ funciones de stdlib |

### C Features Soportados (verificados por 104 tests Rust)

- ✅ **Tipos**: int, char, short, long, long long, float, double, bool, unsigned, signed, void
- ✅ **Qualifiers**: const, volatile, static, extern, inline, register
- ✅ **Control flow**: if/else, for, while, do-while, switch/case/default, break, continue
- ✅ **Funciones**: recursión, prototipos, void, múltiples parámetros (6+), forward declarations
- ✅ **Punteros**: &, *, ->, ptr-to-ptr (**pp), arrays como parámetros
- ✅ **Arrays**: init list `{1,2,3}`, indexing `arr[i]`, como parámetros `int arr[]`
- ✅ **Structs**: definición, nested, array fields, dot access
- ✅ **Enums**: con valores explícitos, auto-incremento
- ✅ **Typedef**: tipos primitivos, structs, punteros
- ✅ **Bitwise**: &, |, ^, ~, <<, >>
- ✅ **Compound assigns**: +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=
- ✅ **Ternary**: `(cond) ? a : b` (anidado)
- ✅ **Cast**: `(char)x`, `(int*)ptr`
- ✅ **Sizeof**: `sizeof(int)`, `sizeof(expr)`
- ✅ **Pre/post increment**: ++x, x++, --x, x--
- ✅ **String concat**: `"hello" " " "world"`
- ✅ **Hex literals**: 0xFF, 0xDEAD
- ✅ **Printf**: format specifiers %d, %s, %c, %x, %f
- ✅ **Memoria dinámica**: malloc, free, realloc

---

## Modo 2c: C++ — `cpp/`

Sin GCC. Sin LLVM. Sin Clang. 100% ADead-BIB C++.

```bash
# Explícito
adeadc cxx cpp/hello.cpp -o hello.exe

# Auto-detección por extensión .cpp
adeadc cpp/cpp_oop.cpp -o oop.exe
adeadc cpp/cpp_templates.cpp -o templates.exe
adeadc cpp/cpp_modern.cpp -o modern.exe
```

### Archivos Showcase (5)

| Archivo | Descripción |
|---------|-------------|
| `hello.cpp` | Hello World C++ con vector y cout |
| `cpp_oop.cpp` | Classes, herencia, virtual, override, constructors, templates |
| `cpp_templates.cpp` | Function/class templates, Pair, Stack, namespaces, recursion |
| `cpp_modern.cpp` | auto, constexpr, nullptr, enum class, noexcept, type aliases |
| `cpp_stdlib_long.cpp` | 10+ funciones, namespaces, clases complejas |

### Tests de Features C++ (24 archivos)

| Test | Feature Verificado |
|------|--------------------|
| `test_class_basic.cpp` | Clase Counter: constructor, increment/decrement/reset |
| `test_class_methods.cpp` | Clase Calculator: add/sub/mul/clear/get |
| `test_inheritance.cpp` | Herencia: Shape → Circle, Rectangle |
| `test_template_basic.cpp` | Templates genéricos: max, min, abs |
| `test_enum_class.cpp` | Scoped enums C++11: Color, Status |
| `test_constexpr.cpp` | constexpr: factorial, fibonacci, compile-time |
| `test_auto_nullptr.cpp` | Auto type deduction + nullptr safety |
| `test_nested_namespace.cpp` | Namespaces anidados: outer::inner, math:: |
| `test_using_alias.cpp` | Type aliases con `using` |
| `test_cpp_control_flow.cpp` | Fibonacci iterativo + is_prime |
| `test_namespace.cpp` | Namespace math:: con add/sub/mul |
| `test_5func.cpp` | 5 funciones en namespace math:: |
| `test_6func.cpp` / `test_7func.cpp` / `test_8func.cpp` | Funciones progresivas |
| `test_minimal.cpp` | Hello World + función add |
| `test_recursion.cpp` | Funciones recursivas |
| `test_forloop.cpp` | For loops en C++ |
| `test_gcd.cpp` | GCD con while |
| `test_prime.cpp` | Test de primalidad |
| `test_counter.cpp` | Contadores con printf |
| `test_multidecl.cpp` | Declaraciones múltiples |
| `test_ns_recursion.cpp` | Recursión en namespaces |
| `test_postinc.cpp` | Post-incremento |
| `test_vardecl.cpp` | Declaración de variables |
| `test_pattern.cpp` | Patrones de código |
| `test_simple_print.cpp` | Printf simple |
| `test_two_vars.cpp` | Dos variables |

### C++ Features Soportados (verificados por 67 tests Rust)

- ✅ **Classes**: fields, methods, constructors, destructors
- ✅ **Herencia**: `class Derived : public Base`, override
- ✅ **Virtual**: devirtualización en compile-time (sin vtable overhead)
- ✅ **Templates**: function templates, class templates, non-type params
- ✅ **Namespaces**: nested (outer::inner), `using`, scope resolution `::`
- ✅ **Enum class**: scoped enums con tipo base
- ✅ **constexpr**: evaluación en compile-time
- ✅ **auto**: type deduction para variables y return type
- ✅ **nullptr**: literal de puntero nulo tipo-safe
- ✅ **Type aliases**: `using Integer = int;`
- ✅ **explicit**: constructores explícitos
- ✅ **const methods**: `int get() const`
- ✅ **noexcept**: funciones sin excepciones
- ✅ **Lambdas**: captures, params, return type (parsed)
- ✅ **Casts**: static_cast, dynamic_cast, const_cast, reinterpret_cast (parsed)
- ✅ **Smart pointers**: unique_ptr, shared_ptr → raw pointers (zero overhead)
- ✅ **Exceptions**: try/catch/throw → eliminados a error codes
- ✅ **End-to-end**: C++ source → x86-64 machine code → PE/ELF

### ADN de Compiladores Reales

| Compilador | Lo que ADead-BIB toma |
|------------|----------------------|
| **GCC** | Optimizaciones agresivas, torture tests, multi-target |
| **MSVC** | Windows x64 ABI, PE format, shadow space, callee-saved regs |
| **LLVM** | IR con SSA, optimization passes (DCE, const fold, inline, GVN) |
| **FASM** | Generación directa de bytes sin ensamblador externo, multi-pass encoding |

---

## Test Summary

| Frontend | Parse Tests | IR Tests | E2E (→ machine code) | Example Files | **Total** |
|----------|-------------|----------|-----------------------|---------------|-----------|
| C99/C11 | 5 | 62 | 18 | 19 | **104** |
| C++ | 14 | 25 | 7 | 21 | **67** |
| ADead-BIB | 40+ | — | 3 | — | **40+** |
| ISA/Encoder | 15 | 12 | — | — | **27** |
| Middle IR | 25 | — | — | — | **25** |
| Optimizer | 6 | — | — | — | **6** |
| Backend | 3 | — | — | — | **3** |
| Runtime | 10 | — | — | — | **10** |
| **TOTAL** | | | | | **394** |

✅ **394 tests — 0 failures**

---

## Estructura de Carpetas

```
examples/
├── boot/               # Modo 1: Boot sectors, kernels, drivers
├── c/                  # Modo 2b: C99/C11 (43 archivos .c)
│   ├── hello.c                 # Showcase completo (21 funciones)
│   ├── c_algorithms.c          # Quicksort, mergesort, binary search
│   ├── c_bitwise.c             # Operaciones bitwise
│   ├── c_compression.c         # RLE, Huffman
│   ├── c_crypto.c              # Ciphers, hash
│   ├── c_database.c            # Key-value store
│   ├── c_math.c                # Trig, linear algebra
│   ├── c_memory.c              # malloc/free, pools
│   ├── c_network.c             # Sockets, HTTP
│   ├── c_structs.c             # Structs, typedefs, unions
│   ├── c_threading.c           # pthread, mutex
│   ├── c_fastos_base.c         # 75+ system headers
│   ├── c_fastos_complete.c     # FastOS completo
│   ├── c_stdlib_long.c         # 20+ funciones stdlib
│   ├── test_dowhile.c          # do-while loops
│   ├── test_switch.c           # switch/case/default
│   ├── test_nested_loops.c     # nested loops + break/continue
│   ├── test_pointers.c         # punteros, &, *
│   ├── test_recursion.c        # fibonacci, power
│   ├── test_enum.c             # enums
│   ├── test_typedef.c          # typedef
│   ├── test_global_vars.c      # globals
│   ├── test_cast.c             # type casting
│   ├── test_sizeof.c           # sizeof
│   ├── test_multivar_decl.c    # int a, b, c;
│   ├── test_compound_assign.c  # +=, -=, *=, etc.
│   ├── test_string_ops.c       # strings, strlen
│   ├── test_bitwise.c          # &, |, ^, ~, <<, >>
│   ├── test_struct_nested.c    # nested structs
│   ├── test_array_init.c       # array init + sum
│   ├── test_void_func.c        # void functions
│   ├── test_complex_expr.c     # operator precedence
│   ├── test_multi_func.c       # many functions
│   ├── test_increment.c        # ++, --
│   ├── test_bsort.c            # bubble sort
│   ├── test_ternary.c          # ternary operator
│   ├── test_forloop.c          # for loops
│   ├── test_while.c            # while loops
│   └── ...
├── cpp/                # Modo 2c: C++11/14/17/20 (29 archivos .cpp)
│   ├── hello.cpp               # Hello World
│   ├── cpp_oop.cpp             # OOP: classes, herencia, virtual
│   ├── cpp_templates.cpp       # Templates, Pair, Stack, namespaces
│   ├── cpp_modern.cpp          # auto, constexpr, nullptr, enum class
│   ├── cpp_stdlib_long.cpp     # stdlib completo
│   ├── test_class_basic.cpp    # Counter class
│   ├── test_class_methods.cpp  # Calculator class
│   ├── test_inheritance.cpp    # Shape → Circle, Rectangle
│   ├── test_template_basic.cpp # Template max/min/abs
│   ├── test_enum_class.cpp     # Scoped enums
│   ├── test_constexpr.cpp      # constexpr functions
│   ├── test_auto_nullptr.cpp   # auto + nullptr
│   ├── test_nested_namespace.cpp # Namespaces anidados
│   ├── test_using_alias.cpp    # Type aliases
│   ├── test_cpp_control_flow.cpp # fibonacci + primes
│   ├── test_namespace.cpp      # math:: namespace
│   ├── test_5func.cpp          # 5 functions in namespace
│   ├── test_minimal.cpp        # Hello + add
│   ├── test_recursion.cpp      # Recursive functions
│   ├── test_gcd.cpp            # GCD algorithm
│   ├── test_prime.cpp          # Prime checker
│   └── ...
├── gpu/                # Modo 3: GPU compute SPIR-V
└── README.md
```

---

**Sin NASM. Sin LLVM. Sin linker externo. Sin ensamblador.**
**C/C++ → x86-64 Machine Code directo. Estilo FASM.**
**100% ADead-BIB. Hecho en Perú 🇵🇪 💀🦈**

Autor: Eddi Andreé Salazar Matos
Versión: ADead-BIB v4.0
Licencia: MIT
