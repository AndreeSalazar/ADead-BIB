/*
 * ADead-BIB Standard Library
 * windows.h - Windows API Compatibility Layer
 * 
 * Based on: Windows SDK, ReactOS, Wine
 * For FastOS Po loader Win32 compatibility
 */

#ifndef _ADEAD_WINDOWS_H
#define _ADEAD_WINDOWS_H

#include "stdint.h"
#include "stddef.h"

/* ============================================================
 * Windows Basic Types
 * ============================================================ */

typedef void VOID;
typedef void* PVOID;
typedef void* LPVOID;
typedef const void* LPCVOID;

typedef int BOOL;
typedef int INT;
typedef unsigned int UINT;
typedef long LONG;
typedef unsigned long ULONG;
typedef unsigned long DWORD;
typedef unsigned short WORD;
typedef unsigned char BYTE;

typedef int64_t LONGLONG;
typedef uint64_t ULONGLONG;
typedef uint64_t DWORD64;
typedef uint64_t QWORD;

typedef char CHAR;
typedef wchar_t WCHAR;
typedef char* LPSTR;
typedef const char* LPCSTR;
typedef wchar_t* LPWSTR;
typedef const wchar_t* LPCWSTR;

typedef CHAR* PCHAR;
typedef WCHAR* PWCHAR;
typedef BYTE* PBYTE;
typedef BYTE* LPBYTE;
typedef WORD* PWORD;
typedef DWORD* PDWORD;
typedef DWORD* LPDWORD;

typedef intptr_t INT_PTR;
typedef uintptr_t UINT_PTR;
typedef intptr_t LONG_PTR;
typedef uintptr_t ULONG_PTR;
typedef uintptr_t DWORD_PTR;
typedef uintptr_t SIZE_T;
typedef intptr_t SSIZE_T;

/* Handle types */
typedef void* HANDLE;
typedef HANDLE HINSTANCE;
typedef HANDLE HMODULE;
typedef HANDLE HWND;
typedef HANDLE HDC;
typedef HANDLE HBITMAP;
typedef HANDLE HBRUSH;
typedef HANDLE HPEN;
typedef HANDLE HFONT;
typedef HANDLE HICON;
typedef HANDLE HCURSOR;
typedef HANDLE HMENU;
typedef HANDLE HKEY;
typedef HANDLE HGLOBAL;
typedef HANDLE HLOCAL;
typedef HANDLE HRSRC;
typedef HANDLE HFILE;
typedef HANDLE HGDIOBJ;
typedef HANDLE HPALETTE;
typedef HANDLE HRGN;
typedef HANDLE HMONITOR;

#define INVALID_HANDLE_VALUE ((HANDLE)(LONG_PTR)-1)

/* Boolean */
#define FALSE 0
#define TRUE  1

/* Null */
#ifndef NULL
#define NULL ((void*)0)
#endif

/* Calling conventions */
#define WINAPI      __attribute__((ms_abi))
#define APIENTRY    WINAPI
#define CALLBACK    WINAPI
#define STDCALL     __attribute__((stdcall))
#define CDECL       __attribute__((cdecl))
#define PASCAL      __attribute__((stdcall))

/* Export/Import */
#define DECLSPEC_IMPORT __attribute__((dllimport))
#define DECLSPEC_EXPORT __attribute__((dllexport))

/* Inline */
#define FORCEINLINE __attribute__((always_inline)) inline

/* ============================================================
 * Windows Structures
 * ============================================================ */

/* GUID */
typedef struct _GUID {
    DWORD Data1;
    WORD  Data2;
    WORD  Data3;
    BYTE  Data4[8];
} GUID, *LPGUID;

typedef GUID IID;
typedef GUID CLSID;
typedef const GUID* REFGUID;
typedef const IID* REFIID;
typedef const CLSID* REFCLSID;

/* RECT */
typedef struct tagRECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
} RECT, *PRECT, *LPRECT;

/* POINT */
typedef struct tagPOINT {
    LONG x;
    LONG y;
} POINT, *PPOINT, *LPPOINT;

/* SIZE */
typedef struct tagSIZE {
    LONG cx;
    LONG cy;
} SIZE, *PSIZE, *LPSIZE;

/* FILETIME */
typedef struct _FILETIME {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
} FILETIME, *PFILETIME, *LPFILETIME;

