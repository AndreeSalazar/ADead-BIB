# ADead-BIB v12.0 💀🦈

**Compilador C Nativo: C99 → Machine Code Puro · C ABI Completo · 256-bit Nativo · Win32/Linux · OpenGL 4.6 · Vulkan 1.3 · DirectX 9/11/12**

> **100% C — Zero C++.** Todo el ecosistema es C puro con ABI nativo.  
> **ASM-BIB = MASM reconstruido** — base de árbol de ensamblador para ADead-BIB.  
> **CLI v12.0 Unificado:** `adB cc` · `adB run` · `adB step` · `adB gpu` · `adB version`  
> **IAT v6:** 18 DLLs · 340+ funciones importadas · Compact IAT · Sin 0xC0000139  
> **ASM-BIB Bridge:** 21 funciones assembly nativas enlazadas via COFF .obj  
> **Linker Especial DLL:** Genera bibliotecas nativas para Windows (.dll) y Linux (.so) sin MSVC/GCC/Clang  
> **GPU C ABI:** OpenGL 1.0-4.6 (18+ módulos) · Vulkan 1.3 (7 módulos) · GLSL · SPIR-V  
> Zero Overhead · Zero Bloat · Zero Dead Code  
> Sin NASM · Sin LLVM · Sin GCC · Sin Clang  
> Sin libc externa · Sin linker · 100% Autosuficiente  
> FASM-style: bytes directos al CPU  
> 256-bit nativo: YMM/AVX2 · SoA natural · VEX prefix  
> `#include <header_main.h>` = TODO disponible  
> `-Wstrict` = Modo estricto (UB = error)  
> Compact IAT = Solo funciones usadas, sin STATUS_ENTRYPOINT_NOT_FOUND

```
Tu Código (.c)
        ↓
┌───────────────────────────────────────────┐
│         ADead-BIB Compiler (adb)          │
│                                           │
│  .c  → Preprocessor → Lexer → Parser     │
│                    ↓                      │
│             CToIR (C ABI)                 │
│                    ↓                      │
│             Program (IR)                  │
│                    ↓                      │
│             IsaCompiler                   │
│             (ADeadOp stream)              │
│                    ↓                      │
│             Optimizer                     │
│             (DCE, Fold, Inline, Peep)     │
│                    ↓                      │
│             BitResolver (v9.0)            │
│             (16/32/64/128/256 bits)       │
│                    ↓                      │
│             SoA Optimizer                 │
│             (float arr[8] → YMM register) │
│                    ↓                      │
│             Encoder + VEX Emitter         │
│             (FASM-style, x86-64/AVX2)     │
│                    ↓                      │
│             Linker Especial DLL           │
│             (PE .dll / ELF .so)           │
│                    ↓                      │
│             PE / ELF / Po / DLL           │
└───────────────────────────────────────────┘
        ↓
  .exe / .elf / .po / .bin / .dll / .so
  (Machine Code Puro · 256-bit)
```

---

## Tabla de Contenidos

