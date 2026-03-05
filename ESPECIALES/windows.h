// ============================================================
// ADead-BIB — ESPECIALES/windows.h
// Win32/Win64 API — Global Headers for ADead-BIB Compiler
// Supports: Windows 11, Windows 10, Windows Server 2022+
// Targeting: x86-64 native, .exe output → .po (FastOS)
// ===========================================================
// This header combines the most-used Win32 APIs in one file:
//   winnt.h  (types, macros, NTSTATUS)
//   windef.h (base types: BOOL, DWORD, HANDLE...)
//   winbase.h (kernel32 core: CreateFile, ReadFile...)
//   winuser.h (GUI: MessageBox, CreateWindow...)
//   winsock2.h (network sockets)
//   tlhelp32.h (process/thread enumeration)
//   psapi.h   (process memory info)
//   shellapi.h (ShellExecute)
//   fileapi.h (file operations)
// ============================================================

#ifndef _ADEAD_WINDOWS_H
#define _ADEAD_WINDOWS_H

// ── Calling conventions ─────────────────────────────────────
#define WINAPI      __stdcall
#define CALLBACK    __stdcall
#define APIENTRY    WINAPI
#define WINAPIV     __cdecl

// ── Base integer types (windef.h / winnt.h) ─────────────────
typedef unsigned char       BYTE;
typedef unsigned char       UCHAR;
typedef signed char         CHAR;
typedef unsigned short      WORD;
typedef unsigned short      USHORT;
typedef signed short        SHORT;
typedef unsigned int        UINT;
typedef signed int          INT;
typedef unsigned long       DWORD;
typedef long                LONG;
typedef unsigned long long  QWORD;
typedef long long           LONGLONG;
typedef unsigned long long  ULONGLONG;
typedef unsigned long long  UINT64;
typedef long long           INT64;
typedef unsigned int        UINT32;
typedef int                 INT32;
typedef unsigned short      UINT16;
typedef short               INT16;
typedef unsigned char       UINT8;
typedef signed char         INT8;

// ── Pointer-width integer types ─────────────────────────────
typedef long long           LONG_PTR;
typedef unsigned long long  ULONG_PTR;
typedef unsigned long long  DWORD_PTR;
typedef long long           INT_PTR;
typedef unsigned long long  UINT_PTR;
typedef unsigned long long  SIZE_T;
typedef long long           SSIZE_T;

// ── Boolean ─────────────────────────────────────────────────
typedef int                 BOOL;
typedef unsigned char       BOOLEAN;
#define TRUE  1
#define FALSE 0

// ── Void / Generic pointer ───────────────────────────────────
typedef void                VOID;
typedef void*               PVOID;
typedef void*               LPVOID;
typedef const void*         LPCVOID;

// ── String types ─────────────────────────────────────────────
typedef char*               LPSTR;
typedef const char*         LPCSTR;
typedef wchar_t*            LPWSTR;
typedef const wchar_t*      LPCWSTR;
typedef char*               PSTR;
typedef const char*         PCSTR;
typedef wchar_t             WCHAR;
typedef wchar_t*            PWCHAR;

// ── Handle types ─────────────────────────────────────────────
typedef void*               HANDLE;
typedef void*               HMODULE;
typedef void*               HINSTANCE;
typedef void*               HWND;
typedef void*               HDC;
typedef void*               HGDIOBJ;
typedef void*               HBITMAP;
typedef void*               HBRUSH;
typedef void*               HPEN;
typedef void*               HFONT;
typedef void*               HMENU;
typedef void*               HICON;
typedef void*               HCURSOR;
typedef void*               HKEY;
typedef void*               HTHREAD;
typedef void*               HPROCESS;
typedef void*               HRSRC;
typedef void*               HGLOBAL;
typedef void*               HLOCAL;
typedef void*               HACCEL;
typedef void*               HDESK;
typedef void*               HWINSTA;
typedef long long           HRESULT;
typedef long long           NTSTATUS;

// ── Special handle values ────────────────────────────────────
#define INVALID_HANDLE_VALUE    ((HANDLE)(long long)-1)
#define NULL                    0
#define NULLPTR                 0

