# FASE 1 — C Standard Library COMPLETO: Plan de Prioridades

> **Fecha:** 2026-04-11  
> **Objetivo:** Completar la C libc para que funcione end-to-end en ADead-BIB  
> **Método:** Priorizar C primero, luego C++

---

## DESCUBRIMIENTO CLAVE: ¿QUÉ YA EXISTE?

Después de analizar todo el código fuente, la situación REAL es:

### ✅ LO QUE YA ESTÁ HECHO (y mucha gente no sabe):

| Capa | Estado | Detalle |
|------|--------|---------|
| **Headers C (declaraciones)** | ✅ **~95% COMPLETO** | `stdlib.rs` tiene 20+ headers con TODAS las funciones declaradas: stdio.h (35 func), stdlib.h (30 func), string.h (24 func), math.h (50+ func), time.h (15 func), signal.h, setjmp.h, locale.h, stdarg.h, errno.h, assert.h, limits.h, float.h, stdbool.h, stdint.h, stddef.h, fenv.h, complex.h, wchar.h, wctype.h, uchar.h, tgmath.h, stdatomic.h, threads.h, stdalign.h, stdnoreturn.h, iso646.h |
| **IAT Registry (DLL imports)** | ✅ **~90% COMPLETO** | `lib.rs` IAT tiene 18 DLLs con 310+ funciones: msvcrt.dll (138 func), kernel32.dll (40 func), user32.dll (42 func), gdi32.dll (30 func), opengl32.dll (31 func), ole32.dll (20), oleaut32.dll (20), dxgi.dll (4), d3d9.dll (9), d3d11.dll (3), d3d12.dll (8), d3dcompiler_47.dll (15), advapi32, shell32, winmm, comdlg32, ws2_32 |
| **C Preprocessor** | ✅ COMPLETO | #include, #define, #ifdef, #if, #elif, #else, #endif, #pragma |
| **C Parser (AST)** | ✅ COMPLETO | C99/C11 completo: structs, unions, enums, typedef, function pointers, ternary, etc. |
| **IR Generation** | ✅ COMPLETO | CToIR lower funciona |
| **UB Detector** | ✅ COMPLETO | 21+ categorías |
| **PE Output** | ✅ FUNCIONA | MZ/PE headers, secciones, IAT filtrado |
| **stdlib symbol registries** | ✅ COMPLETO | fastos_stdio.rs, fastos_stdlib.rs, fastos_string.rs, fastos_math.rs, fastos_time.rs, fastos_signal.rs, fastos_wchar.rs, fastos_com.rs, fastos_dxgi.rs, fastos_d3d9.rs, fastos_d3d11.rs, fastos_d3d12.rs |
| **C test fixtures (parse)** | ✅ 33 archivos | tests/c/fixtures/ — todos parsean OK |

### 🔴 EL CUELLO DE BOTELLA REAL: ISA COMPILER CODEGEN

**El problema NO es que falten funciones declaradas. El problema es que el ISA compiler (x86-64 codegen) no genera código correcto para muchas construcciones C.**

```
┌─────────────────────────────────────────────────────────┐
│           FLUJO DE COMPILACIÓN ADEAD-BIB                │
│                                                         │
│  .c source                                              │
│    ↓ ✅ Preprocessor (#include → inject header decls)   │
│    ↓ ✅ Lexer (tokens)                                  │
│    ↓ ✅ Parser (AST)                                    │
│    ↓ ✅ Semantic (type check)                           │
│    ↓ ✅ UB Detection (21+ categories)                   │
│    ↓ ✅ IR Generation (CToIR)                           │
│    ↓ 🔴 ISA Compiler ← AQUÍ ESTÁ EL PROBLEMA          │
│    ↓ ✅ PE Output (genera .exe válido)                  │
│  .exe                                                   │
│                                                         │
│  🔴 ISA Compiler falla en:                              │
│    - Struct field access (.x, ->y)                      │
│    - Float/double arithmetic                            │
│    - Function pointers / indirect calls                 │
│    - goto / label forward references                    │
│    - Byte-level memory ops (memcpy codegen)             │
│    - Complex pointer casts (void***)                    │
│    - va_list / variadic args                            │
└─────────────────────────────────────────────────────────┘
```