/* SYSTEMTIME */
typedef struct _SYSTEMTIME {
    WORD wYear;
    WORD wMonth;
    WORD wDayOfWeek;
    WORD wDay;
    WORD wHour;
    WORD wMinute;
    WORD wSecond;
    WORD wMilliseconds;
} SYSTEMTIME, *PSYSTEMTIME, *LPSYSTEMTIME;

/* SECURITY_ATTRIBUTES */
typedef struct _SECURITY_ATTRIBUTES {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
} SECURITY_ATTRIBUTES, *PSECURITY_ATTRIBUTES, *LPSECURITY_ATTRIBUTES;

/* OVERLAPPED */
typedef struct _OVERLAPPED {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    union {
        struct {
            DWORD Offset;
            DWORD OffsetHigh;
        };
        PVOID Pointer;
    };
    HANDLE hEvent;
} OVERLAPPED, *LPOVERLAPPED;

/* LARGE_INTEGER */
typedef union _LARGE_INTEGER {
    struct {
        DWORD LowPart;
        LONG HighPart;
    };
    struct {
        DWORD LowPart;
        LONG HighPart;
    } u;
    LONGLONG QuadPart;
} LARGE_INTEGER, *PLARGE_INTEGER;

typedef union _ULARGE_INTEGER {
    struct {
        DWORD LowPart;
        DWORD HighPart;
    };
    struct {
        DWORD LowPart;
        DWORD HighPart;
    } u;
    ULONGLONG QuadPart;
} ULARGE_INTEGER, *PULARGE_INTEGER;

/* ============================================================
 * Error Codes
 * ============================================================ */

#define ERROR_SUCCESS           0
#define ERROR_INVALID_FUNCTION  1
#define ERROR_FILE_NOT_FOUND    2
#define ERROR_PATH_NOT_FOUND    3
#define ERROR_TOO_MANY_OPEN_FILES 4
#define ERROR_ACCESS_DENIED     5
#define ERROR_INVALID_HANDLE    6
#define ERROR_NOT_ENOUGH_MEMORY 8
#define ERROR_INVALID_DATA      13
#define ERROR_OUTOFMEMORY       14
#define ERROR_INVALID_DRIVE     15
#define ERROR_NO_MORE_FILES     18
#define ERROR_WRITE_PROTECT     19
#define ERROR_NOT_READY         21
#define ERROR_SHARING_VIOLATION 32
#define ERROR_LOCK_VIOLATION    33
#define ERROR_HANDLE_EOF        38
#define ERROR_NOT_SUPPORTED     50
#define ERROR_FILE_EXISTS       80
#define ERROR_INVALID_PARAMETER 87
#define ERROR_BROKEN_PIPE       109
#define ERROR_DISK_FULL         112
#define ERROR_INSUFFICIENT_BUFFER 122
#define ERROR_INVALID_NAME      123
#define ERROR_MOD_NOT_FOUND     126
#define ERROR_PROC_NOT_FOUND    127
#define ERROR_ALREADY_EXISTS    183
#define ERROR_ENVVAR_NOT_FOUND  203
#define ERROR_MORE_DATA         234
#define ERROR_NO_MORE_ITEMS     259
#define ERROR_TIMEOUT           1460

/* ============================================================
 * Kernel32 Functions
 * ============================================================ */

/* Error handling */
DWORD WINAPI GetLastError(void);
void WINAPI SetLastError(DWORD dwErrCode);

/* Memory */
LPVOID WINAPI VirtualAlloc(LPVOID lpAddress, SIZE_T dwSize, DWORD flAllocationType, DWORD flProtect);
BOOL WINAPI VirtualFree(LPVOID lpAddress, SIZE_T dwSize, DWORD dwFreeType);
BOOL WINAPI VirtualProtect(LPVOID lpAddress, SIZE_T dwSize, DWORD flNewProtect, PDWORD lpflOldProtect);
SIZE_T WINAPI VirtualQuery(LPCVOID lpAddress, PVOID lpBuffer, SIZE_T dwLength);

HGLOBAL WINAPI GlobalAlloc(UINT uFlags, SIZE_T dwBytes);
HGLOBAL WINAPI GlobalFree(HGLOBAL hMem);
LPVOID WINAPI GlobalLock(HGLOBAL hMem);
BOOL WINAPI GlobalUnlock(HGLOBAL hMem);
SIZE_T WINAPI GlobalSize(HGLOBAL hMem);

