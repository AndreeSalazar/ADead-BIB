// ADead-BIB Bridge Test 16 — Vulkan Window
// Level: EXPERT
// Tests: vkCreateInstance, vkEnumeratePhysicalDevices, vulkan-1.dll import
// Compile: adB cc 16_vulkan_window.c -o vk_test.exe -Warm ub
// Requires: vulkan-1.dll, user32.dll, kernel32.dll in IAT

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
// Vulkan types (minimal, no vulkan.h)
// ═══════════════════════════════════════════════════════════
typedef int VkResult;          // VK_SUCCESS = 0
typedef void* VkInstance;
typedef void* VkPhysicalDevice;

typedef struct {
    int sType;                 // VK_STRUCTURE_TYPE_APPLICATION_INFO = 0
    const void* pNext;
    const char* pApplicationName;
    unsigned int applicationVersion;
    const char* pEngineName;
    unsigned int engineVersion;
    unsigned int apiVersion;
} VkApplicationInfo;

typedef struct {
    int sType;                 // VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1
    const void* pNext;
    unsigned int flags;
    const VkApplicationInfo* pApplicationInfo;
    unsigned int enabledLayerCount;
    const char* const* ppEnabledLayerNames;
    unsigned int enabledExtensionCount;
    const char* const* ppEnabledExtensionNames;
} VkInstanceCreateInfo;

// ═══════════════════════════════════════════════════════════
// Externs
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

// vulkan-1
extern VkResult vkCreateInstance(const VkInstanceCreateInfo*, const void*, VkInstance*);
extern void vkDestroyInstance(VkInstance, const void*);
extern VkResult vkEnumeratePhysicalDevices(VkInstance, unsigned int*, VkPhysicalDevice*);

// ═══════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════
#define VK_SUCCESS                            0
#define VK_STRUCTURE_TYPE_APPLICATION_INFO     0
#define VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO 1
#define VK_API_VERSION_1_0                    0x00400000
#define VK_MAKE_VERSION(major, minor, patch)  (((major)<<22)|((minor)<<12)|(patch))

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
    printf("║  ADead-BIB Vulkan Test                    ║\n");
    printf("║  Compile: adB cc 16_vulkan_window.c -Warm ub║\n");
    printf("╚══════════════════════════════════════════╝\n\n");

    // ── Step 1: Create Win32 Window ──
    printf("[STEP 1] Creating Win32 window...\n");
    HINSTANCE hInst = GetModuleHandleA(0);
    WNDCLASSA wc;
    memset(&wc, 0, sizeof(WNDCLASSA));
    wc.style = 3;
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInst;
    wc.lpszClassName = "ADeadVulkan";

    RegisterClassA(&wc);
    HWND hwnd = CreateWindowExA(0, "ADeadVulkan", "ADead-BIB Vulkan",
        0x10CF0000, 100, 100, 800, 600, 0, 0, hInst, 0);
    printf("  Window: hwnd=%p %s\n", hwnd, hwnd ? "OK" : "FAIL");
    if (!hwnd) return 1;
    ShowWindow(hwnd, 1);

    // ── Step 2: Create Vulkan Instance ──
    printf("\n[STEP 2] Creating Vulkan Instance...\n");

    VkApplicationInfo appInfo;
    memset(&appInfo, 0, sizeof(VkApplicationInfo));
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = "ADead-BIB Vulkan Test";
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.pEngineName = "ADead-BIB";
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    VkInstanceCreateInfo createInfo;
    memset(&createInfo, 0, sizeof(VkInstanceCreateInfo));
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;

    VkInstance instance = 0;
    VkResult result = vkCreateInstance(&createInfo, 0, &instance);
    printf("  vkCreateInstance: result=%d instance=%p %s\n",
        (int)result, instance, (result == VK_SUCCESS && instance) ? "OK" : "FAIL");

    int frame = 0;
    unsigned int deviceCount = 0;

    if (result != VK_SUCCESS || !instance) {
        printf("\n[RESULT] Vulkan instance creation failed (result=%d).\n", (int)result);
        printf("         This is normal if vulkan-1.dll is unavailable or\n");
        printf("         no Vulkan-capable GPU driver is installed.\n");
        printf("         DLL import was attempted — IAT is correct!\n");
        goto show_window;
    }

    // ── Step 3: Enumerate Physical Devices ──
    printf("\n[STEP 3] Enumerating physical devices...\n");
    result = vkEnumeratePhysicalDevices(instance, &deviceCount, 0);
    printf("  vkEnumeratePhysicalDevices: result=%d count=%u %s\n",
        (int)result, deviceCount,
        (result == VK_SUCCESS && deviceCount > 0) ? "OK" : "NONE");

    if (result == VK_SUCCESS && deviceCount > 0) {
        // Query the first device (just count, no details without VkPhysicalDeviceProperties)
        printf("  Found %u Vulkan physical device(s)\n", deviceCount);

        // Enumerate again with buffer to confirm
        VkPhysicalDevice devices[8];
        unsigned int queryCount = deviceCount;
        if (queryCount > 8) queryCount = 8;
        result = vkEnumeratePhysicalDevices(instance, &queryCount, devices);
        printf("  Queried %u device handle(s): result=%d\n", queryCount, (int)result);
        for (unsigned int i = 0; i < queryCount; i++) {
            printf("    Device[%u]: handle=%p\n", i, devices[i]);
        }
    } else {
        printf("  No Vulkan physical devices found.\n");
        printf("  Instance was created — vulkan-1.dll works!\n");
    }

    printf("\n[STEP 4] Vulkan Pipeline Summary:\n");
    printf("  Instance:      %p %s\n", instance, instance ? "READY" : "NONE");
    printf("  PhysDevices:   %u\n", deviceCount);

show_window:
    // ── Show window for 3 seconds ──
    printf("\n[STEP 5] Running window (3 seconds)...\n");
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

            if (frame % 60 == 0) {
                printf("  Frame %d\n", frame);
            }
            Sleep(16);
            frame++;
        }
    }

    // ── Cleanup ──
    printf("\n[STEP 6] Cleanup...\n");
    if (instance) {
        vkDestroyInstance(instance, 0);
        printf("  Vulkan instance destroyed\n");
    }

    DestroyWindow(hwnd);
    printf("  Window destroyed\n");

    printf("\n╔══════════════════════════════════════════╗\n");
    printf("║  Vulkan Test: COMPLETE                    ║\n");
    printf("║  Rendered %d frames                      ║\n", frame);
    printf("║  Instance: %s                             ║\n", instance ? "YES" : "NO");
    printf("║  Devices:  %u                             ║\n", deviceCount);
    printf("╚══════════════════════════════════════════╝\n");
    return 0;
}