---

## ARQUITECTURA DEL CÓDIGO (MAPA DE ARCHIVOS)

```
ADead-BIB/src/rust/crates/
├── app/ADead-BIB-Main/src/
│   ├── main.rs                    ← Entry point
│   ├── builder.rs                 ← Build orchestration
│   ├── cli/                       ← CLI parsing
│   └── driver/
│       ├── c_driver.rs            ← C pipeline (preproc→lex→parse→IR→codegen→PE)
│       ├── cpp_driver.rs          ← C++ pipeline
│       ├── cuda_driver.rs         ← CUDA pipeline
│       └── js_driver.rs           ← JS pipeline
│
├── frontend/c/adeb-frontend-c/src/
│   ├── stdlib.rs                  ← ✅ HEADERS C (20+ headers, ~200 funciones)
│   ├── compiler_extensions.rs     ← ✅ Windows/SIMD/wchar headers
│   ├── preprocessor.rs            ← ✅ #include/#define processing
│   ├── parse/
│   │   ├── lexer.rs               ← ✅ C tokenizer
│   │   └── parser.rs              ← ✅ C parser (AST)
│   ├── ast.rs                     ← ✅ C AST types
│   └── lower/to_ir.rs             ← ✅ C AST → ADead-BIB IR
│
├── middle/adeb-middle/src/
│   ├── ir/                        ← ✅ IR types (basicblock, function, instruction, value)
│   ├── optimizer/                 ← ✅ Optimizaciones (const_fold, dead_code, inline, simd, branchless)
│   └── ub_detector/               ← ✅ UB detection
│
├── backend/cpu/adeb-backend-x64/src/
│   ├── lib.rs                     ← ✅ IAT Registry (18 DLLs, 310+ slots) + PE gen
│   └── isa/
│       ├── isa_compiler.rs        ← 🔴 ISA COMPILER PRINCIPAL (5600+ líneas)
│       ├── c_isa.rs               ← 🔴 C-specific ISA wrapper
│       ├── compiler/
│       │   ├── core.rs            ← 🔴 Estado del compilador
│       │   ├── compile.rs         ← 🔴 Compilación principal
│       │   ├── expressions.rs     ← 🔴 Codegen de expresiones
│       │   ├── statements.rs      ← 🔴 Codegen de statements
│       │   ├── control_flow.rs    ← 🟡 if/for/while/switch (parcialmente funciona)
│       │   ├── functions.rs       ← 🟡 Function calls (directas OK, indirectas NO)
│       │   ├── arrays.rs          ← 🔴 Array codegen
│       │   └── helpers.rs         ← 🟡 IAT call emission
│       ├── encoder.rs             ← 🟡 x86-64 instruction encoder
│       ├── decoder.rs             ← Decoder
│       ├── reg_alloc.rs           ← Register allocator
│       ├── optimizer.rs           ← Peephole optimizer
│       ├── bit_resolver.rs        ← Label/bit resolution
│       ├── vex_emitter.rs         ← AVX/VEX prefix
│       ├── soa_optimizer.rs       ← SoA optimizer
│       └── ymm_allocator.rs       ← YMM register allocator
│
├── shared/
│   ├── adeb-stdlib/src/c/         ← ✅ Symbol registries (fastos_*.rs)
│   ├── adeb-bridge/               ← ✅ ASM-BIB .obj bridge linker
│   ├── adeb-core/                 ← ✅ AST, types, symbols, preprocessor, toolchain compat
│   └── adeb-platform/             ← ✅ PE/ELF/Mach-O output format support
│
└── security/adeb-bg/              ← ✅ Binary analysis, security policy
```

---

## PLAN DE PRIORIDADES: C PRIMERO

### NIVEL 0 — CODEGEN FIXES CRÍTICOS (Desbloquean TODO)

Estos fixes son **prerequisitos** para que CUALQUIER función C nueva funcione correctamente.

