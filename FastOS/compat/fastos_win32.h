/*
 * FastOS v2.2 — Compatibility Layer: Win32 Subset
 * compat/fastos_win32.h
 *
 * Traduce las llamadas Win32 IMPORTANTES a syscalls FastOS nativas.
 * Sin registry. Sin COM. Sin DLL hell. Sin telemetría. Sin WMI.
 *
 * App Windows original:
 *   #include <windows.h>
 *   HANDLE f = CreateFile("data.bin", GENERIC_READ, ...);
 *
 * Con FastOS compat layer:
 *   #include <fastos_win32.h>     // reemplaza windows.h
 *   HANDLE f = CreateFile("data.bin", GENERIC_READ, ...);
 *   // ADead-BIB traduce automáticamente a fs_open()
 *
 * Resultado: binario FastOS puro. Sin DLL. Sin CRT.
 *
 * Compilar con ADead-BIB:
 *   adb cc app_win32.c --target fastos -o app.po
 *
 * Autor: Eddi Andreé Salazar Matos — Perú — GPL v2
 * ADead-BIB — Binary Is Binary — Po:506F4F53 — BG:APPROVE
 */

#ifndef _FASTOS_WIN32_H
#define _FASTOS_WIN32_H

#include "fastos_syscall.h"
#include "fastos_stdlib.h"

/* ══════════════════════════════════════════════════════
 * § 1. Win32 Type Definitions
 *
 * Only the types that matter. No GUID madness.
 * ══════════════════════════════════════════════════════ */

typedef void          *HANDLE;
typedef void          *LPVOID;
typedef void          *PVOID;
typedef const void    *LPCVOID;
typedef unsigned int   DWORD;
typedef unsigned short WORD;
typedef unsigned char  BYTE;
typedef int            BOOL;
typedef int            INT;
typedef unsigned int   UINT;
typedef long           LONG;
typedef unsigned long  ULONG;
typedef long long      LONGLONG;
typedef char          *LPSTR;
typedef const char    *LPCSTR;
typedef DWORD         *LPDWORD;
typedef size_t         SIZE_T;

#define TRUE   1
#define FALSE  0
#define INVALID_HANDLE_VALUE ((HANDLE)(intptr_t)-1)
#define INFINITE 0xFFFFFFFF

/* ══════════════════════════════════════════════════════
 * § 2. Win32 File API → fs_open / fs_read / fs_write / fs_close
 *
 * CRÍTICO — These are the most used Win32 calls.
 * ══════════════════════════════════════════════════════ */

/* Access flags */
#define GENERIC_READ    0x80000000
#define GENERIC_WRITE   0x40000000
#define GENERIC_ALL     0xC0000000

/* Share mode (ignored — FastOS single-process for now) */
#define FILE_SHARE_READ   0x00000001
#define FILE_SHARE_WRITE  0x00000002
#define FILE_SHARE_DELETE 0x00000004

/* Creation disposition */
#define CREATE_NEW        1
#define CREATE_ALWAYS     2
#define OPEN_EXISTING     3
#define OPEN_ALWAYS       4
#define TRUNCATE_EXISTING 5

/* File attributes (ignored — FastOS doesn't have Win32 attrs) */
#define FILE_ATTRIBUTE_NORMAL 0x80

/* Seek methods */
#define FILE_BEGIN   0
#define FILE_CURRENT 1
#define FILE_END     2

/*
 * CreateFile → fs_open
 * Maps Win32 access flags to FastOS FS_READ/FS_WRITE/FS_CREATE
 */
static inline HANDLE CreateFileA(
    LPCSTR lpFileName,
    DWORD dwDesiredAccess,
    DWORD dwShareMode,          /* ignored */
    void *lpSecurityAttributes, /* ignored */
    DWORD dwCreationDisposition,
    DWORD dwFlagsAndAttributes, /* ignored */
    HANDLE hTemplateFile        /* ignored */
) {
    uint32_t flags = 0;

    if (dwDesiredAccess & GENERIC_READ)  flags = flags | FS_READ;
    if (dwDesiredAccess & GENERIC_WRITE) flags = flags | FS_WRITE;

    if (dwCreationDisposition == CREATE_NEW ||
        dwCreationDisposition == CREATE_ALWAYS ||
        dwCreationDisposition == OPEN_ALWAYS) {
        flags = flags | FS_CREATE;
    }
    if (dwCreationDisposition == CREATE_ALWAYS ||
        dwCreationDisposition == TRUNCATE_EXISTING) {
        flags = flags | FS_TRUNCATE;
    }

    int fd = fs_open(lpFileName, flags);
    if (fd < 0) return INVALID_HANDLE_VALUE;
    return (HANDLE)(intptr_t)fd;
}

