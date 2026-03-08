// ================================================================
// ADead-BIB HelloTriangle — GDI Rendering
// ================================================================
// Pipeline gráfico completo de ADead-BIB:
//   1. CreateWindowExA (directo en main — 12 args)
//   2. Gradient triangle via SetPixel scanline fill
//   3. White outline via CreatePen + LineTo
//   4. MessageLoop() helper para mantener ventana
//
// Compilar:  adb cxx src/main.cpp -o bin/dx12_hello.exe
// Paso a paso: adb step src/main.cpp
// ================================================================
#include <header_main.h>

HWND g_hwnd;

int main() {
    printf("=== ADead-BIB HelloTriangle ===\n");

    // 1. Crear ventana (12 args — directo en main)
    HINSTANCE hInstance = GetModuleHandleA(0);
    HCURSOR cur = LoadCursorW(0, 32512);
    g_hwnd = CreateWindowExA(
        0, "STATIC", "ADead-BIB HelloTriangle",
        0x00CF0000, 100, 100, 1280, 720,
        0, 0, hInstance, 0
    );
    if (g_hwnd == 0) {
        printf("ERROR: ventana\n");
        return 1;
    }
    ShowWindow(g_hwnd, 5);
    printf("Ventana: %p\n", g_hwnd);

    // 2. Dibujar triángulo gradiente (scanline fill)
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

    // 3. Outline blanco
    HPEN pen = CreatePen(0, 3, 0x00FFFFFF);
    SelectObject(hdc, pen);
    MoveToEx(hdc, 640, 100, 0);
    LineTo(hdc, 340, 550);
    LineTo(hdc, 940, 550);
    LineTo(hdc, 640, 100);
    DeleteObject(pen);

    printf("Triangulo dibujado!\n");
    ReleaseDC(g_hwnd, hdc);

    // 4. Message loop (helper del header)
    printf("Loop\n");
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
