# tests/c — ADead-BIB C Test Suite

> Suite de pruebas y fixtures del compilador C de ADead-BIB
> 33 fixture files covering C89/C99/C11, all 29 ISO C headers, OpenGL, production patterns
> 250+ total tests (frontend + driver/integration + production validation)

---

## Estructura

```text
tests/c/
├── README.md
└── fixtures/
    ├── 01_ctype_basic.c              ctype.h basics
    ├── 02_ctype_extended.c           ctype.h extended
    ├── 03_ctype_loop_parser.c        ctype.h real usage patterns
    ├── 04_ctype_edge_cases.c         ctype.h boundary cases
    ├── 05_control_flow.c             if/else, while, for, switch, goto
    ├── 06_pointers_arrays.c          pointers, arrays, multi-dim
    ├── 07_structs_enums.c            structs, enums, typedefs, unions
    ├── 08_preprocessor.c             #include, #define, #ifdef, macros
    ├── 09_c99_features.c             _Static_assert, inline, mixed decls
    ├── 10_c11_headers.c              fenv, stdatomic, threads, stdalign
    ├── 11_ub_detection.c             UB detector test
    ├── 12_expressions.c              All operators, casts, sizeof, ternary
    ├── 13_stdio.c                    printf, sprintf, puts, putchar
    ├── 14_stdlib.c                   malloc, free, abs, atoi
    ├── 15_string.c                   strlen, strcpy, strcmp, memset
    ├── 16_math.c                     sin, cos, sqrt, pow, floor, ceil
    ├── 17_stdint_limits.c            int8-64, INT_MAX, CHAR_BIT
    ├── 18_errno_assert.c             errno codes, assert macro
    ├── 19_signal.c                   SIGINT, raise, signal handler
    ├── 20_setjmp.c                   setjmp, longjmp, jmp_buf
    ├── 21_time.c                     time, clock, strftime
    ├── 22_locale.c                   setlocale, localeconv
    ├── 23_stdarg.c                   va_list, va_arg variadic funcs
    ├── 24_complex.c                  _Complex, creal, cimag, cabs
    ├── 25_wchar_wctype.c             wchar_t, wcslen, iswalpha
    ├── 26_iso646_stdalign_noreturn.c and/or/not, alignof, noreturn
    ├── 27_uchar_tgmath.c             char16_t, char32_t
    ├── 28_production_types.c         [PROD] sizeof, limits, casting
    ├── 29_production_memory.c        [PROD] malloc/calloc/realloc/free
    ├── 30_production_strings.c       [PROD] all string operations
    ├── 31_production_control.c       [PROD] fib, bsearch, primes, FSM
    ├── 32_production_structs.c       [PROD] linked list, fn pointers
    └── 33_production_opengl_parse.c  [PROD] GL types, constants, pipeline
```

## Test Categories

| Category | Fixtures | Description |
|---|---|---|
| stdlib (C89) | 01-04, 13-18 | Standard library headers with strict validation |
| language | 05-07, 12 | Control flow, pointers, structs, expressions |
| preprocessor | 08 | #include, #define, #ifdef, macros |
| C99/C11 | 09-10, 24-27 | C99/C11 features and headers |
| UB | 11 | Undefined behavior detection |
| signal/setjmp/time/locale/stdarg | 19-23 | Remaining C89 headers |
| **PRODUCTION** | **28-33** | **Strict pass/fail tests for .exe validation** |

## ISO C Header Coverage (29/29 = 100%)

Every ISO C header has at least one dedicated test fixture.

## How to use

```bash
# Compile a fixture
adB cc tests/c/fixtures/05_control_flow.c -o test.exe

# Compile and run
adB run tests/c/fixtures/28_production_types.c

# Run production test suite (strict — returns exit code on failure)
for /L %i in (28,1,33) do adB run tests/c/fixtures/%i_production_*.c

# Run Rust unit tests
cargo test -p adeb-frontend-c
```