HLOCAL WINAPI LocalAlloc(UINT uFlags, SIZE_T uBytes);
HLOCAL WINAPI LocalFree(HLOCAL hMem);
LPVOID WINAPI LocalLock(HLOCAL hMem);
BOOL WINAPI LocalUnlock(HLOCAL hMem);

HANDLE WINAPI HeapCreate(DWORD flOptions, SIZE_T dwInitialSize, SIZE_T dwMaximumSize);
BOOL WINAPI HeapDestroy(HANDLE hHeap);
LPVOID WINAPI HeapAlloc(HANDLE hHeap, DWORD dwFlags, SIZE_T dwBytes);
LPVOID WINAPI HeapReAlloc(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem, SIZE_T dwBytes);
BOOL WINAPI HeapFree(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem);
SIZE_T WINAPI HeapSize(HANDLE hHeap, DWORD dwFlags, LPCVOID lpMem);
HANDLE WINAPI GetProcessHeap(void);

/* Memory flags */
#define MEM_COMMIT      0x00001000
#define MEM_RESERVE     0x00002000
#define MEM_DECOMMIT    0x00004000
#define MEM_RELEASE     0x00008000
#define MEM_FREE        0x00010000
#define MEM_RESET       0x00080000

#define PAGE_NOACCESS          0x01
#define PAGE_READONLY          0x02
#define PAGE_READWRITE         0x04
#define PAGE_WRITECOPY         0x08
#define PAGE_EXECUTE           0x10
#define PAGE_EXECUTE_READ      0x20
#define PAGE_EXECUTE_READWRITE 0x40
#define PAGE_EXECUTE_WRITECOPY 0x80
#define PAGE_GUARD             0x100
#define PAGE_NOCACHE           0x200

#define HEAP_NO_SERIALIZE      0x00000001
#define HEAP_ZERO_MEMORY       0x00000008
#define HEAP_GENERATE_EXCEPTIONS 0x00000004

#define GMEM_FIXED    0x0000
#define GMEM_MOVEABLE 0x0002
#define GMEM_ZEROINIT 0x0040
#define GPTR          (GMEM_FIXED | GMEM_ZEROINIT)
#define GHND          (GMEM_MOVEABLE | GMEM_ZEROINIT)

#define LMEM_FIXED    0x0000
#define LMEM_MOVEABLE 0x0002
#define LMEM_ZEROINIT 0x0040
#define LPTR          (LMEM_FIXED | LMEM_ZEROINIT)

/* File I/O */
HANDLE WINAPI CreateFileA(LPCSTR lpFileName, DWORD dwDesiredAccess, DWORD dwShareMode,
                          LPSECURITY_ATTRIBUTES lpSecurityAttributes, DWORD dwCreationDisposition,
                          DWORD dwFlagsAndAttributes, HANDLE hTemplateFile);
HANDLE WINAPI CreateFileW(LPCWSTR lpFileName, DWORD dwDesiredAccess, DWORD dwShareMode,
                          LPSECURITY_ATTRIBUTES lpSecurityAttributes, DWORD dwCreationDisposition,
                          DWORD dwFlagsAndAttributes, HANDLE hTemplateFile);
BOOL WINAPI ReadFile(HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToRead,
                     LPDWORD lpNumberOfBytesRead, LPOVERLAPPED lpOverlapped);
BOOL WINAPI WriteFile(HANDLE hFile, LPCVOID lpBuffer, DWORD nNumberOfBytesToWrite,
                      LPDWORD lpNumberOfBytesWritten, LPOVERLAPPED lpOverlapped);
BOOL WINAPI CloseHandle(HANDLE hObject);
DWORD WINAPI SetFilePointer(HANDLE hFile, LONG lDistanceToMove, PLONG lpDistanceToMoveHigh, DWORD dwMoveMethod);
BOOL WINAPI SetEndOfFile(HANDLE hFile);
DWORD WINAPI GetFileSize(HANDLE hFile, LPDWORD lpFileSizeHigh);
BOOL WINAPI GetFileSizeEx(HANDLE hFile, PLARGE_INTEGER lpFileSize);
DWORD WINAPI GetFileAttributesA(LPCSTR lpFileName);
DWORD WINAPI GetFileAttributesW(LPCWSTR lpFileName);
BOOL WINAPI SetFileAttributesA(LPCSTR lpFileName, DWORD dwFileAttributes);
BOOL WINAPI DeleteFileA(LPCSTR lpFileName);
BOOL WINAPI DeleteFileW(LPCWSTR lpFileName);
BOOL WINAPI CopyFileA(LPCSTR lpExistingFileName, LPCSTR lpNewFileName, BOOL bFailIfExists);
BOOL WINAPI MoveFileA(LPCSTR lpExistingFileName, LPCSTR lpNewFileName);
BOOL WINAPI CreateDirectoryA(LPCSTR lpPathName, LPSECURITY_ATTRIBUTES lpSecurityAttributes);
BOOL WINAPI RemoveDirectoryA(LPCSTR lpPathName);
DWORD WINAPI GetCurrentDirectoryA(DWORD nBufferLength, LPSTR lpBuffer);
BOOL WINAPI SetCurrentDirectoryA(LPCSTR lpPathName);

