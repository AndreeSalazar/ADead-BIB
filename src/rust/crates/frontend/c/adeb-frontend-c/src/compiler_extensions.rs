// ============================================================
// ADead-BIB — C Compiler Extensions (GCC + MSVC)
// ============================================================
// Handles compiler-specific extensions that appear in real-world
// C99/C11 code targeting GCC or MSVC:
//
//   GCC:  __attribute__((X)), __asm__, typeof, __alignof__,
//         __builtin_*, GNU statement expressions ({ ... })
//
//   MSVC: __declspec(X), __cdecl, __stdcall, __fastcall,
//         __int8/16/32/64, __assume, __debugbreak,
//         __volatile, __restrict, __unaligned
//
//   Both: alternate keywords __inline__, __volatile__, __const__
//
// The preprocessor / lexer calls into this module when it sees
// double-underscore-prefixed identifiers.
//
// Windows-compatible C headers are also declared here so that
// c_stdlib.rs can include them via `get_header()`.
// ============================================================

// ── Compiler extension keywords ─────────────────────────────────────────────

/// Test whether an identifier is a GCC keyword extension.
pub fn is_gcc_keyword(kw: &str) -> bool {
    matches!(
        kw,
        "__asm__"
            | "__asm"
            | "asm"
            | "__volatile__"
            | "__volatile"
            | "__inline__"
            | "__inline"
            | "__restrict"
            | "__restrict__"
            | "__const__"
            | "__signed__"
            | "__signed"
            | "__typeof__"
            | "__typeof"
            | "typeof"
            | "__alignof__"
            | "__alignof"
            | "__extension__"
            | "__attribute__"
            | "__attribute"
            | "__builtin_va_list"
            | "__gnuc_va_list"
    )
}

/// Test whether an identifier is an MSVC keyword extension.
pub fn is_msvc_keyword(kw: &str) -> bool {
    matches!(
        kw,
        "__cdecl"
            | "__stdcall"
            | "__fastcall"
            | "__thiscall"
            | "__vectorcall"
            | "__clrcall"
            | "__forceinline"
            | "__declspec"
            | "__int8"
            | "__int16"
            | "__int32"
            | "__int64"
            | "__int128"
            | "__w64"
            | "__ptr32"
            | "__ptr64"
            | "__unaligned"
            | "__assume"
            | "__debugbreak"
            | "__noop"
            | "__cpuid"
            | "__cpuidex"
            | "__rdtsc"
            | "__rdtscp"
            | "__pragma"
            | "_Pragma"
    )
}

/// Normalise alternate keywords to their canonical C equivalent.
///
/// Returns `Some(canonical)` if the keyword should be replaced, else `None`.
pub fn normalize_keyword(kw: &str) -> Option<&'static str> {
    match kw {
        "__inline__" | "__inline" | "__forceinline" => Some("inline"),
        "__volatile__" | "__volatile" => Some("volatile"),
        "__const__" | "__const" => Some("const"),
        "__signed__" | "__signed" => Some("signed"),
        "__restrict__" | "__restrict" => Some("restrict"),
        "__typeof__" | "__typeof" | "typeof" => Some("__typeof__"),
        "__alignof__" | "__alignof" | "_Alignof" => Some("_Alignof"),
        "__extension__" => Some(""), // skip
        "__int8" => Some("signed char"),
        "__int16" => Some("short"),
        "__int32" => Some("int"),
        "__int64" => Some("long long"),
        "wchar_t" => Some("unsigned short"),
        _ => None,
    }
}

// ── Windows-compatible C headers ─────────────────────────────────────────────

/// Minimal `windows.h` stub — declares the most commonly used Win32 types
/// and functions so that C code targeting Windows can compile.
pub const HEADER_WINDOWS: &str = r#"
/* windows.h — ADead-BIB built-in stub */
typedef void*          HANDLE;
typedef void*          HMODULE;
typedef void*          HINSTANCE;
typedef void*          HWND;
typedef void*          HDC;
typedef void*          HGDIOBJ;
typedef void*          LPVOID;
typedef const void*    LPCVOID;
typedef char*          LPSTR;
typedef const char*    LPCSTR;
typedef unsigned short LPWSTR;
typedef const unsigned short* LPCWSTR;
typedef unsigned long  DWORD;
typedef unsigned short WORD;
typedef unsigned char  BYTE;
typedef int            BOOL;
typedef long           LONG;
typedef long long      LONGLONG;
typedef unsigned long  ULONG;
typedef unsigned long long ULONGLONG;
typedef unsigned long  UINT;
typedef unsigned long long UINT64;
typedef long           HRESULT;
typedef long long      INT_PTR;
typedef unsigned long long UINT_PTR;
typedef long long      LONG_PTR;
typedef unsigned long long ULONG_PTR;
typedef unsigned long long SIZE_T;
typedef long long      SSIZE_T;

typedef struct _SECURITY_ATTRIBUTES {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
} SECURITY_ATTRIBUTES, *LPSECURITY_ATTRIBUTES;

typedef struct _OVERLAPPED {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    DWORD Offset;
    DWORD OffsetHigh;
    HANDLE hEvent;
} OVERLAPPED, *LPOVERLAPPED;

typedef struct _FILETIME {
    DWORD dwLowDateTime;
    DWORD dwHighDateTime;
} FILETIME, *LPFILETIME;

typedef struct _SYSTEMTIME {
    WORD wYear, wMonth, wDayOfWeek, wDay;
    WORD wHour, wMinute, wSecond, wMilliseconds;
} SYSTEMTIME, *LPSYSTEMTIME;

/* Kernel32 */
HANDLE CreateFileA(LPCSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE);
BOOL   CloseHandle(HANDLE);
BOOL   ReadFile(HANDLE, LPVOID, DWORD, DWORD*, LPOVERLAPPED);
BOOL   WriteFile(HANDLE, LPCVOID, DWORD, DWORD*, LPOVERLAPPED);
DWORD  GetLastError(void);
void   SetLastError(DWORD);
LPVOID VirtualAlloc(LPVOID, SIZE_T, DWORD, DWORD);
BOOL   VirtualFree(LPVOID, SIZE_T, DWORD);
BOOL   VirtualProtect(LPVOID, SIZE_T, DWORD, DWORD*);
HANDLE GetCurrentProcess(void);
HANDLE GetCurrentThread(void);
DWORD  GetCurrentProcessId(void);
DWORD  GetCurrentThreadId(void);
void   ExitProcess(UINT);
void   ExitThread(DWORD);
HMODULE LoadLibraryA(LPCSTR);
BOOL    FreeLibrary(HMODULE);
void*   GetProcAddress(HMODULE, LPCSTR);
void    Sleep(DWORD);
DWORD   WaitForSingleObject(HANDLE, DWORD);
HANDLE  CreateThread(LPSECURITY_ATTRIBUTES, SIZE_T, void*, LPVOID, DWORD, DWORD*);
HANDLE  CreateMutexA(LPSECURITY_ATTRIBUTES, BOOL, LPCSTR);
BOOL    ReleaseMutex(HANDLE);
void    GetSystemTimeAsFileTime(LPFILETIME);
DWORD   GetTickCount(void);
BOOL    QueryPerformanceCounter(long long*);
BOOL    QueryPerformanceFrequency(long long*);
void*   HeapAlloc(HANDLE, DWORD, SIZE_T);
HANDLE  GetProcessHeap(void);
BOOL    HeapFree(HANDLE, DWORD, void*);
void    OutputDebugStringA(LPCSTR);
int     MultiByteToWideChar(UINT, DWORD, LPCSTR, int, unsigned short*, int);
int     WideCharToMultiByte(UINT, DWORD, const unsigned short*, int, LPSTR, int, LPCSTR, BOOL*);

/* Constants */
#define INVALID_HANDLE_VALUE ((HANDLE)(long long)(-1))
#define TRUE  1
#define FALSE 0
#define NULL  ((void*)0)
/* File access */
#define GENERIC_READ    0x80000000UL
#define GENERIC_WRITE   0x40000000UL
#define FILE_SHARE_READ 0x00000001UL
#define CREATE_ALWAYS   2
#define OPEN_EXISTING   3
#define FILE_ATTRIBUTE_NORMAL 0x00000080UL
/* Virtual memory */
#define MEM_COMMIT      0x1000
#define MEM_RESERVE     0x2000
#define MEM_RELEASE     0x8000
#define PAGE_READWRITE  0x04
#define PAGE_EXECUTE_READWRITE 0x40
/* Wait */
#define INFINITE        0xFFFFFFFFUL
#define WAIT_OBJECT_0   0x00000000UL
#define WAIT_TIMEOUT    0x00000102UL
"#;

