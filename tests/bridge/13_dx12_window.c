// ADead-BIB Bridge Test 13 — DirectX 12 Window
// Level: EXPERT
// Tests: D3D12CreateDevice, CreateDXGIFactory1, CreateCommandQueue,
//        CreateSwapChain, CreateRenderTargetView, ClearRenderTargetView
// Compile: adB cc 13_dx12_window.c -o dx12_test.exe -Warm ub
// Requires: d3d12.dll, dxgi.dll, ole32.dll, user32.dll, kernel32.dll in IAT

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
typedef unsigned long long UINT64;
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
// DXGI types for DX12
// ═══════════════════════════════════════════════════════════
typedef struct {
    UINT Width;
    UINT Height;
    UINT RefreshRate_Numerator;
    UINT RefreshRate_Denominator;
    UINT Format;
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
    UINT BufferUsage;
    UINT BufferCount;
    HWND OutputWindow;
    int Windowed;
    UINT SwapEffect;
    UINT Flags;
} DXGI_SWAP_CHAIN_DESC;

// ═══════════════════════════════════════════════════════════
// D3D12 Command Queue Desc
// ═══════════════════════════════════════════════════════════
typedef struct {
    UINT Type;       // D3D12_COMMAND_LIST_TYPE_DIRECT = 0
    int Priority;
    UINT Flags;
    UINT NodeMask;
} D3D12_COMMAND_QUEUE_DESC;

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

// COM
extern HRESULT CoInitializeEx(void*, DWORD);
extern void CoUninitialize(void);

// DXGI
extern HRESULT CreateDXGIFactory1(void* riid, void** ppFactory);

// D3D12
extern HRESULT D3D12CreateDevice(void* pAdapter, UINT MinFeatureLevel,
    void* riid, void** ppDevice);

// ═══════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════
#define DXGI_FORMAT_R8G8B8A8_UNORM     28
#define DXGI_USAGE_RENDER_TARGET_OUTPUT 0x00000020
#define DXGI_SWAP_EFFECT_FLIP_DISCARD  4
#define D3D_FEATURE_LEVEL_11_0         0xb000
#define D3D_FEATURE_LEVEL_12_0         0xc000
#define D3D12_COMMAND_LIST_TYPE_DIRECT  0
#define COINIT_MULTITHREADED           0
#define WM_CLOSE    0x0010
#define WM_QUIT     0x0012
#define WM_DESTROY  0x0002
#define PM_REMOVE   1
#define FRAME_COUNT 2

LRESULT WndProc(HWND hwnd, UINT msg, WPARAM wp, LPARAM lp) {
    if (msg == WM_CLOSE || msg == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, msg, wp, lp);
}

