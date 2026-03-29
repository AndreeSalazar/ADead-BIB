# tests/c — ADead-BIB C Test Suite

> Suite de pruebas y fixtures del compilador C de ADead-BIB
> 27 fixture files covering C89/C99/C11, all 29 ISO C headers, UB detection, all expression types
> 200+ total tests (frontend + driver/integration)

---

## Estructura

```text
tests/c/
├── README.md
└── fixtures/
    ├── 01_ctype_basic.c          ctype.h basics
    ├── 02_ctype_extended.c       ctype.h extended
    ├── 03_ctype_loop_parser.c    ctype.h real usage patterns
    ├── 04_ctype_edge_cases.c     ctype.h boundary cases
    ├── 05_control_flow.c         if/else, while, do-while, for, switch, goto/label
    ├── 06_pointers_arrays.c      pointers, arrays, multi-dim, string ops
    ├── 07_structs_enums.c        structs, enums, typedefs, unions
    ├── 08_preprocessor.c         #include, #define, #ifdef, macros
    ├── 09_c99_features.c         _Static_assert, inline, mixed decls, nested init
    ├── 10_c11_headers.c          fenv.h, stdatomic.h, threads.h, stdalign.h
    ├── 11_ub_detection.c         Intentional UB for testing the detector
    ├── 12_expressions.c          All operators, casts, sizeof, ternary, compound assign
    ├── 13_stdio.c                printf, sprintf, puts, putchar (stdio.h)
    ├── 14_stdlib.c               malloc, free, abs, atoi (stdlib.h)
    ├── 15_string.c               strlen, strcpy, strcmp, memset (string.h)
    ├── 16_math.c                 sin, cos, sqrt, pow, floor, ceil (math.h)
    ├── 17_stdint_limits.c        Fixed-width types, INT_MAX, CHAR_BIT (stdint/limits/float)
    ├── 18_errno_assert.c         errno codes, assert macro (errno.h, assert.h)
    ├── 19_signal.c               SIGINT, raise, signal handler (signal.h)
    ├── 20_setjmp.c               setjmp, longjmp, jmp_buf (setjmp.h)
    ├── 21_time.c                 time, clock, strftime, struct tm (time.h)
    ├── 22_locale.c               setlocale, localeconv (locale.h)
    ├── 23_stdarg.c               va_list, va_start, va_arg, va_end (stdarg.h)
    ├── 24_complex.c              _Complex, creal, cimag, cabs (complex.h)
    ├── 25_wchar_wctype.c         wchar_t, wcslen, iswalpha, towupper (wchar/wctype)
    ├── 26_iso646_stdalign_noreturn.c  and/or/not, alignof, noreturn (iso646/stdalign/stdnoreturn)
    └── 27_uchar_tgmath.c         char16_t, char32_t (uchar.h)
```

## Fixtures