// ── Common macros ────────────────────────────────────────────
#define MAKEWORD(lo, hi)        ((WORD)(((BYTE)(lo)) | ((WORD)((BYTE)(hi))) << 8))
#define MAKELONG(lo, hi)        ((LONG)(((WORD)(lo)) | ((DWORD)((WORD)(hi))) << 16))
#define LOWORD(l)               ((WORD)((DWORD_PTR)(l) & 0xFFFF))
#define HIWORD(l)               ((WORD)((DWORD_PTR)(l) >> 16))
#define LOBYTE(w)               ((BYTE)((DWORD_PTR)(w) & 0xFF))
#define HIBYTE(w)               ((BYTE)((DWORD_PTR)(w) >> 8))
#define MAX_PATH                260
#define MAX_GDI_APP_OBJECTS     16384
#define INFINITE                0xFFFFFFFF
#define WAIT_FAILED             0xFFFFFFFF
#define WAIT_OBJECT_0           0x00000000
#define WAIT_ABANDONED          0x00000080
#define WAIT_TIMEOUT            0x00000102
#define STILL_ACTIVE            259

// ── HRESULT codes ────────────────────────────────────────────
#define S_OK                    0x00000000
#define S_FALSE                 0x00000001
#define E_UNEXPECTED            0x8000FFFF
#define E_NOTIMPL               0x80004001
#define E_OUTOFMEMORY           0x8007000E
#define E_INVALIDARG            0x80070057
#define E_POINTER               0x80004003
#define E_HANDLE                0x80070006
#define E_FAIL                  0x80004005
#define E_ACCESSDENIED          0x80070005
#define SUCCEEDED(hr)           ((HRESULT)(hr) >= 0)
#define FAILED(hr)              ((HRESULT)(hr) < 0)

// ── Error codes ──────────────────────────────────────────────
#define ERROR_SUCCESS           0
#define ERROR_INVALID_FUNCTION  1
#define ERROR_FILE_NOT_FOUND    2
#define ERROR_PATH_NOT_FOUND    3
#define ERROR_TOO_MANY_OPEN_FILES 4
#define ERROR_ACCESS_DENIED     5
#define ERROR_INVALID_HANDLE    6
#define ERROR_NOT_ENOUGH_MEMORY 8
#define ERROR_INVALID_DRIVE     15
#define ERROR_MORE_DATA         234
#define ERROR_NO_MORE_FILES     18
#define ERROR_ALREADY_EXISTS    183
#define ERROR_IO_PENDING        997
#define ERROR_INSUFFICIENT_BUFFER 122

// ── File access / share / create flags ───────────────────────
#define GENERIC_READ            0x80000000
#define GENERIC_WRITE           0x40000000
#define GENERIC_EXECUTE         0x20000000
#define GENERIC_ALL             0x10000000
#define FILE_SHARE_READ         0x00000001
#define FILE_SHARE_WRITE        0x00000002
#define FILE_SHARE_DELETE       0x00000004
#define CREATE_NEW              1
#define CREATE_ALWAYS           2
#define OPEN_EXISTING           3
#define OPEN_ALWAYS             4
#define TRUNCATE_EXISTING       5
#define FILE_ATTRIBUTE_NORMAL   0x00000080
#define FILE_ATTRIBUTE_READONLY 0x00000001
#define FILE_ATTRIBUTE_HIDDEN   0x00000002
#define FILE_ATTRIBUTE_DIRECTORY 0x00000010
#define FILE_BEGIN              0
#define FILE_CURRENT            1
#define FILE_END                2

// ── Memory allocation ────────────────────────────────────────
#define MEM_COMMIT              0x00001000
#define MEM_RESERVE             0x00002000
#define MEM_RELEASE             0x00008000
#define MEM_DECOMMIT            0x00004000
#define PAGE_NOACCESS           0x01
#define PAGE_READONLY           0x02
#define PAGE_READWRITE          0x04
#define PAGE_WRITECOPY          0x08
#define PAGE_EXECUTE            0x10
#define PAGE_EXECUTE_READ       0x20
#define PAGE_EXECUTE_READWRITE  0x40
#define HEAP_ZERO_MEMORY        0x00000008
#define HEAP_NO_SERIALIZE       0x00000001

