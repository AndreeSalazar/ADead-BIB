/* adeb_win32.h — Win32 API mínimo (kernel32 + user32 + gdi32) */
#ifndef ADEB_WIN32_H
#define ADEB_WIN32_H

#include "adeb_types.h"

typedef void*    HANDLE;
typedef void*    HWND;
typedef void*    HINSTANCE;
typedef void*    HMODULE;
typedef uint32_t DWORD;
typedef int32_t  BOOL;
typedef uint16_t WORD;
typedef uint8_t  BYTE;
typedef uint64_t LPARAM;
typedef uint64_t WPARAM;
typedef int64_t  LRESULT;
typedef const char* LPCSTR;
typedef char*       LPSTR;

#define MB_OK        0x00000000
#define MB_OKCANCEL  0x00000001
#define MB_ICONINFO  0x00000040

/* kernel32 */
extern HANDLE GetStdHandle(DWORD nStdHandle);
extern BOOL   WriteFile(HANDLE h, const void* buf, DWORD n, DWORD* written, void* overlap);
extern BOOL   ReadFile(HANDLE h, void* buf, DWORD n, DWORD* read_, void* overlap);
extern HANDLE CreateFileA(LPCSTR name, DWORD access, DWORD share, void* sa, DWORD create, DWORD flags, HANDLE tmpl);
extern BOOL   CloseHandle(HANDLE h);
extern void   ExitProcess(uint32_t code);
extern HMODULE LoadLibraryA(LPCSTR name);
extern void*  GetProcAddress(HMODULE m, LPCSTR name);
extern HANDLE HeapAlloc(HANDLE heap, DWORD flags, size_t size);
extern BOOL   HeapFree(HANDLE heap, DWORD flags, void* ptr);
extern HANDLE GetProcessHeap(void);

/* user32 */
extern int    MessageBoxA(HWND hwnd, LPCSTR text, LPCSTR caption, uint32_t type);
extern HWND   CreateWindowExA(DWORD exStyle, LPCSTR cls, LPCSTR title, DWORD style,
                              int x, int y, int w, int h,
                              HWND parent, HANDLE menu, HINSTANCE inst, void* lParam);
extern BOOL   ShowWindow(HWND hwnd, int cmdShow);
extern BOOL   UpdateWindow(HWND hwnd);
extern BOOL   DestroyWindow(HWND hwnd);
extern void   PostQuitMessage(int code);

#endif /* ADEB_WIN32_H */
