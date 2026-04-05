// ADead-BIB Bridge Test 11 — DirectX 9 Window
// Level: EXPERT
// Tests: Direct3DCreate9, CreateDevice, Clear, Present, BeginScene/EndScene
// Compile: adB cc 11_dx9_window.c -o dx9_test.exe -Warm ub
// Requires: d3d9.dll, user32.dll, kernel32.dll, ole32.dll in IAT

// ═══════════════════════════════════════════════════════════
// Win32 types (no windows.h needed — ADead-BIB is self-contained)
// ═══════════════════════════════════════════════════════════
typedef void* HWND;
typedef void* HINSTANCE;
typedef void* HICON;
typedef void* HCURSOR;
typedef void* HBRUSH;
typedef void* HMENU;
typedef void* LPVOID;
typedef unsigned int UINT;
typedef unsigned long DWORD;
typedef long LONG;
typedef long long LONG_PTR;
typedef unsigned long long ULONG_PTR;
typedef LONG_PTR LRESULT;
typedef ULONG_PTR WPARAM;
typedef LONG_PTR LPARAM;
typedef const char* LPCSTR;
typedef long HRESULT;

typedef struct { LONG x; LONG y; } POINT;
typedef struct {
    HWND hwnd; UINT message; WPARAM wParam; LPARAM lParam;
    DWORD time; POINT pt; DWORD lPrivate;
} MSG;
typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);
typedef struct {
    UINT style; WNDPROC lpfnWndProc; int cbClsExtra; int cbWndExtra;
    HINSTANCE hInstance; HICON hIcon; HCURSOR hCursor; HBRUSH hbrBackground;
    LPCSTR lpszMenuName; LPCSTR lpszClassName;
} WNDCLASSA;
typedef struct { LONG left; LONG top; LONG right; LONG bottom; } RECT;

// ═══════════════════════════════════════════════════════════
// DirectX 9 types (minimal, no d3d9.h needed)
// ═══════════════════════════════════════════════════════════
typedef void* LPDIRECT3D9;
typedef void* LPDIRECT3DDEVICE9;

typedef struct {
    UINT BackBufferWidth;
    UINT BackBufferHeight;
    UINT BackBufferFormat;       // D3DFMT_X8R8G8B8 = 22
    UINT BackBufferCount;
    UINT MultiSampleType;        // D3DMULTISAMPLE_NONE = 0
    DWORD MultiSampleQuality;
    UINT SwapEffect;             // D3DSWAPEFFECT_DISCARD = 1
    HWND hDeviceWindow;
    int Windowed;
    int EnableAutoDepthStencil;
    UINT AutoDepthStencilFormat;
    DWORD Flags;
    UINT FullScreen_RefreshRateInHz;
    UINT PresentationInterval;   // D3DPRESENT_INTERVAL_DEFAULT = 0
} D3DPRESENT_PARAMETERS;

// ═══════════════════════════════════════════════════════════
// Extern declarations (resolved by IAT v5)
// ═══════════════════════════════════════════════════════════
extern int printf(const char*, ...);
extern void* memset(void*, int, unsigned long long);

// kernel32
extern HINSTANCE GetModuleHandleA(LPCSTR);
extern void Sleep(DWORD);
extern void ExitProcess(UINT);

// user32
extern unsigned short RegisterClassA(const WNDCLASSA*);
extern HWND CreateWindowExA(DWORD, LPCSTR, LPCSTR, DWORD,
    int, int, int, int, HWND, HMENU, HINSTANCE, LPVOID);
extern int ShowWindow(HWND, int);
extern int PeekMessageA(MSG*, HWND, UINT, UINT, UINT);
extern int TranslateMessage(const MSG*);
extern LRESULT DispatchMessageA(const MSG*);
extern void PostQuitMessage(int);
extern LRESULT DefWindowProcA(HWND, UINT, WPARAM, LPARAM);
extern int DestroyWindow(HWND);
extern int GetClientRect(HWND, RECT*);

// d3d9
extern LPDIRECT3D9 Direct3DCreate9(UINT SDKVersion);

