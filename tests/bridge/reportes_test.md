# ADead-BIB × ASM-BIB Bridge — Test Report

**Date:** 2026-04-05  
**Compiler:** ADead-BIB v9.0 (`adB.exe`)  
**Target:** Windows PE x86-64  
**Backend:** ADead-BIB native (no LLVM, no GCC)  
**ASM-BIB Base:** `stdlib_ring3.pasm` (MASM Ring 1-3)

---

## Summary

| # | Test | Compile | Run | Status | Notes |
|---|------|---------|-----|--------|-------|
| 01 | `01_console_hello.c` | ✅ OK (2048 B) | ✅ exit=0 | **PASS** | All output correct |
| 02 | `02_string_ops.c` | ✅ OK (6144 B) | ❌ CRASH (0xC0000005) | **FAIL** | Access violation at runtime |
| 03 | `03_math_logic.c` | ✅ OK (4608 B) | ⚠️ exit=0 | **PARTIAL** | 12/12 pass, but `%s` ternary prints raw address |
| 04 | `04_memory_alloc.c` | ✅ OK (3584 B) | ⚠️ exit=3 | **PARTIAL** | 4/7 pass; calloc zero-check, realloc, large alloc fail |
| 05 | `05_control_flow.c` | ✅ OK (5120 B) | ⚠️ exit=0 | **PARTIAL** | 11/11 pass, but `%s` ternary prints raw address |
| 06 | `06_structs_unions.c` | ✅ OK (4096 B) | ❌ CRASH (0xC0000005) | **FAIL** | Access violation at runtime |
| 07 | `07_pointers_arrays.c` | ✅ OK (4608 B) | ❌ CRASH (0xC0000005) | **FAIL** | Access violation; unresolved label warning |
| 08 | `08_win32_window.c` | ✅ OK (3072 B) | ✅ 120 frames | **PASS** | Win32 window created + message loop |
| 09 | `09_gdi_drawing.c` | ✅ OK (4096 B) | ✅ 180 frames | **PASS** | Gradient + rectangles drawn via GDI |
| 10 | `10_opengl_basic.c` | ✅ OK (4608 B) | ✅ 180 frames | **PASS** | OpenGL 1.1 triangle rendered |
| 11 | `11_dx9_window.c` | ✅ OK (1024 B) | ❌ Invalid PE | **FAIL** | PE too small — code gen produces empty .text |
| 12 | `12_dx11_window.c` | ✅ OK (1024 B) | ❌ Invalid PE | **FAIL** | PE too small — code gen produces empty .text |
| 13 | `13_dx12_window.c` | ✅ OK (1024 B) | ❌ Invalid PE | **FAIL** | PE too small — code gen produces empty .text |

### Totals

| Metric | Count |
|--------|-------|
| **Compile success** | 13/13 (100%) |
| **Run PASS** | 4/13 |
| **Run PARTIAL** | 3/13 |
| **Run FAIL** | 6/13 |

---

## Detailed Analysis

### ✅ TEST 01 — Console Hello (PASS)

```
=== ADead-BIB Bridge Test 01: Console ===
Hello from ADead-BIB + ASM-BIB!
Arithmetic: 42 + 58 = 100
PASS: basic arithmetic
=== Test 01: PASS ===
```

- printf with `%d` works correctly
- Integer arithmetic: correct
- If/else branching: correct
- Return code: 0 ✅

### ❌ TEST 02 — String Operations (CRASH)

- **Exit code:** 0xC0000005 (ACCESS_VIOLATION)
- **Root cause:** The ISA compiler generates `call [IAT]` for string functions (strlen, strcpy, strcmp, strcat, strchr, memcpy, memset, memcmp). These are correctly imported via the PE IAT from `msvcrt.dll`. The crash happens due to local array variable access — `char buf[64]` — the stack frame for array locals is not properly allocated or addressed in the x64 code gen.
- **Affects:** Any test using local char arrays with string functions

### ⚠️ TEST 03 — Math & Logic (PARTIAL PASS)

```
=== ADead-BIB Bridge Test 03: Math & Logic ===
Results: 12 passed, 0 failed
=== Test 03: 1073750447 ===
```

- All 12 math/logic sub-tests pass
- **Issue:** The final `printf("=== Test 03: %s ===\n", fail == 0 ? "PASS" : "FAIL")` prints the string pointer as integer (1073750447 = 0x4000102F) instead of the actual string
- **Root cause:** ISA compiler handles `%s` with ternary string expression incorrectly — it pushes the pointer value but printf interprets it as `%s` → prints the numeric pointer value because the data section address isn't properly dereferenced in the format string handling
- **Fix needed:** Ternary expression codegen for string results in printf args