// ── Process / thread creation flags ──────────────────────────
#define CREATE_SUSPENDED        0x00000004
#define CREATE_NEW_CONSOLE      0x00000010
#define NORMAL_PRIORITY_CLASS   0x00000020
#define HIGH_PRIORITY_CLASS     0x00000080
#define CREATE_NO_WINDOW        0x08000000
#define PROCESS_ALL_ACCESS      0x001F0FFF
#define THREAD_ALL_ACCESS       0x001F03FF
#define PROCESS_VM_READ         0x0010
#define PROCESS_VM_WRITE        0x0020
#define PROCESS_VM_OPERATION    0x0008
#define TH32CS_SNAPPROCESS      0x00000002
#define TH32CS_SNAPTHREAD       0x00000004

// ── Standard handles ─────────────────────────────────────────
#define STD_INPUT_HANDLE        ((DWORD)-10)
#define STD_OUTPUT_HANDLE       ((DWORD)-11)
#define STD_ERROR_HANDLE        ((DWORD)-12)

// ── Console modes ────────────────────────────────────────────
#define ENABLE_LINE_INPUT       0x0002
#define ENABLE_ECHO_INPUT       0x0004
#define ENABLE_PROCESSED_INPUT  0x0001
#define ENABLE_PROCESSED_OUTPUT 0x0001
#define ENABLE_WRAP_AT_EOL_OUTPUT 0x0002
#define ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004

// ── MessageBox flags ─────────────────────────────────────────
#define MB_OK               0x00000000
#define MB_OKCANCEL         0x00000001
#define MB_YESNO            0x00000004
#define MB_YESNOCANCEL      0x00000003
#define MB_ICONERROR        0x00000010
#define MB_ICONWARNING      0x00000030
#define MB_ICONINFORMATION  0x00000040
#define MB_ICONQUESTION     0x00000020
#define IDOK                1
#define IDCANCEL            2
#define IDYES               6
#define IDNO                7

// ── Window styles ────────────────────────────────────────────
#define WS_OVERLAPPED       0x00000000
#define WS_POPUP            0x80000000
#define WS_CHILD            0x40000000
#define WS_VISIBLE          0x10000000
#define WS_BORDER           0x00800000
#define WS_CAPTION          0x00C00000
#define WS_SYSMENU          0x00080000
#define WS_MINIMIZEBOX      0x00020000
#define WS_MAXIMIZEBOX      0x00010000
#define WS_OVERLAPPEDWINDOW (WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_MINIMIZEBOX|WS_MAXIMIZEBOX)
#define CW_USEDEFAULT       0x80000000

// ── Window messages ──────────────────────────────────────────
#define WM_NULL             0x0000
#define WM_CREATE           0x0001
#define WM_DESTROY          0x0002
#define WM_MOVE             0x0003
#define WM_SIZE             0x0005
#define WM_ACTIVATE         0x0006
#define WM_PAINT            0x000F
#define WM_CLOSE            0x0010
#define WM_QUIT             0x0012
#define WM_KEYDOWN          0x0100
#define WM_KEYUP            0x0101
#define WM_LBUTTONDOWN      0x0201
#define WM_LBUTTONUP        0x0202
#define WM_RBUTTONDOWN      0x0204
#define WM_RBUTTONUP        0x0205
#define WM_MOUSEMOVE        0x0200
#define WM_MOUSEWHEEL       0x020A
#define WM_COMMAND          0x0111
#define WM_TIMER            0x0113
#define WM_USER             0x0400
#define PM_NOREMOVE         0x0000
#define PM_REMOVE           0x0001

// ── Show window commands ──────────────────────────────────────
#define SW_HIDE         0
#define SW_SHOWNORMAL   1
#define SW_SHOWMINIMIZED 2
#define SW_SHOWMAXIMIZED 3
#define SW_SHOW         5
#define SW_RESTORE      9