| # | Fixture | Category | Description | Status |
|---|---|---|---|---|
| 01 | `01_ctype_basic.c` | stdlib | isalpha, isdigit, isalnum, isspace | ✅ |
| 02 | `02_ctype_extended.c` | stdlib | isprint, isgraph, iscntrl, ispunct, isxdigit | ✅ |
| 03 | `03_ctype_loop_parser.c` | stdlib | Real usage: classify string, parse_hex, validate id | ✅ |
| 04 | `04_ctype_edge_cases.c` | stdlib | Boundary: NUL, EOF, 0x1F, 0x7F | ✅ |
| 05 | `05_control_flow.c` | language | if/else, while, do-while, for, switch, goto/label | ✅ |
| 06 | `06_pointers_arrays.c` | language | Pointers, arrays, multi-dim, string ops | ✅ |
| 07 | `07_structs_enums.c` | language | Structs, enums, typedefs, unions | ✅ |
| 08 | `08_preprocessor.c` | preprocessor | #include, #define, #ifdef, variadic macros | ✅ |
| 09 | `09_c99_features.c` | C99/C11 | _Static_assert, inline, mixed decls, nested init | ✅ |
| 10 | `10_c11_headers.c` | C11 | fenv.h, stdatomic.h, threads.h, stdalign.h | ✅ |
| 11 | `11_ub_detection.c` | UB | Division by zero, shift overflow, negative index | ✅ |
| 12 | `12_expressions.c` | language | All operators, casts, sizeof, ternary, compound assign | ✅ |
| 13 | `13_stdio.c` | stdlib | printf, sprintf, puts, putchar | ✅ |
| 14 | `14_stdlib.c` | stdlib | malloc, free, abs, atoi | ✅ |
| 15 | `15_string.c` | stdlib | strlen, strcpy, strcmp, memset | ✅ |
| 16 | `16_math.c` | stdlib | sin, cos, sqrt, pow, floor, ceil, fabs, log | ✅ |
| 17 | `17_stdint_limits.c` | stdlib | int8_t-int64_t, INT_MAX, CHAR_BIT, FLT_MAX | ✅ |
| 18 | `18_errno_assert.c` | stdlib | errno, EDOM, assert() | ✅ |
| 19 | `19_signal.c` | stdlib | SIGINT, SIGTERM, signal(), raise() | ✅ |
| 20 | `20_setjmp.c` | stdlib | setjmp, longjmp, jmp_buf | ✅ |
| 21 | `21_time.c` | stdlib | time, clock, strftime, CLOCKS_PER_SEC | ✅ |
| 22 | `22_locale.c` | stdlib | setlocale, localeconv, struct lconv | ✅ |
| 23 | `23_stdarg.c` | stdlib | va_list, va_start, va_arg, va_end, variadic funcs | ✅ |
| 24 | `24_complex.c` | C99 | _Complex, creal, cimag, cabs, conj | ✅ |
| 25 | `25_wchar_wctype.c` | C99 | wchar_t, wcslen, iswalpha, towupper | ✅ |
| 26 | `26_iso646_stdalign_noreturn.c` | C99/C11 | and/or/not macros, alignof, noreturn | ✅ |
| 27 | `27_uchar_tgmath.c` | C11 | char16_t, char32_t, mbrtoc16 | ✅ |

## ISO C Header Coverage (29/29)

| Header | Test Fixture(s) |
|---|---|
| `assert.h` | 18_errno_assert.c |
| `complex.h` | 24_complex.c |
| `ctype.h` | 01-04_ctype*.c |
| `errno.h` | 18_errno_assert.c |
| `fenv.h` | 10_c11_headers.c |
| `float.h` | 17_stdint_limits.c |
| `inttypes.h` | 17_stdint_limits.c |
| `iso646.h` | 26_iso646_stdalign_noreturn.c |
| `limits.h` | 17_stdint_limits.c |
| `locale.h` | 22_locale.c |
| `math.h` | 16_math.c |
| `setjmp.h` | 20_setjmp.c |
| `signal.h` | 19_signal.c |
| `stdalign.h` | 26_iso646_stdalign_noreturn.c |
| `stdarg.h` | 23_stdarg.c |
| `stdatomic.h` | 10_c11_headers.c |
| `stdbool.h` | 09_c99_features.c |
| `stddef.h` | 08_preprocessor.c |
| `stdint.h` | 17_stdint_limits.c |
| `stdio.h` | 13_stdio.c |
| `stdlib.h` | 14_stdlib.c |
| `stdnoreturn.h` | 26_iso646_stdalign_noreturn.c |
| `string.h` | 15_string.c |
| `tgmath.h` | 27_uchar_tgmath.c |
| `threads.h` | 10_c11_headers.c |
| `time.h` | 21_time.c |
| `uchar.h` | 27_uchar_tgmath.c |
| `wchar.h` | 25_wchar_wctype.c |
| `wctype.h` | 25_wchar_wctype.c |

## How to use

```bash
# Compile a fixture
adB cc tests/c/fixtures/05_control_flow.c -o test.exe

# Compile and run
adB run tests/c/fixtures/05_control_flow.c

# Step mode — see every compiler phase
adB step tests/c/fixtures/05_control_flow.c

# Run Rust tests (validates all fixtures automatically)
cargo test -p ADead-BIB-Main
cargo test -p adeb-frontend-c
```

## Naming convention

```text
XX_category_description.c
│  │         │
│  │         └── what is being tested
│  └── category (ctype, control, pointer, struct, preprocessor, c99, c11, ub, expr, stdio, stdlib, ...)
└── sequential number
```
