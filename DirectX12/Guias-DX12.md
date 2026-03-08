# Guía: DX12 HelloTriangle con ADead-BIB (9 Pasos)

## Introducción

ADead-BIB compila programas C++ que crean ventanas Windows y dibujan gráficos usando Win32/GDI y DirectX 12. El compilador genera ejecutables PE nativos (`.exe`) que corren directamente en Windows x64 sin dependencias externas.

## Arquitectura

```
main.cpp → C++ Parser → IR → ISA Compiler → x86-64 → PE Builder → .exe
                ↓
        #include <header_main.h>
        (Win32 + GDI + DX12 tipos, constantes, COM interfaces)
```

### Componentes Clave

- **Headers integrados** (`cpp_stdlib.rs`):
  - `fastos_windows.h` — Tipos, constantes Win32, IUnknown, GUID
  - `fastos_wrl.h` — ComPtr\<T\> template
  - `fastos_d3d12.h` — ID3D12Device, CommandQueue, CommandList, Fence, RTV, PSO
  - `fastos_dxgi.h` — IDXGIFactory4, SwapChain, CreateSwapChainForHwnd

- **IAT Registry** (`iat_registry.rs`) — **47 slots, 6 DLLs**:
  - `msvcrt.dll`: printf, scanf, malloc, free, memset, memcpy
  - `kernel32.dll`: GetModuleHandleA/W, ExitProcess, CreateEventA, WaitForSingleObject, CloseHandle, Sleep
  - `user32.dll`: CreateWindowExA/W, ShowWindow, GetDC, PeekMessageA, etc.
  - `gdi32.dll`: SetPixel, CreatePen, LineTo, MoveToEx, SelectObject, etc.
  - `d3d12.dll`: D3D12CreateDevice, D3D12GetDebugInterface, D3D12SerializeRootSignature
  - `dxgi.dll`: CreateDXGIFactory1, CreateDXGIFactory2

- **Dead Code Elimination** — Solo compila funciones reachable desde `main()`.

## Pipeline DX12 (9 Pasos)

```
Paso 0: Window         ✅ CreateWindowExA (12 args, main only)
Paso 1: Device         ✅ D3D12CreateDevice (IAT direct call)
Paso 2: Factory        ✅ CreateDXGIFactory1 (IAT direct call)
Paso 3: Command Queue  ⚠️ device->CreateCommandQueue (COM vtable)
Paso 4: RTV Heap       ⚠️ device->CreateDescriptorHeap (COM vtable)
Paso 5: Command List   ⚠️ device->CreateCommandList (COM vtable)
Paso 6: Root Signature ✅ D3D12SerializeRootSignature (IAT direct call)
Paso 7: Pipeline State ⚠️ needs shader bytecode + COM vtable
Paso 8: Vertex Buffer  ✅ malloc'd CPU buffer (84 bytes, 3 vertices)
Paso 9: Render Loop    ⚠️ Present + fence sync (COM vtable)
```

**Leyenda:** ✅ = funciona via IAT | ⚠️ = necesita COM vtable dispatch

## Inicio Rápido

### 1. Compilar

```bash
adb cxx DirectX12/dx12_test/src/main.cpp -o DirectX12/dx12_test/bin/dx12_hello.exe
```

### 2. Step Mode (diagnóstico del pipeline)

```bash
adb step DirectX12/dx12_test/src/main.cpp
```

Muestra 7 fases del compilador + categorías de API:
- **SOURCE** — líneas, bytes
- **PREPROCESSOR** — #include resolution (header_main.h → fastos_*.h)
- **LEXER** — tokens generados
- **PARSER+IR** — funciones, structs, typedefs, IR statements
- **UB DETECTOR** — undefined behavior checks
- **CODEGEN** — bytes de código, data section, IAT calls
- **OUTPUT** — DLLs usadas, categorías: Win32, GDI, DX12, DXGI

### 3. Ejecutar

```bash
DirectX12\dx12_test\bin\dx12_hello.exe
```

**Output esperado (con GPU DX12):**
```
=== ADead-BIB DX12 HelloTriangle (9 Pasos) ===
[PASO 0] Creando ventana...
[PASO 0] hInstance=0000000140000000
[PASO 0] OK hwnd=0x001A0B34
[PASO 1] Creando D3D12 Device...
[PASO 1] D3D12CreateDevice hr=0x00000000 device=0x...
[PASO 1] OK Device created!
[PASO 2] Creando DXGI Factory...
[PASO 2] CreateDXGIFactory1 hr=0x00000000 factory=0x...
[PASO 2] OK Factory created!
[PASO 3] Creando Command Queue...
[PASO 3] PENDING: Command Queue (needs COM vtable dispatch)
...
=== DX12 Pipeline Status ===
[PASO 0] Window:          OK
[PASO 1] Device:          OK
[PASO 2] Factory:         OK
[PASO 6] Root Signature:  OK
[PASO 8] Vertex Buffer:   PREPARED (84 bytes)
[FALLBACK] Drawing GDI triangle...
[LOOP] Entering message loop...
```

**Output esperado (sin GPU DX12 — e.g. VM):**
```
[PASO 1] WARN: Device creation failed (hr=0x80004005)
[PASO 1] Falling back to GDI triangle rendering...
[PASO 1] GDI triangle drawn (fallback)
```

