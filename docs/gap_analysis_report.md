# ADead-BIB + ASM-BIB — Gap Analysis Report

> Full analysis of what EXISTS, what's MISSING, and priorities for daily C/C++ use.
> Generated from deep inspection of `adeb-stdlib`, `adeb-backend-x64`, `adeb-bridge`, IAT registry.

---

## 1. WHAT EXISTS (Current State)

### A. C Standard Library (`adeb-stdlib/src/c/`)

| Header | File | Status | Notes |
|--------|------|--------|-------|
| `<stdio.h>` | `fastos_stdio.rs` | **DECLARATIONS** | 27 functions listed, routed to msvcrt.dll printf/scanf |
| `<stdlib.h>` | `fastos_stdlib.rs` | **DECLARATIONS** | 29 functions listed, malloc/free via msvcrt IAT |
| `<string.h>` | `fastos_string.rs` | **DECLARATIONS** | 23 functions listed |
| `<math.h>` | `fastos_math.rs` | **DECLARATIONS** | sin/cos/sqrt/pow listed |
| `<time.h>` | `fastos_time.rs` | **DECLARATIONS** | time/clock/strftime listed |
| `<ctype.h>` | `fastos_ctype.rs` | **FULL IMPL** | isalpha/isdigit/toupper etc — fully implemented |
| `<limits.h>` | `fastos_limits.rs` | **FULL IMPL** | INT_MAX, CHAR_BIT etc |
| `<errno.h>` | `fastos_errno.rs` | **DECLARATIONS** | errno codes listed |
| `<assert.h>` | `fastos_assert.rs` | **DECLARATIONS** | assert macro |
| `<stdint.h>` | `fastos_types.rs` | **FULL IMPL** | int8_t..int64_t, uint types |
| `<float.h>` | (in types) | **PARTIAL** | FLT_MAX etc |

### B. C++ Standard Library (`adeb-stdlib/src/cpp/`)

| Header | File | Status |
|--------|------|--------|
| `<iostream>` | `fastos_iostream.rs` | HPP template with cout/cin |
| `<vector>` | `fastos_vector.rs` | HPP template |
| `<string>` | `fastos_string_cpp.rs` | HPP template |
| `<map>` | `fastos_map.rs` | HPP template |
| `<algorithm>` | `fastos_algorithm.rs` | HPP template |
| `<memory>` | `fastos_memory.rs` | HPP template (unique_ptr/shared_ptr) |
| `<functional>` | `fastos_functional.rs` | HPP template |
| `<thread>` | `fastos_thread.rs` | HPP template |
| `<mutex>` | `fastos_mutex.rs` | HPP template |
| `<filesystem>` | `fastos_filesystem.rs` | HPP template |
| `<chrono>` | `fastos_chrono.rs` | HPP template |
| `<optional>` | `fastos_optional.rs` | HPP template |
| `<variant>` | `fastos_variant.rs` | HPP template |
| +20 more | ... | HPP templates |

### C. GPU Libraries (`adeb-stdlib/src/gpu/`)

| Library | Status | Detail |
|---------|--------|--------|
| OpenGL 1.0-4.6 | **FULL DECLARATIONS** | All GL versions, 80K+ lines of constants/types |
| Vulkan | **FULL DECLARATIONS** | 70K lines, complete VkInstance/VkDevice/VkPipeline |
| GLSL shader bridge | **DECLARATIONS** | Shader compilation bridge |

### D. Win32 API (IAT Registry)

| DLL | Functions | Status |
|-----|-----------|--------|
| `msvcrt.dll` | printf, scanf, malloc, free, memset, memcpy | **IN IAT** |
| `kernel32.dll` | GetModuleHandleA, LoadLibraryA, GetProcAddress, Sleep, ExitProcess, GetLastError | **IN IAT** |
| `user32.dll` | RegisterClassA, CreateWindowExA, ShowWindow, PeekMessageA, TranslateMessage, DispatchMessageA, PostQuitMessage, DefWindowProcA, DestroyWindow, GetDC, ReleaseDC, MessageBoxA | **IN IAT** |
| `gdi32.dll` | SwapBuffers, ChoosePixelFormat, SetPixelFormat, SetPixel, CreateSolidBrush, DeleteObject, SelectObject, Rectangle | **IN IAT** |
| `opengl32.dll` | wglCreateContext, wglMakeCurrent, wglDeleteContext, wglGetProcAddress, glClear, glClearColor, glEnable, glDisable, glDepthFunc, glShadeModel, glViewport, glMatrixMode, glLoadIdentity, glTranslatef, glRotatef, glScalef, glFrustum, glBegin, glEnd, glVertex3f, glColor3f, glColor4f, glNormal3f, glLightfv, glMaterialfv, glMaterialf, glColorMaterial, glFlush, glGetString, glGetError | **IN IAT** |

