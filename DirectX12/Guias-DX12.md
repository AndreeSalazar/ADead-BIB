# Guía: Aplicaciones Gráficas con ADead-BIB

## Introducción

ADead-BIB puede compilar programas C++ que crean ventanas Windows y dibujan gráficos usando las APIs Win32/GDI. El compilador genera ejecutables PE nativos (`.exe`) que corren directamente en Windows x64 sin dependencias externas.

## Arquitectura

```
main.cpp → C++ Parser → IR → ISA Compiler → x86-64 → PE Builder → .exe
                ↓
        #include <header_main.h>
        (Win32 + GDI + DX12 tipos, constantes, helpers)
```

### Componentes Clave

- **Header `fastos_windows.h`** — Integrado en el compilador (`cpp_stdlib.rs`), provee:
  - Tipos: `HWND`, `HDC`, `HINSTANCE`, `HPEN`, `HBRUSH`, `COLORREF`, etc.
  - Constantes (`#define`): `WS_OVERLAPPEDWINDOW`, `SW_SHOW`, `WM_*`, `PS_SOLID`, etc.
  - Helpers inline: `RGB()`, `MessageLoop()`, `DrawLine()`, `AllocMSG()`
  - Structs DX12: `GUID`, `ComPtr<T>`, `XMFLOAT2/3/4`, `DXGI_SWAP_CHAIN_DESC`

- **IAT Registry** (`iat_registry.rs`) — 42 slots, 6 DLLs:
  - `msvcrt.dll`: printf, scanf, malloc, free, memset
  - `kernel32.dll`: GetModuleHandleA/W, ExitProcess, CreateEventA
  - `user32.dll`: CreateWindowExA/W, ShowWindow, GetDC, PeekMessageA, etc.
  - `gdi32.dll`: SetPixel, CreatePen, LineTo, MoveToEx, SelectObject, etc.
  - `d3d12.dll`: D3D12CreateDevice, D3D12GetDebugInterface
  - `dxgi.dll`: CreateDXGIFactory1, CreateDXGIFactory2

- **Dead Code Elimination** — Solo compila funciones reachable desde `main()`.

## Inicio Rápido

### 1. Compilar

```bash
adb cxx DirectX12/dx12_test/src/main.cpp -o DirectX12/dx12_test/bin/dx12_hello.exe
```

### 2. Diagnóstico paso a paso

```bash
adb step DirectX12/dx12_test/src/main.cpp
```

Muestra 7 fases: SOURCE → PREPROCESSOR → LEXER → PARSER+IR → UB DETECTOR → CODEGEN, incluyendo:
- Funciones compiladas y IR statements
- Bytes de código y data section
- IAT call sites y DLLs usadas
- Categorías de API detectadas (Win32, GDI, DX12, etc.)

### 3. Ejecutar

```bash
DirectX12\dx12_test\bin\dx12_hello.exe
```

## Ejemplo: HelloTriangle (GDI)

```cpp
#include <header_main.h>

HWND g_hwnd;

int main() {
    // 1. Crear ventana (CreateWindowExA — 12 args, directo en main)
    HINSTANCE hInstance = GetModuleHandleA(0);
    HCURSOR cur = LoadCursorW(0, 32512);
    g_hwnd = CreateWindowExA(
        0, "STATIC", "HelloTriangle",
        0x00CF0000, 100, 100, 1280, 720,
        0, 0, hInstance, 0
    );
    ShowWindow(g_hwnd, 5);

    // 2. Dibujar triángulo gradiente (SetPixel scanline fill)
    HDC hdc = GetDC(g_hwnd);
    int y = 100;
    while (y <= 550) {
        int t = y - 100;
        int lx = 640 - 300 * t / 450;
        int rx = 640 + 300 * t / 450;
        int r = 255 - t / 2;
        int g = t / 3;
        int b = t / 4;
        if (r < 0) { r = 0; }
        int c = r + g * 256 + b * 65536;
        int x = lx;
        while (x <= rx) {
            SetPixel(hdc, x, y, c);
            x = x + 1;
        }
        y = y + 2;
    }

    // 3. Outline blanco (CreatePen + LineTo)
    HPEN pen = CreatePen(0, 3, 0x00FFFFFF);
    SelectObject(hdc, pen);
    MoveToEx(hdc, 640, 100, 0);
    LineTo(hdc, 340, 550);
    LineTo(hdc, 940, 550);
    LineTo(hdc, 640, 100);
    DeleteObject(pen);
    ReleaseDC(g_hwnd, hdc);

    // 4. Message loop
    void* pmsg = malloc(64);
    int running = 1;
    while (running) {
        if (PeekMessageA(pmsg, 0, 0, 0, 1)) {
            TranslateMessage(pmsg);
            DispatchMessageA(pmsg);
        }
    }
    return 0;
}
```