/* CreateFile = CreateFileA (ANSI version) */
#define CreateFile CreateFileA

/*
 * ReadFile → fs_read
 */
static inline BOOL ReadFile(
    HANDLE hFile,
    LPVOID lpBuffer,
    DWORD nNumberOfBytesToRead,
    LPDWORD lpNumberOfBytesRead,
    void *lpOverlapped /* ignored */
) {
    int fd = (int)(intptr_t)hFile;
    ssize_t n = fs_read(fd, lpBuffer, (size_t)nNumberOfBytesToRead);
    if (n < 0) {
        if (lpNumberOfBytesRead) *lpNumberOfBytesRead = 0;
        return FALSE;
    }
    if (lpNumberOfBytesRead) *lpNumberOfBytesRead = (DWORD)n;
    return TRUE;
}

/*
 * WriteFile → fs_write
 */
static inline BOOL WriteFile(
    HANDLE hFile,
    LPCVOID lpBuffer,
    DWORD nNumberOfBytesToWrite,
    LPDWORD lpNumberOfBytesWritten,
    void *lpOverlapped /* ignored */
) {
    int fd = (int)(intptr_t)hFile;
    ssize_t n = fs_write(fd, lpBuffer, (size_t)nNumberOfBytesToWrite);
    if (n < 0) {
        if (lpNumberOfBytesWritten) *lpNumberOfBytesWritten = 0;
        return FALSE;
    }
    if (lpNumberOfBytesWritten) *lpNumberOfBytesWritten = (DWORD)n;
    return TRUE;
}

/*
 * CloseHandle → fs_close
 */
static inline BOOL CloseHandle(HANDLE hObject) {
    int fd = (int)(intptr_t)hObject;
    int ret = fs_close(fd);
    return (ret == FASTOS_OK) ? TRUE : FALSE;
}

/*
 * SetFilePointer → fs_seek
 */
static inline DWORD SetFilePointer(
    HANDLE hFile,
    LONG lDistanceToMove,
    LONG *lpDistanceToMoveHigh, /* ignored for now */
    DWORD dwMoveMethod
) {
    int fd = (int)(intptr_t)hFile;
    int whence;
    if (dwMoveMethod == FILE_BEGIN)   whence = FS_SEEK_SET;
    else if (dwMoveMethod == FILE_CURRENT) whence = FS_SEEK_CUR;
    else whence = FS_SEEK_END;

    int64_t pos = fs_seek(fd, (int64_t)lDistanceToMove, whence);
    if (pos < 0) return 0xFFFFFFFF;
    return (DWORD)pos;
}

/* ══════════════════════════════════════════════════════
 * § 3. Win32 Memory API → mem_alloc / mem_free / mem_map
 *
 * CRÍTICO — VirtualAlloc is the main Win32 memory API.
 * ══════════════════════════════════════════════════════ */

/* Allocation types */
#define MEM_COMMIT   0x00001000
#define MEM_RESERVE  0x00002000
#define MEM_RELEASE  0x00008000

/* Page protection (mapped to FastOS MEM_READ/WRITE/EXEC) */
#define PAGE_NOACCESS          0x01
#define PAGE_READONLY          0x02
#define PAGE_READWRITE         0x04
#define PAGE_EXECUTE           0x10
#define PAGE_EXECUTE_READ      0x20
#define PAGE_EXECUTE_READWRITE 0x40

/*
 * VirtualAlloc → mem_map / mem_alloc
 */
static inline LPVOID VirtualAlloc(
    LPVOID lpAddress,
    SIZE_T dwSize,
    DWORD flAllocationType,
    DWORD flProtect
) {
    uint32_t prot = 0;
    if (flProtect & (PAGE_READONLY | PAGE_READWRITE |
                     PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE))
        prot = prot | MEM_READ;
    if (flProtect & (PAGE_READWRITE | PAGE_EXECUTE_READWRITE))
        prot = prot | MEM_WRITE;
    if (flProtect & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE))
        prot = prot | MEM_EXEC;

    if (lpAddress == NULL) {
        return mem_alloc(dwSize);
    }
    return mem_map(lpAddress, dwSize, prot);
}

/*
 * VirtualFree → mem_free / mem_unmap
 */
static inline BOOL VirtualFree(
    LPVOID lpAddress,
    SIZE_T dwSize,
    DWORD dwFreeType
) {
    if (dwFreeType & MEM_RELEASE) {
        mem_free(lpAddress);
        return TRUE;
    }
    int ret = mem_unmap(lpAddress, dwSize);
    return (ret == FASTOS_OK) ? TRUE : FALSE;
}