/// `winnt.h` — core NT types and constants.
pub const HEADER_WINNT: &str = r#"
/* winnt.h — ADead-BIB built-in stub */
typedef unsigned char  UCHAR;
typedef unsigned short USHORT;
typedef unsigned long  ULONG;
typedef long           NTSTATUS;
typedef void*          PVOID;
typedef char           CHAR;
typedef short          SHORT;

typedef struct _UNICODE_STRING {
    unsigned short Length;
    unsigned short MaximumLength;
    unsigned short* Buffer;
} UNICODE_STRING, *PUNICODE_STRING;

typedef struct _LIST_ENTRY {
    struct _LIST_ENTRY *Flink;
    struct _LIST_ENTRY *Blink;
} LIST_ENTRY, *PLIST_ENTRY;

#define STATUS_SUCCESS          ((NTSTATUS)0x00000000L)
#define STATUS_UNSUCCESSFUL     ((NTSTATUS)0xC0000001L)
#define STATUS_NOT_IMPLEMENTED  ((NTSTATUS)0xC0000002L)
#define STATUS_ACCESS_DENIED    ((NTSTATUS)0xC0000022L)
"#;

/// `windef.h` — fundamental Windows type definitions.
pub const HEADER_WINDEF: &str = r#"
/* windef.h — ADead-BIB built-in stub */
typedef unsigned char  BYTE;
typedef unsigned short WORD;
typedef unsigned long  DWORD;
typedef int            BOOL;
typedef unsigned int   UINT;
typedef void*          HANDLE;
typedef void*          LPVOID;
typedef const void*    LPCVOID;
typedef char*          LPSTR;
typedef const char*    LPCSTR;
typedef long           LONG;

#define WINAPI   __stdcall
#define CALLBACK __stdcall
#define APIENTRY __stdcall
"#;

/// `intrin.h` — MSVC intrinsic function declarations.
pub const HEADER_INTRIN: &str = r#"
/* intrin.h — ADead-BIB built-in stub */
void   __debugbreak(void);
void   __noop(void);
long long __rdtsc(void);
void   __cpuid(int cpuInfo[4], int function_id);
void   __cpuidex(int cpuInfo[4], int function_id, int subfunction_id);
void  *_AddressOfReturnAddress(void);
void  *_ReturnAddress(void);
unsigned char  _BitScanForward(unsigned long *index, unsigned long mask);
unsigned char  _BitScanReverse(unsigned long *index, unsigned long mask);
unsigned char  _BitScanForward64(unsigned long *index, unsigned long long mask);
unsigned char  _BitScanReverse64(unsigned long *index, unsigned long long mask);
unsigned short __bswap_16(unsigned short x);
unsigned int   _byteswap_ulong(unsigned long x);
unsigned long long _byteswap_uint64(unsigned long long x);
unsigned char  _rotl8(unsigned char value, unsigned char shift);
unsigned short _rotl16(unsigned short value, unsigned char shift);
unsigned int   _rotl(unsigned int value, int shift);
unsigned long long _rotl64(unsigned long long value, int shift);
long _InterlockedIncrement(volatile long *p);
long _InterlockedDecrement(volatile long *p);
long _InterlockedExchange(volatile long *p, long val);
long _InterlockedCompareExchange(volatile long *p, long exchange, long comparand);
"#;

/// `immintrin.h` / AVX/SSE — SIMD intrinsics stub.
pub const HEADER_SIMD_INTRIN: &str = r#"
/* immintrin.h — ADead-BIB built-in SIMD stub */
typedef float  __m128  __attribute__((vector_size(16)));
typedef double __m128d __attribute__((vector_size(16)));
typedef long long __m128i __attribute__((vector_size(16)));
typedef float  __m256  __attribute__((vector_size(32)));
typedef double __m256d __attribute__((vector_size(32)));
typedef long long __m256i __attribute__((vector_size(32)));
typedef float  __m512  __attribute__((vector_size(64)));
typedef double __m512d __attribute__((vector_size(64)));
typedef long long __m512i __attribute__((vector_size(64)));

__m128  _mm_add_ps(__m128 a, __m128 b);
__m128  _mm_sub_ps(__m128 a, __m128 b);
__m128  _mm_mul_ps(__m128 a, __m128 b);
__m128  _mm_div_ps(__m128 a, __m128 b);
__m128d _mm_add_pd(__m128d a, __m128d b);
__m128d _mm_mul_pd(__m128d a, __m128d b);
__m128  _mm_load_ps(const float *p);
void    _mm_store_ps(float *p, __m128 a);
__m128d _mm_load_pd(const double *p);
void    _mm_store_pd(double *p, __m128d a);
__m256  _mm256_add_ps(__m256 a, __m256 b);
__m256  _mm256_mul_ps(__m256 a, __m256 b);
__m256d _mm256_add_pd(__m256d a, __m256d b);
__m256  _mm256_load_ps(const float *p);
void    _mm256_store_ps(float *p, __m256 a);
void    _mm_sfence(void);
void    _mm_lfence(void);
void    _mm_mfence(void);
"#;

/// `complex.h` — C99 complex numbers.
pub const HEADER_COMPLEX: &str = r#"
/* complex.h — C99 */
typedef double _Complex double_complex;
typedef float  _Complex float_complex;

double _Complex cadd(double _Complex a, double _Complex b);
double _Complex csub(double _Complex a, double _Complex b);
double _Complex cmul(double _Complex a, double _Complex b);
double _Complex cdiv(double _Complex a, double _Complex b);
double creal(double _Complex z);
double cimag(double _Complex z);
double cabs(double _Complex z);
double carg(double _Complex z);
double _Complex conj(double _Complex z);
double _Complex csqrt(double _Complex z);
double _Complex cexp(double _Complex z);
double _Complex clog(double _Complex z);
double _Complex cpow(double _Complex x, double _Complex y);
double _Complex csin(double _Complex z);
double _Complex ccos(double _Complex z);
"#;

/// `wchar.h` — wide character I/O and string functions.
pub const HEADER_WCHAR: &str = r#"
/* wchar.h */
typedef unsigned int wchar_t;
typedef unsigned long wint_t;
typedef void* mbstate_t;

int wprintf(const wchar_t *format, ...);
int wscanf(const wchar_t *format, ...);
wchar_t *wcscpy(wchar_t *dest, const wchar_t *src);
wchar_t *wcsncpy(wchar_t *dest, const wchar_t *src, size_t n);
wchar_t *wcscat(wchar_t *dest, const wchar_t *src);
size_t  wcslen(const wchar_t *s);
int     wcscmp(const wchar_t *s1, const wchar_t *s2);
int     wcsncmp(const wchar_t *s1, const wchar_t *s2, size_t n);
wchar_t *wcschr(const wchar_t *s, wchar_t c);
wchar_t *wcsstr(const wchar_t *haystack, const wchar_t *needle);
long    wcstol(const wchar_t *nptr, wchar_t **endptr, int base);
double  wcstod(const wchar_t *nptr, wchar_t **endptr);
size_t  mbstowcs(wchar_t *dest, const char *src, size_t n);
size_t  wcstombs(char *dest, const wchar_t *src, size_t n);
int     mbtowc(wchar_t *pwc, const char *s, size_t n);
int     wctomb(char *s, wchar_t wchar);
wint_t  btowc(int c);
int     wctob(wint_t c);
"#;

/// `uchar.h` — C11 Unicode character types.
pub const HEADER_UCHAR: &str = r#"
/* uchar.h — C11 */
typedef unsigned short char16_t;
typedef unsigned int   char32_t;

size_t mbrtoc16(char16_t *pc16, const char *s, size_t n, mbstate_t *ps);
size_t c16rtomb(char *s, char16_t c16, mbstate_t *ps);
size_t mbrtoc32(char32_t *pc32, const char *s, size_t n, mbstate_t *ps);
size_t c32rtomb(char *s, char32_t c32, mbstate_t *ps);
"#;

/// `wctype.h` — wide character classification.
pub const HEADER_WCTYPE: &str = r#"
/* wctype.h */
int iswalpha(int c);
int iswdigit(int c);
int iswalnum(int c);
int iswspace(int c);
int iswupper(int c);
int iswlower(int c);
int iswprint(int c);
int iswpunct(int c);
int iswxdigit(int c);
int towupper(int c);
int towlower(int c);
"#;