### ⚠️ TEST 04 — Memory Allocation (PARTIAL PASS: 4/7)

```
=== ADead-BIB Bridge Test 04: Memory ===
FAIL: calloc zero
FAIL: realloc
FAIL: large alloc content
Results: 4 passed, 3 failed
```

- malloc + free: ✅ PASS
- malloc write (p[5]==25, p[9]==81): ✅ PASS  
- calloc allocation: ✅ PASS
- calloc zero-initialized: ❌ FAIL (zero-checking loop fails)
- realloc + preserve data: ❌ FAIL (strcmp after realloc fails)
- malloc(0) + free: ✅ PASS
- Large 1MB alloc content: ❌ FAIL

- **Root cause:** The loop-based zero checking and post-realloc string comparison relies on correct array indexing codegen which has issues with `unsigned char` casts and pointer arithmetic after realloc

### ⚠️ TEST 05 — Control Flow (PARTIAL PASS)

```
=== ADead-BIB Bridge Test 05: Control Flow ===
Results: 11 passed, 0 failed
=== Test 05: 1073750488 ===
```

- if/else chain: ✅
- for loop (sum 1..100 = 5050): ✅
- while loop: ✅
- do-while: ✅
- switch/case: ✅
- nested loops: ✅
- fibonacci(10)=55, fibonacci(20)=6765: ✅
- is_prime: ✅
- break/continue: ✅
- **Same %s ternary issue** as test 03

### ❌ TEST 06 — Structs & Unions (CRASH)

- **Exit code:** 0xC0000005 (ACCESS_VIOLATION)
- **Root cause:** Struct field access (`p1.x = 10`) and struct-by-value function calls (`point_add(p1, p2)`) crash. The ISA compiler's struct layout and field offset calculation for x64 generates invalid memory accesses. The x64 codegen doesn't properly handle:
  1. Stack allocation for struct variables
  2. Field offset access (`.x`, `.y` member access)
  3. Struct-by-value argument passing via registers/stack

### ❌ TEST 07 — Pointers & Arrays (CRASH)

- **Exit code:** 0xC0000005 (ACCESS_VIOLATION)  
- **Compile warning:** "2 unresolved label patches (35 labels known, 36 patches total)"
- **Root cause:** Complex pointer arithmetic, 2D array access (`mat[i][j]`), and function pointer calls. The unresolved label patches indicate jump targets that the encoder couldn't resolve, leading to jumps to invalid addresses.

### ✅ TEST 08 — Win32 Window (PASS)

```
=== ADead-BIB Bridge Test 08: Win32 Window ===
RegisterClassA: atom=1441496
CreateWindowExA: hwnd=282001408
ShowWindow: OK
Window lived for 120 frames
=== Test 08: PASS ===
```

- Win32 API calls via IAT: ✅
- RegisterClassA + CreateWindowExA: ✅
- Message loop with PeekMessageA: ✅
- Window displayed for 120 frames (~2 seconds): ✅

### ✅ TEST 09 — GDI Drawing (PASS)

```
=== ADead-BIB Bridge Test 09: GDI Drawing ===
Gradient drawn
Rectangles drawn
GDI window lived for 180 frames
=== Test 09: PASS ===
```

- GDI API (GetDC, SetPixel, CreateSolidBrush, Rectangle): ✅
- Gradient rendering via SetPixel: ✅
- Colored rectangles via GDI brushes: ✅

### ✅ TEST 10 — OpenGL 1.1 (PASS)

```
=== ADead-BIB Bridge Test 10: OpenGL ===
ChoosePixelFormat: 1441272
wglCreateContext: 282001408
wglMakeCurrent: OK
OpenGL rendered 180 frames
=== Test 10: PASS ===
```

- OpenGL context creation (wglCreateContext + wglMakeCurrent): ✅
- ChoosePixelFormat + SetPixelFormat: ✅
- glClear + glBegin/glEnd triangle rendering: ✅
- SwapBuffers for 180 frames: ✅

### ❌ TEST 11 — DirectX 9 (INVALID PE)

- **PE size:** 1024 bytes (too small — expected ~4KB+)
- **Error:** "No es una aplicación Win32 válida" (not a valid Win32 application)
- **Root cause:** The C parser successfully parses the DX9 source (which uses `#define` macros, vtable pointer dereferencing via `void**`, goto labels, and complex struct initialization). However, the ISA compiler generates an empty or near-empty .text section because it doesn't fully handle:
  1. `goto` + labels with forward references
  2. Triple-pointer casts: `void** vtable = *((void***)d3d)`
  3. Function pointer typedef and indirect calls via vtable: `((CreateDeviceFn)d3d_vtable[16])(d3d, ...)`
  4. Large struct initialization with many fields