1. [Filosofía](#filosofía)
2. [Arquitectura C ABI](#arquitectura-c-abi)
3. [ASM-BIB — El Fundamento](#asm-bib--el-fundamento)
4. [Instalación](#instalación)
5. [Inicio Rápido](#inicio-rápido)
6. [Step Compiler](#step-compiler)
7. [Frontend C99](#frontend-c99)
8. [256-bit Pipeline (v9.0)](#256-bit-pipeline-v90)
9. [Linker Especial DLL](#linker-especial-dll)
10. [GPU Backend — C ABI Completo](#gpu-backend--c-abi-completo)
11. [Referencia Técnica](#referencia-técnica)
12. [Estructura del Proyecto (src_v2)](#estructura-del-proyecto-src_v2)
13. [Tamaños de Binario](#tamaños-de-binario)
14. [Resultados de Tests](#resultados-de-tests)
15. [Comandos CLI](#comandos-cli)

---

## Filosofía

### ¿Por qué existe ADead-BIB?

ADead-BIB es un **compilador C puro** — no C++, no multi-lenguaje. C es el lenguaje más cercano al hardware y ADead-BIB lo lleva a su máxima expresión: bytes directos al CPU sin intermediarios.

Los compiladores industriales (MSVC, GCC, Clang/LLVM) son **referencias técnicas invaluables** — definieron cómo se compila C durante décadas. ADead-BIB los estudia, los respeta, y toma sus decisiones de ABI y calling convention como referencia. Lo que rechaza es el overhead que arrastran.

| Referencia | Lo que ADead-BIB toma | Lo que ADead-BIB rechaza |
|---|---|---|
| **MSVC** | Windows x64 ABI (RCX, RDX, R8, R9), shadow space 32 bytes, PE format | Runtime >100 KB, CRT implícito, excepciones SEH |
| **GCC** | System V AMD64 ABI (RDI, RSI, RDX, RCX), ELF format, optimizaciones agresivas | Múltiples backends indirectos, código generado inflado |
| **LLVM** | Concepto de IR intermedio, passes de optimización, instruction selection | IR genérico que no llega a bytes directos, overhead de abstracción |
| **FASM** | **Generación directa de bytes sin ensamblador externo** | — (FASM es la referencia que ADead-BIB sigue fielmente) |
| **MASM** | **Base de árbol de ensamblador → ASM-BIB** | Dependencia de Microsoft, formato propietario |

**El resultado:** ADead-BIB genera binarios de **2–10 KB** donde GCC genera **50+ KB** y MSVC genera **100+ KB** para el mismo programa.

### Canon: C99

ADead-BIB compila **C99** como estándar canónico — representa la intención más clara del lenguaje C.

**C99 — El Canon de C:**  
`int` = 32 bits, `char` = 8 bits, `long long` = 64 bits — tamaños exactos.  
Punteros = direcciones reales. `malloc/free` = control manual. `arr[i]` = `*(arr + i * sizeof(element))`.  
El programador sabe exactamente qué bytes genera cada línea.

### ¿Por qué ADead-BIB está escrito en Rust?

Rust es el **guardián** que detecta los problemas que C no puede ver en sí mismo:

| Problema en C | Rust lo detecta porque... |
|---|---|
| Buffer overflow | Ownership + bounds checking |
| Use-after-free | Borrow checker |
| Data races | Send + Sync traits |
| Null pointer | `Option<T>` obligatorio |
| Memory leaks | RAII + Drop |

**Rust no es el lenguaje que ADead-BIB compila — es el lenguaje que garantiza que ADead-BIB compile correctamente.**

### Eliminación Absoluta

Todo lo que no contribuye a la ejecución final se elimina:

```
Funciones no llamadas          → eliminadas por DCE
Variables no leídas            → eliminadas por DCE
Branches inalcanzables         → eliminadas por constant folding
Headers no usados              → tree shaking
```

**Machine Code Puro** = solo las instrucciones x86-64 que el CPU necesita ejecutar. Nada más.

---

## Arquitectura C ABI

ADead-BIB tiene un ecosistema completo de C ABI — cada API del sistema, cada librería gráfica, cada header está definido como C puro:

```
┌─────────────────────────────────────────────────────────────────┐
│                    adeb-stdlib (C ABI Only)                      │
├─────────────────────────┬───────────────────────────────────────┤
│    c/ — Platform & DX   │      gpu/ — Graphics APIs             │
├─────────────────────────┼───────────────────────────────────────┤
│ C99 Standard Library:   │ fastos_gpu.rs    — GPU Header          │
│  stdio, stdlib, string  │ fastos_com.rs    — COM Types Gen       │
│  math, time, assert     │                                        │
│  errno, limits, types   │ opengl/ (18+ módulos):                 │
│  ctype, signal, wchar   │  GL 1.0-4.6 completo                  │
│  setjmp, fenv, complex  │  GLSL, shader_bridge                  │
│  stdatomic, threads     │  loader, optimizer                     │
│  inttypes               │                                        │
│                         │ vulkan/ (7 módulos):                   │
│ Platform APIs:          │  vk_types — 30 handles + constants     │
│  fastos_win32 — Win32   │  vk_enums — 12 categorías, 250+ vals  │
│  fastos_linux — Linux   │  vk_structs — 85+ structs              │
│                         │  vk_functions — 130+ funciones         │
│ DirectX / COM:          │  vk_loader — DLL/SO paths              │
│  fastos_com — COM       │  vk_symbols — checker unificado        │
│  fastos_dxgi — DXGI     │                                        │
│  fastos_d3d9 — DX9      │                                        │
│  fastos_d3d11 — DX11    │                                        │
│  fastos_d3d12 — DX12    │                                        │
│                         │                                        │
│ Kernel:                 │                                        │
│  fastos_kernel — OS API │                                        │
│  fastos_io — I/O x86-64 │                                        │
│  fastos_asm — builtins  │                                        │
└─────────────────────────┴───────────────────────────────────────┘
```

---

## ASM-BIB — El Fundamento

**ASM-BIB es MASM reconstruido** — ADead-BIB toma la base del ensamblador MASM de Microsoft y la reconstruye como su propio árbol de ensamblador. Este es el fundamento sobre el cual se construye el compilador C:

```
MASM (Microsoft Macro Assembler)
        ↓ reconstruido
ASM-BIB (ADead-BIB Assembler)
        ↓ base de árbol
ADead-BIB C Compiler
        ↓
Machine Code Puro (x86-64)
```

**Pipeline:** `.pasm → ASM-BIB → COFF .obj → adeb-bridge → merge → PE`

```
21 funciones x86-64 (Win64 fastcall ABI):
  String:  asm_strlen, asm_strcpy, asm_strcmp, asm_strcat, asm_strchr,
           asm_memcpy, asm_memset, asm_memcmp
  Math:    asm_abs, asm_min, asm_max, asm_clamp, asm_swap
  Bit:     asm_popcount, asm_bsr64, asm_bsf64, asm_bswap32, asm_bswap64
  Utility: asm_is_aligned, asm_align_up, asm_noop
```

---

## Instalación

```bash
# 1. Clonar y compilar
git clone https://github.com/AndreeSalazar/ADead-BIB.git
cd ADead-BIB
cargo build --release

# 2. Agregar adb al PATH
#    Windows (PowerShell):
$env:Path += ";C:\ruta\a\ADead-BIB\target\release"
#    Linux / macOS:
export PATH="$PATH:$HOME/ADead-BIB/target/release"

# 3. Verificar
adb --version
```

---

## Inicio Rápido

```bash
adb create hola          # Proyecto C
cd hola
adb run                  # Compila src/main.c → bin/hola.exe y ejecuta
# → "Hola desde hola"

adb cc hello.c -o hello.exe    # Compilar archivo suelto C
adb run test.c                 # Compilar y ejecutar directo
adb step main.c                # Step Compiler — ver cada fase
```

---

## Step Compiler

```bash
adb step main.c
```

Muestra cada fase del pipeline en tiempo real:

```
[SOURCE]   12 lines, 245 bytes

--- Phase 1: PREPROCESSOR ---
[PREPROC]  165 lines after preprocessing
[PREPROC]  #include <stdio.h> -> resolved internally

--- Phase 2: LEXER ---
[LEXER]    78 tokens generated

--- Phase 3: PARSER ---
[PARSER]   function 'main' (0 params, 3 stmts) OK

--- Phase 4: IR ---
[IR]       function 'main' -> 5 IR statements OK

--- Phase 5: UB DETECTOR ---
[UB]       No undefined behavior detected OK

--- Phase 6: CODEGEN (x86-64) ---
[CODEGEN]  127 bytes of machine code generated

--- Phase 7: OUTPUT ---
[OUTPUT]   Target: Windows PE x86-64
[OUTPUT]   Code: 127 bytes  |  Data: 32 bytes
[OUTPUT]   Est. binary: ~1183 bytes
```

---

## Frontend C99

**Pipeline:** `C source → Preprocessor → Lexer → Parser → AST → IR → IsaCompiler → Encoder → x86-64 → PE/ELF`

| Característica | Estado | Intención |
|---|---|---|
| Variables y tipos (`int`, `char`, `float`, `double`, ...) | ✅ | Tamaños exactos → registros correctos |
| Punteros y aritmética de punteros | ✅ | Direcciones reales, stride por `sizeof` |
| Arrays y acceso directo | ✅ | `arr[i]` = `*(arr + i * size)` |
| Structs, unions, typedefs | ✅ | Layout en memoria explícito |
| Enums | ✅ | Constantes enteras |
| Function pointers | ✅ | `call reg` directo |
| Preprocesador (`#include`, `#define`, `#ifdef`) | ✅ | 75+ headers de sistema inyectados |
| Control de flujo (`if`, `for`, `while`, `switch`) | ✅ | Branch → `jcc` directo |
| Recursión | ✅ | Stack frame real por llamada |
| `malloc/free` | ✅ | Enlace directo Win32 API / Linux syscall |
| Bitwise (`&`, `\|`, `^`, `<<`, `>>`, `~`) | ✅ | Instrucciones x86-64 directas |
| Operadores compuestos (`+=`, `-=`, `*=`, ...) | ✅ | In-place, sin temporales |

---

## 256-bit Pipeline (v9.0)

```c
#include <header_main.h>

int main() {
    // SoA natural → detectado automáticamente → YMM register
    float pos_x[8] = {1,2,3,4,5,6,7,8};
    float pos_y[8] = {8,7,6,5,4,3,2,1};

    // 8 sumas en 1 instrucción: VADDPS ymm0, ymm0, ymm1
    for (int i = 0; i < 8; i++)
        pos_x[i] += pos_y[i];

    return 0;
}
```

| Target | Bits | Registros | Uso |
|---|---|---|---|
| `boot16` | 16 | AX-DX | Stage1 bootloader |
| `boot32` | 32 | EAX-EDI | Stage2 protected mode |
| `fastos64` | 64 | RAX-R15 | FastOS standard |
| `fastos128` | 128 | XMM0-XMM15 | SSE/SSE4.2 vectorial |
| `fastos256` | 256 | **YMM0-YMM15** | **AVX2 nativo** ★ |
| `dll64` | 64 | RAX-R15 | **DLL Windows/Linux** ★ |

---

## GPU Backend — C ABI Completo

ADead-BIB tiene soporte completo de GPU via C ABI — sin wrappers, sin C++:

### OpenGL 1.0 — 4.6 (Completo)

```
18+ módulos de versión: GL10, GL11, GL12, GL13, GL14, GL15,
GL20, GL21, GL30, GL31, GL32, GL33, GL40, GL41, GL42, GL43, GL44, GL45, GL46
+ GLSL compiler + shader_bridge + loader + optimizer
+ types (GLint, GLfloat, etc.) + constants (3000+)
```

### Vulkan 1.3 (Completo — 7 módulos)

```
vk_types.rs      — 30 handles (VkDevice, VkPipeline, ...) + 16 constants + 10 macros
vk_enums.rs      — 12 categorías: VkResult, VkStructureType, VkFormat, Pipeline,
                   Memory, Image, RenderPass, Descriptor, Blend, DepthStencil,
                   Command, Presentation — 250+ valores enum
vk_structs.rs    — 85+ structs (VkApplicationInfo → VkPipelineRenderingCreateInfo)
vk_functions.rs  — 130+ funciones: Core 1.0, KHR (surface/swapchain), Vulkan 1.3
vk_loader.rs     — vulkan-1.dll (Win32) / libvulkan.so.1 (Linux) + extensions
vk_symbols.rs    — is_vulkan_symbol() unificado con tests
```

### DirectX 9 / 11 / 12 + COM

```
COM        — CoInitializeEx, CoCreateInstance, IUnknown, HRESULT
DXGI       — CreateDXGIFactory, IDXGISwapChain, DXGI_FORMAT
Direct3D 9 — Direct3DCreate9, IDirect3DDevice9
Direct3D 11 — D3D11CreateDevice, ID3D11Device, D3DCompile
Direct3D 12 — D3D12CreateDevice, ID3D12Device, Raytracing, Mesh Shaders
HLSL       — D3DCompile, D3DReflect, D3DDisassemble (d3dcompiler_47.dll)
```

### Win32 API Nativo

```
kernel32.dll — 84 funciones (proceso, memoria, archivos, threads, sync, tiempo)
user32.dll   — 56 funciones (ventanas, mensajes, input, painting)
gdi32.dll    — 40 funciones (pixelformat, DC, bitmaps, WGL/OpenGL)
+ 93 tipos Win32 + 100+ constantes
```

---

## Referencia Técnica

### Calling Conventions

```
Windows x64 (referencia MSVC):
  Args:         RCX, RDX, R8, R9, stack
  Ret:          RAX (int), XMM0 (float)
  Shadow space: 32 bytes
  Callee-saved: RBX, RBP, RDI, RSI, R12–R15

System V AMD64 (referencia GCC):
  Args:         RDI, RSI, RDX, RCX, R8, R9, stack
  Ret:          RAX (int), XMM0 (float)
  Red zone:     128 bytes
  Callee-saved: RBX, RBP, R12–R15
```

### Encoding FASM-Style (Bytes Directos)

```
Instrucción        Bytes            Encoding
───────────────────────────────────────────────────
mov rax, rbx       48 89 D8         REX.W + MOV r/m64, r64
add rax, 42        48 83 C0 2A      REX.W + ADD r/m64, imm8
call printf        E8 xx xx xx xx   CALL rel32
ret                C3               RET
push rbp           55               PUSH r64
VADDPS ymm0,y0,y1  C5 FC 58 C1     VEX.256.0F 58 /r
```

### Optimizaciones

| Optimización | Referencia | Qué hace |
|---|---|---|
| Dead Code Elimination | GCC -O1, LLVM `dce` | Elimina funciones, variables y branches no usados |
| Constant Folding | GCC -O1, LLVM `constprop` | `2 + 3 * 4` → `14` en compilación |
| Inlining | GCC -O2, LLVM `inline` | Funciones pequeñas expandidas en el caller |
| Peephole | GCC -O2 | `add reg, 1` → `inc reg` |
| Register Allocation | GCC/LLVM `regalloc` | Temporales en R10–R15 |
| Strength Reduction | GCC -O2 | `x * 2` → `shl x, 1` |

---

## Estructura del Proyecto (src_v2)

```
ADead-BIB/
├── src_v2/                             # Arquitectura v2 (numerada)
│   ├── 00_bin/
│   │   └── adeb-cli/                  # CLI driver (adb)
│   │
│   ├── 01_compiler/
│   │   ├── 01_frontend/               # C99 lexer, parser, preprocessor
│   │   ├── 02_middle/                 # IR, optimizer, UB detector
│   │   ├── 03_backend_cpu/            # x86-64 encoder, PE/ELF
│   │   └── 04_backend_gpu/            # SPIR-V, GPU codegen
│   │
│   ├── 02_core/
│   │   ├── adeb-core/                 # Core types y utilities
│   │   ├── adeb-platform/             # Platform detection
│   │   ├── adeb-stdlib/               # ★ C ABI Standard Library
│   │   │   └── src/
│   │   │       ├── c/                 # C99 stdlib + Win32 + Linux + DX
│   │   │       │   ├── fastos_stdio.rs    — printf, fopen, fread...
│   │   │       │   ├── fastos_stdlib.rs   — malloc, free, qsort...
│   │   │       │   ├── fastos_string.rs   — strlen, memcpy, strcmp...
│   │   │       │   ├── fastos_math.rs     — sin, cos, sqrt, pow...
│   │   │       │   ├── fastos_types.rs    — int8_t-uint64_t, size_t...
│   │   │       │   ├── fastos_win32.rs    — kernel32 + user32 + gdi32
│   │   │       │   ├── fastos_linux.rs    — syscalls + X11 + Wayland
│   │   │       │   ├── fastos_com.rs      — COM runtime
│   │   │       │   ├── fastos_dxgi.rs     — DXGI
│   │   │       │   ├── fastos_d3d9.rs     — DirectX 9
│   │   │       │   ├── fastos_d3d11.rs    — DirectX 11
│   │   │       │   ├── fastos_d3d12.rs    — DirectX 12
│   │   │       │   └── ... (20+ módulos)
│   │   │       └── gpu/               # GPU C ABI
│   │   │           ├── fastos_gpu.rs      — GPU header
│   │   │           ├── fastos_com.rs      — COM type generator
│   │   │           ├── opengl/            — GL 1.0-4.6 (18+ módulos)
│   │   │           └── vulkan/            — Vulkan 1.3 (7 módulos) ★ NEW
│   │   │               ├── mod.rs
│   │   │               ├── vk_types.rs
│   │   │               ├── vk_enums.rs
│   │   │               ├── vk_structs.rs
│   │   │               ├── vk_functions.rs
│   │   │               ├── vk_loader.rs
│   │   │               └── vk_symbols.rs
│   │   ├── adeb-bridge/               # ASM-BIB COFF .obj bridge
│   │   └── adeb-bg/                   # Binary Guardian
│   │
│   ├── 03_libc/                       # libc propia (sin externa)
│   │
│   ├── 04_tests/
│   │   └── win32_intensive/           # Win32 stress tests (5 fases)
│   │       ├── 01_win_memory.c        — HeapAlloc, VirtualAlloc, GlobalAlloc
│   │       ├── 02_win_files.c         — CreateFileA, ReadFile, WriteFile
│   │       ├── 03_win_threads.c       — CreateThread, Mutex, WaitForMultipleObjects
│   │       ├── 04_win_gui.c           — WNDCLASSEX, CreateWindowEx, message loop
│   │       └── 05_win_dll.c           — LoadLibraryA, GetProcAddress, indirect call
│   │
│   ├── _scratch/                      # Experimental
│   └── Cargo.toml                     # Workspace
│
├── Cargo.toml
├── LICENSE                            # Techne License v1.0
└── README.md
```

---

## Tamaños de Binario

| Programa | ADead-BIB | GCC -Os | MSVC /O1 |
|---|---|---|---|
| Hello World | **2.0 KB** | ~50 KB | ~100 KB |
| Counter + printf | **2.0 KB** | ~50 KB | ~100 KB |
| Recursión (fib, power) | **2.5 KB** | ~50 KB | ~100 KB |
| Win32 Window | **3.0 KB** | ~55 KB | ~110 KB |
| Stdlib largo (~100 funcs) | **42 KB** | ~200 KB | ~300 KB |

Sin CRT. Sin exception handling tables. Sin RTTI. Sin debug info por defecto. Solo machine code puro.

---

## Resultados de Tests

### Rust Unit Tests (adeb-stdlib)

| Suite | Tests | Estado |
|---|---|---|
| C stdlib (ctype, asm, io, kernel) | 14 | ✅ ALL PASS |
| GPU COM | 2 | ✅ ALL PASS |
| OpenGL (types, constants, GLSL, loader, optimizer, shader_bridge) | 23 | ✅ ALL PASS |
| Vulkan (vk_symbols) | 1 | ✅ ALL PASS |
| **Total** | **40** | **100%** ✅ |

### Win32 Intensive Tests (C)

| Fase | Test | Symbols Needed | Coverage |
|---|---|---|---|
| 01 | Memory Management | HeapAlloc, VirtualAlloc, GlobalAlloc, GetProcessHeap | ✅ |
| 02 | File I/O | CreateFileA, WriteFile, ReadFile, DeleteFileA, CloseHandle | ✅ |
| 03 | Threads & Sync | CreateThread, CreateMutex, WaitForMultipleObjects | ✅ |
| 04 | GUI | RegisterClassEx, CreateWindowEx, message loop, ShowWindow | ✅ |
| 05 | DLL Dynamic Loading | LoadLibraryA, GetProcAddress, FreeLibrary, indirect call | ✅ |

---

## IAT Registry v6 — 18 DLLs · 340+ Funciones

```
┌─────────────────────────────────────────────────────────┐
│  DLL                    │ Funciones │ Categoría         │
├─────────────────────────┼───────────┼───────────────────┤
│  msvcrt.dll             │   158     │ C Runtime         │
│  kernel32.dll           │    84     │ Win32 Core        │
│  user32.dll             │    56     │ Win32 UI          │
│  gdi32.dll              │    40     │ Win32 GDI + WGL   │
│  opengl32.dll           │    17     │ OpenGL 1.1        │
│  ole32.dll              │    12     │ COM               │
│  oleaut32.dll           │    10     │ COM Automation    │
│  dxgi.dll               │     4     │ DXGI              │
│  d3d9.dll               │     7     │ DirectX 9         │
│  d3d11.dll              │     3     │ DirectX 11        │
│  d3d12.dll              │     8     │ DirectX 12        │
│  d3dcompiler_47.dll     │    15     │ HLSL Compiler     │
│  advapi32.dll           │    13     │ Security/Registry │
│  shell32.dll            │     6     │ Shell             │
│  winmm.dll              │     9     │ Multimedia        │
│  comdlg32.dll           │     6     │ Common Dialogs    │
│  ws2_32.dll             │    11     │ Winsock           │
│  vulkan-1.dll           │   130+    │ Vulkan 1.3        │
├─────────────────────────┼───────────┼───────────────────┤
│  TOTAL                  │   500+    │                   │
└─────────────────────────┴───────────┴───────────────────┘
```

---

## Comandos CLI

```bash
# ── C99 ──────────────────────────────────────────────────────────
adB cc hello.c -o hello.exe            # Compilar C
adB cc file.c -step                    # Step mode
adB cc file.c -Wstrict                 # Modo estricto

# ── Auto-detect / Run ────────────────────────────────────────────
adB run hello.c                        # Compilar + ejecutar
adB step main.c                        # Ver pipeline paso a paso

# ── Proyectos ────────────────────────────────────────────────────
adB create hola                        # Nuevo proyecto C
adB build                              # Compilar proyecto (adb.toml)
adB run                                # Compilar y ejecutar proyecto

# ── Flat Binary (OS/Kernel) ──────────────────────────────────────
adB cc kernel.c -o kernel.bin --flat

# ── DLL / SO (Linker Especial) ───────────────────────────────────
adB cc lib.c --dll -o mylib.dll        # DLL Windows
adB cc lib.c --so -o libmylib.so       # SO Linux

# ── FastOS targets ────────────────────────────────────────────────
adB cc kernel.c --target fastos256 -o kernel.po   # 256-bit YMM/AVX2

# ── GPU ───────────────────────────────────────────────────────────
adB gpu                                # Detectar GPU + generar shader
adB spirv matmul 1024                  # SPIR-V compute shader

# ── Versión ──────────────────────────────────────────────────────
adB version                            # ASCII banner + versión
```

| Flag | Efecto |
|---|---|
| `-Wstrict` | Promueve UB warnings a errors |
| `-step` | Muestra cada fase del pipeline |
| `--flat` | Genera flat binary (OS/Kernel) |
| `--dll` | Genera DLL Windows (.dll) |
| `--so` | Genera shared object Linux (.so) |

---

## Autor

**Eddi Andreé Salazar Matos**  
eddi.salazar.dev@gmail.com  
Hecho en Perú 🇵🇪

## Licencia

**Techne License v1.0 (τέχνη)**

```
Copyright (C) 2026 Eddi Andreé Salazar Matos
Lima, República del Perú

Uso personal, educativo, open source y startups < $1M: FREE
Uso comercial > $1M: 10% royalty sobre revenue atribuible
Contacto: eddi.salazar.dev@gmail.com
```

Ver [LICENSE](LICENSE) para los términos completos.

---

**ADead-BIB v12.0: C99 → Machine Code Puro · C ABI Completo 💀🦈**

```
MSVC, GCC, LLVM  = referencias técnicas estudiadas y respetadas
MASM             = base de árbol → ASM-BIB reconstruido
FASM             = el modelo de encoding directo que ADead-BIB sigue
Rust             = el guardián que garantiza que el compilador nunca falle
C99              = el único lenguaje — intención absoluta del programador
header_main.h    = un include, todo disponible
OpenGL 4.6       = 18+ módulos, GL completo
Vulkan 1.3       = 7 módulos, C ABI nativo
DirectX 9/11/12  = COM + DXGI + HLSL
YMM/AVX2         = 256-bit nativo, SoA natural
DLL/SO           = Linker Especial para fusionar con Windows/Linux
```

> *"C = intención absoluta del programador*  
> *ASM-BIB = MASM reconstruido, el fundamento*  
> *Rust = guardián de correctitud*  
> *FASM = bytes directos al CPU*  
> *YMM = 256 bits nativos, 8 floats en paralelo*  
> *OpenGL + Vulkan + DirectX = GPU C ABI completo*  
> **DLL = tu código en cualquier programa Windows/Linux**  
> *ADead-BIB = único en el mundo 💀🦈 🇵🇪*"