| # | Fix | Archivos a Modificar | Bloquea | Impacto |
|---|-----|---------------------|---------|---------|
| **C-01** | **Struct field access codegen** (`.x`, `->y`, `struct por valor`) | `isa/compiler/expressions.rs`, `isa/compiler/statements.rs` | Tests 06, 07, 15, 18, 21 + DX + tm, FILE, lconv, div_t | **MÁXIMO** — Structs son fundamentales para time.h (tm), stdio.h (FILE), locale.h (lconv), signal.h (sigaction), + TODO DirectX |
| **C-02** | **Float/double arithmetic** (SSE2 codegen: MOVSD/ADDSD/MULSD/DIVSD/SQRTSD + conversiones) | `isa/compiler/expressions.rs`, `isa/encoder.rs` | Test 23, math.h COMPLETO, OpenGL, DX, atof, strtod | **MÁXIMO** — Sin floats, math.h es inútil y OpenGL/DX no pueden funcionar |
| **C-03** | **Byte-level memory ops** (unsigned char ptr arithmetic, REP MOVSB) | `isa/compiler/expressions.rs` | Tests 02, 14, 19 — strchr, memcpy, memset, memcmp | **ALTO** — Funciones de memoria son las más usadas |
| **C-04** | **Function pointer / indirect call** (`call rax` via vtable) | `isa/compiler/functions.rs` | Tests 11-13 (DX), 16 (Vulkan), 22 (signal), qsort, bsearch, atexit | **ALTO** — qsort/bsearch/signal necesitan fn pointers |
| **C-05** | **goto / label forward-ref** | `isa/bit_resolver.rs`, `isa/compiler/statements.rs` | Tests 11-13 (DX) | **MEDIO** — Principalmente DX vtable patterns |
| **C-06** | **va_list / variadic args** (Win64: spill RCX/RDX/R8/R9 + stack access) | `isa/compiler/functions.rs` | Test 19, vprintf, vfprintf, vsprintf, custom variadic | **MEDIO** |
| **C-07** | **Global/static variables** (.data section, RIP-relative access) | `isa/compiler/statements.rs` | errno, stdin/stdout/stderr, global state | **MEDIO** |
| **C-08** | **Array initializers** (`int a[] = {1,2,3}`, `char s[] = "hello"`) | `isa/compiler/arrays.rs` | Múltiples tests | **MEDIO** |
| **C-09** | **Cast codegen** (pointer casts, int↔ptr, signed↔unsigned) | `isa/compiler/expressions.rs` | DX vtable, COM | **MEDIO** |
| **C-10** | **sizeof codegen** (runtime sizeof para structs, arrays) | `isa/compiler/expressions.rs` | calloc, memset, fread/fwrite | **MEDIO** |

### NIVEL 1 — FUNCIONES C QUE YA FUNCIONAN (IAT msvcrt.dll)

Estas funciones ya están en el IAT y producen código correcto HOY:

| Header | Funciones que YA funcionan | Test Verificado |
|--------|---------------------------|-----------------|
| `<stdio.h>` | `printf` (con %d, %x, %p), `sprintf` | Tests 01, 03, 05 |
| `<stdlib.h>` | `malloc`, `free`, `exit` | Tests 01, 04, 08 |
| `<string.h>` | `strlen`, `strcpy`, `strncpy`, `strcat`, `strncat`, `strcmp`, `strncmp`, `strstr`, `strtok` | Test 02 (10/14) |

### NIVEL 2 — FUNCIONES C EN IAT PERO SIN CODEGEN VERIFICADO

Estas ya están registradas en msvcrt.dll IAT. Solo necesitan que los codegen fixes de Nivel 0 se apliquen:

| Header | Funciones en IAT pendientes de verificación | Depende de Fix |
|--------|---------------------------------------------|----------------|
| `<stdio.h>` | `fprintf`, `_snprintf`, `scanf`, `sscanf`, `puts`, `putchar`, `getchar`, `fgets`, `fputs`, `fopen`, `fclose`, `fread`, `fwrite`, `fseek`, `ftell`, `rewind`, `feof`, `ferror`, `fflush`, `perror` | C-01 (FILE struct), C-03 (byte ops), C-04 (fn ptr for callbacks) |
| `<stdlib.h>` | `calloc` (funciona parcial), `realloc` (parcial), `atoi`, `atof`, `atol`, `strtol`, `strtoul`, `strtod`, `abs`, `rand`, `srand`, `qsort`, `bsearch`, `getenv`, `system` | C-02 (atof/strtod necesitan float), C-04 (qsort/bsearch necesitan fn ptr), C-01 (div_t struct) |
| `<string.h>` | `memset`, `memcpy`, `memmove`, `memcmp`, `strchr`, `strrchr`, `strtok` | C-03 (byte-level ops) |
| `<time.h>` | `time`, `clock`, `difftime`, `strftime` | C-01 (struct tm), C-02 (difftime retorna double) |