/*
 * HeapAlloc → mem_alloc (simplified)
 */
static inline LPVOID HeapAlloc(HANDLE hHeap, DWORD dwFlags, SIZE_T dwBytes) {
    return mem_alloc(dwBytes);
}

/*
 * HeapFree → mem_free
 */
static inline BOOL HeapFree(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem) {
    mem_free(lpMem);
    return TRUE;
}

/*
 * GetProcessHeap — returns dummy handle (FastOS has one global heap)
 */
static inline HANDLE GetProcessHeap(void) {
    return (HANDLE)(intptr_t)1;
}

/* ══════════════════════════════════════════════════════
 * § 4. Win32 System Info → sys_info / CPUID nativo
 *
 * IMPORTANTE — GetSystemInfo, QueryPerformanceCounter
 * ══════════════════════════════════════════════════════ */

typedef struct {
    WORD  wProcessorArchitecture;
    WORD  wReserved;
    DWORD dwPageSize;
    LPVOID lpMinimumApplicationAddress;
    LPVOID lpMaximumApplicationAddress;
    DWORD dwActiveProcessorMask;
    DWORD dwNumberOfProcessors;
    DWORD dwProcessorType;
    DWORD dwAllocationGranularity;
    WORD  wProcessorLevel;
    WORD  wProcessorRevision;
} SYSTEM_INFO;

#define PROCESSOR_ARCHITECTURE_AMD64 9

static inline void GetSystemInfo(SYSTEM_INFO *lpSystemInfo) {
    fastos_sysinfo_t info;
    sys_info(&info);

    lpSystemInfo->wProcessorArchitecture = PROCESSOR_ARCHITECTURE_AMD64;
    lpSystemInfo->dwPageSize = FASTOS_PAGE_SIZE;
    lpSystemInfo->lpMinimumApplicationAddress = (LPVOID)0x10000;
    lpSystemInfo->lpMaximumApplicationAddress = (LPVOID)0x7FFFFFFFFFFF;
    lpSystemInfo->dwActiveProcessorMask = 1;
    lpSystemInfo->dwNumberOfProcessors = info.cpu_cores;
    lpSystemInfo->dwProcessorType = 8664;  /* AMD64 */
    lpSystemInfo->dwAllocationGranularity = 65536;
    lpSystemInfo->wProcessorLevel = (WORD)info.cpu_family;
    lpSystemInfo->wProcessorRevision = (WORD)info.cpu_model;
}

/*
 * QueryPerformanceCounter → rdtsc
 */
typedef union {
    struct { DWORD LowPart; LONG HighPart; };
    LONGLONG QuadPart;
} LARGE_INTEGER;

static inline BOOL QueryPerformanceCounter(LARGE_INTEGER *lpPerformanceCount) {
    lpPerformanceCount->QuadPart = (LONGLONG)rdtsc();
    return TRUE;
}

static inline BOOL QueryPerformanceFrequency(LARGE_INTEGER *lpFrequency) {
    /* Ryzen 5 5600X TSC ~3.7 GHz nominal */
    lpFrequency->QuadPart = 3700000000LL;
    return TRUE;
}

/* ══════════════════════════════════════════════════════
 * § 5. Win32 Thread API → proc_spawn
 *
 * IMPORTANTE — CreateThread is the main threading call.
 * ══════════════════════════════════════════════════════ */

typedef DWORD (*LPTHREAD_START_ROUTINE)(LPVOID);

static inline HANDLE CreateThread(
    void *lpThreadAttributes,   /* ignored */
    SIZE_T dwStackSize,
    LPTHREAD_START_ROUTINE lpStartAddress,
    LPVOID lpParameter,         /* ignored for now */
    DWORD dwCreationFlags,      /* ignored */
    LPDWORD lpThreadId
) {
    int pid = proc_spawn((void (*)(void))lpStartAddress,
                         dwStackSize == 0 ? 65536 : (uint32_t)dwStackSize);
    if (pid < 0) return NULL;
    if (lpThreadId) *lpThreadId = (DWORD)pid;
    return (HANDLE)(intptr_t)pid;
}

/* ══════════════════════════════════════════════════════
 * § 6. Win32 Misc → FastOS native
 *
 * ÚTIL — Sleep, GetLastError, GetTickCount
 * ══════════════════════════════════════════════════════ */

static int _fastos_last_error = 0;

/*
 * Sleep → time_sleep
 */
static inline void Sleep(DWORD dwMilliseconds) {
    time_sleep(dwMilliseconds);
}

/*
 * GetLastError → FastOS errno
 */
