// ============================================================
// fastos_win32.rs — Windows API (C ABI) for ADead-BIB
// ============================================================
// Core Win32 API — kernel32.dll, user32.dll, gdi32.dll
// Pure C declarations — no C++ needed
// ============================================================

// ── kernel32.dll Functions ──
pub const KERNEL32_FUNCTIONS: &[&str] = &[
    // Process / Module
    "GetModuleHandleA", "GetModuleHandleW",
    "GetModuleFileNameA", "GetModuleFileNameW",
    "LoadLibraryA", "LoadLibraryW", "LoadLibraryExA", "LoadLibraryExW",
    "FreeLibrary", "GetProcAddress",
    "GetCurrentProcess", "GetCurrentProcessId",
    "ExitProcess", "TerminateProcess",
    "CreateProcessA", "CreateProcessW",
    // Memory
    "VirtualAlloc", "VirtualFree", "VirtualProtect", "VirtualQuery",
    "HeapCreate", "HeapDestroy", "HeapAlloc", "HeapReAlloc", "HeapFree",
    "GetProcessHeap",
    "GlobalAlloc", "GlobalFree", "GlobalLock", "GlobalUnlock",
    "LocalAlloc", "LocalFree",
    // File I/O
    "CreateFileA", "CreateFileW",
    "ReadFile", "WriteFile",
    "CloseHandle",
    "GetFileSize", "GetFileSizeEx",
    "SetFilePointer", "SetFilePointerEx",
    "FlushFileBuffers",
    "CreateFileMappingA", "CreateFileMappingW",
    "MapViewOfFile", "UnmapViewOfFile",
    "GetTempPathA", "GetTempFileNameA",
    "DeleteFileA", "DeleteFileW",
    "CopyFileA", "MoveFileA",
    "GetFileAttributesA", "SetFileAttributesA",
    "FindFirstFileA", "FindNextFileA", "FindClose",
    // Console
    "GetStdHandle", "SetConsoleMode", "GetConsoleMode",
    "WriteConsoleA", "WriteConsoleW",
    "ReadConsoleA", "ReadConsoleW",
    "AllocConsole", "FreeConsole",
    "SetConsoleTitleA",
    // Threading
    "CreateThread", "ExitThread", "GetCurrentThread", "GetCurrentThreadId",
    "ResumeThread", "SuspendThread", "TerminateThread",
    "WaitForSingleObject", "WaitForMultipleObjects",
    "Sleep", "SleepEx",
    "SwitchToThread",
    "CreateMutexA", "ReleaseMutex",
    "CreateEventA", "SetEvent", "ResetEvent",
    "CreateSemaphoreA", "ReleaseSemaphore",
    "InitializeCriticalSection", "DeleteCriticalSection",
    "EnterCriticalSection", "LeaveCriticalSection",
    "TryEnterCriticalSection",
    "InitializeCriticalSectionAndSpinCount",
    "InitializeSRWLock", "AcquireSRWLockExclusive", "ReleaseSRWLockExclusive",
    "AcquireSRWLockShared", "ReleaseSRWLockShared",
    "InitializeConditionVariable", "SleepConditionVariableSRW", "WakeConditionVariable", "WakeAllConditionVariable",
    "TlsAlloc", "TlsFree", "TlsGetValue", "TlsSetValue",
    "InterlockedIncrement", "InterlockedDecrement", "InterlockedExchange",
    "InterlockedCompareExchange",
    // Time
    "GetTickCount", "GetTickCount64",
    "QueryPerformanceCounter", "QueryPerformanceFrequency",
    "GetSystemTime", "GetLocalTime", "SystemTimeToFileTime",
    "GetSystemTimeAsFileTime",
    // System info
    "GetSystemInfo", "GetNativeSystemInfo",
    "GetVersionExA", "GetVersionExW",
    "GetComputerNameA", "GetUserNameA",
    "GetEnvironmentVariableA", "SetEnvironmentVariableA",
    "GetCommandLineA", "GetCommandLineW",
    // Error
    "GetLastError", "SetLastError",
    "FormatMessageA", "FormatMessageW",
    // Interlocked
    "InterlockedAdd", "InterlockedOr", "InterlockedAnd", "InterlockedXor",
    // Misc
    "OutputDebugStringA", "OutputDebugStringW",
    "IsDebuggerPresent",
    "QueryDosDeviceA",
    "GetDiskFreeSpaceExA",
];

