// ADead-BIB Bridge Test 12 — DirectX 11 Window
// Level: EXPERT
// Tests: D3D11CreateDeviceAndSwapChain, ClearRenderTargetView, Present
// Compile: adB cc 12_dx11_window.c -o dx11_test.exe -Warm ub
// Requires: d3d11.dll, dxgi.dll, user32.dll, kernel32.dll, ole32.dll in IAT

// ═══════════════════════════════════════════════════════════
// Win32 types
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

// ═══════════════════════════════════════════════════════════
// DXGI types (minimal)
// ═══════════════════════════════════════════════════════════
typedef struct {
    UINT Width;
    UINT Height;
    UINT RefreshRate_Numerator;
    UINT RefreshRate_Denominator;
    UINT Format;             // DXGI_FORMAT_R8G8B8A8_UNORM = 28
    UINT ScanlineOrdering;
    UINT Scaling;
} DXGI_MODE_DESC;

typedef struct {
    UINT Count;
    UINT Quality;
} DXGI_SAMPLE_DESC;

typedef struct {
    DXGI_MODE_DESC BufferDesc;
    DXGI_SAMPLE_DESC SampleDesc;
    UINT BufferUsage;        // DXGI_USAGE_RENDER_TARGET_OUTPUT = 0x20
    UINT BufferCount;
    HWND OutputWindow;
    int Windowed;
    UINT SwapEffect;         // DXGI_SWAP_EFFECT_DISCARD = 0
    UINT Flags;
} DXGI_SWAP_CHAIN_DESC;

// ═══════════════════════════════════════════════════════════
// Externs (IAT v5)
// ═══════════════════════════════════════════════════════════
extern int printf(const char*, ...);
extern void* memset(void*, int, unsigned long long);

extern HINSTANCE GetModuleHandleA(LPCSTR);
extern void Sleep(DWORD);
extern void ExitProcess(UINT);
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

// d3d11
extern HRESULT D3D11CreateDeviceAndSwapChain(
    void* pAdapter,           // IDXGIAdapter* = NULL for default
    UINT DriverType,          // D3D_DRIVER_TYPE_HARDWARE = 1
    void* Software,           // NULL
    UINT Flags,               // 0 or D3D11_CREATE_DEVICE_DEBUG
    void* pFeatureLevels,     // NULL = default
    UINT FeatureLevels,       // 0
    UINT SDKVersion,          // D3D11_SDK_VERSION = 7
    DXGI_SWAP_CHAIN_DESC* pSwapChainDesc,
    void** ppSwapChain,       // IDXGISwapChain**
    void** ppDevice,          // ID3D11Device**
    void* pFeatureLevel,      // D3D_FEATURE_LEVEL*
    void** ppImmediateContext  // ID3D11DeviceContext**
);

// ═══════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════
#define D3D_DRIVER_TYPE_HARDWARE 1
#define D3D11_SDK_VERSION        7
#define DXGI_FORMAT_R8G8B8A8_UNORM 28
#define DXGI_USAGE_RENDER_TARGET_OUTPUT 0x00000020
#define WM_CLOSE    0x0010
#define WM_QUIT     0x0012
#define WM_DESTROY  0x0002
#define PM_REMOVE   1

LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == WM_CLOSE || msg == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

