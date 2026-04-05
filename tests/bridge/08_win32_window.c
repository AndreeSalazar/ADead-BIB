// ADead-BIB Bridge Test 08 — Win32 Window Creation
// Level: ADVANCED
// Tests: RegisterClassA, CreateWindowExA, ShowWindow, message loop
// Requires: user32.dll, kernel32.dll in IAT

typedef void* HWND;
typedef void* HINSTANCE;
typedef void* HCURSOR;
typedef void* HICON;
typedef void* HBRUSH;
typedef void* HMENU;
typedef void* LPVOID;
typedef unsigned int UINT;
typedef long long LONG_PTR;
typedef unsigned long long ULONG_PTR;
typedef LONG_PTR LRESULT;
typedef ULONG_PTR WPARAM;
typedef LONG_PTR LPARAM;
typedef const char* LPCSTR;

typedef struct { long x; long y; } POINT;
typedef struct {
    HWND hwnd;
    UINT message;
    WPARAM wParam;
    LPARAM lParam;
    unsigned long time;
    POINT pt;
    unsigned long lPrivate;
} MSG;

typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);

typedef struct {
    UINT style;
    WNDPROC lpfnWndProc;
    int cbClsExtra;
    int cbWndExtra;
    HINSTANCE hInstance;
    HICON hIcon;
    HCURSOR hCursor;
    HBRUSH hbrBackground;
    LPCSTR lpszMenuName;
    LPCSTR lpszClassName;
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
extern int DestroyWindow(HWND);
extern void Sleep(unsigned long);
extern void ExitProcess(unsigned int);

int printf(const char*, ...);

LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == 0x0010) { PostQuitMessage(0); return 0; }
    if (msg == 0x000F) { return 0; }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

int main() {
    printf("=== ADead-BIB Bridge Test 08: Win32 Window ===\n");

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
    wc.lpszClassName = "ADeadBIBTest";

    unsigned short atom = RegisterClassA(&wc);
    if (atom == 0) {
        printf("FAIL: RegisterClassA returned 0\n");
        return 1;
    }
    printf("RegisterClassA: atom=%d\n", (int)atom);

    HWND hwnd = CreateWindowExA(
        0, "ADeadBIBTest", "ADead-BIB Test Window",
        0x10CF0000,
        100, 100, 800, 600,
        0, 0, hInst, 0
    );

    if (hwnd == 0) {
        printf("FAIL: CreateWindowExA returned NULL\n");
        return 1;
    }
    printf("CreateWindowExA: hwnd=%p\n", hwnd);

    ShowWindow(hwnd, 1);
    printf("ShowWindow: OK\n");

    // Run message loop for ~2 seconds then exit
    int frame = 0;
    int running = 1;
    while (running && frame < 120) {
        MSG msg;
        while (PeekMessageA(&msg, 0, 0, 0, 1)) {
            if (msg.message == 0x0012) {
                running = 0;
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        Sleep(16);
        frame++;
    }

    DestroyWindow(hwnd);
    printf("Window lived for %d frames\n", frame);
    printf("=== Test 08: PASS ===\n");
    return 0;
}