// ── user32.dll Functions ──
pub const USER32_FUNCTIONS: &[&str] = &[
    // Window
    "RegisterClassA", "RegisterClassW", "RegisterClassExA", "RegisterClassExW",
    "UnregisterClassA",
    "CreateWindowExA", "CreateWindowExW",
    "DestroyWindow",
    "ShowWindow", "UpdateWindow",
    "MoveWindow", "SetWindowPos",
    "GetWindowRect", "GetClientRect",
    "AdjustWindowRect", "AdjustWindowRectEx",
    "SetWindowTextA", "GetWindowTextA",
    "GetWindowLongA", "SetWindowLongA",
    "GetWindowLongPtrA", "SetWindowLongPtrA",
    "FindWindowA", "FindWindowExA",
    "IsWindow", "IsWindowVisible",
    "EnableWindow", "SetForegroundWindow",
    "BringWindowToTop",
    "GetDesktopWindow", "GetForegroundWindow",
    "GetActiveWindow", "SetActiveWindow",
    "SetFocus", "GetFocus",
    "GetParent", "SetParent",
    // Message loop
    "GetMessageA", "GetMessageW",
    "PeekMessageA", "PeekMessageW",
    "TranslateMessage", "DispatchMessageA", "DispatchMessageW",
    "PostMessageA", "SendMessageA",
    "PostQuitMessage",
    "DefWindowProcA", "DefWindowProcW",
    "WaitMessage",
    "MsgWaitForMultipleObjects",
    // Input
    "GetKeyState", "GetAsyncKeyState", "GetKeyboardState",
    "SetCapture", "ReleaseCapture", "GetCapture",
    "GetCursorPos", "SetCursorPos",
    "ShowCursor", "SetCursor", "LoadCursorA",
    "ClipCursor",
    "MapVirtualKeyA",
    // Painting
    "GetDC", "ReleaseDC",
    "InvalidateRect", "ValidateRect",
    "BeginPaint", "EndPaint",
    "RedrawWindow",
    // Misc
    "MessageBoxA", "MessageBoxW",
    "LoadIconA",
    "GetSystemMetrics",
    "SystemParametersInfoA",
    "SetTimer", "KillTimer",
    "TrackMouseEvent",
    "GetClassInfoExA",
    "MonitorFromWindow", "GetMonitorInfoA",
    "EnumDisplayMonitors",
    "EnumDisplaySettingsA", "ChangeDisplaySettingsA",
    "SwapBuffers",
];

// ── gdi32.dll Functions ──
pub const GDI32_FUNCTIONS: &[&str] = &[
    "ChoosePixelFormat", "SetPixelFormat", "DescribePixelFormat",
    "SwapBuffers",
    "CreateCompatibleDC", "CreateCompatibleBitmap",
    "DeleteDC", "DeleteObject",
    "SelectObject", "GetObject",
    "BitBlt", "StretchBlt",
    "CreateSolidBrush", "CreatePen",
    "Rectangle", "Ellipse", "MoveToEx", "LineTo",
    "TextOutA", "TextOutW",
    "SetBkMode", "SetTextColor", "SetBkColor",
    "GetDeviceCaps",
    "CreateFontA", "CreateFontIndirectA",
    "GetStockObject",
    "FillRect", "FrameRect",
    "CreateDIBSection",
    "SetDIBitsToDevice", "StretchDIBits",
    "GetDIBits",
    "PatBlt",
    "SaveDC", "RestoreDC",
    "SetViewportOrgEx", "SetWindowOrgEx",
    "GetTextMetricsA",
    "BeginPath", "EndPath", "StrokeAndFillPath",
    // WGL (OpenGL context on Windows)
    "wglCreateContext", "wglDeleteContext",
    "wglMakeCurrent", "wglGetCurrentContext", "wglGetCurrentDC",
    "wglGetProcAddress",
    "wglSwapLayerBuffers",
    "wglChoosePixelFormatARB", "wglCreateContextAttribsARB",
];

