# ADead-BIB × ASM-BIB Bridge Test Suite

## Test Programs (Basic → Advanced → Expert)

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
| 14 | `14_file_io.c` | Intermediate | fopen, fprintf, fread, fseek, ftell, remove | 🔵 NEW |
| 15 | `15_threads.c` | Advanced | CreateThread, WaitForSingleObject, InterlockedIncrement | 🔵 NEW |
| 16 | `16_vulkan_window.c` | Expert | vkCreateInstance, vkEnumeratePhysicalDevices | 🔵 NEW |
| 17 | `17_recursion_deep.c` | Basic | Ackermann, factorial, binary search, mutual recursion | 🔵 NEW |
| 18 | `18_bitfields.c` | Intermediate | struct bitfields, packed structs, union type-punning | 🔵 NEW |
| 19 | `19_variadic.c` | Intermediate | va_list, va_start, va_arg, va_end | 🔵 NEW |
| 20 | `20_enum_switch.c` | Basic | large enums, exhaustive switch/case | 🔵 NEW |
| 21 | `21_linked_list.c` | Intermediate | singly linked list, malloc, pointer chasing | 🔵 NEW |
| 22 | `22_signal_handler.c` | Intermediate | signal(), raise(), SIG_DFL, SIG_IGN | 🔵 NEW |
| 23 | `23_float_math.c` | Intermediate | float/double arithmetic, Newton sqrt, precision | 🔵 NEW |

**Totals:** 6 PASS · 2 PARTIAL · 5 FAIL · 10 NEW · 23/23 files (100%)

## Test Levels

| Level | Tests | Description |
|-------|-------|-------------|
| **Basic** | 01, 03, 17, 20 | Core C: printf, arithmetic, recursion, enums |
| **Intermediate** | 02, 04, 05, 06, 07, 14, 18, 19, 21, 22, 23 | Memory, control flow, structs, file I/O, variadic, linked list, signals, floats |
| **Advanced** | 08, 09, 10, 15 | Win32 API: windows, GDI, OpenGL, threads |
| **Expert** | 11, 12, 13, 16 | DirectX 9/11/12, Vulkan — vtable/COM/indirect calls |

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

# Run all bridge tests
for /L %i in (1,1,23) do adB run tests/bridge/%02i_*.c
```

## Full Report

See [reportes_test.md](reportes_test.md) for detailed results and analysis.