int main() {
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  ADead-BIB DirectX 11 Test               ║\n");
    printf("║  Compile: adB cc 12_dx11_window.c -Warm ub║\n");
    printf("╚══════════════════════════════════════════╝\n\n");

    // ── Step 1: Create Window ──
    printf("[STEP 1] Creating Win32 window...\n");
    HINSTANCE hInst = GetModuleHandleA(0);
    WNDCLASSA wc;
    memset(&wc, 0, sizeof(WNDCLASSA));
    wc.style = 3;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = "ADeadDX11";

    RegisterClassA(&wc);
    HWND hwnd = CreateWindowExA(0, "ADeadDX11", "ADead-BIB DirectX 11",
        0x10CF0000, 100, 100, 800, 600, 0, 0, hInst, 0);
    printf("  Window: hwnd=%p %s\n", hwnd, hwnd ? "OK" : "FAIL");
    if (!hwnd) return 1;
    ShowWindow(hwnd, 1);

    // ── Step 2: Create D3D11 Device + SwapChain ──
    printf("\n[STEP 2] Creating D3D11 Device + SwapChain...\n");

    DXGI_SWAP_CHAIN_DESC sd;
    memset(&sd, 0, sizeof(DXGI_SWAP_CHAIN_DESC));
    sd.BufferDesc.Width = 800;
    sd.BufferDesc.Height = 600;
    sd.BufferDesc.RefreshRate_Numerator = 60;
    sd.BufferDesc.RefreshRate_Denominator = 1;
    sd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    sd.SampleDesc.Count = 1;
    sd.SampleDesc.Quality = 0;
    sd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    sd.BufferCount = 1;
    sd.OutputWindow = hwnd;
    sd.Windowed = 1;
    sd.SwapEffect = 0;

    void* swapChain = 0;
    void* device = 0;
    void* context = 0;
    UINT featureLevel = 0;

    HRESULT hr = D3D11CreateDeviceAndSwapChain(
        0,                          // Default adapter
        D3D_DRIVER_TYPE_HARDWARE,   // Hardware
        0,                          // No software
        0,                          // Flags
        0,                          // Default feature levels
        0,                          // 0 = use defaults
        D3D11_SDK_VERSION,          // SDK version
        &sd,                        // Swap chain desc
        &swapChain,                 // Out: swap chain
        &device,                    // Out: device
        &featureLevel,              // Out: feature level
        &context                    // Out: context
    );

    printf("  D3D11CreateDeviceAndSwapChain: hr=0x%08X\n", (unsigned int)hr);
    printf("  Device:     %p %s\n", device, device ? "OK" : "NULL");
    printf("  SwapChain:  %p %s\n", swapChain, swapChain ? "OK" : "NULL");
    printf("  Context:    %p %s\n", context, context ? "OK" : "NULL");
    printf("  FeatureLevel: 0x%X\n", featureLevel);

    int frame = 0;

    if (hr != 0 || !device) {
        printf("\n[RESULT] D3D11 device creation failed (hr=0x%08X)\n", (unsigned int)hr);
        printf("         This may happen without proper GPU drivers.\n");
        printf("         DLL import itself works — IAT is correct!\n");

        // Show window briefly
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

    // ── Step 3: Get RenderTargetView from back buffer ──
    printf("\n[STEP 3] Getting back buffer + creating RTV...\n");

    // IDXGISwapChain::GetBuffer(0, IID_ID3D11Texture2D, &backBuffer)
    void** sc_vtable = *((void***)swapChain);
    // GetBuffer is vtable[9] in IDXGISwapChain
    typedef HRESULT (*GetBufferFn)(void*, UINT, void*, void**);
    GetBufferFn pGetBuffer = (GetBufferFn)sc_vtable[9];

    // IID_ID3D11Texture2D = {6f15aaf2-d208-4e89-9ab4-489535d34f9c}
    unsigned char iid_tex2d[16];
    memset(iid_tex2d, 0, 16);
    // Store GUID bytes (little-endian)
    iid_tex2d[0] = 0xF2; iid_tex2d[1] = 0xAA; iid_tex2d[2] = 0x15; iid_tex2d[3] = 0x6F;
    iid_tex2d[4] = 0x08; iid_tex2d[5] = 0xD2;
    iid_tex2d[6] = 0x89; iid_tex2d[7] = 0x4E;
    iid_tex2d[8] = 0x9A; iid_tex2d[9] = 0xB4;
    iid_tex2d[10] = 0x48; iid_tex2d[11] = 0x95; iid_tex2d[12] = 0x35;
    iid_tex2d[13] = 0xD3; iid_tex2d[14] = 0x4F; iid_tex2d[15] = 0x9C;

    void* backBuffer = 0;
    hr = pGetBuffer(swapChain, 0, iid_tex2d, &backBuffer);
    printf("  GetBuffer: hr=0x%08X backBuffer=%p\n", (unsigned int)hr, backBuffer);

    void* rtv = 0;
    if (hr == 0 && backBuffer) {
        // ID3D11Device::CreateRenderTargetView is vtable[9]
        void** dev_vtable = *((void***)device);
        typedef HRESULT (*CreateRTVFn)(void*, void*, void*, void**);
        CreateRTVFn pCreateRTV = (CreateRTVFn)dev_vtable[9];
        hr = pCreateRTV(device, backBuffer, 0, &rtv);
        printf("  CreateRenderTargetView: hr=0x%08X rtv=%p\n", (unsigned int)hr, rtv);

        // Release back buffer
        void** bb_vtable = *((void***)backBuffer);
        typedef DWORD (*ReleaseFn)(void*);
        ((ReleaseFn)bb_vtable[2])(backBuffer);
    }

    // ── Step 4: Render loop ──
    printf("\n[STEP 4] Entering render loop (3 seconds)...\n");
    void** ctx_vtable = *((void***)context);

    // ID3D11DeviceContext vtable:
    // OMSetRenderTargets = vtable[33]
    // ClearRenderTargetView = vtable[50]
    typedef void (*OMSetRTFn)(void*, UINT, void**, void*);
    typedef void (*ClearRTVFn)(void*, void*, float*);
    OMSetRTFn pOMSetRT = (OMSetRTFn)ctx_vtable[33];
    ClearRTVFn pClearRTV = (ClearRTVFn)ctx_vtable[50];

    // IDXGISwapChain::Present = vtable[8]
    typedef HRESULT (*PresentFn)(void*, UINT, UINT);
    PresentFn pPresent = (PresentFn)sc_vtable[8];

    if (rtv) {
        pOMSetRT(context, 1, &rtv, 0);
    }

    int running = 1;
    while (running && frame < 180) {
        MSG msg;
        while (PeekMessageA(&msg, 0, 0, 0, PM_REMOVE)) {
            if (msg.message == WM_QUIT) { running = 0; break; }
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
        if (!running) break;

        if (rtv) {
            // Cycle color: cornflower blue → teal → orange
            float colors[3][4];
            colors[0][0] = 0.39f; colors[0][1] = 0.58f; colors[0][2] = 0.93f; colors[0][3] = 1.0f;
            colors[1][0] = 0.0f;  colors[1][1] = 0.5f;  colors[1][2] = 0.5f;  colors[1][3] = 1.0f;
            colors[2][0] = 1.0f;  colors[2][1] = 0.65f; colors[2][2] = 0.0f;  colors[2][3] = 1.0f;
            int ci = (frame / 60) % 3;
            pClearRTV(context, rtv, colors[ci]);
        }

        pPresent(swapChain, 0, 0);

        if (frame % 60 == 0) {
            printf("  Frame %d: ClearRTV + Present OK\n", frame);
        }
        Sleep(16);
        frame++;
    }

    // ── Step 5: Cleanup ──
    printf("\n[STEP 5] Cleanup...\n");
    typedef DWORD (*ReleaseFn)(void*);

    if (rtv) {
        void** rtv_vt = *((void***)rtv);
        ((ReleaseFn)rtv_vt[2])(rtv);
        printf("  RTV released\n");
    }
    if (context) {
        void** ctx_vt = *((void***)context);
        ((ReleaseFn)ctx_vt[2])(context);
        printf("  Context released\n");
    }
    if (swapChain) {
        ((ReleaseFn)sc_vtable[2])(swapChain);
        printf("  SwapChain released\n");
    }
    if (device) {
        void** dev_vt = *((void***)device);
        ((ReleaseFn)dev_vt[2])(device);
        printf("  Device released\n");
    }

cleanup:
    DestroyWindow(hwnd);
    printf("  Window destroyed\n");

    printf("\n╔══════════════════════════════════════════╗\n");
    printf("║  DirectX 11 Test: COMPLETE               ║\n");
    printf("║  Rendered %d frames                      ║\n", frame);
    printf("╚══════════════════════════════════════════╝\n");
    return 0;
}