// ── Registry ─────────────────────────────────────────────────
#define HKEY_CLASSES_ROOT   ((HKEY)0x80000000)
#define HKEY_CURRENT_USER   ((HKEY)0x80000001)
#define HKEY_LOCAL_MACHINE  ((HKEY)0x80000002)
#define HKEY_USERS          ((HKEY)0x80000003)
#define KEY_READ            0x20019
#define KEY_WRITE           0x20006
#define KEY_ALL_ACCESS      0xF003F
#define REG_NONE            0
#define REG_SZ              1
#define REG_EXPAND_SZ       2
#define REG_BINARY          3
#define REG_DWORD           4
#define REG_MULTI_SZ        7
#define REG_QWORD           11

// ── WinSock ──────────────────────────────────────────────────
typedef unsigned int        SOCKET;
#define INVALID_SOCKET      ((SOCKET)(~0))
#define SOCKET_ERROR        (-1)
#define AF_INET             2
#define AF_INET6            23
#define SOCK_STREAM         1
#define SOCK_DGRAM          2
#define IPPROTO_TCP         6
#define IPPROTO_UDP         17
#define INADDR_ANY          0
#define INADDR_LOOPBACK     0x7f000001
#define INADDR_BROADCAST    0xFFFFFFFF
#define SD_RECEIVE          0
#define SD_SEND             1
#define SD_BOTH             2
#define WSADESCRIPTION_LEN  256
#define WSASYS_STATUS_LEN   128
#define SOL_SOCKET          0xFFFF
#define SO_REUSEADDR        0x0004
#define SO_KEEPALIVE        0x0008
#define SO_SNDBUF           0x1001
#define SO_RCVBUF           0x1002

// ── Structures ────────────────────────────────────────────────
struct POINT {
    LONG x;
    LONG y;
};

struct RECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
};

struct SIZE {
    LONG cx;
    LONG cy;
};

struct FILETIME {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
};

struct SYSTEMTIME {
    WORD wYear;
    WORD wMonth;
    WORD wDayOfWeek;
    WORD wDay;
    WORD wHour;
    WORD wMinute;
    WORD wSecond;
    WORD wMilliseconds;
};

struct OVERLAPPED {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    DWORD Offset;
    DWORD OffsetHigh;
    HANDLE hEvent;
};

struct SECURITY_ATTRIBUTES {
    DWORD  nLength;
    LPVOID lpSecurityDescriptor;
    BOOL   bInheritHandle;
};

struct STARTUPINFOA {
    DWORD  cb;
    LPSTR  lpReserved;
    LPSTR  lpDesktop;
    LPSTR  lpTitle;
    DWORD  dwX;
    DWORD  dwY;
    DWORD  dwXSize;
    DWORD  dwYSize;
    DWORD  dwXCountChars;
    DWORD  dwYCountChars;
    DWORD  dwFillAttribute;
    DWORD  dwFlags;
    WORD   wShowWindow;
    WORD   cbReserved2;
    BYTE*  lpReserved2;
    HANDLE hStdInput;
    HANDLE hStdOutput;
    HANDLE hStdError;
};

struct PROCESS_INFORMATION {
    HANDLE hProcess;
    HANDLE hThread;
    DWORD  dwProcessId;
    DWORD  dwThreadId;
};

struct WIN32_FIND_DATAA {
    DWORD    dwFileAttributes;
    FILETIME ftCreationTime;
    FILETIME ftLastAccessTime;
    FILETIME ftLastWriteTime;
    DWORD    nFileSizeHigh;
    DWORD    nFileSizeLow;
    DWORD    dwReserved0;
    DWORD    dwReserved1;
    char     cFileName[260];
    char     cAlternateFileName[14];
};

struct PROCESSENTRY32 {
    DWORD  dwSize;
    DWORD  cntUsage;
    DWORD  th32ProcessID;
    ULONG_PTR th32DefaultHeapID;
    DWORD  th32ModuleID;
    DWORD  cntThreads;
    DWORD  th32ParentProcessID;
    LONG   pcPriClassBase;
    DWORD  dwFlags;
    char   szExeFile[260];
};

struct MEMORY_BASIC_INFORMATION {
    PVOID  BaseAddress;
    PVOID  AllocationBase;
    DWORD  AllocationProtect;
    SIZE_T RegionSize;
    DWORD  State;
    DWORD  Protect;
    DWORD  Type;
};

