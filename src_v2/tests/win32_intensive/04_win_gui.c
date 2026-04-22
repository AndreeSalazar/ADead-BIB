#include <windows.h>
#include <stdio.h>

// Window Proc Callback (requires __stdcall via LRESULT CALLBACK)
LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    if (uMsg == WM_DESTROY) {
        PostQuitMessage(0);
        return 0;
    }
    return DefWindowProc(hwnd, uMsg, wParam, lParam);
}

int main() {
    printf("--- Test 04: Win32 GUI ---\n");

    HINSTANCE hInstance = GetModuleHandle(NULL);
    const char* CLASS_NAME = "ADeadWin32Class";

    WNDCLASSEX wc;
    memset(&wc, 0, sizeof(WNDCLASSEX));
    wc.cbSize = sizeof(WNDCLASSEX);
    wc.lpfnWndProc = WindowProc; // Function pointer indirect call!
    wc.hInstance = hInstance;
    wc.lpszClassName = CLASS_NAME;

    if (!RegisterClassEx(&wc)) {
        printf("RegisterClassEx failed!\n");
        return 1;
    }

    HWND hwnd = CreateWindowEx(
        0,                              // Optional window styles
        CLASS_NAME,                     // Window class
        "ADead-BIB Native Window",      // Window text
        WS_OVERLAPPEDWINDOW,            // Window style
        CW_USEDEFAULT, CW_USEDEFAULT,   // Position
        400, 300,                       // Size
        NULL,                           // Parent window
        NULL,                           // Menu
        hInstance,                      // Instance handle
        NULL                            // Additional application data
    );

    if (hwnd == NULL) {
        printf("CreateWindowEx failed!\n");
        return 1;
    }

    printf("Window created successfully! Showing for 2 seconds...\n");
    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);

    // Run message loop for a short time to prove it works
    MSG msg;
    DWORD start_time = GetTickCount();
    while (GetTickCount() - start_time < 2000) {
        if (PeekMessage(&msg, NULL, 0, 0, PM_REMOVE)) {
            if (msg.message == WM_QUIT) {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessage(&msg);
        }
    }

    DestroyWindow(hwnd);
    printf("GUI test passed.\n");
    return 0;
}