### NIVEL 3 — FUNCIONES C EN IAT PERO NECESITAN MSVCRT EXPANSION

Estas funciones existen en msvcrt.dll pero NO están en nuestro IAT todavía:

| Header | Funciones a AGREGAR al IAT msvcrt.dll | Esfuerzo |
|--------|---------------------------------------|----------|
| `<stdio.h>` | `vprintf`, `vfprintf`, `vsprintf`, `vsnprintf`, `setbuf`, `setvbuf`, `ungetc`, `freopen`, `rename`, `remove`, `tmpfile`, `tmpnam` | **Bajo** — Solo agregar a `DLL_IMPORTS` en lib.rs |
| `<stdlib.h>` | `abort`, `atexit`, `strtoll`, `strtoull`, `strtof`, `labs`, `llabs`, `div`, `ldiv`, `lldiv`, `mbstowcs`, `wcstombs`, `mbtowc`, `wctomb` | **Bajo** |
| `<string.h>` | `strrchr`, `memmove`, `memchr`, `strerror`, `strpbrk`, `strspn`, `strcspn`, `strcoll`, `strxfrm` | **Bajo** |
| `<math.h>` | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`, `exp`, `log`, `log10`, `pow`, `sqrt`, `ceil`, `floor`, `fabs`, `fmod`, `sinh`, `cosh`, `tanh`, `ldexp`, `frexp`, `modf` | **Bajo** — pero necesita C-02 (float codegen) |
| `<time.h>` | `mktime`, `localtime`, `gmtime`, `asctime`, `ctime` | **Bajo** |
| `<signal.h>` | `signal`, `raise` | **Bajo** — pero C-04 (fn ptr) |
| `<setjmp.h>` | `setjmp`, `longjmp` | **Medio** — necesita asm especial |
| `<locale.h>` | `setlocale`, `localeconv` | **Bajo** |

### NIVEL 4 — FUNCIONES QUE NECESITAN IMPLEMENTACIÓN PROPIA (no msvcrt)

| Función | Por qué no es IAT | Cómo implementar |
|---------|-------------------|------------------|
| `isalpha/isdigit/toupper/etc` (ctype.h) | ✅ **YA IMPLEMENTADO** inline en headers | N/A — ya funciona |
| `assert` | Macro que llama `__assert_fail` | Implementar `__assert_fail` como print+abort |
| `errno` | Variable global thread-local | Implementar como global en .data o IAT `_errno` de msvcrt |
| `va_start/va_arg/va_end` | Intrínsecos del compilador | Fix C-06 — codegen en ISA compiler |
| `setjmp/longjmp` | Intrínsecos ASM | Implementar en ASM-BIB `stdlib_setjmp.pasm` |
| `offsetof` | Macro que evalúa en compilador | Ya definido como macro en HEADER_STDDEF |

---

## ORDEN DE IMPLEMENTACIÓN RECOMENDADO

### Semana 1: Codegen Fixes Core
```
1. C-01: Struct field access → DESBLOQUEA: struct tm, FILE*, div_t, sigaction
2. C-02: Float/double SSE2  → DESBLOQUEA: math.h completo, atof, strtod
3. C-03: Byte-level memory  → DESBLOQUEA: memcpy/memset/memcmp/strchr
```

### Semana 2: Codegen Fixes Medium
```
4. C-04: Function pointers  → DESBLOQUEA: qsort, bsearch, signal, atexit, DX vtable
5. C-07: Global variables   → DESBLOQUEA: errno, stdin/stdout/stderr
6. C-08: Array initializers → DESBLOQUEA: string literals en arrays, lookup tables
```

### Semana 3: IAT Expansion + Tests
```
7. Agregar funciones faltantes al IAT msvcrt.dll en lib.rs
8. Agregar signal/raise al IAT msvcrt.dll
9. Agregar math functions al IAT msvcrt.dll
10. Crear nuevos tests bridge/
```

### Semana 4: Verificación End-to-End
```
11. C-06: va_list codegen   → DESBLOQUEA: vprintf, custom variadic functions
12. C-09: Cast codegen      → DESBLOQUEA: COM/DX patterns
13. C-10: sizeof codegen    → DESBLOQUEA: fread/fwrite correctos
14. Run all 23+ bridge tests, fix regressions
```

---

## IAT EXPANSION: FUNCIONES A AGREGAR EN `lib.rs`

### Archivo: `src/rust/crates/backend/cpu/adeb-backend-x64/src/lib.rs`
### Ubicación: `DLL_IMPORTS` → `msvcrt.dll` functions array

**Funciones actuales en msvcrt.dll IAT (56):**
```
printf, fprintf, sprintf, _snprintf, scanf, sscanf,
puts, putchar, getchar, fgets, fputs,
fopen, fclose, fread, fwrite, fseek, ftell, rewind,
feof, ferror, fflush, perror,
malloc, calloc, realloc, free,
atoi, atof, atol, strtol, strtoul, strtod,
abs, rand, srand, qsort, bsearch,
exit, getenv, system,
memset, memcpy, memmove, memcmp,
strlen, strcpy, strncpy, strcat, strncat,
strcmp, strncmp, strchr, strrchr, strstr, strtok,
time, clock, difftime, strftime
```

**Funciones a AGREGAR (38 nuevas → total 94):**

```rust
// === AGREGAR A msvcrt.dll ===