### E. ASM-BIB Bridge (21 functions)

All verified working via `.obj` import:
- String: asm_strlen, asm_strcpy, asm_strcmp, asm_strcat, asm_strchr, asm_memcpy, asm_memset, asm_memcmp
- Math: asm_abs, asm_min, asm_max, asm_clamp, asm_swap
- Bit: asm_popcount, asm_bsr64, asm_bsf64, asm_bswap32, asm_bswap64
- Utility: asm_is_aligned, asm_align_up, asm_noop

---

## 2. WHAT'S MISSING (Gaps for Daily Use)

### PRIORITY 1 — CRITICAL (Blocks basic applications)

| Gap | Why it matters | Effort |
|-----|---------------|--------|
| **`d3d9.dll` IAT** | DirectX 9 — most legacy games/tools use it | Medium |
| **`d3d11.dll` IAT** | DirectX 11 — primary modern DX API | Medium |
| **`d3d12.dll` IAT** | DirectX 12 — high-performance rendering | Medium |
| **`dxgi.dll` IAT** | DXGI — required for all DX10+ swap chains | Medium |
| **`d3dcompiler_47.dll` IAT** | HLSL shader compilation | Low |
| **msvcrt.dll expansion**: `fopen`, `fclose`, `fread`, `fwrite`, `fprintf`, `sprintf`, `snprintf`, `sscanf`, `atoi`, `atof`, `strtol`, `strtod`, `qsort`, `bsearch`, `abs`, `rand`, `srand`, `exit`, `getenv` | File I/O, string conversion, sort — essential for any real program | Medium |
| **Float codegen** | `float`/`double` arithmetic in ISA compiler — needed for OpenGL, DX, math | High |

### PRIORITY 2 — IMPORTANT (Blocks intermediate applications)

| Gap | Why it matters | Effort |
|-----|---------------|--------|
| **`ws2_32.dll` IAT** (Winsock) | Networking: socket, connect, send, recv, bind, listen, accept | Medium |
| **`advapi32.dll` IAT** | Registry: RegOpenKeyExA, RegQueryValueExA, RegSetValueExA | Low |
| **`shell32.dll` IAT** | Shell: ShellExecuteA, SHGetFolderPathA | Low |
| **`ole32.dll` + `oleaut32.dll` IAT** | COM: CoInitializeEx, CoCreateInstance (needed for DX) | Medium |
| **`winmm.dll` IAT** | Audio: PlaySoundA, waveOutOpen, timeGetTime | Low |
| **user32.dll expansion**: `SetWindowTextA`, `GetWindowTextA`, `GetClientRect`, `InvalidateRect`, `UpdateWindow`, `SetTimer`, `KillTimer`, `GetKeyState`, `GetAsyncKeyState`, `LoadCursorA`, `LoadIconA`, `GetSystemMetrics`, `AdjustWindowRect`, `MoveWindow` | Window management, input handling, timers | Medium |
| **gdi32.dll expansion**: `CreateFontA`, `TextOutA`, `BitBlt`, `CreateCompatibleDC`, `CreateCompatibleBitmap`, `GetObject`, `CreatePen`, `MoveToEx`, `LineTo`, `Ellipse`, `Polygon`, `SetBkMode`, `SetTextColor` | Text rendering, advanced 2D drawing | Medium |

### PRIORITY 3 — NICE TO HAVE (Professional polish)

| Gap | Why it matters | Effort |
|-----|---------------|--------|
| **`<windows.h>` typedef header** | Single include for all Win32 types (HANDLE, DWORD, BOOL, etc.) | Low |
| **`<d3d9.h>` / `<d3d11.h>` / `<d3d12.h>` headers** | DirectX C/C++ headers with structs/interfaces | High |
| **`<xinput.h>` IAT** | Gamepad input: XInputGetState, XInputSetState | Low |
| **`<dsound.h>` IAT** | DirectSound audio | Medium |
| **Struct-by-value passing** | Needed for RECT, POINT, SIZE params in Win32 | Medium |
| **Multi-file compilation** | #include resolving across .c files | High |
| **Static/global initializers** | `int x = func();` at global scope | Medium |
| **Variadic functions** | va_list/va_arg for custom printf-like functions | High |

---

## 3. WHAT YOU DON'T NEED TO EXPAND IN ASM-BIB

ASM-BIB is already sufficient for the bridge role:
- **21 optimized utility functions** cover everyday string/math/bit operations
- **Ring 0/1/2 instructions** already encoded (CR/DR registers, GDT/IDT, MSR, I/O)
- **Ring 3 instructions** cover all standard x86-64 (MOV, ADD, SUB, CMP, SSE, AVX, CMOVcc, SETcc, shifts, string ops, bit manipulation)
- **COFF .obj export** is production-quality