struct WSADATA {
    WORD    wVersion;
    WORD    wHighVersion;
    unsigned short iMaxSockets;
    unsigned short iMaxUdpDg;
    char*   lpVendorInfo;
    char    szDescription[WSADESCRIPTION_LEN + 1];
    char    szSystemStatus[WSASYS_STATUS_LEN + 1];
};

struct sockaddr {
    unsigned short sa_family;
    char           sa_data[14];
};

struct in_addr {
    unsigned int s_addr;
};

struct sockaddr_in {
    short          sin_family;
    unsigned short sin_port;
    struct in_addr sin_addr;
    char           sin_zero[8];
};

struct WNDCLASSA {
    UINT      style;
    void*     lpfnWndProc;
    int       cbClsExtra;
    int       cbWndExtra;
    HINSTANCE hInstance;
    HICON     hIcon;
    HCURSOR   hCursor;
    HBRUSH    hbrBackground;
    LPCSTR    lpszMenuName;
    LPCSTR    lpszClassName;
};

struct MSG {
    HWND    hwnd;
    UINT    message;
    UINT    wParam;
    LONG    lParam;
    DWORD   time;
    POINT   pt;
};

struct COORD {
    SHORT X;
    SHORT Y;
};

struct SMALL_RECT {
    SHORT Left;
    SHORT Top;
    SHORT Right;
    SHORT Bottom;
};

struct CONSOLE_SCREEN_BUFFER_INFO {
    COORD      dwSize;
    COORD      dwCursorPosition;
    WORD       wAttributes;
    SMALL_RECT srWindow;
    COORD      dwMaximumWindowSize;
};

