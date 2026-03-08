// ADead-BIB HelloTriangle — GDI Rendering
#include <header_main.h>

HWND g_hwnd;

int main() {
    printf("Step 1\n");
    HINSTANCE hInstance = GetModuleHandleA(0);
    printf("hInstance=%p\n", hInstance);

    // Test: does LoadCursorW (2 args) work?
    printf("Step 1b\n");
    HCURSOR cur = LoadCursorW(0, 32512);
    printf("cur=%p\n", cur);

    // Test: does ShowWindow (2 args) work with null hwnd? (will fail but shouldn't crash)
    printf("Step 2\n");
    g_hwnd = CreateWindowExA(
        0, "STATIC", "ADead-BIB HelloTriangle",
        0x00CF0000, 100, 100, 1280, 720,
        0, 0, hInstance, 0
    );
    printf("hwnd=%p\n", g_hwnd);

    if (g_hwnd == 0) {
        printf("No window\n");
        return 1;
    }

    ShowWindow(g_hwnd, 5);
    printf("Window shown\n");

    HDC hdc = GetDC(g_hwnd);
    printf("hdc=%p\n", hdc);

    // Draw gradient triangle with SetPixel
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

    // White outline
    HPEN pen = CreatePen(0, 3, 0x00FFFFFF);
    SelectObject(hdc, pen);
    MoveToEx(hdc, 640, 100, 0);
    LineTo(hdc, 340, 550);
    LineTo(hdc, 940, 550);
    LineTo(hdc, 640, 100);
    DeleteObject(pen);

    printf("Triangle drawn\n");
    ReleaseDC(g_hwnd, hdc);

    // Message loop — use malloc'd MSG buffer for correct ABI
    printf("Loop\n");
    void* pmsg = malloc(64);
    int running = 1;
    while (running) {
        if (PeekMessageA(pmsg, 0, 0, 0, 1)) {
            TranslateMessage(pmsg);
            DispatchMessageA(pmsg);
        }
    }

    printf("Done\n");
    return 0;
}