/// `tgmath.h` — type-generic math (C99).
pub const HEADER_TGMATH: &str = r#"
/* tgmath.h — type-generic, maps to math.h for doubles */
#define sin(x)  sin(x)
#define cos(x)  cos(x)
#define tan(x)  tan(x)
#define sqrt(x) sqrt(x)
#define pow(x,y) pow(x,y)
#define fabs(x) fabs(x)
#define exp(x)  exp(x)
#define log(x)  log(x)
#define ceil(x) ceil(x)
#define floor(x) floor(x)
#define round(x) round(x)
"#;

// ══════════════════════════════════════════════════════════════
// DirectX / DXGI / HLSL — Built-in C header stubs
// ══════════════════════════════════════════════════════════════

/// `d3dcompiler.h` — HLSL shader compiler.
pub const HEADER_D3DCOMPILER: &str = r#"
/* d3dcompiler.h — ADead-BIB built-in stub */
typedef void* ID3DBlob;

/* Compile flags */
#define D3DCOMPILE_DEBUG                          0x00000001
#define D3DCOMPILE_SKIP_VALIDATION                0x00000002
#define D3DCOMPILE_SKIP_OPTIMIZATION              0x00000004
#define D3DCOMPILE_PACK_MATRIX_ROW_MAJOR          0x00000008
#define D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR       0x00000010
#define D3DCOMPILE_ENABLE_STRICTNESS              0x00000800
#define D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY 0x00001000
#define D3DCOMPILE_OPTIMIZATION_LEVEL0            0x00004000
#define D3DCOMPILE_OPTIMIZATION_LEVEL1            0x00000000
#define D3DCOMPILE_OPTIMIZATION_LEVEL2            0x0000C000
#define D3DCOMPILE_OPTIMIZATION_LEVEL3            0x00008000
#define D3DCOMPILE_WARNINGS_ARE_ERRORS            0x00040000
#define D3D_COMPILE_STANDARD_FILE_INCLUDE         ((void*)1)

HRESULT D3DCompile(const void *pSrcData, unsigned long long SrcDataSize,
    const char *pSourceName, const void *pDefines, void *pInclude,
    const char *pEntrypoint, const char *pTarget, unsigned int Flags1,
    unsigned int Flags2, ID3DBlob **ppCode, ID3DBlob **ppErrorMsgs);
HRESULT D3DCompile2(const void *pSrcData, unsigned long long SrcDataSize,
    const char *pSourceName, const void *pDefines, void *pInclude,
    const char *pEntrypoint, const char *pTarget, unsigned int Flags1,
    unsigned int Flags2, unsigned int SecondaryDataFlags,
    const void *pSecondaryData, unsigned long long SecondaryDataSize,
    ID3DBlob **ppCode, ID3DBlob **ppErrorMsgs);
HRESULT D3DCreateBlob(unsigned long long Size, ID3DBlob **ppBlob);
HRESULT D3DDisassemble(const void *pSrcData, unsigned long long SrcDataSize,
    unsigned int Flags, const char *szComments, ID3DBlob **ppDisassembly);
HRESULT D3DReflect(const void *pSrcData, unsigned long long SrcDataSize,
    const void *pInterface, void **ppReflector);
HRESULT D3DReadFileToBlob(const unsigned short *pFileName, ID3DBlob **ppContents);
HRESULT D3DWriteBlobToFile(ID3DBlob *pBlob, const unsigned short *pFileName, int bOverwrite);

/* ID3DBlob vtable access macros */
#define ID3DBlob_GetBufferPointer(blob) ((void*(*)(void*))(*((void***)blob))[3])(blob)
#define ID3DBlob_GetBufferSize(blob)    ((unsigned long long(*)(void*))(*((void***)blob))[4])(blob)
#define ID3DBlob_Release(blob)          ((unsigned long(*)(void*))(*((void***)blob))[2])(blob)
"#;

/// `dxgi.h` — DirectX Graphics Infrastructure.
pub const HEADER_DXGI: &str = r#"
/* dxgi.h — ADead-BIB built-in stub */

/* Base types needed by all DX */
typedef struct _GUID { unsigned long Data1; unsigned short Data2; unsigned short Data3; unsigned char Data4[8]; } GUID;
typedef GUID IID;
typedef const GUID* REFIID;
typedef const GUID* REFCLSID;

typedef unsigned int DXGI_FORMAT;

typedef struct DXGI_RATIONAL {
    unsigned int Numerator;
    unsigned int Denominator;
} DXGI_RATIONAL;

typedef struct DXGI_SAMPLE_DESC {
    unsigned int Count;
    unsigned int Quality;
} DXGI_SAMPLE_DESC;

typedef struct DXGI_MODE_DESC {
    unsigned int Width;
    unsigned int Height;
    DXGI_RATIONAL RefreshRate;
    DXGI_FORMAT Format;
    unsigned int ScanlineOrdering;
    unsigned int Scaling;
} DXGI_MODE_DESC;

typedef struct DXGI_SWAP_CHAIN_DESC {
    DXGI_MODE_DESC BufferDesc;
    DXGI_SAMPLE_DESC SampleDesc;
    unsigned int BufferUsage;
    unsigned int BufferCount;
    HWND OutputWindow;
    BOOL Windowed;
    unsigned int SwapEffect;
    unsigned int Flags;
} DXGI_SWAP_CHAIN_DESC;

typedef struct DXGI_SWAP_CHAIN_DESC1 {
    unsigned int Width;
    unsigned int Height;
    DXGI_FORMAT Format;
    BOOL Stereo;
    DXGI_SAMPLE_DESC SampleDesc;
    unsigned int BufferUsage;
    unsigned int BufferCount;
    unsigned int Scaling;
    unsigned int SwapEffect;
    unsigned int AlphaMode;
    unsigned int Flags;
} DXGI_SWAP_CHAIN_DESC1;

typedef struct DXGI_ADAPTER_DESC1 {
    unsigned short Description[128];
    unsigned int VendorId;
    unsigned int DeviceId;
    unsigned int SubSysId;
    unsigned int Revision;
    unsigned long long DedicatedVideoMemory;
    unsigned long long DedicatedSystemMemory;
    unsigned long long SharedSystemMemory;
    long long AdapterLuid_LowPart;
    long AdapterLuid_HighPart;
    unsigned int Flags;
} DXGI_ADAPTER_DESC1;

/* DXGI Format constants */
#define DXGI_FORMAT_UNKNOWN              0
#define DXGI_FORMAT_R32G32B32A32_FLOAT   2
#define DXGI_FORMAT_R32G32B32_FLOAT      6
#define DXGI_FORMAT_R8G8B8A8_UNORM      28
#define DXGI_FORMAT_B8G8R8A8_UNORM      87
#define DXGI_FORMAT_D32_FLOAT           40
#define DXGI_FORMAT_D24_UNORM_S8_UINT   45
#define DXGI_FORMAT_R32_FLOAT           41
#define DXGI_FORMAT_R32_UINT            42
#define DXGI_FORMAT_R16_UINT            57
#define DXGI_FORMAT_R32G32_FLOAT        16

/* DXGI swap effect */
#define DXGI_SWAP_EFFECT_DISCARD          0
#define DXGI_SWAP_EFFECT_SEQUENTIAL       1
#define DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL  3
#define DXGI_SWAP_EFFECT_FLIP_DISCARD     4

/* DXGI usage */
#define DXGI_USAGE_RENDER_TARGET_OUTPUT  0x00000020
#define DXGI_USAGE_SHADER_INPUT          0x00000010
#define DXGI_CREATE_FACTORY_DEBUG        0x01
#define DXGI_MWA_NO_ALT_ENTER            0x2
#define DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE 2

/* DXGI factory functions */
HRESULT CreateDXGIFactory(const IID *riid, void **ppFactory);
HRESULT CreateDXGIFactory1(const IID *riid, void **ppFactory);
HRESULT CreateDXGIFactory2(unsigned int Flags, const IID *riid, void **ppFactory);

/* IUnknown vtable macros */
#define IUnknown_QueryInterface(p,riid,ppv) ((HRESULT(*)(void*,const IID*,void**))(*((void***)p))[0])(p,riid,ppv)
#define IUnknown_AddRef(p)                  ((unsigned long(*)(void*))(*((void***)p))[1])(p)
#define IUnknown_Release(p)                 ((unsigned long(*)(void*))(*((void***)p))[2])(p)