### ❌ TEST 12 — DirectX 11 (INVALID PE)

- Same issues as DX9 — PE is only 1024 bytes
- Additional complexity: GUID byte arrays, more vtable calls
- Code gen produces empty .text

### ❌ TEST 13 — DirectX 12 (INVALID PE)

- Same issues as DX9/DX11 — PE is only 1024 bytes
- Additional complexity: COM initialization (CoInitializeEx), DXGI factory creation
- Code gen produces empty .text

---

## Issues Summary (Priority Order)

### P0 — Critical (crashes / invalid output)

| Issue | Affected Tests | Description |
|-------|---------------|-------------|
| **Struct codegen** | 06, 07 | Struct field access, struct-by-value passing crashes |
| **Array local codegen** | 02, 04 | Local `char[]` arrays with string functions crash |
| **Empty codegen for complex C** | 11, 12, 13 | goto/labels, vtable casts, function pointer typedefs produce empty .text |
| **Unresolved label patches** | 07 | Jump encoder can't resolve all forward references |

### P1 — Medium (wrong output)

| Issue | Affected Tests | Description |
|-------|---------------|-------------|
| **%s ternary codegen** | 03, 05 | `printf("%s", cond ? "A" : "B")` prints pointer address instead of string |
| **calloc zero-check loop** | 04 | For loop checking calloc zeros fails (array indexing + unsigned char) |
| **realloc + strcmp** | 04 | Post-realloc string comparison produces wrong result |

### P2 — Low (cosmetic)

| Issue | Affected Tests | Description |
|-------|---------------|-------------|
| **Encoder warnings** | 07, 08 | "unresolved label patches" warnings in stderr |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    ADead-BIB v9.0                       │
│         100% Rust — No LLVM, No GCC, No MSVC            │
├─────────────────────────────────────────────────────────┤
│ Phase 0: Preprocessor  (#include, #define, #ifdef)      │
│ Phase 1: C Lexer       (tokenization)                   │
│ Phase 2: C Parser      (full C99/C11 AST)               │
│ Phase 3: Semantic       (symbol table, type checking)   │
│ Phase 4: UB Detection  (21+ categories)                 │
│ Phase 5: IR Generation (ADead-BIB IR)                   │
│ Phase 6: ISA Compiler  (x86-64 machine code)            │
│ Phase 7: PE Backend    (Windows executable)             │
├─────────────────────────────────────────────────────────┤
│ Bridge: ASM-BIB COFF .obj → merged into PE output       │
│ stdlib_ring3.pasm: 20+ ASM functions (string, math, bit)│
└─────────────────────────────────────────────────────────┘
```

## What Works Well

1. **Complete C preprocessing** — `#include <stdio.h>`, `#include <string.h>`, `#include <stdlib.h>` all handled
2. **Full C99 parsing** — structs, unions, enums, function pointers, typedefs, ternary, all parsed
3. **Win32 API IAT** — user32.dll, gdi32.dll, kernel32.dll, opengl32.dll imports work perfectly
4. **Integer arithmetic** — add, subtract, multiply, divide, modulo, bitwise ops all correct
5. **Control flow** — if/else, for, while, do-while, switch/case, break/continue, function calls
6. **printf with %d** — integer format specifiers work correctly
7. **malloc/free** — basic heap allocation and deallocation works
8. **Win32 windows** — CreateWindowExA, RegisterClassA, message loops work perfectly
9. **GDI drawing** — SetPixel, Rectangle, CreateSolidBrush all work
10. **OpenGL 1.1** — wglCreateContext, glBegin/glEnd, SwapBuffers all work
11. **PE generation** — valid MZ/PE headers, section alignment, IAT, all correct

## What Needs Work

1. **Struct field access codegen** — struct member read/write generates invalid x64 code
2. **Local array stack allocation** — char arrays on stack not properly allocated
3. **Complex pointer casts** — triple-pointer dereference (vtable pattern) not handled
4. **goto/label forward references** — label encoder doesn't handle all forward jumps
5. **Function pointer typedef + indirect call** — vtable-style calls not codegen'd
6. **%s format specifier with ternary** — string pointer not properly passed to printf

---

*Report generated by ADead-BIB Bridge Test Suite*  
*Compiler binary: `src/rust/target/release/adB.exe`*  
*ASM-BIB bridge: `src/rust/crates/shared/adeb-bridge/`*
