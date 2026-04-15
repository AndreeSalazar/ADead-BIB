# tests/c — ADead-BIB C Test Suite

> Suite completa de pruebas del compilador C de ADead-BIB
> 35 fixture files: Básico → Intermedio → Avanzado → Producción
> Cobertura total de punteros, structs, memory management, algorithms

---

## Estructura

```text
tests/c/
├── README.md
└── fixtures/
    │
    │ ── BÁSICO (01-10) ──────────────────────────────
    ├── 01_types_basic.c            Tipos: int, char, short, long, unsigned, stdint
    ├── 02_arithmetic_ops.c         Aritmética: +, -, *, /, %, ++, --, compound
    ├── 03_bitwise_ops.c            Bitwise: &, |, ^, ~, <<, >>, máscaras
    ├── 04_comparison_logical.c     Comparación: ==, !=, <, >, &&, ||, !, ternary
    ├── 05_control_flow.c           Control: if, else, for, while, switch, goto, break
    ├── 06_functions.c              Funciones: params, retorno, recursión, prototipos
    ├── 07_arrays_basic.c           Arrays: 1D, 2D, init, sort, strings como arrays
    ├── 08_strings_char.c           Strings: literals, escape, manual strlen/strcpy
    ├── 09_structs_basic.c          Structs: declaración, campos (.), typedef, nested
    ├── 10_enums_unions.c           Enums y Unions: valores, type punning, sizeof
    │
    │ ── INTERMEDIO: PUNTEROS (11-18) ────────────────
    ├── 11_pointers_basic.c         Punteros: *, &, NULL, swap, const ptr
    ├── 12_pointer_arithmetic.c     Aritmética ptr: p+n, p-q, arrays como ptrs
    ├── 13_pointer_to_pointer.c     ** y ***: doble/triple indirección, char**
    ├── 14_pointer_structs.c        Punteros a structs: ->, malloc struct, linked list
    ├── 15_function_pointers.c      Function ptrs: callbacks, dispatch table, typedef
    ├── 16_void_pointer.c           void*: generic swap, tagged values, opaque handles
    ├── 17_memory_management.c      Memoria: malloc, calloc, realloc, free, dynamic arrays
    ├── 18_cast_sizeof.c            Casts: int↔ptr, truncamiento, sizeof
    │
    │ ── INTERMEDIO: LENGUAJE (19-27) ────────────────
    ├── 19_preprocessor.c           Preprocessor: #define, macros, #ifdef, stringify
    ├── 20_ctype_full.c             <ctype.h>: isalpha, isdigit, toupper, tolower
    ├── 21_stdio_full.c             <stdio.h>: printf formats, sprintf, puts, putchar
    ├── 22_stdlib_full.c            <stdlib.h>: atoi, strtol, rand, abs, getenv
    ├── 23_string_full.c            <string.h>: str*, mem*, strtok completo
    ├── 24_math_basic.c             <math.h>: sin, cos, sqrt, pow, floor, ceil
    ├── 25_expressions_full.c       Expresiones: precedencia, comma, complex
    ├── 26_c99_features.c           C99: bool, inline, designated init, compound literals
    ├── 27_scope_lifetime.c         Scope: static, global, block scope, shadowing
    │
    │ ── AVANZADO (28-35) ────────────────────────────
    ├── 28_advanced_pointers.c      Ptr avanzados: ptr a array, fn ptr pipeline, const
    ├── 29_bitfield_packed.c        Bitfields: struct flags, Color565, register encoding
    ├── 30_algorithms.c             Algoritmos: bsearch, quicksort, stack, hash table
    ├── 31_linked_list_full.c       Lista enlazada: insert, delete, reverse, merge sort
    ├── 32_binary_tree.c            BST: insert, search, traversals, height
    ├── 33_state_machine.c          FSM: fn ptrs + enum + switch state machine
    ├── 34_memory_patterns.c        Memoria avanzada: arena, ring buffer, pool allocator
    └── 35_production_complete.c    PRODUCCIÓN: entity system, tokenizer, hashmap, todo junto
```

## Niveles de Cobertura

| Nivel | Tests | Qué Cubre |
|-------|-------|-----------|
| **Básico** | 01-10 | Tipos, operadores, control flow, funciones, arrays, structs, enums |
| **Punteros** | 11-18 | *, &, **, ***, ->, fn ptrs, void*, malloc/free, casts |
| **Lenguaje** | 19-27 | Preprocessor, stdlib headers, C99, scope/lifetime |
| **Avanzado** | 28-35 | Algorithms, data structures, memory patterns, production code |

## Cobertura de Punteros (Foco Principal)

| Concepto | Test(s) | Detalle |
|----------|---------|---------|
| `*p` deref, `&x` address-of | 11 | Básico: leer/escribir via puntero |
| `p++`, `p+n`, `p-q` | 12 | Aritmética de punteros completa |
| `int**`, `int***` | 13 | Doble/triple indirección |
| `struct->field`, malloc struct | 14 | Punteros a structs, linked list |
| `int (*fn)(int)`, callbacks | 15 | Function pointers, dispatch tables |
| `void*` casting, generic ops | 16 | Type erasure, opaque handles |
| malloc/calloc/realloc/free | 17 | Dynamic arrays, 2D matrices |
| Casts: int↔ptr, uintptr_t | 18 | Todos los casts válidos |
| `int (*arr)[N]`, `const int*` | 28 | Ptr a array fijo, const correctness |
| Arena/pool/ring via ptrs | 34 | Memory allocators con void* |
| Entity system con fn ptrs | 35 | Producción: structs + ptrs + fn ptrs |

## Cómo Usar

```bash
# Compilar un fixture
adB cc tests/c/fixtures/11_pointers_basic.c -o test.exe

# Compilar y ejecutar
adB run tests/c/fixtures/35_production_complete.c

# Run Rust unit tests
cargo test -p adeb-frontend-c

# Ejecutar todos los tests de producción
for /L %i in (28,1,35) do adB run tests/c/fixtures/%i_*.c
```

## Dependencias de Codegen

| Fix Necesario | Tests que lo requieren |
|---------------|----------------------|
| C-01: Struct field access | 09, 14, 29, 31, 32, 33, 34, 35 |
| C-02: Float/double SSE2 | 24 |
| C-03: Byte-level memory | 08, 23, 34 |
| C-04: Function pointers | 15, 28, 33, 35 |
| C-07: Global/static vars | 27, 35 |
| C-08: Array initializers | 07, 12, 25, 26, 28, 30 |
| C-09: Cast codegen | 16, 18 |
| C-10: sizeof codegen | 01, 18, 29 |