/* DXGI vtable macros */
#define IDXGIFactory_EnumAdapters(f,i,pp)        ((HRESULT(*)(void*,unsigned int,void**))(*((void***)f))[7])(f,i,pp)
#define IDXGIFactory_CreateSwapChain(f,d,desc,pp) ((HRESULT(*)(void*,void*,void*,void**))(*((void***)f))[10])(f,d,desc,pp)
#define IDXGIFactory_MakeWindowAssociation(f,w,fl) ((HRESULT(*)(void*,HWND,unsigned int))(*((void***)f))[8])(f,w,fl)
#define IDXGISwapChain_Present(s,si,fl)          ((HRESULT(*)(void*,unsigned int,unsigned int))(*((void***)s))[8])(s,si,fl)
#define IDXGISwapChain_GetBuffer(s,b,riid,pp)    ((HRESULT(*)(void*,unsigned int,const IID*,void**))(*((void***)s))[9])(s,b,riid,pp)
#define IDXGISwapChain_ResizeBuffers(s,c,w,h,f,fl) ((HRESULT(*)(void*,unsigned int,unsigned int,unsigned int,unsigned int,unsigned int))(*((void***)s))[13])(s,c,w,h,f,fl)
#define IDXGISwapChain3_GetCurrentBackBufferIndex(s) ((unsigned int(*)(void*))(*((void***)s))[36])(s)
"#;

/// `d3d9.h` — Direct3D 9 API.
pub const HEADER_D3D9: &str = r#"
/* d3d9.h — ADead-BIB built-in stub */
typedef unsigned int D3DFORMAT;
typedef unsigned int D3DDEVTYPE;
typedef unsigned int D3DPRIMITIVETYPE;

typedef struct D3DPRESENT_PARAMETERS {
    unsigned int BackBufferWidth;
    unsigned int BackBufferHeight;
    D3DFORMAT BackBufferFormat;
    unsigned int BackBufferCount;
    unsigned int MultiSampleType;
    unsigned long MultiSampleQuality;
    unsigned int SwapEffect;
    HWND hDeviceWindow;
    BOOL Windowed;
    BOOL EnableAutoDepthStencil;
    D3DFORMAT AutoDepthStencilFormat;
    unsigned long Flags;
    unsigned int FullScreen_RefreshRateInHz;
    unsigned int PresentationInterval;
} D3DPRESENT_PARAMETERS;

typedef struct D3DVIEWPORT9 {
    unsigned long X, Y, Width, Height;
    float MinZ, MaxZ;
} D3DVIEWPORT9;

typedef struct D3DMATRIX {
    float m[4][4];
} D3DMATRIX;

typedef struct D3DVECTOR { float x, y, z; } D3DVECTOR;
typedef struct D3DCOLORVALUE { float r, g, b, a; } D3DCOLORVALUE;

typedef struct D3DMATERIAL9 {
    D3DCOLORVALUE Diffuse, Ambient, Specular, Emissive;
    float Power;
} D3DMATERIAL9;

typedef struct D3DLIGHT9 {
    unsigned int Type;
    D3DCOLORVALUE Diffuse, Specular, Ambient;
    D3DVECTOR Position, Direction;
    float Range, Falloff, Attenuation0, Attenuation1, Attenuation2, Theta, Phi;
} D3DLIGHT9;

typedef struct D3DLOCKED_RECT { int Pitch; void *pBits; } D3DLOCKED_RECT;

/* Constants */
#define D3D_SDK_VERSION                  32
#define D3DADAPTER_DEFAULT               0
#define D3DCREATE_HARDWARE_VERTEXPROCESSING 0x00000040
#define D3DCREATE_SOFTWARE_VERTEXPROCESSING 0x00000020
#define D3DSWAPEFFECT_DISCARD            1
#define D3DDEVTYPE_HAL                   1
#define D3DFMT_UNKNOWN                   0
#define D3DFMT_A8R8G8B8                  21
#define D3DFMT_X8R8G8B8                  22
#define D3DFMT_D16                       80
#define D3DFMT_D24S8                     75
#define D3DPT_TRIANGLELIST               4
#define D3DPT_TRIANGLESTRIP              5
#define D3DCLEAR_TARGET                  0x00000001
#define D3DCLEAR_ZBUFFER                 0x00000002
#define D3DRS_ZENABLE                    7
#define D3DRS_LIGHTING                   137
#define D3DRS_CULLMODE                   22
#define D3DTS_WORLD                      256
#define D3DTS_VIEW                       2
#define D3DTS_PROJECTION                 3
#define D3DFVF_XYZ                       0x002
#define D3DFVF_DIFFUSE                   0x040
#define D3DFVF_NORMAL                    0x010
#define D3DPOOL_DEFAULT                  0
#define D3DPOOL_MANAGED                  1
#define D3DUSAGE_WRITEONLY               0x00000008
#define D3DCOLOR_XRGB(r,g,b)            ((0xFF<<24)|((r)<<16)|((g)<<8)|(b))

/* D3D9 entry */
void* Direct3DCreate9(unsigned int SDKVersion);

/* IDirect3D9 vtable macros */
#define IDirect3D9_Release(p)           IUnknown_Release(p)
#define IDirect3D9_CreateDevice(p,a,dt,fw,bf,pp,dev) ((HRESULT(*)(void*,unsigned int,unsigned int,HWND,unsigned long,D3DPRESENT_PARAMETERS*,void**))(*((void***)p))[16])(p,a,dt,fw,bf,pp,dev)

/* IDirect3DDevice9 vtable macros */
#define IDirect3DDevice9_Release(p)     IUnknown_Release(p)
#define IDirect3DDevice9_Clear(p,c,r,f,col,z,s) ((HRESULT(*)(void*,unsigned long,const void*,unsigned long,unsigned long,float,unsigned long))(*((void***)p))[43])(p,c,r,f,col,z,s)
#define IDirect3DDevice9_BeginScene(p)  ((HRESULT(*)(void*))(*((void***)p))[41])(p)
#define IDirect3DDevice9_EndScene(p)    ((HRESULT(*)(void*))(*((void***)p))[42])(p)
#define IDirect3DDevice9_Present(p,s,d,w,r) ((HRESULT(*)(void*,const void*,const void*,HWND,const void*))(*((void***)p))[17])(p,s,d,w,r)
#define IDirect3DDevice9_SetRenderState(p,st,val) ((HRESULT(*)(void*,unsigned int,unsigned long))(*((void***)p))[57])(p,st,val)
#define IDirect3DDevice9_SetFVF(p,fvf)  ((HRESULT(*)(void*,unsigned long))(*((void***)p))[89])(p,fvf)
#define IDirect3DDevice9_DrawPrimitiveUP(p,pt,pc,pv,vs) ((HRESULT(*)(void*,unsigned int,unsigned int,const void*,unsigned int))(*((void***)p))[83])(p,pt,pc,pv,vs)
#define IDirect3DDevice9_SetTransform(p,st,m) ((HRESULT(*)(void*,unsigned int,const D3DMATRIX*))(*((void***)p))[44])(p,st,m)
"#;

/// `d3d11.h` — Direct3D 11 API.
pub const HEADER_D3D11: &str = r#"
/* d3d11.h — ADead-BIB built-in stub */
typedef void* ID3D11Device;
typedef void* ID3D11DeviceContext;
typedef void* ID3D11RenderTargetView;
typedef void* ID3D11DepthStencilView;
typedef void* ID3D11Buffer;
typedef void* ID3D11Texture2D;
typedef void* ID3D11VertexShader;
typedef void* ID3D11PixelShader;
typedef void* ID3D11InputLayout;
typedef void* ID3D11ShaderResourceView;
typedef void* ID3D11SamplerState;
typedef void* ID3D11BlendState;
typedef void* ID3D11DepthStencilState;
typedef void* ID3D11RasterizerState;

typedef struct D3D11_BUFFER_DESC {
    unsigned int ByteWidth;
    unsigned int Usage;
    unsigned int BindFlags;
    unsigned int CPUAccessFlags;
    unsigned int MiscFlags;
    unsigned int StructureByteStride;
} D3D11_BUFFER_DESC;

typedef struct D3D11_SUBRESOURCE_DATA {
    const void *pSysMem;
    unsigned int SysMemPitch;
    unsigned int SysMemSlicePitch;
} D3D11_SUBRESOURCE_DATA;