## Constantes DX12/DXGI Disponibles (`#define`)

| Constante | Valor | Uso |
|---|---|---|
| `D3D_FEATURE_LEVEL_11_0` | `0xB000` | Feature level mínimo |
| `D3D_FEATURE_LEVEL_12_0` | `0xC000` | Feature level DX12 |
| `D3D12_COMMAND_LIST_TYPE_DIRECT` | `0` | Command list tipo directo |
| `D3D12_DESCRIPTOR_HEAP_TYPE_RTV` | `2` | Heap para render targets |
| `D3D12_FENCE_FLAG_NONE` | `0` | Fence sin flags |
| `D3D12_RESOURCE_STATE_PRESENT` | `0` | Estado present |
| `D3D12_RESOURCE_STATE_RENDER_TARGET` | `4` | Estado render target |
| `DXGI_FORMAT_R8G8B8A8_UNORM` | `28` | Formato RGBA 8-bit |
| `DXGI_SWAP_EFFECT_FLIP_DISCARD` | `4` | Swap effect moderno |
| `DXGI_USAGE_RENDER_TARGET_OUTPUT` | `0x20` | Usage render target |

## Constantes Win32/GDI

| Constante | Valor |
|---|---|
| `WS_OVERLAPPEDWINDOW` | `0x00CF0000` |
| `SW_SHOW` | `5` |
| `WM_CLOSE / WM_DESTROY / WM_PAINT` | `0x10 / 0x02 / 0x0F` |
| `PS_SOLID` | `0` |
| `VK_ESCAPE` | `0x1B` |

## COM Interfaces Disponibles

### ID3D12Device (fastos_d3d12.h)
- `CreateCommandQueue`, `CreateCommandAllocator`, `CreateCommandList`
- `CreateFence`, `CreateDescriptorHeap`, `CreateRenderTargetView`
- `CreateCommittedResource`, `CreateRootSignature`, `CreateGraphicsPipelineState`

### ID3D12GraphicsCommandList
- `Close`, `Reset`, `RSSetViewports`, `RSSetScissorRects`
- `ResourceBarrier`, `OMSetRenderTargets`, `ClearRenderTargetView`
- `IASetPrimitiveTopology`, `IASetVertexBuffers`, `DrawInstanced`

### IDXGIFactory4 (fastos_dxgi.h)
- `CreateSwapChainForHwnd`, `EnumAdapters1`

### IDXGISwapChain3
- `Present`, `GetBuffer`, `GetCurrentBackBufferIndex`

## Limitaciones Conocidas

1. **`CreateWindowExA` en `main()` solamente** — 12 argumentos requiere generación directa en main. No desde funciones anidadas.

2. **MSG struct via `malloc(64)`** — El codegen usa 8 bytes por campo. Usar `malloc(64)` como buffer para PeekMessageA.

3. **Clase de ventana `"STATIC"`** — Usar la clase built-in. La struct `WNDCLASSEXA` tiene issues de ABI.

4. **COM vtable calls** — Las llamadas a métodos COM (device->CreateCommandQueue, etc.) requieren dispatch indirecto: `load [obj+0] → vtable, load [vtable+idx*8] → method, call method(obj, args)`. El ISA compiler actualmente compila MethodCall como llamadas a funciones locales por label, no indirectas por vtable.

5. **Struct ABI** — El codegen usa 8 bytes por campo. Las structs DX12 (DXGI_SWAP_CHAIN_DESC1, etc.) se preparan via `malloc + memset` con offsets manuales.

6. **Shader bytecode** — HLSL → DXBC/DXIL requiere compilación separada. Futuro: ADead-BIB HLSL frontend o bytecode pre-compilado.

## Roadmap: COM Vtable Dispatch

Para completar los pasos ⚠️, el ISA compiler necesita:

```
1. Emit: mov rax, [rcx]        ; load vtable ptr from this
2. Emit: call [rax + idx*8]    ; indirect call through vtable
3. With: this in RCX (MSVC x64 thiscall)
```

**Prioridad:**
1. **Paso 3** — CreateCommandQueue (vtable[4]) → habilita GPU commands
2. **Paso 4** — CreateDescriptorHeap → habilita render targets
3. **Paso 5** — CreateCommandAllocator + CreateCommandList → habilita recording
4. **Paso 9** — ExecuteCommandLists + Present + Signal → habilita rendering

## Helpers Inline

| Helper | Uso |
|---|---|
| `RGB(r,g,b)` | Color COLORREF |
| `HIWORD(x)` / `LOWORD(x)` | Bits altos/bajos |
| `SUCCEEDED(hr)` / `FAILED(hr)` | Check HRESULT |
| `AllocMSG()` | Buffer MSG heap |
| `DrawLine(hdc,x1,y1,x2,y2,color,width)` | Línea GDI |
| `MessageLoop()` | Loop de mensajes |

## Estadísticas

- **47 IAT slots**, 6 DLLs (msvcrt, kernel32, user32, gdi32, d3d12, dxgi)
- **PE output**: ~6-8KB ejecutable
- **GDI fallback**: Gradient fill + white outline, ventana 1280x720
- **DX12 pasos completados**: 4/9 via IAT (Pasos 0,1,2,6), 1 preparado (Paso 8)
