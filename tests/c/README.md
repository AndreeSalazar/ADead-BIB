# tests/c — ADead-BIB C Test Suite

> Suite de pruebas y fixtures del compilador C de ADead-BIB
> 12 fixture files covering C99/C11, UB detection, all expression types
> 157 total tests (124 frontend + 33 driver/integration)

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
    └── 12_expressions.c          All operators, casts, sizeof, ternary, compound assign
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

## How to use

```bash
# Compile a fixture
adB cc tests/c/fixtures/05_control_flow.c -o test.exe

# Compile and run
adB run tests/c/fixtures/05_control_flow.c

# Step mode — see every compiler phase
adB step tests/c/fixtures/05_control_flow.c

# Step mode with C++ (preview)
adB cxx app.cpp -step

# Run Rust tests (validates all fixtures automatically)
cargo test -p ADead-BIB-Main
cargo test -p adeb-frontend-c
```

## Naming convention

```text
XX_category_description.c
│  │         │
│  │         └── what is being tested
│  └── category (ctype, control, pointer, struct, preprocessor, c99, c11, ub, expr)
└── sequential number
```