typedef struct D3D11_TEXTURE2D_DESC {
    unsigned int Width;
    unsigned int Height;
    unsigned int MipLevels;
    unsigned int ArraySize;
    DXGI_FORMAT Format;
    DXGI_SAMPLE_DESC SampleDesc;
    unsigned int Usage;
    unsigned int BindFlags;
    unsigned int CPUAccessFlags;
    unsigned int MiscFlags;
} D3D11_TEXTURE2D_DESC;

typedef struct D3D11_MAPPED_SUBRESOURCE {
    void *pData;
    unsigned int RowPitch;
    unsigned int DepthPitch;
} D3D11_MAPPED_SUBRESOURCE;

typedef struct D3D11_VIEWPORT {
    float TopLeftX, TopLeftY, Width, Height, MinDepth, MaxDepth;
} D3D11_VIEWPORT;

typedef struct D3D11_INPUT_ELEMENT_DESC {
    const char *SemanticName;
    unsigned int SemanticIndex;
    DXGI_FORMAT Format;
    unsigned int InputSlot;
    unsigned int AlignedByteOffset;
    unsigned int InputSlotClass;
    unsigned int InstanceDataStepRate;
} D3D11_INPUT_ELEMENT_DESC;

typedef struct D3D11_RASTERIZER_DESC {
    unsigned int FillMode;
    unsigned int CullMode;
    BOOL FrontCounterClockwise;
    int DepthBias;
    float DepthBiasClamp;
    float SlopeScaledDepthBias;
    BOOL DepthClipEnable;
    BOOL ScissorEnable;
    BOOL MultisampleEnable;
    BOOL AntialiasedLineEnable;
} D3D11_RASTERIZER_DESC;

typedef struct D3D11_SAMPLER_DESC {
    unsigned int Filter;
    unsigned int AddressU, AddressV, AddressW;
    float MipLODBias;
    unsigned int MaxAnisotropy;
    unsigned int ComparisonFunc;
    float BorderColor[4];
    float MinLOD, MaxLOD;
} D3D11_SAMPLER_DESC;

/* Constants */
#define D3D_DRIVER_TYPE_HARDWARE    1
#define D3D_DRIVER_TYPE_WARP        5
#define D3D_FEATURE_LEVEL_11_0      0xb000
#define D3D_FEATURE_LEVEL_11_1      0xb100
#define D3D11_SDK_VERSION           7
#define D3D11_CREATE_DEVICE_DEBUG   0x2
#define D3D11_BIND_VERTEX_BUFFER    0x1
#define D3D11_BIND_INDEX_BUFFER     0x2
#define D3D11_BIND_CONSTANT_BUFFER  0x4
#define D3D11_BIND_RENDER_TARGET    0x20
#define D3D11_BIND_DEPTH_STENCIL    0x40
#define D3D11_USAGE_DEFAULT         0
#define D3D11_USAGE_IMMUTABLE       1
#define D3D11_USAGE_DYNAMIC         2
#define D3D11_CPU_ACCESS_WRITE      0x10000
#define D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST 4
#define D3D11_FILL_SOLID            3
#define D3D11_CULL_BACK             3
#define D3D11_CULL_NONE             1
#define D3D11_CLEAR_DEPTH           0x1
#define D3D11_APPEND_ALIGNED_ELEMENT 0xFFFFFFFF
#define D3D11_INPUT_PER_VERTEX_DATA 0
#define D3D11_FILTER_MIN_MAG_MIP_LINEAR 0x15
#define D3D11_TEXTURE_ADDRESS_WRAP  1
#define D3D11_TEXTURE_ADDRESS_CLAMP 3

/* D3D11 entry */
HRESULT D3D11CreateDevice(void *pAdapter, unsigned int DriverType,
    HMODULE Software, unsigned int Flags, const unsigned int *pFeatureLevels,
    unsigned int FeatureLevels, unsigned int SDKVersion,
    ID3D11Device **ppDevice, unsigned int *pFeatureLevel,
    ID3D11DeviceContext **ppImmediateContext);
HRESULT D3D11CreateDeviceAndSwapChain(void *pAdapter, unsigned int DriverType,
    HMODULE Software, unsigned int Flags, const unsigned int *pFeatureLevels,
    unsigned int FeatureLevels, unsigned int SDKVersion,
    const DXGI_SWAP_CHAIN_DESC *pSwapChainDesc, void **ppSwapChain,
    ID3D11Device **ppDevice, unsigned int *pFeatureLevel,
    ID3D11DeviceContext **ppImmediateContext);

/* D3D11 Device vtable macros */
#define ID3D11Device_Release(p)         IUnknown_Release(p)
#define ID3D11Device_CreateBuffer(p,d,i,b) ((HRESULT(*)(void*,const D3D11_BUFFER_DESC*,const D3D11_SUBRESOURCE_DATA*,ID3D11Buffer**))(*((void***)p))[3])(p,d,i,b)
#define ID3D11Device_CreateTexture2D(p,d,i,t) ((HRESULT(*)(void*,const D3D11_TEXTURE2D_DESC*,const D3D11_SUBRESOURCE_DATA*,ID3D11Texture2D**))(*((void***)p))[5])(p,d,i,t)
#define ID3D11Device_CreateVertexShader(p,bc,len,cl,vs) ((HRESULT(*)(void*,const void*,unsigned long long,void*,ID3D11VertexShader**))(*((void***)p))[12])(p,bc,len,cl,vs)
#define ID3D11Device_CreatePixelShader(p,bc,len,cl,ps) ((HRESULT(*)(void*,const void*,unsigned long long,void*,ID3D11PixelShader**))(*((void***)p))[15])(p,bc,len,cl,ps)
#define ID3D11Device_CreateInputLayout(p,d,n,bc,len,il) ((HRESULT(*)(void*,const D3D11_INPUT_ELEMENT_DESC*,unsigned int,const void*,unsigned long long,ID3D11InputLayout**))(*((void***)p))[11])(p,d,n,bc,len,il)
#define ID3D11Device_CreateRenderTargetView(p,r,d,v) ((HRESULT(*)(void*,void*,const void*,ID3D11RenderTargetView**))(*((void***)p))[9])(p,r,d,v)

/* D3D11 DeviceContext vtable macros */
#define ID3D11DeviceContext_Release(p)  IUnknown_Release(p)
#define ID3D11DeviceContext_IASetInputLayout(p,il) ((void(*)(void*,void*))(*((void***)p))[17])(p,il)
#define ID3D11DeviceContext_IASetVertexBuffers(p,s,n,b,st,o) ((void(*)(void*,unsigned int,unsigned int,void*const*,const unsigned int*,const unsigned int*))(*((void***)p))[18])(p,s,n,b,st,o)
#define ID3D11DeviceContext_IASetPrimitiveTopology(p,t) ((void(*)(void*,unsigned int))(*((void***)p))[24])(p,t)
#define ID3D11DeviceContext_VSSetShader(p,s,ci,n) ((void(*)(void*,void*,void*const*,unsigned int))(*((void***)p))[11])(p,s,ci,n)
#define ID3D11DeviceContext_PSSetShader(p,s,ci,n) ((void(*)(void*,void*,void*const*,unsigned int))(*((void***)p))[9])(p,s,ci,n)
#define ID3D11DeviceContext_Draw(p,c,s) ((void(*)(void*,unsigned int,unsigned int))(*((void***)p))[13])(p,c,s)
#define ID3D11DeviceContext_DrawIndexed(p,c,si,bo) ((void(*)(void*,unsigned int,unsigned int,int))(*((void***)p))[12])(p,c,si,bo)
#define ID3D11DeviceContext_OMSetRenderTargets(p,n,v,d) ((void(*)(void*,unsigned int,void*const*,void*))(*((void***)p))[33])(p,n,v,d)
#define ID3D11DeviceContext_RSSetViewports(p,n,v) ((void(*)(void*,unsigned int,const D3D11_VIEWPORT*))(*((void***)p))[44])(p,n,v)
#define ID3D11DeviceContext_ClearRenderTargetView(p,v,c) ((void(*)(void*,void*,const float*))(*((void***)p))[50])(p,v,c)
#define ID3D11DeviceContext_ClearDepthStencilView(p,v,f,d,s) ((void(*)(void*,void*,unsigned int,float,unsigned char))(*((void***)p))[53])(p,v,f,d,s)
#define ID3D11DeviceContext_VSSetConstantBuffers(p,s,n,b) ((void(*)(void*,unsigned int,unsigned int,void*const*))(*((void***)p))[7])(p,s,n,b)
#define ID3D11DeviceContext_UpdateSubresource(p,r,si,b,d,rp,dp) ((void(*)(void*,void*,unsigned int,const void*,const void*,unsigned int,unsigned int))(*((void***)p))[48])(p,r,si,b,d,rp,dp)
"#;