// ── kernel32 — Core API ───────────────────────────────────────
extern HANDLE  WINAPI GetStdHandle(DWORD nStdHandle);
extern BOOL    WINAPI WriteConsoleA(HANDLE, const void*, DWORD, DWORD*, void*);
extern BOOL    WINAPI ReadConsoleA(HANDLE, void*, DWORD, DWORD*, void*);
extern BOOL    WINAPI WriteFile(HANDLE, LPCVOID, DWORD, DWORD*, OVERLAPPED*);
extern BOOL    WINAPI ReadFile(HANDLE, LPVOID, DWORD, DWORD*, OVERLAPPED*);
extern HANDLE  WINAPI CreateFileA(LPCSTR, DWORD, DWORD, SECURITY_ATTRIBUTES*, DWORD, DWORD, HANDLE);
extern BOOL    WINAPI CloseHandle(HANDLE);
extern BOOL    WINAPI FlushFileBuffers(HANDLE);
extern DWORD   WINAPI GetLastError();
extern VOID    WINAPI SetLastError(DWORD);
extern DWORD   WINAPI GetFileSize(HANDLE, DWORD*);
extern DWORD   WINAPI SetFilePointer(HANDLE, LONG, LONG*, DWORD);
extern BOOL    WINAPI DeleteFileA(LPCSTR);
extern BOOL    WINAPI MoveFileA(LPCSTR, LPCSTR);
extern BOOL    WINAPI CopyFileA(LPCSTR, LPCSTR, BOOL);
extern BOOL    WINAPI CreateDirectoryA(LPCSTR, SECURITY_ATTRIBUTES*);
extern BOOL    WINAPI RemoveDirectoryA(LPCSTR);
extern HANDLE  WINAPI FindFirstFileA(LPCSTR, WIN32_FIND_DATAA*);
extern BOOL    WINAPI FindNextFileA(HANDLE, WIN32_FIND_DATAA*);
extern BOOL    WINAPI FindClose(HANDLE);
extern DWORD   WINAPI GetCurrentDirectory(DWORD, LPSTR);
extern BOOL    WINAPI SetCurrentDirectoryA(LPCSTR);
extern DWORD   WINAPI GetTempPathA(DWORD, LPSTR);
extern DWORD   WINAPI GetEnvironmentVariableA(LPCSTR, LPSTR, DWORD);
extern BOOL    WINAPI SetEnvironmentVariableA(LPCSTR, LPCSTR);
extern VOID    WINAPI ExitProcess(UINT);
extern VOID    WINAPI ExitThread(DWORD);
extern VOID    WINAPI Sleep(DWORD);
extern DWORD   WINAPI GetTickCount();
extern BOOL    WINAPI QueryPerformanceCounter(LONGLONG*);
extern BOOL    WINAPI QueryPerformanceFrequency(LONGLONG*);
extern VOID*   WINAPI VirtualAlloc(VOID*, SIZE_T, DWORD, DWORD);
extern BOOL    WINAPI VirtualFree(VOID*, SIZE_T, DWORD);
extern BOOL    WINAPI VirtualProtect(VOID*, SIZE_T, DWORD, DWORD*);
extern SIZE_T  WINAPI VirtualQuery(LPCVOID, MEMORY_BASIC_INFORMATION*, SIZE_T);
extern VOID*   WINAPI HeapAlloc(HANDLE, DWORD, SIZE_T);
extern BOOL    WINAPI HeapFree(HANDLE, DWORD, VOID*);
extern VOID*   WINAPI HeapReAlloc(HANDLE, DWORD, VOID*, SIZE_T);
extern HANDLE  WINAPI GetProcessHeap();
extern HANDLE  WINAPI CreateThread(SECURITY_ATTRIBUTES*, SIZE_T, void*, VOID*, DWORD, DWORD*);
extern HANDLE  WINAPI OpenProcess(DWORD, BOOL, DWORD);
extern DWORD   WINAPI WaitForSingleObject(HANDLE, DWORD);
extern DWORD   WINAPI WaitForMultipleObjects(DWORD, HANDLE*, BOOL, DWORD);
extern BOOL    WINAPI TerminateProcess(HANDLE, UINT);
extern BOOL    WINAPI TerminateThread(HANDLE, DWORD);
extern DWORD   WINAPI GetCurrentProcessId();
extern DWORD   WINAPI GetCurrentThreadId();
extern HANDLE  WINAPI GetCurrentProcess();
extern HANDLE  WINAPI GetCurrentThread();
extern BOOL    WINAPI CreateProcessA(LPCSTR, LPSTR, SECURITY_ATTRIBUTES*, SECURITY_ATTRIBUTES*, BOOL, DWORD, VOID*, LPCSTR, STARTUPINFOA*, PROCESS_INFORMATION*);
extern BOOL    WINAPI ReadProcessMemory(HANDLE, LPCVOID, LPVOID, SIZE_T, SIZE_T*);
extern BOOL    WINAPI WriteProcessMemory(HANDLE, LPVOID, LPCVOID, SIZE_T, SIZE_T*);
extern HANDLE  WINAPI CreateToolhelp32Snapshot(DWORD, DWORD);
extern BOOL    WINAPI Process32First(HANDLE, PROCESSENTRY32*);
extern BOOL    WINAPI Process32Next(HANDLE, PROCESSENTRY32*);
extern HANDLE  WINAPI CreateMutexA(SECURITY_ATTRIBUTES*, BOOL, LPCSTR);
extern HANDLE  WINAPI CreateEventA(SECURITY_ATTRIBUTES*, BOOL, BOOL, LPCSTR);
extern BOOL    WINAPI SetEvent(HANDLE);
extern BOOL    WINAPI ResetEvent(HANDLE);
extern HMODULE WINAPI LoadLibraryA(LPCSTR);
extern BOOL    WINAPI FreeLibrary(HMODULE);
extern VOID*   WINAPI GetProcAddress(HMODULE, LPCSTR);
extern DWORD   WINAPI GetModuleFileNameA(HMODULE, LPSTR, DWORD);
extern HMODULE WINAPI GetModuleHandleA(LPCSTR);
extern BOOL    WINAPI GetSystemInfo_NotImpl();  // Use SYSTEM_INFO struct manually
extern VOID    WINAPI GetSystemTime(SYSTEMTIME*);
extern VOID    WINAPI GetLocalTime(SYSTEMTIME*);
extern BOOL    WINAPI SetSystemTime(const SYSTEMTIME*);
extern BOOL    WINAPI GetConsoleScreenBufferInfo(HANDLE, CONSOLE_SCREEN_BUFFER_INFO*);
extern BOOL    WINAPI SetConsoleTextAttribute(HANDLE, WORD);
extern BOOL    WINAPI SetConsoleCursorPosition(HANDLE, COORD);
extern BOOL    WINAPI SetConsoleMode(HANDLE, DWORD);
extern BOOL    WINAPI GetConsoleMode(HANDLE, DWORD*);
extern BOOL    WINAPI SetConsoleTitleA(LPCSTR);
extern DWORD   WINAPI FormatMessageA(DWORD, LPCVOID, DWORD, DWORD, LPSTR, DWORD, void*);
extern VOID*   WINAPI LocalAlloc(UINT, SIZE_T);
extern BOOL    WINAPI LocalFree(VOID*);
extern VOID*   WINAPI GlobalAlloc(UINT, SIZE_T);
extern BOOL    WINAPI GlobalFree(VOID*);

