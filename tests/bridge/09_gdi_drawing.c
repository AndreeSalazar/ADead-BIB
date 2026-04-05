// ADead-BIB Bridge Test 09 — GDI Drawing
// Level: ADVANCED
// Tests: GetDC, SetPixel, Rectangle, CreateSolidBrush, SelectObject
// Requires: user32.dll, gdi32.dll, kernel32.dll in IAT

typedef void* HWND;
typedef void* HDC;
typedef void* HINSTANCE;
typedef void* HBRUSH;
typedef void* HGDIOBJ;
typedef void* HMENU;
typedef void* LPVOID;
typedef void* HICON;
typedef void* HCURSOR;
typedef unsigned int UINT;
typedef long long LONG_PTR;
typedef unsigned long long ULONG_PTR;
typedef LONG_PTR LRESULT;
typedef ULONG_PTR WPARAM;
typedef LONG_PTR LPARAM;
typedef const char* LPCSTR;
typedef unsigned long COLORREF;

typedef struct { long x; long y; } POINT;
typedef struct {
    HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam;
    unsigned long time; POINT pt; unsigned long lPrivate;
} MSG;
typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);
typedef struct {
    UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int cbWndExtra;
    HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground;
    LPCSTR lpszMenuName; LPCSTR lpszClassName;
} WNDCLASSA;

extern unsigned short RegisterClassA(const WNDCLASSA*);
extern HWND CreateWindowExA(unsigned long, LPCSTR, LPCSTR, unsigned long,
    int, int, int, int, HWND, HMENU, HINSTANCE, LPVOID);
extern int ShowWindow(HWND, int);
extern int PeekMessageA(MSG*, HWND, UINT, UINT, UINT);
extern int TranslateMessage(const MSG*);
extern LRESULT DispatchMessageA(const MSG*);
extern void PostQuitMessage(int);
extern LRESULT DefWindowProcA(HWND, UINT, WPARAM, LPARAM);
extern HINSTANCE GetModuleHandleA(LPCSTR);
extern HDC GetDC(HWND);
extern int ReleaseDC(HWND, HDC);
extern COLORREF SetPixel(HDC, int, int, COLORREF);
extern HBRUSH CreateSolidBrush(COLORREF);
extern HGDIOBJ SelectObject(HDC, HGDIOBJ);
extern int Rectangle(HDC, int, int, int, int);
extern int DeleteObject(HGDIOBJ);
extern void Sleep(unsigned long);
extern int DestroyWindow(HWND);

int printf(const char*, ...);

LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == 0x0010) { PostQuitMessage(0); return 0; }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

void draw_gradient(HDC hdc, int w, int h) {
    for (int y = 0; y < h; y += 4) {
        for (int x = 0; x < w; x += 4) {
            int r = (x * 255) / w;
            int g = (y * 255) / h;
            int b = 128;
            COLORREF color = (COLORREF)(r | (g << 8) | (b << 16));
            SetPixel(hdc, x, y, color);
        }
    }
}

void draw_rectangles(HDC hdc) {
    HBRUSH red   = CreateSolidBrush(0x000000FF);
    HBRUSH green = CreateSolidBrush(0x0000FF00);
    HBRUSH blue  = CreateSolidBrush(0x00FF0000);

    HGDIOBJ old = SelectObject(hdc, (HGDIOBJ)red);
    Rectangle(hdc, 50, 50, 250, 200);

    SelectObject(hdc, (HGDIOBJ)green);
    Rectangle(hdc, 100, 100, 300, 250);

    SelectObject(hdc, (HGDIOBJ)blue);
    Rectangle(hdc, 150, 150, 350, 300);

    SelectObject(hdc, old);
    DeleteObject((HGDIOBJ)red);
    DeleteObject((HGDIOBJ)green);
    DeleteObject((HGDIOBJ)blue);
}

int main() {
    printf("=== ADead-BIB Bridge Test 09: GDI Drawing ===\n");

    HINSTANCE hInst = GetModuleHandleA(0);
    WNDCLASSA wc;
    wc.style = 3;
    wc.lpfnWndProc = WndProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hInstance = hInst;
    wc.hIcon = 0;
    wc.hCursor = 0;
    wc.hbrBackground = 0;
    wc.lpszMenuName = 0;
    wc.lpszClassName = "ADeadGDI";

    RegisterClassA(&wc);

    HWND hwnd = CreateWindowExA(0, "ADeadGDI", "ADead-BIB GDI Test",
        0x10CF0000, 100, 100, 640, 480, 0, 0, hInst, 0);
    if (hwnd == 0) { printf("FAIL: CreateWindow\n"); return 1; }

    ShowWindow(hwnd, 1);

    HDC hdc = GetDC(hwnd);
    if (hdc == 0) { printf("FAIL: GetDC\n"); return 1; }

    draw_gradient(hdc, 640, 480);
    printf("Gradient drawn\n");

    draw_rectangles(hdc);
    printf("Rectangles drawn\n");

    ReleaseDC(hwnd, hdc);

    int frame = 0;
    int running = 1;
    while (running && frame < 180) {
        MSG msg;
        while (PeekMessageA(&msg, 0, 0, 0, 1)) {
            if (msg.message == 0x0012) { running = 0; break; }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        Sleep(16);
        frame++;
    }

    DestroyWindow(hwnd);
    printf("GDI window lived for %d frames\n", frame);
    printf("=== Test 09: PASS ===\n");
    return 0;
}
