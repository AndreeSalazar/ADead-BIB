// ADead-BIB Bridge Test 10 — OpenGL 1.1 Basic Triangle
// Level: ADVANCED
// Tests: wglCreateContext, wglMakeCurrent, glClear, glBegin/glEnd, glVertex3f
// Requires: user32.dll, gdi32.dll, opengl32.dll in IAT

typedef void* HWND;
typedef void* HDC;
typedef void* HGLRC;
typedef void* HINSTANCE;
typedef void* HBRUSH;
typedef void* HICON;
typedef void* HCURSOR;
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
    HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam;
    unsigned long time; POINT pt; unsigned long lPrivate;
} MSG;
typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);
typedef struct {
    UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int cbWndExtra;
    HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground;
    LPCSTR lpszMenuName; LPCSTR lpszClassName;
} WNDCLASSA;

typedef struct {
    unsigned short nSize;
    unsigned short nVersion;
    unsigned long dwFlags;
    unsigned char iPixelType;
    unsigned char cColorBits;
    unsigned char cRedBits;
    unsigned char cRedShift;
    unsigned char cGreenBits;
    unsigned char cGreenShift;
    unsigned char cBlueBits;
    unsigned char cBlueShift;
    unsigned char cAlphaBits;
    unsigned char cAlphaShift;
    unsigned char cAccumBits;
    unsigned char cAccumRedBits;
    unsigned char cAccumGreenBits;
    unsigned char cAccumBlueBits;
    unsigned char cAccumAlphaBits;
    unsigned char cDepthBits;
    unsigned char cStencilBits;
    unsigned char cAuxBuffers;
    unsigned char iLayerType;
    unsigned char bReserved;
    unsigned long dwLayerMask;
    unsigned long dwVisibleMask;
    unsigned long dwDamageMask;
} PIXELFORMATDESCRIPTOR;

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
extern int DestroyWindow(HWND);
extern void Sleep(unsigned long);
extern int SwapBuffers(HDC);
extern int ChoosePixelFormat(HDC, const PIXELFORMATDESCRIPTOR*);
extern int SetPixelFormat(HDC, int, const PIXELFORMATDESCRIPTOR*);
extern HGLRC wglCreateContext(HDC);
extern int wglMakeCurrent(HDC, HGLRC);
extern int wglDeleteContext(HGLRC);
extern void glClear(unsigned int);
extern void glClearColor(float, float, float, float);
extern void glBegin(unsigned int);
extern void glEnd(void);
extern void glVertex3f(float, float, float);
extern void glColor3f(float, float, float);
extern void glViewport(int, int, int, int);
extern void glMatrixMode(unsigned int);
extern void glLoadIdentity(void);
extern void glFlush(void);

int printf(const char*, ...);
void* memset(void*, int, unsigned long long);

LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == 0x0010) { PostQuitMessage(0); return 0; }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

void render(void) {
    glClearColor(0.1f, 0.1f, 0.2f, 1.0f);
    glClear(0x00004000);

    glBegin(0x0004);
        glColor3f(1.0f, 0.0f, 0.0f);
        glVertex3f(0.0f, 0.5f, 0.0f);
        glColor3f(0.0f, 1.0f, 0.0f);
        glVertex3f(-0.5f, -0.5f, 0.0f);
        glColor3f(0.0f, 0.0f, 1.0f);
        glVertex3f(0.5f, -0.5f, 0.0f);
    glEnd();

    glFlush();
}

int main() {
    printf("=== ADead-BIB Bridge Test 10: OpenGL ===\n");

    HINSTANCE hInst = GetModuleHandleA(0);
    WNDCLASSA wc;
    memset(&wc, 0, sizeof(WNDCLASSA));
    wc.style = 0x0023;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = "ADeadGL";
    RegisterClassA(&wc);

    HWND hwnd = CreateWindowExA(0, "ADeadGL", "ADead-BIB OpenGL Test",
        0x10CF0000, 100, 100, 800, 600, 0, 0, hInst, 0);
    if (hwnd == 0) { printf("FAIL: CreateWindow\n"); return 1; }

    HDC hdc = GetDC(hwnd);

    PIXELFORMATDESCRIPTOR pfd;
    memset(&pfd, 0, sizeof(PIXELFORMATDESCRIPTOR));
    pfd.nSize = sizeof(PIXELFORMATDESCRIPTOR);
    pfd.nVersion = 1;
    pfd.dwFlags = 0x00000025;
    pfd.iPixelType = 0;
    pfd.cColorBits = 32;
    pfd.cDepthBits = 24;
    pfd.cStencilBits = 8;

    int pf = ChoosePixelFormat(hdc, &pfd);
    printf("ChoosePixelFormat: %d\n", pf);
    SetPixelFormat(hdc, pf, &pfd);

    HGLRC hrc = wglCreateContext(hdc);
    if (hrc == 0) { printf("FAIL: wglCreateContext\n"); ReleaseDC(hwnd, hdc); return 1; }
    printf("wglCreateContext: %p\n", hrc);

    wglMakeCurrent(hdc, hrc);
    printf("wglMakeCurrent: OK\n");

    ShowWindow(hwnd, 1);
    glViewport(0, 0, 800, 600);
    glMatrixMode(0x1701);
    glLoadIdentity();

    int frame = 0;
    int running = 1;
    while (running && frame < 180) {
        MSG msg;
        while (PeekMessageA(&msg, 0, 0, 0, 1)) {
            if (msg.message == 0x0012) { running = 0; break; }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        render();
        SwapBuffers(hdc);
        Sleep(16);
        frame++;
    }

    wglMakeCurrent(0, 0);
    wglDeleteContext(hrc);
    ReleaseDC(hwnd, hdc);
    DestroyWindow(hwnd);

    printf("OpenGL rendered %d frames\n", frame);
    printf("=== Test 10: PASS ===\n");
    return 0;
}