// ── Win32 Types ──
pub const WIN32_TYPES: &[&str] = &[
    "HWND", "HINSTANCE", "HMODULE", "HDC", "HGLRC",
    "HCURSOR", "HICON", "HBRUSH", "HMENU", "HBITMAP", "HFONT", "HPEN", "HRGN",
    "HANDLE", "HRESULT",
    "WPARAM", "LPARAM", "LRESULT",
    "ATOM", "BOOL", "BYTE", "WORD", "DWORD", "LONG", "ULONG", "UINT",
    "LONG_PTR", "ULONG_PTR", "DWORD_PTR", "SIZE_T", "INT_PTR", "UINT_PTR",
    "LPVOID", "LPCVOID", "PVOID",
    "LPSTR", "LPCSTR", "LPWSTR", "LPCWSTR",
    "WCHAR", "TCHAR",
    "WNDPROC", "DLGPROC", "TIMERPROC",
    "LARGE_INTEGER", "ULARGE_INTEGER",
    "FILETIME", "SYSTEMTIME",
    "SECURITY_ATTRIBUTES", "OVERLAPPED",
    "CRITICAL_SECTION", "SRWLOCK", "CONDITION_VARIABLE",
    "POINT", "RECT", "SIZE", "MSG",
    "WNDCLASSA", "WNDCLASSW", "WNDCLASSEXA", "WNDCLASSEXW",
    "PAINTSTRUCT",
    "PIXELFORMATDESCRIPTOR",
    "STARTUPINFOA", "PROCESS_INFORMATION",
    "WIN32_FIND_DATAA",
    "MONITORINFO", "DEVMODEA",
    "CREATESTRUCTA",
    "MINMAXINFO",
    "WINDOWPLACEMENT",
    "BITMAPINFO", "BITMAPINFOHEADER",
    "RGBQUAD",
];

