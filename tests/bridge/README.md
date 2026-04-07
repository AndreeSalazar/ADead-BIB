# ADead-BIB × ASM-BIB Bridge Test Suite

## Test Programs (Basic → Advanced)

| # | File | Level | Tests | Status |
|---|------|-------|-------|--------|
| 01 | `01_console_hello.c` | Basic | printf, return codes | ✅ PASS |
| 02 | `02_string_ops.c` | Basic | strlen, strcpy, strcmp, strcat, strchr, memcpy, memset, memcmp | ⚠️ PARTIAL (10/14) |
| 03 | `03_math_logic.c` | Basic | abs, min, max, clamp, swap, popcount, bsr, bsf, bswap | ✅ PASS (12/12) |
| 04 | `04_memory_alloc.c` | Intermediate | malloc, free, calloc, realloc patterns | ⚠️ PARTIAL (4/7) |
| 05 | `05_control_flow.c` | Intermediate | if/else, for, while, switch, fibonacci, primes | ✅ PASS (11/11) |
| 06 | `06_structs_unions.c` | Intermediate | struct layout, union access, typedef, enum | ❌ CRASH |
| 07 | `07_pointers_arrays.c` | Intermediate | pointer arithmetic, multi-dim arrays, function pointers | ❌ CRASH |
| 08 | `08_win32_window.c` | Advanced | RegisterClassA, CreateWindowExA, message loop | ✅ PASS |
| 09 | `09_gdi_drawing.c` | Advanced | SetPixel, Rectangle, GDI brushes | ✅ PASS |
| 10 | `10_opengl_basic.c` | Advanced | wglCreateContext, glBegin/glEnd, GL 1.1 triangle | ✅ PASS |
| 11 | `11_dx9_window.c` | Expert | Direct3DCreate9, vtable calls, DX9 render loop | ❌ INVALID PE |
| 12 | `12_dx11_window.c` | Expert | D3D11CreateDeviceAndSwapChain, vtable calls | ❌ INVALID PE |
| 13 | `13_dx12_window.c` | Expert | D3D12CreateDevice, DXGI Factory, COM | ❌ INVALID PE |

**Totals:** 6 PASS · 2 PARTIAL · 5 FAIL · 13/13 compile (100%)

## Build & Run

```bash
# Build the compiler
cd src/rust
cargo build --release

# Compile a test
adB cc tests/bridge/01_console_hello.c -o 01_console_hello.exe

# Compile + run
adB run tests/bridge/01_console_hello.c

# Step mode (show all compiler phases)
adB cc tests/bridge/01_console_hello.c -o test.exe -step
```

## Full Report

See [reportes_test.md](reportes_test.md) for detailed results and analysis.