**No further ASM-BIB expansion needed** for the C/C++ daily use case. The bridge is complete.

Future ASM-BIB expansion only if you want:
- More specialized SIMD routines (AVX-512)
- Crypto intrinsics (AES-NI, SHA)
- Math library (sin/cos/sqrt in assembly)

---

## 4. RECOMMENDED EXPANSION ROADMAP

### Phase 1: msvcrt.dll expansion (enables tests 01-07)
Add to IAT: `fopen`, `fclose`, `fread`, `fwrite`, `fprintf`, `sprintf`, `snprintf`, `sscanf`, `atoi`, `atof`, `strtol`, `strtod`, `qsort`, `bsearch`, `abs`, `rand`, `srand`, `exit`, `getenv`, `system`, `putchar`, `puts`, `fgets`, `fputs`

### Phase 2: DirectX core DLLs (enables DX9/11/12 programs)
Add new DLL imports:
- `d3d9.dll`: Direct3DCreate9
- `d3d11.dll`: D3D11CreateDevice, D3D11CreateDeviceAndSwapChain
- `d3d12.dll`: D3D12CreateDevice, D3D12GetDebugInterface, D3D12SerializeRootSignature
- `dxgi.dll`: CreateDXGIFactory1, CreateDXGIFactory2
- `d3dcompiler_47.dll`: D3DCompile, D3DCompileFromFile
- `ole32.dll`: CoInitializeEx, CoCreateInstance, CoUninitialize

### Phase 3: Win32 expansion (enables real desktop apps)
Expand user32/gdi32/kernel32 IAT slots with the functions listed in Priority 2.

### Phase 4: Networking & Audio
- `ws2_32.dll` for Winsock
- `winmm.dll` for basic audio

---

## 5. CURRENT TEST COVERAGE

### Bridge Tests (`tests/bridge/`)
| Test | Level | What it tests |
|------|-------|---------------|
| `01_console_hello.c` | Basic | printf, arithmetic, return codes |
| `02_string_ops.c` | Basic | strlen, strcpy, strcmp, strcat, strchr, memcpy, memset, memcmp |
| `03_math_logic.c` | Basic | abs, min/max, clamp, swap, popcount, bsr, bsf, bswap |
| `04_memory_alloc.c` | Intermediate | malloc, free, calloc, realloc, large alloc |
| `05_control_flow.c` | Intermediate | if/else, for, while, switch, fibonacci, primes, break/continue |
| `06_structs_unions.c` | Intermediate | struct, union, enum, typedef, nested, arrays of structs |
| `07_pointers_arrays.c` | Intermediate | pointer arithmetic, 2D arrays, function pointers, void*, ptr-to-ptr |
| `08_win32_window.c` | Advanced | RegisterClassA, CreateWindowExA, message loop, WndProc |
| `09_gdi_drawing.c` | Advanced | GetDC, SetPixel gradient, Rectangle, brushes |
| `10_opengl_basic.c` | Advanced | wglCreateContext, GL 1.1 triangle, SwapBuffers |

### Existing C Tests (`tests/c/fixtures/`)
33 fixture files covering all 29 ISO C headers, production patterns.

### Bridge Rust Tests (`adeb-bridge`)
33 tests: COFF parsing, symbol resolution, merge, call patching — all PASS.

---

## 6. SUMMARY

| Area | Score | Notes |
|------|-------|-------|
| C Language (parsing) | **95%** | Missing: _Generic, compound literals |
| C Stdlib (declarations) | **80%** | Headers declared, need more msvcrt IAT slots |
| C Stdlib (runtime) | **40%** | Only printf/scanf/malloc/free/memset/memcpy actually callable |
| C++ STL | **70%** | HPP templates exist, need actual codegen |
| Win32 API | **60%** | Core window/input/GDI works, need expansion |
| OpenGL | **70%** | GL 1.1 works via IAT, modern GL needs wglGetProcAddress loading |
| DirectX 9 | **0%** | No d3d9.dll IAT, no headers |
| DirectX 11 | **0%** | No d3d11.dll IAT, no headers |
| DirectX 12 | **10%** | Some struct work done, no DLL IAT |
| ASM-BIB Bridge | **100%** | Complete: 21 functions, COFF export, merge verified |
| Float codegen | **30%** | SSE registers exist, codegen incomplete |
| Networking | **0%** | No ws2_32.dll IAT |
| Audio | **0%** | No winmm/dsound IAT |

**Bottom line:** ADead-BIB + ASM-BIB can today build console apps, Win32 windows, GDI drawing, and GL 1.1 rendering. The main gaps for daily C/C++ use are: (1) more msvcrt IAT slots, (2) float codegen, (3) DirectX DLL imports.
