// ad_vulkan.hpp - ADead-BIB Vulkan C++ Wrapper
// Autor: Eddi Andreé Salazar Matos (ADead-BIB ecosystem)

#pragma once
#include "vulkan.h"

extern "C" {
    void* malloc(unsigned long long);
    void free(void*);
    int printf(const char*, ...);
    
    // Win32 Minimal Definitions for ADead-BIB Native
    typedef void* HWND;
    typedef void* HINSTANCE;
    typedef void* HICON;
    typedef void* HCURSOR;
    typedef void* HBRUSH;
    typedef void* HMODULE;
    
    struct WNDCLASSA {
        unsigned int style;
        long long lpfnWndProc; // function pointer
        int cbClsExtra;
        int cbWndExtra;
        HINSTANCE hInstance;
        HICON hIcon;
        HCURSOR hCursor;
        HBRUSH hbrBackground;
        const char* lpszMenuName;
        const char* lpszClassName;
    };
    
    struct POINT { long x; long y; };
    struct MSG {
        HWND hwnd;
        unsigned int message;
        unsigned long long wParam;
        long long lParam;
        unsigned int time;
        POINT pt;
        unsigned int lPrivate;
    };

    unsigned short RegisterClassA(const WNDCLASSA* lpWndClass);
    HWND CreateWindowExA(unsigned int dwExStyle, const char* lpClassName, const char* lpWindowName, unsigned int dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, void* hMenu, HINSTANCE hInstance, void* lpParam);
    int ShowWindow(HWND hWnd, int nCmdShow);
    int UpdateWindow(HWND hWnd);
    int PeekMessageA(MSG* lpMsg, HWND hWnd, unsigned int wMsgFilterMin, unsigned int wMsgFilterMax, unsigned int wRemoveMsg);
    int TranslateMessage(const MSG* lpMsg);
    long long DispatchMessageA(const MSG* lpMsg);
    long long DefWindowProcA(HWND hWnd, unsigned int Msg, unsigned long long wParam, long long lParam);
    void PostQuitMessage(int nExitCode);
    void* GetModuleHandleA(const char* lpModuleName);
    HMODULE LoadLibraryA(const char* lpLibFileName);
    void* GetProcAddress(HMODULE hModule, const char* lpProcName);
}

// Win32 Constants
#define WS_OVERLAPPEDWINDOW 0x00CF0000
#define SW_SHOW 5
#define PM_REMOVE 1
#define WM_DESTROY (unsigned)0x0002
#define WM_CLOSE (unsigned)0x0010

static bool global_window_running = true;

extern "C" long long WindowProc(HWND hwnd, unsigned int uMsg, unsigned long long wParam, long long lParam) {
    if (uMsg == WM_DESTROY || uMsg == WM_CLOSE) {
        global_window_running = false;
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

namespace ad {

    inline void Check(VkResult result, const char* msg) {
        if ((int)result != (int)VK_SUCCESS) {
            printf("[ADead-BIB Vulkan Error] %s\n", msg);
        }
    }

    class Window {
    public:
        HWND handle = nullptr;
        HINSTANCE hInstance = nullptr;
        unsigned int width;
        unsigned int height;

        Window(unsigned int w, unsigned int h, const char* title) : width(w), height(h) {
            hInstance = (HINSTANCE)GetModuleHandleA(nullptr);
            
            WNDCLASSA wc;
            wc.style = 0;
            wc.lpfnWndProc = (long long)WindowProc;
            wc.cbClsExtra = 0;
            wc.cbWndExtra = 0;
            wc.hInstance = hInstance;
            wc.hIcon = nullptr;
            wc.hCursor = nullptr;
            wc.hbrBackground = nullptr;
            wc.lpszMenuName = nullptr;
            wc.lpszClassName = "ADeadBIBWindowClass";
            
            RegisterClassA(&wc);
            
            handle = CreateWindowExA(
                0, "ADeadBIBWindowClass", title, WS_OVERLAPPEDWINDOW,
                100, 100, width, height, nullptr, nullptr, hInstance, nullptr
            );
            
            ShowWindow(handle, SW_SHOW);
            UpdateWindow(handle);
            printf("ADead-BIB Native Window Created!\n");
        }

        bool update() {
            MSG msg;
            while (PeekMessageA(&msg, nullptr, 0, 0, PM_REMOVE)) {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
            return global_window_running;
        }
    };

} // namespace ad