/* File access flags */
#define GENERIC_READ    0x80000000
#define GENERIC_WRITE   0x40000000
#define GENERIC_EXECUTE 0x20000000
#define GENERIC_ALL     0x10000000

#define FILE_SHARE_READ   0x00000001
#define FILE_SHARE_WRITE  0x00000002
#define FILE_SHARE_DELETE 0x00000004

#define CREATE_NEW        1
#define CREATE_ALWAYS     2
#define OPEN_EXISTING     3
#define OPEN_ALWAYS       4
#define TRUNCATE_EXISTING 5

#define FILE_ATTRIBUTE_READONLY  0x00000001
#define FILE_ATTRIBUTE_HIDDEN    0x00000002
#define FILE_ATTRIBUTE_SYSTEM    0x00000004
#define FILE_ATTRIBUTE_DIRECTORY 0x00000010
#define FILE_ATTRIBUTE_ARCHIVE   0x00000020
#define FILE_ATTRIBUTE_NORMAL    0x00000080

#define FILE_BEGIN   0
#define FILE_CURRENT 1
#define FILE_END     2

#define INVALID_FILE_ATTRIBUTES ((DWORD)-1)
#define INVALID_FILE_SIZE       ((DWORD)0xFFFFFFFF)
#define INVALID_SET_FILE_POINTER ((DWORD)-1)

/* Process/Thread */
HANDLE WINAPI GetCurrentProcess(void);
DWORD WINAPI GetCurrentProcessId(void);
HANDLE WINAPI GetCurrentThread(void);
DWORD WINAPI GetCurrentThreadId(void);

BOOL WINAPI CreateProcessA(LPCSTR lpApplicationName, LPSTR lpCommandLine,
                           LPSECURITY_ATTRIBUTES lpProcessAttributes,
                           LPSECURITY_ATTRIBUTES lpThreadAttributes,
                           BOOL bInheritHandles, DWORD dwCreationFlags,
                           LPVOID lpEnvironment, LPCSTR lpCurrentDirectory,
                           LPVOID lpStartupInfo, LPVOID lpProcessInformation);
void WINAPI ExitProcess(UINT uExitCode);
BOOL WINAPI TerminateProcess(HANDLE hProcess, UINT uExitCode);
DWORD WINAPI WaitForSingleObject(HANDLE hHandle, DWORD dwMilliseconds);
DWORD WINAPI WaitForMultipleObjects(DWORD nCount, const HANDLE* lpHandles, BOOL bWaitAll, DWORD dwMilliseconds);

HANDLE WINAPI CreateThread(LPSECURITY_ATTRIBUTES lpThreadAttributes, SIZE_T dwStackSize,
                           LPVOID lpStartAddress, LPVOID lpParameter,
                           DWORD dwCreationFlags, LPDWORD lpThreadId);
void WINAPI ExitThread(DWORD dwExitCode);
BOOL WINAPI TerminateThread(HANDLE hThread, DWORD dwExitCode);
DWORD WINAPI SuspendThread(HANDLE hThread);
DWORD WINAPI ResumeThread(HANDLE hThread);
void WINAPI Sleep(DWORD dwMilliseconds);
BOOL WINAPI SwitchToThread(void);

#define INFINITE 0xFFFFFFFF
#define WAIT_OBJECT_0    0x00000000
#define WAIT_ABANDONED   0x00000080
#define WAIT_TIMEOUT     0x00000102
#define WAIT_FAILED      0xFFFFFFFF

