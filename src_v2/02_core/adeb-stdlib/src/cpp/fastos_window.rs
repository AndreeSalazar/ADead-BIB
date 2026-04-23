pub fn generate_window_hpp() -> String { 
    r#"#ifndef AD_STD_WINDOW_HPP
#define AD_STD_WINDOW_HPP

// Declaraciones nativas de Win32 limpias sin <windows.h>
extern "C" {
    void* malloc(unsigned long long);
    void free(void*);
    
    typedef void* HWND;
    typedef void* HINSTANCE;
    typedef void* HCURSOR;
    typedef void* HICON;
    typedef void* HBRUSH;
    typedef void* HMENU;
    typedef void* LPVOID;
    
    typedef unsigned int UINT;
    typedef long LONG_PTR;
    typedef unsigned long ULONG_PTR;
    typedef LONG_PTR LRESULT;
    typedef ULONG_PTR WPARAM;
    typedef LONG_PTR LPARAM;
    typedef const char* LPCSTR;
    
    struct POINT {
        long x;
        long y;
    };

    struct MSG {
        HWND hwnd;
        UINT message;
        WPARAM wParam;
        LPARAM lParam;
        unsigned long time;
        POINT pt;
        unsigned long lPrivate;
    };

    typedef LRESULT (*WNDPROC)(HWND, UINT, WPARAM, LPARAM);

    struct WNDCLASSA {
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
    };

    unsigned short RegisterClassA(const WNDCLASSA* lpWndClass);
    HWND CreateWindowExA(
        unsigned long dwExStyle,
        LPCSTR lpClassName,
        LPCSTR lpWindowName,
        unsigned long dwStyle,
        int X, int Y, int nWidth, int nHeight,
        HWND hWndParent,
        HMENU hMenu,
        HINSTANCE hInstance,
        LPVOID lpParam
    );
    int ShowWindow(HWND hWnd, int nCmdShow);
    int PeekMessageA(MSG* lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax, UINT wRemoveMsg);
    int TranslateMessage(const MSG* lpMsg);
    LRESULT DispatchMessageA(const MSG* lpMsg);
    void PostQuitMessage(int nExitCode);
    LRESULT DefWindowProcA(HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
    HINSTANCE GetModuleHandleA(LPCSTR lpModuleName);
    int DestroyWindow(HWND hWnd);
}

// Interfaz Limpia Estilo Ash/Rust para ADead-BIB
struct WindowInfo {
    HWND handle;
    bool shouldClose;
};

// Callback interno win32
extern "C" LRESULT WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    if (uMsg == (UINT)0x0010) { // WM_CLOSE
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

// Funciones nativas
WindowInfo* ad_window_create(const char* title, int width, int height) {
    HINSTANCE hInst = GetModuleHandleA(nullptr);
    
    WNDCLASSA wc;
    wc.style = 0;
    wc.lpfnWndProc = WindowProc;
    wc.cbClsExtra = 0;
    wc.cbWndExtra = 0;
    wc.hInstance = hInst;
    wc.hIcon = nullptr;
    wc.hCursor = nullptr;
    wc.hbrBackground = nullptr;
    wc.lpszMenuName = nullptr;
    wc.lpszClassName = "ADeadBIBWindow";
    
    RegisterClassA(&wc);
    
    HWND hwnd = CreateWindowExA(
        0,
        "ADeadBIBWindow",
        title,
        0x10C80000, // WS_OVERLAPPEDWINDOW 
        (int)0x80000000, (int)0x80000000, width, height, // CW_USEDEFAULT
        nullptr, nullptr, hInst, nullptr
    );
    
    if (hwnd != nullptr) {
        ShowWindow(hwnd, 1); // SW_SHOWNORMAL
    }
    
    WindowInfo* g_window = (WindowInfo*)malloc(16); // sizeof(HWND) + bool
    g_window->handle = hwnd;
    g_window->shouldClose = false;
    return g_window;
}

bool ad_window_update(WindowInfo* win) {
    MSG msg;
    while (PeekMessageA(&msg, nullptr, 0, 0, 1)) { // PM_REMOVE = 1
        if (msg.message == (UINT)0x0012) { // WM_QUIT
            win->shouldClose = true;
            return false;
        }
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
    return !win->shouldClose;
}

void ad_window_destroy(WindowInfo* win) {
    if (win != nullptr) {
        if (win->handle != nullptr) {
            DestroyWindow(win->handle);
            win->handle = nullptr;
        }
        free(win);
    }
}

#endif
"#.
    to_string()
}