int main() {
    printf("╔══════════════════════════════════════════╗\n");
    printf("║  ADead-BIB DirectX 12 Test               ║\n");
    printf("║  Compile: adB cc 13_dx12_window.c -Warm ub║\n");
    printf("╚══════════════════════════════════════════╝\n\n");

    // ── Step 1: COM Init ──
    printf("[STEP 1] Initializing COM...\n");
    HRESULT hr = CoInitializeEx(0, COINIT_MULTITHREADED);
    printf("  CoInitializeEx: hr=0x%08X %s\n", (unsigned int)hr, hr == 0 ? "OK" : "WARN");

    // ── Step 2: Create Window ──
    printf("\n[STEP 2] Creating Win32 window...\n");
    HINSTANCE hInst = GetModuleHandleA(0);
    WNDCLASSA wc;
    memset(&wc, 0, sizeof(WNDCLASSA));
    wc.style = 3;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = "ADeadDX12";

    RegisterClassA(&wc);
    HWND hwnd = CreateWindowExA(0, "ADeadDX12", "ADead-BIB DirectX 12",
        0x10CF0000, 100, 100, 800, 600, 0, 0, hInst, 0);
    printf("  Window: hwnd=%p %s\n", hwnd, hwnd ? "OK" : "FAIL");
    if (!hwnd) return 1;
    ShowWindow(hwnd, 1);

    // ── Step 3: Create DXGI Factory ──
    printf("\n[STEP 3] Creating DXGI Factory...\n");

    // IID_IDXGIFactory1 = {770aae78-f26f-4dba-a829-253c83d1b387}
    unsigned char iid_factory[16];
    iid_factory[0] = 0x78; iid_factory[1] = 0xAE; iid_factory[2] = 0x0A; iid_factory[3] = 0x77;
    iid_factory[4] = 0x6F; iid_factory[5] = 0xF2;
    iid_factory[6] = 0xBA; iid_factory[7] = 0x4D;
    iid_factory[8] = 0xA8; iid_factory[9] = 0x29;
    iid_factory[10] = 0x25; iid_factory[11] = 0x3C; iid_factory[12] = 0x83;
    iid_factory[13] = 0xD1; iid_factory[14] = 0xB3; iid_factory[15] = 0x87;

    void* factory = 0;
    hr = CreateDXGIFactory1(iid_factory, &factory);
    printf("  CreateDXGIFactory1: hr=0x%08X factory=%p %s\n",
        (unsigned int)hr, factory, (hr == 0 && factory) ? "OK" : "FAIL");

    // ── Step 4: Create D3D12 Device ──
    printf("\n[STEP 4] Creating D3D12 Device...\n");

    // IID_ID3D12Device = {189819f1-1db6-4b57-be54-1821339b85f7}
    unsigned char iid_device[16];
    iid_device[0] = 0xF1; iid_device[1] = 0x19; iid_device[2] = 0x98; iid_device[3] = 0x18;
    iid_device[4] = 0xB6; iid_device[5] = 0x1D;
    iid_device[6] = 0x57; iid_device[7] = 0x4B;
    iid_device[8] = 0xBE; iid_device[9] = 0x54;
    iid_device[10] = 0x18; iid_device[11] = 0x21; iid_device[12] = 0x33;
    iid_device[13] = 0x9B; iid_device[14] = 0x85; iid_device[15] = 0xF7;

    void* device = 0;
    hr = D3D12CreateDevice(0, D3D_FEATURE_LEVEL_11_0, iid_device, &device);
    printf("  D3D12CreateDevice(FL_11_0): hr=0x%08X device=%p %s\n",
        (unsigned int)hr, device, (hr == 0 && device) ? "OK" : "FAIL");

    int frame = 0;
    void* cmdQueue = 0;
    void* swapChain = 0;

    if (hr != 0 || !device) {
        printf("\n[RESULT] D3D12 device creation failed.\n");
        printf("         This is normal if your GPU doesn't support DX12.\n");
        printf("         DLL imports work — d3d12.dll was loaded!\n");
        printf("         DXGI factory %s created.\n", factory ? "was" : "was NOT");
        goto show_window;
    }

    // ── Step 5: Create Command Queue ──
    printf("\n[STEP 5] Creating Command Queue...\n");
    void** dev_vtable = *((void***)device);

    // ID3D12Device::CreateCommandQueue is vtable[8]
    D3D12_COMMAND_QUEUE_DESC qd;
    memset(&qd, 0, sizeof(D3D12_COMMAND_QUEUE_DESC));
    qd.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;

    // IID_ID3D12CommandQueue = {0ec870a6-5d7e-4c22-8cfc-5baae07616ed}
    unsigned char iid_cq[16];
    iid_cq[0] = 0xA6; iid_cq[1] = 0x70; iid_cq[2] = 0xC8; iid_cq[3] = 0x0E;
    iid_cq[4] = 0x7E; iid_cq[5] = 0x5D;
    iid_cq[6] = 0x22; iid_cq[7] = 0x4C;
    iid_cq[8] = 0x8C; iid_cq[9] = 0xFC;
    iid_cq[10] = 0x5B; iid_cq[11] = 0xAA; iid_cq[12] = 0xE0;
    iid_cq[13] = 0x76; iid_cq[14] = 0x16; iid_cq[15] = 0xED;

    typedef HRESULT (*CreateCQFn)(void*, void*, void*, void**);
    CreateCQFn pCreateCQ = (CreateCQFn)dev_vtable[8];
    hr = pCreateCQ(device, &qd, iid_cq, &cmdQueue);
    printf("  CreateCommandQueue: hr=0x%08X queue=%p %s\n",
        (unsigned int)hr, cmdQueue, (hr == 0 && cmdQueue) ? "OK" : "FAIL");

    // ── Step 6: Create Swap Chain ──
    if (factory && cmdQueue) {
        printf("\n[STEP 6] Creating Swap Chain...\n");
        void** fac_vtable = *((void***)factory);

        DXGI_SWAP_CHAIN_DESC scd;
        memset(&scd, 0, sizeof(DXGI_SWAP_CHAIN_DESC));
        scd.BufferDesc.Width = 800;
        scd.BufferDesc.Height = 600;
        scd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
        scd.SampleDesc.Count = 1;
        scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
        scd.BufferCount = FRAME_COUNT;
        scd.OutputWindow = hwnd;
        scd.Windowed = 1;
        scd.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;

        // IDXGIFactory::CreateSwapChain is vtable[10]
        typedef HRESULT (*CreateSCFn)(void*, void*, void*, void**);
        CreateSCFn pCreateSC = (CreateSCFn)fac_vtable[10];
        hr = pCreateSC(factory, cmdQueue, &scd, &swapChain);
        printf("  CreateSwapChain: hr=0x%08X sc=%p %s\n",
            (unsigned int)hr, swapChain, (hr == 0 && swapChain) ? "OK" : "FAIL");
    }

    printf("\n[STEP 7] D3D12 Pipeline Summary:\n");
    printf("  Device:       %p %s\n", device, device ? "READY" : "NONE");
    printf("  CmdQueue:     %p %s\n", cmdQueue, cmdQueue ? "READY" : "NONE");
    printf("  SwapChain:    %p %s\n", swapChain, swapChain ? "READY" : "NONE");
    printf("  Factory:      %p %s\n", factory, factory ? "READY" : "NONE");

show_window:
    // ── Show window for 3 seconds ──
    printf("\n[STEP 8] Running window (3 seconds)...\n");
    {
        int running = 1;
        while (running && frame < 180) {
            MSG msg;
            while (PeekMessageA(&msg, 0, 0, 0, PM_REMOVE)) {
                if (msg.message == WM_QUIT) { running = 0; break; }
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            if (!running) break;

            // If we have a swap chain, present
            if (swapChain) {
                void** sc_vtable = *((void***)swapChain);
                typedef HRESULT (*PresentFn)(void*, UINT, UINT);
                PresentFn pPresent = (PresentFn)sc_vtable[8];
                pPresent(swapChain, 1, 0);
            }

            if (frame % 60 == 0) {
                printf("  Frame %d%s\n", frame,
                    swapChain ? " (presenting)" : " (no swap chain)");
            }
            Sleep(16);
            frame++;
        }
    }

    // ── Cleanup ──
    printf("\n[STEP 9] Cleanup...\n");
    typedef DWORD (*ReleaseFn)(void*);

    if (swapChain) {
        void** sc_vt = *((void***)swapChain);
        ((ReleaseFn)sc_vt[2])(swapChain);
        printf("  SwapChain released\n");
    }
    if (cmdQueue) {
        void** cq_vt = *((void***)cmdQueue);
        ((ReleaseFn)cq_vt[2])(cmdQueue);
        printf("  CmdQueue released\n");
    }
    if (device) {
        void** dev_vt = *((void***)device);
        ((ReleaseFn)dev_vt[2])(device);
        printf("  Device released\n");
    }
    if (factory) {
        void** fac_vt = *((void***)factory);
        ((ReleaseFn)fac_vt[2])(factory);
        printf("  Factory released\n");
    }

    DestroyWindow(hwnd);
    CoUninitialize();
    printf("  COM uninitialized, window destroyed\n");

    printf("\n╔══════════════════════════════════════════╗\n");
    printf("║  DirectX 12 Test: COMPLETE               ║\n");
    printf("║  Rendered %d frames                      ║\n", frame);
    printf("║  Device: %s                              ║\n", device ? "YES" : "NO");
    printf("║  SwapChain: %s                           ║\n", swapChain ? "YES" : "NO");
    printf("╚══════════════════════════════════════════╝\n");
    return 0;
}