// ── user32 — GUI API ──────────────────────────────────────────
extern int     WINAPI MessageBoxA(HWND, LPCSTR, LPCSTR, UINT);
extern int     WINAPI MessageBoxW(HWND, LPCWSTR, LPCWSTR, UINT);
extern HWND    WINAPI CreateWindowExA(DWORD, LPCSTR, LPCSTR, DWORD, int, int, int, int, HWND, HMENU, HINSTANCE, LPVOID);
extern BOOL    WINAPI DestroyWindow(HWND);
extern BOOL    WINAPI ShowWindow(HWND, int);
extern BOOL    WINAPI UpdateWindow(HWND);
extern BOOL    WINAPI GetMessage(MSG*, HWND, UINT, UINT);
extern BOOL    WINAPI PeekMessageA(MSG*, HWND, UINT, UINT, UINT);
extern BOOL    WINAPI TranslateMessage(const MSG*);
extern LONG    WINAPI DispatchMessageA(const MSG*);
extern VOID    WINAPI PostQuitMessage(int);
extern LONG    WINAPI DefWindowProcA(HWND, UINT, UINT, LONG);
extern BOOL    WINAPI RegisterClassA(const WNDCLASSA*);
extern BOOL    WINAPI UnregisterClassA(LPCSTR, HINSTANCE);
extern HDC     WINAPI GetDC(HWND);
extern int     WINAPI ReleaseDC(HWND, HDC);
extern BOOL    WINAPI InvalidateRect(HWND, const RECT*, BOOL);
extern BOOL    WINAPI SetWindowTextA(HWND, LPCSTR);
extern int     WINAPI GetWindowTextA(HWND, LPSTR, int);
extern BOOL    WINAPI GetWindowRect(HWND, RECT*);
extern BOOL    WINAPI SetWindowPos(HWND, HWND, int, int, int, int, UINT);
extern BOOL    WINAPI MoveWindow(HWND, int, int, int, int, BOOL);
extern HWND    WINAPI GetForegroundWindow();
extern BOOL    WINAPI SetForegroundWindow(HWND);
extern HWND    WINAPI FindWindowA(LPCSTR, LPCSTR);
extern BOOL    WINAPI IsWindow(HWND);
extern BOOL    WINAPI IsWindowVisible(HWND);
extern HWND    WINAPI GetDesktopWindow();
extern BOOL    WINAPI GetCursorPos(POINT*);
extern BOOL    WINAPI SetCursorPos(int, int);
extern int     WINAPI GetSystemMetrics(int);
extern UINT    WINAPI SetTimer(HWND, UINT, UINT, void*);
extern BOOL    WINAPI KillTimer(HWND, UINT);
extern SHORT   WINAPI GetKeyState(int);
extern SHORT   WINAPI GetAsyncKeyState(int);

// ── gdi32 — Drawing ───────────────────────────────────────────
extern BOOL    WINAPI TextOutA(HDC, int, int, LPCSTR, int);
extern BOOL    WINAPI Rectangle(HDC, int, int, int, int);
extern BOOL    WINAPI Ellipse(HDC, int, int, int, int);
extern BOOL    WINAPI MoveToEx(HDC, int, int, POINT*);
extern BOOL    WINAPI LineTo(HDC, int, int);
extern DWORD   WINAPI SetTextColor(HDC, DWORD);
extern DWORD   WINAPI SetBkColor(HDC, DWORD);
extern HBRUSH  WINAPI CreateSolidBrush(DWORD);
extern BOOL    WINAPI DeleteObject(HGDIOBJ);
extern HPEN    WINAPI CreatePen(int, int, DWORD);
extern HGDIOBJ WINAPI SelectObject(HDC, HGDIOBJ);
extern BOOL    WINAPI BitBlt(HDC, int, int, int, int, HDC, int, int, DWORD);
extern HDC     WINAPI CreateCompatibleDC(HDC);
extern HBITMAP WINAPI CreateCompatibleBitmap(HDC, int, int);
extern BOOL    WINAPI DeleteDC(HDC);
extern HDC     WINAPI BeginPaint(HWND, void*);
extern BOOL    WINAPI EndPaint(HWND, void*);