**Output confirmado:**
```
=== ADead-BIB HelloTriangle ===
Ventana: 0x00040B54
Triangulo dibujado!
Loop (running indefinitely)
```

## Constantes Disponibles (`#define`)

### Window Styles
| Constante | Valor |
|---|---|
| `WS_OVERLAPPEDWINDOW` | `0x00CF0000` |
| `WS_VISIBLE` | `0x10000000` |
| `WS_CAPTION` | `0x00C00000` |
| `WS_POPUP` | `0x80000000` |

### ShowWindow
| Constante | Valor |
|---|---|
| `SW_SHOW` | `5` |
| `SW_HIDE` | `0` |
| `SW_SHOWNORMAL` | `1` |

### Window Messages
| Constante | Valor |
|---|---|
| `WM_CLOSE` | `0x0010` |
| `WM_DESTROY` | `0x0002` |
| `WM_PAINT` | `0x000F` |
| `WM_KEYDOWN` | `0x0100` |

### GDI
| Constante | Valor |
|---|---|
| `PS_SOLID` | `0` |
| `PS_DASH` | `1` |
| `NULL_BRUSH` | `5` |
| `WHITE_PEN` | `6` |

### Virtual Keys
| Constante | Valor |
|---|---|
| `VK_ESCAPE` | `0x1B` |
| `VK_RETURN` | `0x0D` |
| `VK_SPACE` | `0x20` |
| `VK_LEFT/UP/RIGHT/DOWN` | `0x25-0x28` |

### DX12/DXGI
| Constante | Valor |
|---|---|
| `D3D_FEATURE_LEVEL_12_0` | `0xC000` |
| `DXGI_FORMAT_R8G8B8A8_UNORM` | `28` |
| `DXGI_SWAP_EFFECT_FLIP_DISCARD` | `4` |
| `D3D12_COMMAND_LIST_TYPE_DIRECT` | `0` |

## Helpers Inline Disponibles

| Helper | Uso | Notas |
|---|---|---|
| `RGB(r,g,b)` | Color COLORREF | 3 args, funciona en cualquier contexto |
| `HIWORD(x)` | Bits altos | `(x >> 16) & 0xFFFF` |
| `LOWORD(x)` | Bits bajos | `x & 0xFFFF` |
| `SUCCEEDED(hr)` | Check HRESULT | `hr >= 0` |
| `FAILED(hr)` | Check HRESULT | `hr < 0` |
| `AllocMSG()` | Buffer MSG | `malloc(64)` — usa esto para PeekMessageA |
| `DrawLine(hdc,x1,y1,x2,y2,color,width)` | Línea GDI | Crea pen, dibuja, limpia |
| `MessageLoop()` | Loop de mensajes | PeekMessageA con Translate+Dispatch |

## Limitaciones Conocidas

1. **`CreateWindowExA` en `main()` solamente** — La llamada de 12 argumentos debe hacerse directamente en `main()`. El codegen no soporta >4 args stack correctamente desde funciones anidadas.

2. **MSG struct via `malloc(64)`** — No usar `MSG msg;` en stack. El codegen usa 8 bytes por campo, lo que no coincide con el layout real de MSG. Usar `malloc(64)` y pasar el puntero a `PeekMessageA`.

3. **Clase de ventana `"STATIC"`** — Usar la clase built-in `"STATIC"` en vez de `RegisterClassExA`. La struct `WNDCLASSEXA` tiene issues de ABI con el codegen.

4. **Dead Code Elimination activo** — Solo se compilan funciones alcanzables desde `main()`. Las funciones del header que no uses no generan código.

## Pipeline DX12 (Futuro)

El header incluye tipos y constantes DX12/DXGI para trabajo futuro:
- `D3D12CreateDevice`, `CreateDXGIFactory1/2` registrados en IAT
- `ComPtr<T>` template con Get(), GetAddressOf(), Reset()
- `XMFLOAT2/3/4` structs para vértices
- `DXGI_SWAP_CHAIN_DESC` struct

Para un pipeline DX12 completo se necesita:
1. Soporte COM (interfaces virtuales, QueryInterface)
2. Struct ABI compliant (actualmente 8-byte fields)
3. Buffer upload heap + vertex buffer
4. Shader compilation (HLSL → DXBC/DXIL)

## Estadísticas

- **539/539** unit tests passing
- **PE output**: ~5KB ejecutable, 42 IAT slots, 6 DLLs
- **Triángulo GDI**: Gradient fill + outline, ventana 1280x720