/// `d3d12.h` — Direct3D 12 API.
pub const HEADER_D3D12: &str = r#"
/* d3d12.h — ADead-BIB built-in stub */
typedef void* ID3D12Device;
typedef void* ID3D12CommandQueue;
typedef void* ID3D12CommandAllocator;
typedef void* ID3D12GraphicsCommandList;
typedef void* ID3D12PipelineState;
typedef void* ID3D12RootSignature;
typedef void* ID3D12DescriptorHeap;
typedef void* ID3D12Resource;
typedef void* ID3D12Fence;
typedef void* ID3D12Debug;

typedef struct D3D12_CPU_DESCRIPTOR_HANDLE { unsigned long long ptr; } D3D12_CPU_DESCRIPTOR_HANDLE;
typedef struct D3D12_GPU_DESCRIPTOR_HANDLE { unsigned long long ptr; } D3D12_GPU_DESCRIPTOR_HANDLE;

typedef struct D3D12_COMMAND_QUEUE_DESC {
    unsigned int Type;
    int Priority;
    unsigned int Flags;
    unsigned int NodeMask;
} D3D12_COMMAND_QUEUE_DESC;

typedef struct D3D12_DESCRIPTOR_HEAP_DESC {
    unsigned int Type;
    unsigned int NumDescriptors;
    unsigned int Flags;
    unsigned int NodeMask;
} D3D12_DESCRIPTOR_HEAP_DESC;

typedef struct D3D12_HEAP_PROPERTIES {
    unsigned int Type;
    unsigned int CPUPageProperty;
    unsigned int MemoryPoolPreference;
    unsigned int CreationNodeMask;
    unsigned int VisibleNodeMask;
} D3D12_HEAP_PROPERTIES;

typedef struct D3D12_RESOURCE_DESC {
    unsigned int Dimension;
    unsigned long long Alignment;
    unsigned long long Width;
    unsigned int Height;
    unsigned short DepthOrArraySize;
    unsigned short MipLevels;
    DXGI_FORMAT Format;
    DXGI_SAMPLE_DESC SampleDesc;
    unsigned int Layout;
    unsigned int Flags;
} D3D12_RESOURCE_DESC;

typedef struct D3D12_CLEAR_VALUE {
    DXGI_FORMAT Format;
    float Color[4];
} D3D12_CLEAR_VALUE;

typedef struct D3D12_RESOURCE_TRANSITION_BARRIER {
    ID3D12Resource *pResource;
    unsigned int Subresource;
    unsigned int StateBefore;
    unsigned int StateAfter;
} D3D12_RESOURCE_TRANSITION_BARRIER;

typedef struct D3D12_RESOURCE_BARRIER {
    unsigned int Type;
    unsigned int Flags;
    D3D12_RESOURCE_TRANSITION_BARRIER Transition;
} D3D12_RESOURCE_BARRIER;

typedef struct D3D12_VERTEX_BUFFER_VIEW {
    unsigned long long BufferLocation;
    unsigned int SizeInBytes;
    unsigned int StrideInBytes;
} D3D12_VERTEX_BUFFER_VIEW;

typedef struct D3D12_INDEX_BUFFER_VIEW {
    unsigned long long BufferLocation;
    unsigned int SizeInBytes;
    DXGI_FORMAT Format;
} D3D12_INDEX_BUFFER_VIEW;

typedef struct D3D12_INPUT_ELEMENT_DESC {
    const char *SemanticName;
    unsigned int SemanticIndex;
    DXGI_FORMAT Format;
    unsigned int InputSlot;
    unsigned int AlignedByteOffset;
    unsigned int InputSlotClass;
    unsigned int InstanceDataStepRate;
} D3D12_INPUT_ELEMENT_DESC;

typedef struct D3D12_INPUT_LAYOUT_DESC {
    const D3D12_INPUT_ELEMENT_DESC *pInputElementDescs;
    unsigned int NumElements;
} D3D12_INPUT_LAYOUT_DESC;

typedef struct D3D12_SHADER_BYTECODE {
    const void *pShaderBytecode;
    unsigned long long BytecodeLength;
} D3D12_SHADER_BYTECODE;

typedef struct D3D12_VIEWPORT {
    float TopLeftX, TopLeftY, Width, Height, MinDepth, MaxDepth;
} D3D12_VIEWPORT;

typedef struct D3D12_RECT { long left, top, right, bottom; } D3D12_RECT;

typedef struct D3D12_RENDER_TARGET_BLEND_DESC {
    BOOL BlendEnable;
    BOOL LogicOpEnable;
    unsigned int SrcBlend, DestBlend, BlendOp;
    unsigned int SrcBlendAlpha, DestBlendAlpha, BlendOpAlpha;
    unsigned int LogicOp;
    unsigned char RenderTargetWriteMask;
} D3D12_RENDER_TARGET_BLEND_DESC;

typedef struct D3D12_BLEND_DESC {
    BOOL AlphaToCoverageEnable;
    BOOL IndependentBlendEnable;
    D3D12_RENDER_TARGET_BLEND_DESC RenderTarget[8];
} D3D12_BLEND_DESC;

typedef struct D3D12_RASTERIZER_DESC {
    unsigned int FillMode, CullMode;
    BOOL FrontCounterClockwise;
    int DepthBias;
    float DepthBiasClamp, SlopeScaledDepthBias;
    BOOL DepthClipEnable, MultisampleEnable, AntialiasedLineEnable;
    unsigned int ForcedSampleCount, ConservativeRaster;
} D3D12_RASTERIZER_DESC;

typedef struct D3D12_DEPTH_STENCILOP_DESC {
    unsigned int StencilFailOp, StencilDepthFailOp, StencilPassOp, StencilFunc;
} D3D12_DEPTH_STENCILOP_DESC;

typedef struct D3D12_DEPTH_STENCIL_DESC {
    BOOL DepthEnable;
    unsigned int DepthWriteMask, DepthFunc;
    BOOL StencilEnable;
    unsigned char StencilReadMask, StencilWriteMask;
    D3D12_DEPTH_STENCILOP_DESC FrontFace, BackFace;
} D3D12_DEPTH_STENCIL_DESC;

typedef struct D3D12_DESCRIPTOR_RANGE {
    unsigned int RangeType, NumDescriptors, BaseShaderRegister, RegisterSpace;
    unsigned int OffsetInDescriptorsFromTableStart;
} D3D12_DESCRIPTOR_RANGE;

typedef struct D3D12_ROOT_DESCRIPTOR_TABLE {
    unsigned int NumDescriptorRanges;
    const D3D12_DESCRIPTOR_RANGE *pDescriptorRanges;
} D3D12_ROOT_DESCRIPTOR_TABLE;

typedef struct D3D12_ROOT_CONSTANTS {
    unsigned int ShaderRegister, RegisterSpace, Num32BitValues;
} D3D12_ROOT_CONSTANTS;

typedef struct D3D12_ROOT_DESCRIPTOR {
    unsigned int ShaderRegister, RegisterSpace;
} D3D12_ROOT_DESCRIPTOR;

typedef struct D3D12_ROOT_PARAMETER {
    unsigned int ParameterType;
    D3D12_ROOT_DESCRIPTOR_TABLE DescriptorTable;
    unsigned int ShaderVisibility;
} D3D12_ROOT_PARAMETER;

typedef struct D3D12_STATIC_SAMPLER_DESC {
    unsigned int Filter, AddressU, AddressV, AddressW;
    float MipLODBias;
    unsigned int MaxAnisotropy, ComparisonFunc, BorderColor;
    float MinLOD, MaxLOD;
    unsigned int ShaderRegister, RegisterSpace, ShaderVisibility;
} D3D12_STATIC_SAMPLER_DESC;

typedef struct D3D12_ROOT_SIGNATURE_DESC {
    unsigned int NumParameters;
    const D3D12_ROOT_PARAMETER *pParameters;
    unsigned int NumStaticSamplers;
    const D3D12_STATIC_SAMPLER_DESC *pStaticSamplers;
    unsigned int Flags;
} D3D12_ROOT_SIGNATURE_DESC;