// stdio extras
"vprintf", "vfprintf", "vsprintf", "vsnprintf",
"setbuf", "setvbuf", "ungetc", "freopen",
"remove", "rename", "tmpfile", "tmpnam",
"fgetc", "fputc", "clearerr",

// stdlib extras
"abort", "_exit", "atexit",
"strtoll", "strtoull", "strtof",
"labs", "llabs",

// string extras
"memchr", "strerror", "strpbrk", "strspn", "strcspn",
"strcoll", "strxfrm",

// math (via msvcrt — estas son exports reales de msvcrt.dll)
"sin", "cos", "tan", "asin", "acos", "atan", "atan2",
"exp", "log", "log10", "pow", "sqrt",
"ceil", "floor", "fabs", "fmod",
"sinh", "cosh", "tanh",
"ldexp", "frexp", "modf",

// time extras
"mktime", "localtime", "gmtime", "asctime", "ctime",

// signal
"signal", "raise",

// locale
"setlocale", "localeconv",

// errno
"_errno",
```

---

## NUEVA ESTRUCTURA DE TESTS

### Eliminar: `tests/bridge/` (contenido actual — ya documentado en reportes_test.md)

### Nueva estructura `tests/bridge/`:

```
tests/bridge/
├── README.md                    ← Reporte actualizado
├── run_tests.ps1                ← Script de ejecución
├── Makefile                     ← Build all tests
│
├── fase1_libc/                  ← C Standard Library tests
│   ├── 01_stdio_printf.c       ← printf %d %s %x %f %p %ld %llu
│   ├── 02_stdio_file_io.c      ← fopen/fread/fwrite/fclose/fseek/ftell
│   ├── 03_stdio_scanf.c        ← scanf/sscanf/fscanf
│   ├── 04_stdio_format.c       ← sprintf/snprintf/fprintf/vprintf
│   ├── 05_stdlib_alloc.c       ← malloc/calloc/realloc/free/aligned_alloc
│   ├── 06_stdlib_convert.c     ← atoi/atof/strtol/strtod/strtoll
│   ├── 07_stdlib_sort.c        ← qsort/bsearch (con function pointers)
│   ├── 08_stdlib_env.c         ← getenv/system/exit/abort/atexit
│   ├── 09_stdlib_random.c      ← rand/srand
│   ├── 10_string_basic.c       ← strlen/strcpy/strcat/strcmp/strncpy/strncat/strncmp
│   ├── 11_string_search.c      ← strchr/strrchr/strstr/strtok/strpbrk/strspn/strcspn
│   ├── 12_string_memory.c      ← memcpy/memmove/memset/memcmp/memchr
│   ├── 13_math_basic.c         ← sin/cos/tan/sqrt/pow/exp/log/fabs/ceil/floor
│   ├── 14_math_advanced.c      ← asin/acos/atan/atan2/sinh/cosh/fmod/hypot/ldexp/frexp
│   ├── 15_time_basic.c         ← time/clock/difftime/localtime/gmtime/strftime/mktime
│   ├── 16_ctype_full.c         ← isalpha/isdigit/toupper/tolower (inline — ya funciona)
│   ├── 17_signal_basic.c       ← signal/raise/SIG_DFL/SIG_IGN
│   ├── 18_errno_assert.c       ← errno check/assert macro
│   ├── 19_stdarg_variadic.c    ← va_list/va_start/va_arg/va_end
│   ├── 20_locale_basic.c       ← setlocale/localeconv
│   ├── 21_limits_types.c       ← INT_MAX/INT_MIN/UINT_MAX/sizeof checks
│   └── 22_setjmp_basic.c       ← setjmp/longjmp
│
├── fase2_structs/               ← Struct/Union/Enum codegen tests
│   ├── 01_struct_basic.c       ← Struct declaration, field access (.x)
│   ├── 02_struct_pointer.c     ← Struct pointer (->y), malloc struct
│   ├── 03_struct_nested.c      ← Nested structs, array of structs
│   ├── 04_struct_byvalue.c     ← Struct passed by value to function
│   ├── 05_union_basic.c        ← Union declaration, type-punning
│   ├── 06_enum_switch.c        ← Large enum + exhaustive switch
│   ├── 07_bitfield.c           ← Struct bitfields
│   ├── 08_linked_list.c        ← Self-referential struct (linked list)
│   └── 09_complex_types.c      ← typedef struct, struct in struct, struct array
│
├── fase3_pointers/              ← Pointer & Array codegen tests
│   ├── 01_pointer_basic.c      ← int *p, *p = x, p++, p--
│   ├── 02_pointer_arith.c      ← p + n, p - q, array via pointer
│   ├── 03_array_1d.c           ← int a[10], a[i] = x
│   ├── 04_array_2d.c           ← int a[3][4], a[i][j]
│   ├── 05_array_init.c         ← int a[] = {1,2,3}, char s[] = "hello"
│   ├── 06_fnptr_basic.c        ← int (*f)(int) = &square; f(5)
│   ├── 07_fnptr_callback.c     ← qsort-style callbacks
│   ├── 08_void_ptr.c           ← void* casting, generic containers
│   └── 09_multi_ptr.c          ← int**, void***, vtable patterns
│
├── fase4_control/               ← Control flow (ya funciona mayormente)
│   ├── 01_if_else.c            ← if/else/else if chains
│   ├── 02_loops.c              ← for/while/do-while/break/continue
│   ├── 03_switch.c             ← switch/case/default, fallthrough
│   ├── 04_goto_labels.c        ← goto + labels, forward references
│   ├── 05_recursion.c          ← fibonacci, factorial, ackermann
│   └── 06_nested_complex.c     ← Nested loops + switch + if
│
├── fase5_float/                 ← Float/double codegen tests
│   ├── 01_float_basic.c        ← float a = 3.14; float b = a + 1.0;
│   ├── 02_double_basic.c       ← double x = sin(3.14);
│   ├── 03_float_compare.c      ← f > g, f == 0.0, isnan, isinf
│   ├── 04_float_convert.c      ← int↔float, float↔double conversions
│   ├── 05_float_printf.c       ← printf("%f %e %g", ...)
│   └── 06_newton_sqrt.c        ← Newton-Raphson sqrt approximation
│
├── fase6_win32/                 ← Win32 API tests (ya funcionan algunos)
│   ├── 01_console_hello.c      ← ✅ printf hello (ya pasa)
│   ├── 02_win32_window.c       ← ✅ CreateWindowExA (ya pasa)
│   ├── 03_gdi_drawing.c        ← ✅ GDI gradient (ya pasa)
│   ├── 04_opengl_basic.c       ← ✅ OpenGL 1.1 (ya pasa)
│   ├── 05_file_io_win32.c      ← CreateFileA/ReadFile/WriteFile
│   ├── 06_threading.c          ← CreateThread/WaitForSingleObject
│   ├── 07_timer_perf.c         ← QueryPerformanceCounter
│   └── 08_keyboard_input.c     ← GetAsyncKeyState
│
└── fase7_dx/                    ← DirectX tests (6 tests)
    ├── 01_com_init.c           ← CoInitializeEx + CoUninitialize
    ├── 02_dxgi_factory.c       ← CreateDXGIFactory1 + GUID construction
    ├── 03_d3d9_create.c        ← Direct3DCreate9
    ├── 04_d3d11_device.c       ← D3D11CreateDevice (WARP + HW)
    ├── 05_d3d12_device.c       ← D3D12CreateDevice + D3D12GetDebugInterface
    └── 06_d3dcompiler.c        ← D3DCreateBlob + D3DCompile (VS/PS)
