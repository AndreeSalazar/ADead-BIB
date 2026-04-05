# ADead-BIB × ASM-BIB Bridge Test Suite

## Test Programs (Basic → Advanced)

| # | File | Level | Tests |
|---|------|-------|-------|
| 01 | `01_console_hello.c` | Basic | printf, return codes |
| 02 | `02_string_ops.c` | Basic | strlen, strcpy, strcmp, memcpy, memset via ASM-BIB bridge |
| 03 | `03_math_logic.c` | Basic | abs, min, max, clamp, swap, bitops |
| 04 | `04_memory_alloc.c` | Intermediate | malloc, free, calloc, realloc patterns |
| 05 | `05_control_flow.c` | Intermediate | if/else, for, while, switch, goto, nested loops |
| 06 | `06_structs_unions.c` | Intermediate | struct layout, union access, typedef, enum |
| 07 | `07_pointers_arrays.c` | Intermediate | pointer arithmetic, multi-dim arrays, function pointers |
| 08 | `08_file_io.c` | Intermediate | fopen, fread, fwrite, fclose |
| 09 | `09_win32_window.c` | Advanced | CreateWindowExA, message loop, WndProc |
| 10 | `10_gdi_drawing.c` | Advanced | SetPixel, Rectangle, GDI brushes |
| 11 | `11_opengl_triangle.c` | Advanced | wglCreateContext, glBegin/glEnd, GL 1.1 |
| 12 | `12_bridge_combined.c` | Expert | All ASM-BIB functions called from C, full bridge test |

## Run

```bash
cargo test -p adeb-bridge --test asm_bib_full_test -- --nocapture
```