static inline DWORD GetLastError(void) {
    return (DWORD)_fastos_last_error;
}

/*
 * SetLastError
 */
static inline void SetLastError(DWORD dwErrCode) {
    _fastos_last_error = (int)dwErrCode;
}

/*
 * GetTickCount → PIT ticks converted to ms
 */
static inline DWORD GetTickCount(void) {
    fastos_time_t t;
    time_get(&t);
    return (DWORD)(t.seconds * 1000 + t.milliseconds);
}

/* ══════════════════════════════════════════════════════
 * § 7. VirtualAlloc / VirtualFree → kmalloc/kfree
 * ══════════════════════════════════════════════════════ */

#define MEM_COMMIT      0x1000
#define MEM_RESERVE     0x2000
#define MEM_RELEASE     0x8000
#define PAGE_READWRITE  0x04
#define PAGE_EXECUTE_READWRITE 0x40
#define HEAP_ZERO_MEMORY 0x08

static inline LPVOID VirtualAlloc(LPVOID lpAddress, SIZE_T dwSize,
                                   DWORD flAllocationType, DWORD flProtect) {
    (void)lpAddress; (void)flAllocationType; (void)flProtect;
    return heap_alloc((uint32_t)dwSize);
}

static inline BOOL VirtualFree(LPVOID lpAddress, SIZE_T dwSize, DWORD dwFreeType) {
    (void)dwSize; (void)dwFreeType;
    heap_free(lpAddress);
    return TRUE;
}

static inline HANDLE HeapCreate(DWORD flOptions, SIZE_T dwInitialSize, SIZE_T dwMaximumSize) {
    (void)flOptions; (void)dwInitialSize; (void)dwMaximumSize;
    return (HANDLE)1; /* FastOS uses single global heap */
}

static inline LPVOID HeapAlloc(HANDLE hHeap, DWORD dwFlags, SIZE_T dwBytes) {
    (void)hHeap;
    LPVOID p = heap_alloc((uint32_t)dwBytes);
    if (p && (dwFlags & HEAP_ZERO_MEMORY)) {
        char *cp = (char*)p;
        for (SIZE_T i = 0; i < dwBytes; i++) cp[i] = 0;
    }
    return p;
}

static inline BOOL HeapFree(HANDLE hHeap, DWORD dwFlags, LPVOID lpMem) {
    (void)hHeap; (void)dwFlags;
    heap_free(lpMem);
    return TRUE;
}

/* ══════════════════════════════════════════════════════
 * § 8. Console I/O → serial/VGA
 * ══════════════════════════════════════════════════════ */

static inline HANDLE GetStdHandle(DWORD nStdHandle) {
    return (HANDLE)(intptr_t)(nStdHandle + 1);
}

#define STD_INPUT_HANDLE  ((DWORD)-10)
#define STD_OUTPUT_HANDLE ((DWORD)-11)
#define STD_ERROR_HANDLE  ((DWORD)-12)

static inline BOOL WriteConsoleA(HANDLE hOutput, const void *lpBuffer,
                                  DWORD nCharsToWrite, LPDWORD lpCharsWritten,
                                  LPVOID lpReserved) {
    (void)hOutput; (void)lpReserved;
    const char *s = (const char *)lpBuffer;
    for (DWORD i = 0; i < nCharsToWrite; i++) serial_putc(s[i]);
    if (lpCharsWritten) *lpCharsWritten = nCharsToWrite;
    return TRUE;
}

static inline BOOL ReadConsoleA(HANDLE hInput, LPVOID lpBuffer,
                                 DWORD nCharsToRead, LPDWORD lpCharsRead,
                                 LPVOID lpReserved) {
    (void)hInput; (void)lpReserved;
    /* Stub: read from keyboard buffer */
    if (lpCharsRead) *lpCharsRead = 0;
    return TRUE;
}

/* ══════════════════════════════════════════════════════
 * § 9. Process/Thread Info
 * ══════════════════════════════════════════════════════ */

static inline DWORD GetCurrentProcessId(void) { return 1; }
static inline DWORD GetCurrentThreadId(void)  { return 1; }

static inline void ExitProcess(UINT uExitCode) {
    proc_exit((int)uExitCode);
}

/* ══════════════════════════════════════════════════════
 * § 10. Timing — GetSystemTime, QueryPerformanceCounter
 * ══════════════════════════════════════════════════════ */

typedef struct _SYSTEMTIME {
    WORD wYear; WORD wMonth; WORD wDayOfWeek; WORD wDay;
    WORD wHour; WORD wMinute; WORD wSecond; WORD wMilliseconds;
} SYSTEMTIME, *LPSYSTEMTIME;