typedef struct D3D12_STREAM_OUTPUT_DESC {
    const void *pSODeclaration;
    unsigned int NumEntries;
    const unsigned int *pBufferStrides;
    unsigned int NumStrides;
    unsigned int RasterizedStream;
} D3D12_STREAM_OUTPUT_DESC;

typedef struct D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    ID3D12RootSignature *pRootSignature;
    D3D12_SHADER_BYTECODE VS;
    D3D12_SHADER_BYTECODE PS;
    D3D12_SHADER_BYTECODE DS;
    D3D12_SHADER_BYTECODE HS;
    D3D12_SHADER_BYTECODE GS;
    D3D12_STREAM_OUTPUT_DESC StreamOutput;
    D3D12_BLEND_DESC BlendState;
    unsigned int SampleMask;
    D3D12_RASTERIZER_DESC RasterizerState;
    D3D12_DEPTH_STENCIL_DESC DepthStencilState;
    D3D12_INPUT_LAYOUT_DESC InputLayout;
    unsigned int IBStripCutValue;
    unsigned int PrimitiveTopologyType;
    unsigned int NumRenderTargets;
    DXGI_FORMAT RTVFormats[8];
    DXGI_FORMAT DSVFormat;
    DXGI_SAMPLE_DESC SampleDesc;
    unsigned int NodeMask;
    void *CachedPSO_pCachedBlob;
    unsigned long long CachedPSO_CachedBlobSizeInBytes;
    unsigned int Flags;
} D3D12_GRAPHICS_PIPELINE_STATE_DESC;

typedef struct D3D12_SUBRESOURCE_DATA {
    const void *pData;
    long long RowPitch, SlicePitch;
} D3D12_SUBRESOURCE_DATA;

typedef struct D3D12_RANGE {
    unsigned long long Begin, End;
} D3D12_RANGE;

/* Constants */
#define D3D12_COMMAND_LIST_TYPE_DIRECT       0
#define D3D12_COMMAND_LIST_TYPE_COMPUTE      2
#define D3D12_COMMAND_LIST_TYPE_COPY         3
#define D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV 0
#define D3D12_DESCRIPTOR_HEAP_TYPE_RTV       2
#define D3D12_DESCRIPTOR_HEAP_TYPE_DSV       3
#define D3D12_DESCRIPTOR_HEAP_FLAG_NONE      0
#define D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE 1
#define D3D12_HEAP_TYPE_DEFAULT              1
#define D3D12_HEAP_TYPE_UPLOAD               2
#define D3D12_HEAP_TYPE_READBACK             3
#define D3D12_HEAP_FLAG_NONE                 0
#define D3D12_RESOURCE_STATE_COMMON          0
#define D3D12_RESOURCE_STATE_RENDER_TARGET   0x4
#define D3D12_RESOURCE_STATE_DEPTH_WRITE     0x10
#define D3D12_RESOURCE_STATE_PRESENT         0
#define D3D12_RESOURCE_STATE_GENERIC_READ    0xAC3
#define D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER 0x1
#define D3D12_RESOURCE_STATE_INDEX_BUFFER    0x2
#define D3D12_RESOURCE_STATE_COPY_DEST       0x400
#define D3D12_RESOURCE_BARRIER_TYPE_TRANSITION 0
#define D3D12_RESOURCE_BARRIER_FLAG_NONE     0
#define D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES 0xFFFFFFFF
#define D3D12_RESOURCE_DIMENSION_BUFFER      1
#define D3D12_RESOURCE_DIMENSION_TEXTURE2D   3
#define D3D12_RESOURCE_FLAG_NONE             0
#define D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET 0x1
#define D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL 0x2
#define D3D12_TEXTURE_LAYOUT_UNKNOWN         0
#define D3D12_TEXTURE_LAYOUT_ROW_MAJOR       1
#define D3D12_FENCE_FLAG_NONE                0
#define D3D12_COMMAND_QUEUE_FLAG_NONE        0
#define D3D12_DEFAULT_SAMPLE_MASK            0xFFFFFFFF
#define D3D12_APPEND_ALIGNED_ELEMENT         0xFFFFFFFF
#define D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT 0x1
#define D3D12_ROOT_PARAMETER_TYPE_CBV        2
#define D3D12_DESCRIPTOR_RANGE_TYPE_CBV      2
#define D3D12_DESCRIPTOR_RANGE_TYPE_SRV      0
#define D3D12_SHADER_VISIBILITY_ALL          0
#define D3D12_SHADER_VISIBILITY_VERTEX       1
#define D3D12_SHADER_VISIBILITY_PIXEL        5
#define D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE 3
#define D3D12_FILL_MODE_SOLID                3
#define D3D12_CULL_MODE_NONE                 1
#define D3D12_CULL_MODE_BACK                 3
#define D3D12_DEPTH_WRITE_MASK_ALL           1
#define D3D12_COMPARISON_FUNC_LESS           2
#define D3D12_STENCIL_OP_KEEP                1
#define D3D12_BLEND_ONE                      2
#define D3D12_BLEND_ZERO                     1
#define D3D12_BLEND_OP_ADD                   1
#define D3D12_LOGIC_OP_NOOP                  5
#define D3D12_COLOR_WRITE_ENABLE_ALL         15
#define D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA 0
#define D3D12_CLEAR_FLAG_DEPTH               0x1
#define D3D12_MEMORY_POOL_UNKNOWN            0
#define D3D12_CPU_PAGE_PROPERTY_UNKNOWN      0
#define D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST  4
#define D3D_FEATURE_LEVEL_11_0               0xb000
#define D3D_FEATURE_LEVEL_12_0               0xc000

/* D3D12 entry functions */
HRESULT D3D12CreateDevice(void *pAdapter, unsigned int MinimumFeatureLevel,
    const IID *riid, void **ppDevice);
HRESULT D3D12GetDebugInterface(const IID *riid, void **ppvDebug);
HRESULT D3D12SerializeRootSignature(const D3D12_ROOT_SIGNATURE_DESC *pRootSignature,
    unsigned int Version, ID3DBlob **ppBlob, ID3DBlob **ppErrorBlob);

/* Well-known IIDs */
static const IID IID_ID3D12Device = {0x189819f1,0x1db6,0x4b57,{0xbe,0x54,0x18,0x21,0x33,0x9b,0x85,0xf7}};
static const IID IID_ID3D12CommandQueue = {0x0ec870a6,0x5d7e,0x4c22,{0x8c,0xfc,0x5b,0xaa,0xe0,0x76,0x16,0xed}};
static const IID IID_ID3D12CommandAllocator = {0x6102dee4,0xaf59,0x4b09,{0xb9,0x99,0xb4,0x4d,0x73,0xf0,0x9b,0x24}};
static const IID IID_ID3D12GraphicsCommandList = {0x5b160d0f,0xac1b,0x4185,{0x8b,0xa8,0xb3,0xae,0x42,0xa5,0xa4,0x55}};
static const IID IID_ID3D12PipelineState = {0x765a30f3,0xf624,0x4c6f,{0xa8,0x28,0xac,0xe9,0x48,0x62,0x24,0x45}};
static const IID IID_ID3D12RootSignature = {0xc54a6b66,0x72df,0x4ee8,{0x8b,0xe5,0xa9,0x46,0xa1,0x42,0x92,0x14}};
static const IID IID_ID3D12DescriptorHeap = {0x8efb471d,0x616c,0x4f49,{0x90,0xf7,0x12,0x7b,0xb7,0x63,0xfa,0x51}};
static const IID IID_ID3D12Resource = {0x696442be,0xa72e,0x4059,{0xbc,0x79,0x5b,0x5c,0x98,0x04,0x0f,0xad}};
static const IID IID_ID3D12Fence = {0x0a753dcf,0xc4d8,0x4b91,{0xad,0xf6,0xbe,0x5a,0x60,0xd9,0x5a,0x76}};
static const IID IID_ID3D12Debug = {0x344488b7,0x6846,0x474b,{0xb9,0x89,0xf0,0x27,0x44,0x82,0x45,0xe0}};
static const IID IID_IDXGIFactory4 = {0x1bc6ea02,0xef36,0x464f,{0xbf,0x0c,0x21,0xca,0x39,0xe5,0x16,0x8a}};
static const IID IID_IDXGISwapChain3 = {0x94d99bdb,0xf1f8,0x4ab0,{0xb2,0x36,0x7d,0xa0,0x17,0x0e,0xda,0xb1}};
static const IID IID_IDXGIAdapter1 = {0x29038f61,0x3839,0x4626,{0x91,0xfd,0x08,0x68,0x79,0x01,0x1a,0x05}};