```

---

## ARCHIVOS EXACTOS A MODIFICAR (en orden)

### 1. Expandir IAT msvcrt.dll
**Archivo:** `src/rust/crates/backend/cpu/adeb-backend-x64/src/lib.rs`  
**Línea:** ~27-44 (DLL_IMPORTS msvcrt.dll functions array)  
**Acción:** Agregar las 38 funciones faltantes listadas arriba  

### 2. Fix struct codegen (C-01)
**Archivos:**
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/expressions.rs` — field access
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/statements.rs` — struct alloc
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/core.rs` — struct layout info

### 3. Fix float codegen (C-02)
**Archivos:**
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/expressions.rs` — MOVSD/ADDSD
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/encoder.rs` — SSE2 instruction encoding

### 4. Fix byte-level memory (C-03)
**Archivos:**
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/expressions.rs` — ptr arithmetic
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/helpers.rs` — memory codegen

### 5. Fix function pointers (C-04)
**Archivos:**
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/functions.rs` — call rax
- `src/rust/crates/backend/cpu/adeb-backend-x64/src/isa/compiler/expressions.rs` — fn ptr load

### 6. Crear tests bridge/
**Directorio:** `tests/bridge/` — crear nueva estructura con los archivos .c listados arriba

---

## CONTEO FINAL DE FUNCIONES C

| Categoría | Declaradas (header) | En IAT | Funcionan | Falta IAT | Falta Codegen |
|-----------|-------------------|--------|-----------|-----------|---------------|
| stdio.h | 35 | 22 | 3 | 13 | Fix C-01,C-03,C-06 |
| stdlib.h | 30 | 19 | 3 | 8 | Fix C-02,C-04 |
| string.h | 24 | 16 | 10 | 7 | Fix C-03 |
| math.h | 50+ | 0 | 0 | 50+ | Fix C-02 |
| time.h | 15 | 4 | 0 | 9 | Fix C-01,C-02 |
| ctype.h | 13 | 0 (inline) | 13 | 0 | ✅ COMPLETO |
| signal.h | 4 | 0 | 0 | 2 | Fix C-04 |
| setjmp.h | 2 | 0 | 0 | 2 | ASM-BIB module |
| stdarg.h | 5 | 0 (intrinsic) | 0 | 0 | Fix C-06 |
| locale.h | 2 | 0 | 0 | 2 | Fix C-01 |
| errno.h | 1 | 0 | 0 | 1 | Fix C-07 |
| assert.h | 1 | 0 | 0 | 1 | Minimal impl |
| limits.h | Macros | N/A | ✅ | 0 | ✅ COMPLETO |
| float.h | Macros | N/A | ✅ | 0 | ✅ COMPLETO |
| stdint.h | Types+Macros | N/A | ✅ | 0 | ✅ COMPLETO |
| stdbool.h | Macros | N/A | ✅ | 0 | ✅ COMPLETO |
| stddef.h | Types+Macros | N/A | ✅ | 0 | ✅ COMPLETO |
| **TOTAL** | **~200** | **61** | **29** | **~95** | **10 fixes** |

**Resumen:** Con 10 codegen fixes + agregar ~95 funciones al IAT, la C libc queda **100% completa**.

---

*Generado para ADead-BIB v9.0 — Fase 1 C Completo*