// ── Win32 Constants ──
pub const WIN32_CONSTANTS: &[(&str, &str)] = &[
    // Window styles
    ("WS_OVERLAPPEDWINDOW", "0x00CF0000"),
    ("WS_POPUP", "0x80000000"),
    ("WS_CHILD", "0x40000000"),
    ("WS_VISIBLE", "0x10000000"),
    ("WS_MINIMIZE", "0x20000000"),
    ("WS_MAXIMIZE", "0x01000000"),
    ("WS_CAPTION", "0x00C00000"),
    ("WS_BORDER", "0x00800000"),
    ("WS_THICKFRAME", "0x00040000"),
    ("WS_SYSMENU", "0x00080000"),
    ("WS_MINIMIZEBOX", "0x00020000"),
    ("WS_MAXIMIZEBOX", "0x00010000"),
    ("WS_EX_TOPMOST", "0x00000008"),
    ("WS_EX_APPWINDOW", "0x00040000"),
    // Window messages
    ("WM_CREATE", "0x0001"),
    ("WM_DESTROY", "0x0002"),
    ("WM_CLOSE", "0x0010"),
    ("WM_QUIT", "0x0012"),
    ("WM_PAINT", "0x000F"),
    ("WM_SIZE", "0x0005"),
    ("WM_MOVE", "0x0003"),
    ("WM_KEYDOWN", "0x0100"),
    ("WM_KEYUP", "0x0101"),
    ("WM_CHAR", "0x0102"),
    ("WM_SYSKEYDOWN", "0x0104"),
    ("WM_SYSKEYUP", "0x0105"),
    ("WM_MOUSEMOVE", "0x0200"),
    ("WM_LBUTTONDOWN", "0x0201"),
    ("WM_LBUTTONUP", "0x0202"),
    ("WM_RBUTTONDOWN", "0x0204"),
    ("WM_RBUTTONUP", "0x0205"),
    ("WM_MBUTTONDOWN", "0x0207"),
    ("WM_MBUTTONUP", "0x0208"),
    ("WM_MOUSEWHEEL", "0x020A"),
    ("WM_TIMER", "0x0113"),
    ("WM_ERASEBKGND", "0x0014"),
    ("WM_SETFOCUS", "0x0007"),
    ("WM_KILLFOCUS", "0x0008"),
    ("WM_ACTIVATE", "0x0006"),
    ("WM_GETMINMAXINFO", "0x0024"),
    ("WM_ENTERSIZEMOVE", "0x0231"),
    ("WM_EXITSIZEMOVE", "0x0232"),
    ("WM_INPUT", "0x00FF"),
    // ShowWindow
    ("SW_HIDE", "0"),
    ("SW_SHOWNORMAL", "1"),
    ("SW_SHOWMINIMIZED", "2"),
    ("SW_SHOWMAXIMIZED", "3"),
    ("SW_SHOW", "5"),
    // CW_USEDEFAULT
    ("CW_USEDEFAULT", "((int)0x80000000)"),
    // MessageBox
    ("MB_OK", "0x00000000"),
    ("MB_OKCANCEL", "0x00000001"),
    ("MB_YESNO", "0x00000004"),
    ("MB_ICONERROR", "0x00000010"),
    ("MB_ICONWARNING", "0x00000030"),
    ("MB_ICONINFORMATION", "0x00000040"),
    ("IDOK", "1"),
    ("IDCANCEL", "2"),
    ("IDYES", "6"),
    ("IDNO", "7"),
    // PeekMessage
    ("PM_NOREMOVE", "0x0000"),
    ("PM_REMOVE", "0x0001"),
    // VK_ keys
    ("VK_ESCAPE", "0x1B"),
    ("VK_RETURN", "0x0D"),
    ("VK_SPACE", "0x20"),
    ("VK_LEFT", "0x25"),
    ("VK_UP", "0x26"),
    ("VK_RIGHT", "0x27"),
    ("VK_DOWN", "0x28"),
    ("VK_F1", "0x70"),
    ("VK_F2", "0x71"),
    ("VK_F11", "0x7A"),
    ("VK_F12", "0x7B"),
    // Wait
    ("WAIT_OBJECT_0", "0x00000000"),
    ("WAIT_TIMEOUT", "0x00000102"),
    ("INFINITE", "0xFFFFFFFF"),
    // Generic access
    ("GENERIC_READ", "0x80000000"),
    ("GENERIC_WRITE", "0x40000000"),
    ("FILE_SHARE_READ", "0x00000001"),
    ("CREATE_ALWAYS", "2"),
    ("OPEN_EXISTING", "3"),
    ("OPEN_ALWAYS", "4"),
    ("INVALID_HANDLE_VALUE", "((HANDLE)(LONG_PTR)-1)"),
    // Memory
    ("MEM_COMMIT", "0x00001000"),
    ("MEM_RESERVE", "0x00002000"),
    ("MEM_RELEASE", "0x00008000"),
    ("PAGE_READWRITE", "0x04"),
    ("PAGE_EXECUTE_READWRITE", "0x40"),
    // Console
    ("STD_INPUT_HANDLE", "((DWORD)-10)"),
    ("STD_OUTPUT_HANDLE", "((DWORD)-11)"),
    ("STD_ERROR_HANDLE", "((DWORD)-12)"),
    // GDI
    ("SRCCOPY", "0x00CC0020"),
    ("TRANSPARENT", "1"),
    ("OPAQUE", "2"),
    // Pixel format
    ("PFD_DRAW_TO_WINDOW", "0x00000004"),
    ("PFD_SUPPORT_OPENGL", "0x00000020"),
    ("PFD_DOUBLEBUFFER", "0x00000001"),
    ("PFD_TYPE_RGBA", "0"),
    ("PFD_MAIN_PLANE", "0"),
    // CS_ class styles
    ("CS_HREDRAW", "0x0002"),
    ("CS_VREDRAW", "0x0001"),
    ("CS_OWNDC", "0x0020"),
    // COLOR_
    ("COLOR_WINDOW", "5"),
    ("COLOR_BACKGROUND", "1"),
    // NULL
    ("NULL", "((void*)0)"),
    ("TRUE", "1"),
    ("FALSE", "0"),
];

// ── Well-known DLLs ──
pub const WIN32_DLLS: &[&str] = &[
    "kernel32.dll", "user32.dll", "gdi32.dll",
    "ntdll.dll", "advapi32.dll", "shell32.dll",
    "ole32.dll", "oleaut32.dll",
    "ws2_32.dll", "winmm.dll",
    "opengl32.dll", "vulkan-1.dll",
    "d3d9.dll", "d3d11.dll", "d3d12.dll",
    "dxgi.dll", "d3dcompiler_47.dll",
];

pub fn is_win32_symbol(name: &str) -> bool {
    KERNEL32_FUNCTIONS.contains(&name)
        || USER32_FUNCTIONS.contains(&name)
        || GDI32_FUNCTIONS.contains(&name)
        || WIN32_TYPES.contains(&name)
        || WIN32_CONSTANTS.iter().any(|(n, _)| *n == name)
}