/* Synchronization */
HANDLE WINAPI CreateMutexA(LPSECURITY_ATTRIBUTES lpMutexAttributes, BOOL bInitialOwner, LPCSTR lpName);
BOOL WINAPI ReleaseMutex(HANDLE hMutex);
HANDLE WINAPI CreateEventA(LPSECURITY_ATTRIBUTES lpEventAttributes, BOOL bManualReset, BOOL bInitialState, LPCSTR lpName);
BOOL WINAPI SetEvent(HANDLE hEvent);
BOOL WINAPI ResetEvent(HANDLE hEvent);
HANDLE WINAPI CreateSemaphoreA(LPSECURITY_ATTRIBUTES lpSemaphoreAttributes, LONG lInitialCount, LONG lMaximumCount, LPCSTR lpName);
BOOL WINAPI ReleaseSemaphore(HANDLE hSemaphore, LONG lReleaseCount, LPLONG lpPreviousCount);

void WINAPI InitializeCriticalSection(LPVOID lpCriticalSection);
void WINAPI DeleteCriticalSection(LPVOID lpCriticalSection);
void WINAPI EnterCriticalSection(LPVOID lpCriticalSection);
void WINAPI LeaveCriticalSection(LPVOID lpCriticalSection);
BOOL WINAPI TryEnterCriticalSection(LPVOID lpCriticalSection);

/* Module/Library */
HMODULE WINAPI LoadLibraryA(LPCSTR lpLibFileName);
HMODULE WINAPI LoadLibraryW(LPCWSTR lpLibFileName);
HMODULE WINAPI LoadLibraryExA(LPCSTR lpLibFileName, HANDLE hFile, DWORD dwFlags);
BOOL WINAPI FreeLibrary(HMODULE hLibModule);
LPVOID WINAPI GetProcAddress(HMODULE hModule, LPCSTR lpProcName);
HMODULE WINAPI GetModuleHandleA(LPCSTR lpModuleName);
HMODULE WINAPI GetModuleHandleW(LPCWSTR lpModuleName);
DWORD WINAPI GetModuleFileNameA(HMODULE hModule, LPSTR lpFilename, DWORD nSize);

/* Environment */
DWORD WINAPI GetEnvironmentVariableA(LPCSTR lpName, LPSTR lpBuffer, DWORD nSize);
BOOL WINAPI SetEnvironmentVariableA(LPCSTR lpName, LPCSTR lpValue);
LPSTR WINAPI GetCommandLineA(void);
LPWSTR WINAPI GetCommandLineW(void);
LPVOID WINAPI GetEnvironmentStringsA(void);
BOOL WINAPI FreeEnvironmentStringsA(LPSTR lpszEnvironmentBlock);

/* System Info */
void WINAPI GetSystemInfo(LPVOID lpSystemInfo);
void WINAPI GetNativeSystemInfo(LPVOID lpSystemInfo);
DWORD WINAPI GetVersion(void);
BOOL WINAPI GetVersionExA(LPVOID lpVersionInformation);
DWORD WINAPI GetTickCount(void);
ULONGLONG WINAPI GetTickCount64(void);
void WINAPI GetSystemTime(LPSYSTEMTIME lpSystemTime);
void WINAPI GetLocalTime(LPSYSTEMTIME lpLocalTime);
BOOL WINAPI QueryPerformanceCounter(PLARGE_INTEGER lpPerformanceCount);
BOOL WINAPI QueryPerformanceFrequency(PLARGE_INTEGER lpFrequency);

/* String */
int WINAPI lstrlenA(LPCSTR lpString);
int WINAPI lstrlenW(LPCWSTR lpString);
LPSTR WINAPI lstrcpyA(LPSTR lpString1, LPCSTR lpString2);
LPSTR WINAPI lstrcatA(LPSTR lpString1, LPCSTR lpString2);
int WINAPI lstrcmpA(LPCSTR lpString1, LPCSTR lpString2);
int WINAPI lstrcmpiA(LPCSTR lpString1, LPCSTR lpString2);
int WINAPI MultiByteToWideChar(UINT CodePage, DWORD dwFlags, LPCSTR lpMultiByteStr, int cbMultiByte, LPWSTR lpWideCharStr, int cchWideChar);
int WINAPI WideCharToMultiByte(UINT CodePage, DWORD dwFlags, LPCWSTR lpWideCharStr, int cchWideChar, LPSTR lpMultiByteStr, int cbMultiByte, LPCSTR lpDefaultChar, LPBOOL lpUsedDefaultChar);

#define CP_ACP   0
#define CP_UTF8  65001
#define CP_UTF16 1200