// ── advapi32 — Registry / Security ───────────────────────────
extern LONG    WINAPI RegOpenKeyExA(HKEY, LPCSTR, DWORD, DWORD, HKEY*);
extern LONG    WINAPI RegQueryValueExA(HKEY, LPCSTR, DWORD*, DWORD*, BYTE*, DWORD*);
extern LONG    WINAPI RegSetValueExA(HKEY, LPCSTR, DWORD, DWORD, const BYTE*, DWORD);
extern LONG    WINAPI RegCreateKeyExA(HKEY, LPCSTR, DWORD, LPSTR, DWORD, DWORD, SECURITY_ATTRIBUTES*, HKEY*, DWORD*);
extern LONG    WINAPI RegDeleteKeyA(HKEY, LPCSTR);
extern LONG    WINAPI RegDeleteValueA(HKEY, LPCSTR);
extern LONG    WINAPI RegCloseKey(HKEY);
extern LONG    WINAPI RegEnumKeyExA(HKEY, DWORD, LPSTR, DWORD*, DWORD*, LPSTR, DWORD*, FILETIME*);
extern LONG    WINAPI RegEnumValueA(HKEY, DWORD, LPSTR, DWORD*, DWORD*, DWORD*, BYTE*, DWORD*);

// ── ws2_32 — WinSock 2 ────────────────────────────────────────
extern int     WINAPI WSAStartup(WORD, WSADATA*);
extern int     WINAPI WSACleanup();
extern SOCKET  WINAPI socket(int, int, int);
extern int     WINAPI bind(SOCKET, const sockaddr*, int);
extern int     WINAPI connect(SOCKET, const sockaddr*, int);
extern int     WINAPI listen(SOCKET, int);
extern SOCKET  WINAPI accept(SOCKET, sockaddr*, int*);
extern int     WINAPI send(SOCKET, const char*, int, int);
extern int     WINAPI recv(SOCKET, char*, int, int);
extern int     WINAPI closesocket(SOCKET);
extern int     WINAPI setsockopt(SOCKET, int, int, const char*, int);
extern int     WINAPI getsockopt(SOCKET, int, int, char*, int*);
extern unsigned long WINAPI inet_addr(const char*);
extern char*   WINAPI inet_ntoa(struct in_addr);
extern unsigned short WINAPI htons(unsigned short);
extern unsigned short WINAPI ntohs(unsigned short);
extern unsigned int WINAPI htonl(unsigned int);
extern unsigned int WINAPI ntohl(unsigned int);
extern int     WINAPI WSAGetLastError();
extern int     WINAPI shutdown(SOCKET, int);
extern int     WINAPI sendto(SOCKET, const char*, int, int, const sockaddr*, int);
extern int     WINAPI recvfrom(SOCKET, char*, int, int, sockaddr*, int*);
extern int     WINAPI select(int, void*, void*, void*, void*);

// ── shell32 ───────────────────────────────────────────────────
extern LONG    WINAPI ShellExecuteA(HWND, LPCSTR, LPCSTR, LPCSTR, LPCSTR, int);

// ── Utility macros ───────────────────────────────────────────
#define RGB(r, g, b)    ((DWORD)(((BYTE)(r) | ((WORD)((BYTE)(g)) << 8)) | (((DWORD)(BYTE)(b)) << 16)))
#define STDCALL         __stdcall
#define IN
#define OUT
#define OPTIONAL
#define FAR
#define NEAR
#define CONST               const
#define VOID_               void
#define DECLARE_HANDLE(n)   typedef void* n

#endif // _ADEAD_WINDOWS_H