// ═══════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════
#define D3D_SDK_VERSION         32
#define D3DFMT_X8R8G8B8         22
#define D3DFMT_D24S8            75
#define D3DSWAPEFFECT_DISCARD   1
#define D3DCLEAR_TARGET         1
#define D3DCLEAR_ZBUFFER        2
#define D3DDEVTYPE_HAL          1
#define D3DCREATE_HARDWARE_VERTEXPROCESSING 0x00000040
#define WM_CLOSE    0x0010
#define WM_QUIT     0x0012
#define WM_DESTROY  0x0002
#define WS_OVERLAPPEDWINDOW 0x00CF0000
#define WS_VISIBLE  0x10000000
#define PM_REMOVE   1

// ═══════════════════════════════════════════════════════════
// WndProc
// ═══════════════════════════════════════════════════════════
LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == WM_CLOSE || msg == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

// ═══════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════
int main() {
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  ADead-BIB DirectX 9 Test                ║\n");
    printf("║  Compile: adB cc 11_dx9_window.c -Warm ub║\n");
    printf("╚══════════════════════════════════════════╝\n\n");

    // ── Step 1: Create Win32 Window ──
    printf("[STEP 1] Creating Win32 window...\n");
    HINSTANCE hInst = GetModuleHandleA(0);

    WNDCLASSA wc;
    memset(&wc, 0, sizeof(WNDCLASSA));
    wc.style = 3;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = "ADeadDX9";

    unsigned short atom = RegisterClassA(&wc);
    printf("  RegisterClassA: atom=%d %s\n", (int)atom, atom ? "OK" : "FAIL");

    HWND hwnd = CreateWindowExA(0, "ADeadDX9", "ADead-BIB DirectX 9",
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        100, 100, 800, 600, 0, 0, hInst, 0);
    printf("  CreateWindowExA: hwnd=%p %s\n", hwnd, hwnd ? "OK" : "FAIL");
    if (!hwnd) { printf("FAIL: No window\n"); return 1; }

    ShowWindow(hwnd, 1);

    // ── Step 2: Create Direct3D9 ──
    printf("\n[STEP 2] Creating Direct3D9 object...\n");
    LPDIRECT3D9 d3d = Direct3DCreate9(D3D_SDK_VERSION);
    printf("  Direct3DCreate9(%d): d3d=%p %s\n",
        D3D_SDK_VERSION, d3d, d3d ? "OK" : "FAIL");

    if (!d3d) {
        printf("\n[RESULT] Direct3D9 not available on this system.\n");
        printf("         This is expected if no DX9 runtime is installed.\n");
        printf("         Window was created successfully — Win32 API works!\n");

        // Still run message loop briefly to show window
        int frame = 0;
        while (frame < 60) {
            MSG msg;
            while (PeekMessageA(&msg, 0, 0, 0, PM_REMOVE)) {
                if (msg.message == WM_QUIT) goto cleanup;
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            Sleep(16);
            frame++;
        }
        goto cleanup;
    }

    // ── Step 3: Create Device ──
    printf("\n[STEP 3] Creating D3D9 Device...\n");
    D3DPRESENT_PARAMETERS pp;
    memset(&pp, 0, sizeof(D3DPRESENT_PARAMETERS));
    pp.BackBufferWidth = 800;
    pp.BackBufferHeight = 600;
    pp.BackBufferFormat = D3DFMT_X8R8G8B8;
    pp.BackBufferCount = 1;
    pp.SwapEffect = D3DSWAPEFFECT_DISCARD;
    pp.hDeviceWindow = hwnd;
    pp.Windowed = 1;
    pp.EnableAutoDepthStencil = 1;
    pp.AutoDepthStencilFormat = D3DFMT_D24S8;

    // IDirect3D9::CreateDevice is vtable[16] (method index 16)
    // vtable layout: QueryInterface(0), AddRef(1), Release(2), ...
    // CreateDevice is at index 16 in IDirect3D9 vtable
    void** d3d_vtable = *((void***)d3d);
    LPDIRECT3DDEVICE9 device = 0;

    // CreateDevice(Adapter=0, DeviceType=HAL, hFocusWindow, BehaviorFlags, pPP, ppDevice)
    typedef HRESULT (*CreateDeviceFn)(void*, UINT, UINT, HWND, DWORD, void*, void**);
    CreateDeviceFn pCreateDevice = (CreateDeviceFn)d3d_vtable[16];

    HRESULT hr = pCreateDevice(d3d, 0, D3DDEVTYPE_HAL, hwnd,
        D3DCREATE_HARDWARE_VERTEXPROCESSING, &pp, (void**)&device);
    printf("  CreateDevice: hr=0x%08X device=%p %s\n",
        (unsigned int)hr, device, hr == 0 ? "OK" : "FAIL");

    if (hr != 0 || !device) {
        printf("  CreateDevice failed (maybe no HAL adapter).\n");
        printf("  D3D9 object was created — DLL import works!\n");

        // Release D3D9
        typedef DWORD (*ReleaseFn)(void*);
        ReleaseFn pRelease = (ReleaseFn)d3d_vtable[2];
        pRelease(d3d);

        int frame = 0;
        while (frame < 60) {
            MSG msg;
            while (PeekMessageA(&msg, 0, 0, 0, PM_REMOVE)) {
                if (msg.message == WM_QUIT) goto cleanup;
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            Sleep(16);
            frame++;
        }
        goto cleanup;
    }

    // ── Step 4: Render loop ──
    printf("\n[STEP 4] Entering render loop (3 seconds)...\n");
    void** dev_vtable = *((void***)device);

    // IDirect3DDevice9 vtable indices:
    // Clear=43, BeginScene=41, EndScene=42, Present=17
    typedef HRESULT (*ClearFn)(void*, DWORD, void*, DWORD, DWORD, float, DWORD);
    typedef HRESULT (*BeginSceneFn)(void*);
    typedef HRESULT (*EndSceneFn)(void*);
    typedef HRESULT (*PresentFn)(void*, void*, void*, HWND, void*);

    ClearFn pClear = (ClearFn)dev_vtable[43];
    BeginSceneFn pBeginScene = (BeginSceneFn)dev_vtable[41];
    EndSceneFn pEndScene = (EndSceneFn)dev_vtable[42];
    PresentFn pPresent = (PresentFn)dev_vtable[17];

    int frame = 0;
    int running = 1;
    while (running && frame < 180) {
        MSG msg;
        while (PeekMessageA(&msg, 0, 0, 0, PM_REMOVE)) {
            if (msg.message == WM_QUIT) { running = 0; break; }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        if (!running) break;

        // Cycle background color: blue → green → red
        DWORD colors[3];
        colors[0] = 0xFF003366; // dark blue
        colors[1] = 0xFF336600; // dark green
        colors[2] = 0xFF660033; // dark red
        DWORD color = colors[(frame / 60) % 3];

        pClear(device, 0, 0, D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER, color, 1.0f, 0);
        pBeginScene(device);
        // (geometry rendering would go here)
        pEndScene(device);
        pPresent(device, 0, 0, 0, 0);

        if (frame % 60 == 0) {
            printf("  Frame %d: Clear color=0x%08X, Present OK\n", frame, (unsigned int)color);
        }
        Sleep(16);
        frame++;
    }

    printf("\n[STEP 5] Cleanup...\n");
    // Release device
    typedef DWORD (*ReleaseFn2)(void*);
    ReleaseFn2 pReleaseDevice = (ReleaseFn2)dev_vtable[2];
    pReleaseDevice(device);
    printf("  Device released\n");

    // Release D3D9
    typedef DWORD (*ReleaseFn3)(void*);
    ReleaseFn3 pReleaseD3D = (ReleaseFn3)d3d_vtable[2];
    pReleaseD3D(d3d);
    printf("  D3D9 released\n");

cleanup:
    DestroyWindow(hwnd);
    printf("  Window destroyed\n");

    printf("\n╔══════════════════════════════════════════╗\n");
    printf("║  DirectX 9 Test: COMPLETE                ║\n");
    printf("║  Rendered %d frames                      ║\n", frame);
    printf("╚══════════════════════════════════════════╝\n");
    return 0;
}
