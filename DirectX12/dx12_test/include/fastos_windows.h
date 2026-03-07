// fastos_windows.h — ADead-BIB Windows API typedefs and macros
// Minimal windows.h replacement for DirectX 12 compilation
#pragma once

// ============================================================
// Basic Windows types
// ============================================================
typedef unsigned char      BYTE;
typedef unsigned short     WORD;
typedef unsigned int       UINT;
typedef unsigned long      DWORD;
typedef unsigned long long UINT64;
typedef unsigned long long ULONG_PTR;
typedef unsigned long long SIZE_T;
typedef unsigned long long ULONGLONG;
typedef unsigned char      UINT8;
typedef unsigned short     UINT16;
typedef unsigned int       UINT32;
typedef int                INT;
typedef long               LONG;
typedef long long          LONGLONG;
typedef long long          INT64;
typedef long               HRESULT;
typedef int                BOOL;
typedef void*              HANDLE;
typedef void*              HWND;
typedef void*              HINSTANCE;
typedef void*              HMODULE;
typedef void*              HDC;
typedef void*              HICON;
typedef void*              HCURSOR;
typedef void*              HBRUSH;
typedef void*              HMENU;
typedef void*              HMONITOR;
typedef void*              LPVOID;
typedef const void*        LPCVOID;
typedef char*              LPSTR;
typedef const char*        LPCSTR;
typedef wchar_t*           LPWSTR;
typedef const wchar_t*     LPCWSTR;
typedef wchar_t            WCHAR;
typedef float              FLOAT;

// ============================================================
// Boolean constants
// ============================================================
#define TRUE  1
#define FALSE 0
#define NULL  0

// ============================================================
// HRESULT codes
// ============================================================
#define S_OK          0
#define S_FALSE       1
#define E_FAIL        ((HRESULT)0x80004005)
#define E_NOINTERFACE ((HRESULT)0x80004002)
#define E_OUTOFMEMORY ((HRESULT)0x8007000E)
#define E_INVALIDARG  ((HRESULT)0x80070057)
#define E_NOTIMPL     ((HRESULT)0x80004001)
#define E_UNEXPECTED  ((HRESULT)0x8000FFFF)
#define E_ACCESSDENIED ((HRESULT)0x80070005)
#define E_ABORT       ((HRESULT)0x80004004)
#define E_POINTER     ((HRESULT)0x80004003)

// ============================================================
// HRESULT macros
// ============================================================
#define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#define FAILED(hr)    (((HRESULT)(hr)) < 0)

// ============================================================
// Win32 message constants
// ============================================================
#define WM_CREATE     0x0001
#define WM_DESTROY    0x0002
#define WM_SIZE       0x0005
#define WM_PAINT      0x000F
#define WM_CLOSE      0x0010
#define WM_QUIT       0x0012
#define WM_KEYDOWN    0x0100
#define WM_KEYUP      0x0101
#define WM_SYSKEYDOWN 0x0104
#define WM_SYSKEYUP   0x0105

// ============================================================
// Win32 structures
// ============================================================
struct RECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
};

struct POINT {
    LONG x;
    LONG y;
};

struct MSG {
    HWND hwnd;
    UINT message;
    ULONG_PTR wParam;
    LONG lParam;
    DWORD time;
    POINT pt;
};

struct WNDCLASSEX {
    UINT cbSize;
    UINT style;
    void* lpfnWndProc;
    int cbClsExtra;
    int cbWndExtra;
    HINSTANCE hInstance;
    HICON hIcon;
    HCURSOR hCursor;
    HBRUSH hbrBackground;
    LPCWSTR lpszMenuName;
    LPCWSTR lpszClassName;
    HICON hIconSm;
};

// ============================================================
// Win32 API function declarations
// ============================================================
extern "C" {
    HWND CreateWindowExW(DWORD dwExStyle, LPCWSTR lpClassName, LPCWSTR lpWindowName,
        DWORD dwStyle, int X, int Y, int nWidth, int nHeight,
        HWND hWndParent, HMENU hMenu, HINSTANCE hInstance, LPVOID lpParam);
    BOOL ShowWindow(HWND hWnd, int nCmdShow);
    BOOL UpdateWindow(HWND hWnd);
    BOOL DestroyWindow(HWND hWnd);
    void PostQuitMessage(int nExitCode);
    BOOL GetClientRect(HWND hWnd, RECT* lpRect);
    BOOL AdjustWindowRect(RECT* lpRect, DWORD dwStyle, BOOL bMenu);
    LONG SetWindowLongPtrW(HWND hWnd, int nIndex, LONG dwNewLong);
    LONG GetWindowLongPtrW(HWND hWnd, int nIndex);
    BOOL PeekMessageW(MSG* lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax, UINT wRemoveMsg);
    BOOL TranslateMessage(const MSG* lpMsg);
    LONG DispatchMessageW(const MSG* lpMsg);
    UINT RegisterClassExW(const WNDCLASSEX* wc);
    HMODULE GetModuleHandleW(LPCWSTR lpModuleName);
    void* GetProcAddress(HMODULE hModule, LPCSTR lpProcName);
    HMODULE LoadLibraryW(LPCWSTR lpLibFileName);
    BOOL SetWindowTextW(HWND hWnd, LPCWSTR lpString);
}

// ============================================================
// Window style constants
// ============================================================
#define WS_OVERLAPPEDWINDOW 0x00CF0000
#define WS_VISIBLE          0x10000000
#define CW_USEDEFAULT       ((int)0x80000000)
#define SW_SHOW             5
#define SW_SHOWDEFAULT      10
#define PM_REMOVE           0x0001
#define GWLP_USERDATA       (-21)

// ============================================================
// WIN32_LEAN_AND_MEAN support (no-op, we're already lean)
// ============================================================
#define WIN32_LEAN_AND_MEAN

// ============================================================
// GUID structure
// ============================================================
struct GUID {
    DWORD Data1;
    WORD  Data2;
    WORD  Data3;
    BYTE  Data4[8];
};

typedef GUID IID;
typedef GUID CLSID;
typedef const GUID* REFGUID;
typedef const IID*  REFIID;
typedef const CLSID* REFCLSID;

// ============================================================
// LARGE_INTEGER
// ============================================================
struct LARGE_INTEGER {
    LONGLONG QuadPart;
};

// ============================================================
// Security attributes (minimal)
// ============================================================
struct SECURITY_ATTRIBUTES {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
};

// ============================================================
// Inline helpers
// ============================================================
inline LONG HIWORD(ULONG_PTR l) { return (LONG)((l >> 16) & 0xFFFF); }
inline LONG LOWORD(ULONG_PTR l) { return (LONG)(l & 0xFFFF); }

// ============================================================
// IUnknown base interface
// ============================================================
struct IUnknown {
    virtual UINT AddRef() = 0;
    virtual UINT Release() = 0;
    virtual HRESULT QueryInterface(REFIID riid, void** ppvObject) = 0;
};