/* D3D12 Device vtable macros */
#define ID3D12Device_Release(p)            IUnknown_Release(p)
#define ID3D12Device_CreateCommandQueue(p,d,riid,pp) ((HRESULT(*)(void*,const D3D12_COMMAND_QUEUE_DESC*,const IID*,void**))(*((void***)p))[8])(p,d,riid,pp)
#define ID3D12Device_CreateCommandAllocator(p,t,riid,pp) ((HRESULT(*)(void*,unsigned int,const IID*,void**))(*((void***)p))[9])(p,t,riid,pp)
#define ID3D12Device_CreateGraphicsPipelineState(p,d,riid,pp) ((HRESULT(*)(void*,const D3D12_GRAPHICS_PIPELINE_STATE_DESC*,const IID*,void**))(*((void***)p))[10])(p,d,riid,pp)
#define ID3D12Device_CreateCommandList(p,nm,t,a,ps,riid,pp) ((HRESULT(*)(void*,unsigned int,unsigned int,void*,void*,const IID*,void**))(*((void***)p))[12])(p,nm,t,a,ps,riid,pp)
#define ID3D12Device_CreateDescriptorHeap(p,d,riid,pp) ((HRESULT(*)(void*,const D3D12_DESCRIPTOR_HEAP_DESC*,const IID*,void**))(*((void***)p))[14])(p,d,riid,pp)
#define ID3D12Device_GetDescriptorHandleIncrementSize(p,t) ((unsigned int(*)(void*,unsigned int))(*((void***)p))[15])(p,t)
#define ID3D12Device_CreateRootSignature(p,nm,b,len,riid,pp) ((HRESULT(*)(void*,unsigned int,const void*,unsigned long long,const IID*,void**))(*((void***)p))[16])(p,nm,b,len,riid,pp)
#define ID3D12Device_CreateRenderTargetView(p,r,d,h) ((void(*)(void*,void*,const void*,D3D12_CPU_DESCRIPTOR_HANDLE))(*((void***)p))[19])(p,r,d,h)
#define ID3D12Device_CreateCommittedResource(p,hp,hf,rd,is,cv,riid,pp) ((HRESULT(*)(void*,const D3D12_HEAP_PROPERTIES*,unsigned int,const D3D12_RESOURCE_DESC*,unsigned int,const D3D12_CLEAR_VALUE*,const IID*,void**))(*((void***)p))[27])(p,hp,hf,rd,is,cv,riid,pp)
#define ID3D12Device_CreateFence(p,iv,f,riid,pp) ((HRESULT(*)(void*,unsigned long long,unsigned int,const IID*,void**))(*((void***)p))[29])(p,iv,f,riid,pp)

/* CommandQueue */
#define ID3D12CommandQueue_Release(p)      IUnknown_Release(p)
#define ID3D12CommandQueue_ExecuteCommandLists(p,n,cl) ((void(*)(void*,unsigned int,void*const*))(*((void***)p))[10])(p,n,cl)
#define ID3D12CommandQueue_Signal(p,f,v)   ((HRESULT(*)(void*,void*,unsigned long long))(*((void***)p))[11])(p,f,v)

/* CommandAllocator */
#define ID3D12CommandAllocator_Release(p)  IUnknown_Release(p)
#define ID3D12CommandAllocator_Reset(p)    ((HRESULT(*)(void*))(*((void***)p))[7])(p)

/* GraphicsCommandList */
#define ID3D12GraphicsCommandList_Release(p) IUnknown_Release(p)
#define ID3D12GraphicsCommandList_Close(p) ((HRESULT(*)(void*))(*((void***)p))[7])(p)
#define ID3D12GraphicsCommandList_Reset(p,a,ps) ((HRESULT(*)(void*,void*,void*))(*((void***)p))[8])(p,a,ps)
#define ID3D12GraphicsCommandList_ClearRenderTargetView(p,h,c,n,r) ((void(*)(void*,D3D12_CPU_DESCRIPTOR_HANDLE,const float*,unsigned int,const void*))(*((void***)p))[47])(p,h,c,n,r)
#define ID3D12GraphicsCommandList_OMSetRenderTargets(p,n,rtv,s,dsv) ((void(*)(void*,unsigned int,const D3D12_CPU_DESCRIPTOR_HANDLE*,BOOL,const D3D12_CPU_DESCRIPTOR_HANDLE*))(*((void***)p))[46])(p,n,rtv,s,dsv)
#define ID3D12GraphicsCommandList_RSSetViewports(p,n,v) ((void(*)(void*,unsigned int,const D3D12_VIEWPORT*))(*((void***)p))[44])(p,n,v)
#define ID3D12GraphicsCommandList_RSSetScissorRects(p,n,r) ((void(*)(void*,unsigned int,const D3D12_RECT*))(*((void***)p))[45])(p,n,r)
#define ID3D12GraphicsCommandList_IASetPrimitiveTopology(p,t) ((void(*)(void*,unsigned int))(*((void***)p))[38])(p,t)
#define ID3D12GraphicsCommandList_IASetVertexBuffers(p,s,n,v) ((void(*)(void*,unsigned int,unsigned int,const D3D12_VERTEX_BUFFER_VIEW*))(*((void***)p))[39])(p,s,n,v)
#define ID3D12GraphicsCommandList_IASetIndexBuffer(p,v) ((void(*)(void*,const D3D12_INDEX_BUFFER_VIEW*))(*((void***)p))[40])(p,v)
#define ID3D12GraphicsCommandList_DrawInstanced(p,vc,ic,sv,si) ((void(*)(void*,unsigned int,unsigned int,unsigned int,unsigned int))(*((void***)p))[12])(p,vc,ic,sv,si)
#define ID3D12GraphicsCommandList_DrawIndexedInstanced(p,ic,inc,si,bv,sii) ((void(*)(void*,unsigned int,unsigned int,unsigned int,int,unsigned int))(*((void***)p))[13])(p,ic,inc,si,bv,sii)
#define ID3D12GraphicsCommandList_ResourceBarrier(p,n,b) ((void(*)(void*,unsigned int,const D3D12_RESOURCE_BARRIER*))(*((void***)p))[25])(p,n,b)
#define ID3D12GraphicsCommandList_SetGraphicsRootSignature(p,rs) ((void(*)(void*,void*))(*((void***)p))[30])(p,rs)
#define ID3D12GraphicsCommandList_SetPipelineState(p,ps) ((void(*)(void*,void*))(*((void***)p))[24])(p,ps)
#define ID3D12GraphicsCommandList_SetDescriptorHeaps(p,n,h) ((void(*)(void*,unsigned int,void*const*))(*((void***)p))[31])(p,n,h)
#define ID3D12GraphicsCommandList_SetGraphicsRoot32BitConstants(p,i,n,d,o) ((void(*)(void*,unsigned int,unsigned int,const void*,unsigned int))(*((void***)p))[34])(p,i,n,d,o)

/* DescriptorHeap */
#define ID3D12DescriptorHeap_Release(p)    IUnknown_Release(p)

/* Fence */
#define ID3D12Fence_Release(p)             IUnknown_Release(p)
#define ID3D12Fence_GetCompletedValue(p)   ((unsigned long long(*)(void*))(*((void***)p))[8])(p)
#define ID3D12Fence_SetEventOnCompletion(p,v,e) ((HRESULT(*)(void*,unsigned long long,HANDLE))(*((void***)p))[9])(p,v,e)

/* Resource */
#define ID3D12Resource_Release(p)          IUnknown_Release(p)
#define ID3D12Resource_Map(p,s,r,d)        ((HRESULT(*)(void*,unsigned int,const D3D12_RANGE*,void**))(*((void***)p))[8])(p,s,r,d)
#define ID3D12Resource_Unmap(p,s,r)        ((void(*)(void*,unsigned int,const D3D12_RANGE*))(*((void***)p))[9])(p,s,r)
#define ID3D12Resource_GetGPUVirtualAddress(p) ((unsigned long long(*)(void*))(*((void***)p))[10])(p)

/* Debug */
#define ID3D12Debug_Release(p)             IUnknown_Release(p)
#define ID3D12Debug_EnableDebugLayer(p)    ((void(*)(void*))(*((void***)p))[3])(p)

/* Misc */
#define ID3D12PipelineState_Release(p)     IUnknown_Release(p)
#define ID3D12RootSignature_Release(p)     IUnknown_Release(p)
"#;