/* Console */
HANDLE WINAPI GetStdHandle(DWORD nStdHandle);
BOOL WINAPI SetStdHandle(DWORD nStdHandle, HANDLE hHandle);
BOOL WINAPI WriteConsoleA(HANDLE hConsoleOutput, LPCVOID lpBuffer, DWORD nNumberOfCharsToWrite, LPDWORD lpNumberOfCharsWritten, LPVOID lpReserved);
BOOL WINAPI ReadConsoleA(HANDLE hConsoleInput, LPVOID lpBuffer, DWORD nNumberOfCharsToRead, LPDWORD lpNumberOfCharsRead, LPVOID lpReserved);
BOOL WINAPI AllocConsole(void);
BOOL WINAPI FreeConsole(void);
BOOL WINAPI SetConsoleTitleA(LPCSTR lpConsoleTitle);

#define STD_INPUT_HANDLE  ((DWORD)-10)
#define STD_OUTPUT_HANDLE ((DWORD)-11)
#define STD_ERROR_HANDLE  ((DWORD)-12)

/* Debug */
void WINAPI OutputDebugStringA(LPCSTR lpOutputString);
void WINAPI OutputDebugStringW(LPCWSTR lpOutputString);
BOOL WINAPI IsDebuggerPresent(void);
void WINAPI DebugBreak(void);

/* Interlocked */
LONG WINAPI InterlockedIncrement(LONG volatile* Addend);
LONG WINAPI InterlockedDecrement(LONG volatile* Addend);
LONG WINAPI InterlockedExchange(LONG volatile* Target, LONG Value);
LONG WINAPI InterlockedCompareExchange(LONG volatile* Destination, LONG Exchange, LONG Comparand);
LONG WINAPI InterlockedExchangeAdd(LONG volatile* Addend, LONG Value);

/* TLS */
DWORD WINAPI TlsAlloc(void);
BOOL WINAPI TlsFree(DWORD dwTlsIndex);
LPVOID WINAPI TlsGetValue(DWORD dwTlsIndex);
BOOL WINAPI TlsSetValue(DWORD dwTlsIndex, LPVOID lpTlsValue);

#define TLS_OUT_OF_INDEXES ((DWORD)0xFFFFFFFF)

/* ============================================================
 * Unicode Macros
 * ============================================================ */

#ifdef UNICODE
#define CreateFile CreateFileW
#define LoadLibrary LoadLibraryW
#define GetModuleHandle GetModuleHandleW
#define GetModuleFileName GetModuleFileNameW
#define GetFileAttributes GetFileAttributesW
#define DeleteFile DeleteFileW
#define lstrlen lstrlenW
#define GetCommandLine GetCommandLineW
#define OutputDebugString OutputDebugStringW
#else
#define CreateFile CreateFileA
#define LoadLibrary LoadLibraryA
#define GetModuleHandle GetModuleHandleA
#define GetModuleFileName GetModuleFileNameA
#define GetFileAttributes GetFileAttributesA
#define DeleteFile DeleteFileA
#define lstrlen lstrlenA
#define GetCommandLine GetCommandLineA
#define OutputDebugString OutputDebugStringA
#endif

/* ============================================================
 * Macros
 * ============================================================ */

#define MAKEWORD(a, b)  ((WORD)(((BYTE)(a)) | ((WORD)((BYTE)(b))) << 8))
#define MAKELONG(a, b)  ((LONG)(((WORD)(a)) | ((DWORD)((WORD)(b))) << 16))
#define LOWORD(l)       ((WORD)(l))
#define HIWORD(l)       ((WORD)(((DWORD)(l) >> 16) & 0xFFFF))
#define LOBYTE(w)       ((BYTE)(w))
#define HIBYTE(w)       ((BYTE)(((WORD)(w) >> 8) & 0xFF))

#define MAX_PATH 260

#define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#define FAILED(hr)    (((HRESULT)(hr)) < 0)

typedef LONG HRESULT;
#define S_OK     ((HRESULT)0)
#define S_FALSE  ((HRESULT)1)
#define E_FAIL   ((HRESULT)0x80004005)
#define E_NOTIMPL ((HRESULT)0x80004001)
#define E_OUTOFMEMORY ((HRESULT)0x8007000E)
#define E_INVALIDARG ((HRESULT)0x80070057)
#define E_NOINTERFACE ((HRESULT)0x80004002)
#define E_POINTER ((HRESULT)0x80004003)
#define E_UNEXPECTED ((HRESULT)0x8000FFFF)

#endif /* _ADEAD_WINDOWS_H */