typedef union _LARGE_INTEGER {
    struct { DWORD LowPart; LONG HighPart; };
    LONGLONG QuadPart;
} LARGE_INTEGER;

static inline void GetSystemTime(LPSYSTEMTIME lpST) {
    fastos_time_t t;
    time_get(&t);
    lpST->wYear = 2026; lpST->wMonth = 1; lpST->wDay = 1;
    lpST->wDayOfWeek = 0;
    lpST->wHour = (WORD)(t.seconds / 3600);
    lpST->wMinute = (WORD)((t.seconds % 3600) / 60);
    lpST->wSecond = (WORD)(t.seconds % 60);
    lpST->wMilliseconds = (WORD)t.milliseconds;
}

static inline BOOL QueryPerformanceCounter(LARGE_INTEGER *lpPC) {
    fastos_time_t t;
    time_get(&t);
    lpPC->QuadPart = (LONGLONG)t.seconds * 1000000LL + (LONGLONG)t.milliseconds * 1000LL;
    return TRUE;
}

static inline BOOL QueryPerformanceFrequency(LARGE_INTEGER *lpFreq) {
    lpFreq->QuadPart = 1000000LL; /* 1 MHz */
    return TRUE;
}

/* ══════════════════════════════════════════════════════
 * § 11. String Conversion (stubs)
 * ══════════════════════════════════════════════════════ */

#define CP_UTF8 65001

static inline int MultiByteToWideChar(UINT cp, DWORD flags, LPCSTR lpMB,
                                       int cbMB, void *lpWC, int cchWC) {
    (void)cp; (void)flags; (void)lpMB; (void)cbMB; (void)lpWC; (void)cchWC;
    return 0; /* Stub — FastOS uses UTF-8 natively */
}

static inline int WideCharToMultiByte(UINT cp, DWORD flags, const void *lpWC,
                                       int cchWC, LPSTR lpMB, int cbMB,
                                       LPCSTR lpDef, BOOL *lpUsed) {
    (void)cp; (void)flags; (void)lpWC; (void)cchWC;
    (void)lpMB; (void)cbMB; (void)lpDef; (void)lpUsed;
    return 0; /* Stub — FastOS uses UTF-8 natively */
}

/* ══════════════════════════════════════════════════════
 * § 12. Registry (stubs → ENOSYS)
 * ══════════════════════════════════════════════════════ */

typedef HANDLE HKEY;
#define HKEY_LOCAL_MACHINE ((HKEY)(intptr_t)0x80000002)
#define HKEY_CURRENT_USER  ((HKEY)(intptr_t)0x80000001)
#define ERROR_SUCCESS      0L
#define ERROR_FILE_NOT_FOUND 2L

static inline LONG RegOpenKeyExA(HKEY hKey, LPCSTR lpSubKey, DWORD ulOptions,
                                  DWORD samDesired, HKEY *phkResult) {
    (void)hKey; (void)lpSubKey; (void)ulOptions; (void)samDesired; (void)phkResult;
    return FASTOS_ENOSYS;
}

static inline LONG RegQueryValueExA(HKEY hKey, LPCSTR lpValueName, LPDWORD lpReserved,
                                     LPDWORD lpType, BYTE *lpData, LPDWORD lpcbData) {
    (void)hKey; (void)lpValueName; (void)lpReserved;
    (void)lpType; (void)lpData; (void)lpcbData;
    return FASTOS_ENOSYS;
}

static inline LONG RegCloseKey(HKEY hKey) {
    (void)hKey;
    return FASTOS_ENOSYS;
}

/* ══════════════════════════════════════════════════════
 * § 13. IGNORADOS — Basura Win32 que NO se traduce
 *
 * COM, DCOM, WMI, telemetría → IGNORAR
 * LoadLibrary → FUTURO (Po loader)
 * ══════════════════════════════════════════════════════ */

/* These are defined as no-ops or errors to prevent silent bugs */
#define RegOpenKeyExA(...)   FASTOS_ENOSYS
#define RegCloseKey(...)     FASTOS_ENOSYS
#define CoInitialize(...)    FASTOS_ENOSYS
#define CoCreateInstance(...) FASTOS_ENOSYS
#define LoadLibraryA(...)    NULL  /* FUTURO: Po loader */
#define GetProcAddress(...)  NULL  /* FUTURO: Po loader */
#define FreeLibrary(...)     FALSE

/* Telemetría → BLOCKED by Binary Guardian */
#define WmiOpenBlock(...)    FASTOS_EBGDENY
#define WmiCloseBlock(...)   FASTOS_EBGDENY

#endif /* _FASTOS_WIN32_H */
