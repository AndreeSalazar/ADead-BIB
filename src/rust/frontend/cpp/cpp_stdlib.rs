// ============================================================
// ADead-BIB Built-in C++ Standard Library Headers
// ============================================================
// Provides C++ standard library declarations as built-in strings.
// When #include <iostream> is found, we inject these declarations
// directly — no filesystem, no libstdc++, no libc++ needed.
//
// Strategy: inject only flat C-style declarations that the parser
// already handles. STL types (vector, string, cout) are recognized
// by the parser's type_names set and handled specially during IR
// conversion. The headers only need to declare functions.
//
// ADead-BIB owns the headers. 💀🦈
// ============================================================

/// Common C++ prologue — fundamental types and C-compatible declarations
pub const CPP_COMMON_PROLOGUE: &str = r#"
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;

int printf(const char *format, ...);
int scanf(const char *format, ...);
int sprintf(char *str, const char *format, ...);
int puts(const char *s);
int putchar(int c);
void *malloc(size_t size);
void *calloc(size_t num, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
void *memcpy(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
size_t strlen(const char *s);
int strcmp(const char *s1, const char *s2);
char *strcpy(char *dest, const char *src);
int atoi(const char *s);
double atof(const char *s);
void exit(int status);
void abort();
"#;

/// Look up a C++ header by name and return its declarations.
/// All headers inject flat C-compatible declarations only.
/// STL types are recognized by the parser's type_names prescan.
pub fn get_cpp_header(name: &str) -> Option<&'static str> {
    match name {
        // C++ Standard Library — all map to flat declarations
        "iostream" | "iomanip" | "sstream" | "fstream" => Some(HEADER_IO),
        "string" | "string_view" => Some(HEADER_EMPTY),
        "vector" | "array" | "list" | "deque" | "forward_list" => Some(HEADER_EMPTY),
        "map" | "unordered_map" | "set" | "unordered_set" => Some(HEADER_EMPTY),
        "stack" | "queue" | "span" => Some(HEADER_EMPTY),
        "algorithm" | "numeric" | "ranges" => Some(HEADER_EMPTY),
        "memory" | "functional" | "utility" | "tuple" => Some(HEADER_EMPTY),
        "optional" | "variant" | "any" => Some(HEADER_EMPTY),
        "type_traits" => Some(HEADER_TYPE_TRAITS),
        "limits" | "concepts" => Some(HEADER_EMPTY),
        "chrono" | "thread" | "mutex" | "atomic" | "future" | "condition_variable" => {
            Some(HEADER_EMPTY)
        }
        "initializer_list" | "iterator" => Some(HEADER_EMPTY),
        "stdexcept" | "exception" => Some(HEADER_EMPTY),
        "regex" | "random" | "filesystem" | "format" | "coroutine" | "numbers" | "bit" => {
            Some(HEADER_EMPTY)
        }
        "cassert" => Some(HEADER_EMPTY),
        "cstdio" | "stdio.h" => Some(HEADER_IO),
        "cstdlib" | "stdlib.h" => Some(HEADER_CSTDLIB),
        "cstring" | "string.h" => Some(HEADER_CSTRING),
        "cmath" | "math.h" => Some(HEADER_CMATH),
        "climits" | "cstdint" | "stdint.h" | "inttypes.h" => Some(HEADER_CLIMITS),
        "cstddef" | "stddef.h" => Some(HEADER_EMPTY),

        // ==========================================
        // ADead-BIB v7.0 — header_main.h (HEREDA TODO)
        // ==========================================
        // Un solo include. Todo C + C++ disponible. Sin linker.
        "header_main.h" => Some(HEADER_MAIN_CPP_COMPLETE),

        // ==========================================
        // ADead-BIB v7.0 — fastos C++ headers (aliases)
        // ==========================================
        "fastos_iostream" => Some(HEADER_IO),
        "fastos_vector" => Some(HEADER_EMPTY),
        "fastos_string_cpp" => Some(HEADER_EMPTY),
        "fastos_map" => Some(HEADER_EMPTY),
        "fastos_memory" => Some(HEADER_EMPTY),
        "fastos_algorithm" => Some(HEADER_EMPTY),
        "fastos_functional" => Some(HEADER_EMPTY),
        "fastos_utility" => Some(HEADER_EMPTY),
        "fastos_exception" => Some(HEADER_EMPTY),

        // fastos C headers (C-compatible in C++ mode)
        "fastos_stdio.h" => Some(HEADER_IO),
        "fastos_stdlib.h" => Some(HEADER_CSTDLIB),
        "fastos_string.h" => Some(HEADER_CSTRING),
        "fastos_math.h" => Some(HEADER_CMATH),
        "fastos_types.h" => Some(HEADER_CLIMITS),

        // ==========================================
        // DirectX 12 fastos headers
        // ==========================================
        "fastos_windows.h" => Some(HEADER_FASTOS_WINDOWS),
        "fastos_wrl.h" => Some(HEADER_FASTOS_WRL),
        "fastos_d3d12.h" => Some(HEADER_FASTOS_D3D12),
        "fastos_dxgi.h" => Some(HEADER_FASTOS_DXGI),

        _ => None,
    }
}

/// Check if a symbol name is a known C++ stdlib function/type/class.
/// Uses the stdlib/cpp/ registries for authoritative lookup.
pub fn is_known_cpp_symbol(name: &str) -> bool {
    use crate::stdlib::cpp::fastos_iostream;
    use crate::stdlib::cpp::fastos_vector;
    use crate::stdlib::cpp::fastos_string_cpp;
    use crate::stdlib::cpp::fastos_map;
    use crate::stdlib::cpp::fastos_memory;
    use crate::stdlib::cpp::fastos_algorithm;
    use crate::stdlib::cpp::fastos_functional;
    use crate::stdlib::cpp::fastos_utility;
    use crate::stdlib::cpp::fastos_exceptions;

    fastos_iostream::is_iostream_symbol(name)
        || fastos_vector::is_vector_symbol(name)
        || fastos_string_cpp::is_string_cpp_symbol(name)
        || fastos_map::is_map_symbol(name)
        || fastos_memory::is_memory_symbol(name)
        || fastos_algorithm::is_algorithm_symbol(name)
        || fastos_functional::is_functional_symbol(name)
        || fastos_utility::is_utility_symbol(name)
        || fastos_exceptions::is_exception_symbol(name)
}

// ========================================
// Header constants — flat C-compatible declarations only
// STL types (vector, string, cout, etc.) are recognized by the
// parser's prescan and handled during IR lowering.
// ========================================

/// Empty header — no declarations needed, types recognized by parser
pub const HEADER_EMPTY: &str = "";

/// I/O header — injects printf/scanf/puts
pub const HEADER_IO: &str = r#"
int printf(const char *format, ...);
int scanf(const char *format, ...);
int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);
int puts(const char *s);
int putchar(int c);
int getchar();
"#;

/// <cstdlib> / <stdlib.h>
pub const HEADER_CSTDLIB: &str = r#"
void *malloc(size_t size);
void *calloc(size_t num, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
int atoi(const char *s);
long atol(const char *s);
double atof(const char *s);
void exit(int status);
void abort();
int abs(int x);
long labs(long x);
int rand();
void srand(unsigned int seed);
int system(const char *command);
char *getenv(const char *name);
"#;

/// <cstring> / <string.h>
pub const HEADER_CSTRING: &str = r#"
void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
size_t strlen(const char *s);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);
char *strcat(char *dest, const char *src);
char *strchr(const char *s, int c);
char *strrchr(const char *s, int c);
char *strstr(const char *haystack, const char *needle);
char *strdup(const char *s);
"#;

/// <cmath> / <math.h>
pub const HEADER_CMATH: &str = r#"
double sin(double x);
double cos(double x);
double tan(double x);
double asin(double x);
double acos(double x);
double atan(double x);
double atan2(double y, double x);
double exp(double x);
double log(double x);
double log2(double x);
double log10(double x);
double pow(double base, double exp);
double sqrt(double x);
double cbrt(double x);
double ceil(double x);
double floor(double x);
double round(double x);
double fabs(double x);
double fmod(double x, double y);
double hypot(double x, double y);
int abs(int x);
"#;

/// <climits> / <cstdint>
#[allow(dead_code)]
pub const HEADER_CLIMITS: &str = r#"
typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;
"#;

/// <type_traits> — C++11/14/17/20 type traits
/// ADead-BIB implements these as template structs with static constexpr value.
/// The parser recognizes these as known template types.
pub const HEADER_TYPE_TRAITS: &str = r#"
/* ADead-BIB <type_traits> — C++11/14/17/20 */

/* integral_constant */
template<typename T, T v>
struct integral_constant {
    static constexpr T value = v;
};

typedef integral_constant<bool, true> true_type;
typedef integral_constant<bool, false> false_type;

/* Primary type categories */
template<typename T> struct is_void : false_type {};
template<> struct is_void<void> : true_type {};

template<typename T> struct is_integral : false_type {};
template<> struct is_integral<bool> : true_type {};
template<> struct is_integral<char> : true_type {};
template<> struct is_integral<short> : true_type {};
template<> struct is_integral<int> : true_type {};
template<> struct is_integral<long> : true_type {};

template<typename T> struct is_floating_point : false_type {};
template<> struct is_floating_point<float> : true_type {};
template<> struct is_floating_point<double> : true_type {};

template<typename T> struct is_pointer : false_type {};
template<typename T> struct is_pointer<T*> : true_type {};

template<typename T> struct is_reference : false_type {};
template<typename T> struct is_reference<T&> : true_type {};
template<typename T> struct is_reference<T&&> : true_type {};

template<typename T> struct is_array : false_type {};

template<typename T> struct is_const : false_type {};
template<typename T> struct is_const<const T> : true_type {};

/* Type relationships */
template<typename T, typename U> struct is_same : false_type {};
template<typename T> struct is_same<T, T> : true_type {};

/* Type modifications */
template<typename T> struct remove_const { typedef T type; };
template<typename T> struct remove_const<const T> { typedef T type; };

template<typename T> struct remove_volatile { typedef T type; };
template<typename T> struct remove_volatile<volatile T> { typedef T type; };

template<typename T> struct remove_cv { typedef T type; };
template<typename T> struct remove_cv<const T> { typedef T type; };
template<typename T> struct remove_cv<volatile T> { typedef T type; };
template<typename T> struct remove_cv<const volatile T> { typedef T type; };

template<typename T> struct remove_reference { typedef T type; };
template<typename T> struct remove_reference<T&> { typedef T type; };
template<typename T> struct remove_reference<T&&> { typedef T type; };

template<typename T> struct remove_pointer { typedef T type; };
template<typename T> struct remove_pointer<T*> { typedef T type; };

template<typename T> struct add_pointer { typedef T* type; };
template<typename T> struct add_const { typedef const T type; };
template<typename T> struct add_lvalue_reference { typedef T& type; };
template<typename T> struct add_rvalue_reference { typedef T&& type; };

/* SFINAE helpers */
template<bool B, typename T = void> struct enable_if {};
template<typename T> struct enable_if<true, T> { typedef T type; };

template<bool B, typename T, typename F> struct conditional { typedef T type; };
template<typename T, typename F> struct conditional<false, T, F> { typedef F type; };

/* C++14 _t aliases (template type aliases) */
template<typename T> using remove_const_t = typename remove_const<T>::type;
template<typename T> using remove_volatile_t = typename remove_volatile<T>::type;
template<typename T> using remove_cv_t = typename remove_cv<T>::type;
template<typename T> using remove_reference_t = typename remove_reference<T>::type;
template<typename T> using remove_pointer_t = typename remove_pointer<T>::type;
template<typename T> using add_pointer_t = typename add_pointer<T>::type;
template<typename T> using add_const_t = typename add_const<T>::type;
template<bool B, typename T = void> using enable_if_t = typename enable_if<B, T>::type;
template<bool B, typename T, typename F> using conditional_t = typename conditional<B, T, F>::type;

/* C++17 _v aliases (variable templates) */
template<typename T, typename U> constexpr bool is_same_v = is_same<T, U>::value;
template<typename T> constexpr bool is_integral_v = is_integral<T>::value;
template<typename T> constexpr bool is_floating_point_v = is_floating_point<T>::value;
template<typename T> constexpr bool is_pointer_v = is_pointer<T>::value;
template<typename T> constexpr bool is_reference_v = is_reference<T>::value;
template<typename T> constexpr bool is_void_v = is_void<T>::value;
template<typename T> constexpr bool is_const_v = is_const<T>::value;
template<typename T> constexpr bool is_array_v = is_array<T>::value;

/* void_t (C++17 SFINAE helper) */
template<typename...> using void_t = void;

/* decay — strips references and cv-qualifiers */
template<typename T> struct decay { typedef T type; };
template<typename T> struct decay<T&> { typedef T type; };
template<typename T> struct decay<T&&> { typedef T type; };
template<typename T> struct decay<const T> { typedef T type; };
template<typename T> struct decay<volatile T> { typedef T type; };
template<typename T> using decay_t = typename decay<T>::type;
"#;

// ================================================================
// ADead-BIB v7.0 — header_main.h for C++ (COMPLETE)
// ================================================================
// Includes ALL C declarations + C++ stream/STL type recognition
// Sin linker externo — NUNCA
// ================================================================

const HEADER_MAIN_CPP_COMPLETE: &str = r#"
/* header_main.h — ADead-BIB Universal Header v7.0 (C++ mode) */
/* Un solo include. Todo C + C++ disponible. Sin linker. */

typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef long intptr_t;
typedef unsigned long uintptr_t;

typedef signed char int8_t;
typedef short int16_t;
typedef int int32_t;
typedef long int64_t;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long uint64_t;

/* C Standard Library (available in C++ mode) */
int printf(const char *format, ...);
int scanf(const char *format, ...);
int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);
int puts(const char *s);
int putchar(int c);
int getchar();

void *malloc(size_t size);
void *calloc(size_t num, size_t size);
void *realloc(void *ptr, size_t size);
void free(void *ptr);
int atoi(const char *s);
long atol(const char *s);
double atof(const char *s);
void exit(int status);
void abort();
int abs(int x);
int rand();
void srand(unsigned int seed);

void *memcpy(void *dest, const void *src, size_t n);
void *memmove(void *dest, const void *src, size_t n);
void *memset(void *s, int c, size_t n);
int memcmp(const void *s1, const void *s2, size_t n);
size_t strlen(const char *s);
int strcmp(const char *s1, const char *s2);
char *strcpy(char *dest, const char *src);
char *strncpy(char *dest, const char *src, size_t n);
char *strcat(char *dest, const char *src);
char *strchr(const char *s, int c);
char *strstr(const char *haystack, const char *needle);
char *strdup(const char *s);

double sin(double x);
double cos(double x);
double tan(double x);
double sqrt(double x);
double pow(double base, double exp);
double log(double x);
double log2(double x);
double log10(double x);
double exp(double x);
double ceil(double x);
double floor(double x);
double round(double x);
double fabs(double x);
double fmod(double x, double y);
double atan2(double y, double x);

/* C++ STL types are recognized by parser prescan. */
/* std::cout, std::cin, std::string, std::vector<T>, etc. */
/* No declarations needed — handled during IR lowering. */

/* TREE SHAKING: ADead-BIB includes only what you use. */
/* std::cout << "Hello" → only cout implementation in binary. */

/* === DirectX 12 Headers (fastos) === */
#include <fastos_windows.h>
#include <fastos_wrl.h>
#include <fastos_d3d12.h>
#include <fastos_dxgi.h>
"#;

// ================================================================
// fastos_windows.h — Windows API types and macros
// ================================================================
const HEADER_FASTOS_WINDOWS: &str = r#"
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef unsigned int UINT;
typedef unsigned long DWORD;
typedef unsigned long long UINT64;
typedef unsigned long long ULONG_PTR;
typedef unsigned long long SIZE_T;
typedef unsigned long long ULONGLONG;
typedef unsigned char UINT8;
typedef unsigned short UINT16;
typedef unsigned int UINT32;
typedef int INT;
typedef long LONG;
typedef long long LONGLONG;
typedef long long INT64;
typedef long HRESULT;
typedef int BOOL;
typedef void* HANDLE;
typedef void* HWND;
typedef void* HINSTANCE;
typedef void* HMODULE;
typedef void* HDC;
typedef void* HICON;
typedef void* HCURSOR;
typedef void* HBRUSH;
typedef void* HMENU;
typedef void* HMONITOR;
typedef void* LPVOID;
typedef const void* LPCVOID;
typedef char* LPSTR;
typedef const char* LPCSTR;
typedef wchar_t* LPWSTR;
typedef const wchar_t* LPCWSTR;
typedef wchar_t WCHAR;
typedef float FLOAT;

struct GUID {
    DWORD Data1;
    DWORD Data2_3;
    DWORD Data4_lo;
    DWORD Data4_hi;
};
typedef GUID IID;
typedef const GUID* REFGUID;
typedef const IID* REFIID;

struct LARGE_INTEGER {
    LONGLONG QuadPart;
};

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

struct SECURITY_ATTRIBUTES {
    DWORD nLength;
    LPVOID lpSecurityDescriptor;
    BOOL bInheritHandle;
};

struct IUnknown {
    virtual UINT AddRef() = 0;
    virtual UINT Release() = 0;
    virtual HRESULT QueryInterface(REFIID riid, void** ppvObject) = 0;
};

typedef ULONG_PTR WPARAM;
typedef LONG LPARAM;
typedef LONG LRESULT;
typedef void* WNDPROC;

// ANSI window class (same layout as Rust windows-rs WNDCLASSEXA)
// Total size = 80 bytes on x64 (MSVC ABI)
struct WNDCLASSEXA {
    UINT cbSize;
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
    HICON hIconSm;
};

struct WNDCLASSEXW {
    UINT cbSize;
    UINT style;
    WNDPROC lpfnWndProc;
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

typedef const char* PCSTR;
typedef const wchar_t* PCWSTR;

inline LONG HIWORD(ULONG_PTR l) { return (LONG)((l >> 16) & 0xFFFF); }
inline LONG LOWORD(ULONG_PTR l) { return (LONG)(l & 0xFFFF); }

int SUCCEEDED(HRESULT hr) { return hr >= 0; }
int FAILED(HRESULT hr) { return hr < 0; }

// Win32 API function declarations — ANSI (A) + Wide (W) variants
// Inspired by Rust windows-rs: prefer A variants for simple string handling
extern "C" {
// kernel32.dll
HMODULE GetModuleHandleA(LPCSTR lpModuleName);
HMODULE GetModuleHandleW(LPCWSTR lpModuleName);
void ExitProcess(UINT uExitCode);
HANDLE CreateEventA(void* lpSecurity, BOOL bManualReset, BOOL bInitialState, LPCSTR lpName);
// msvcrt.dll
void* memset(void* dest, int c, int count);
// user32.dll — ANSI
UINT RegisterClassExA(const WNDCLASSEXA* lpwcx);
HWND CreateWindowExA(DWORD dwExStyle, LPCSTR lpClassName, LPCSTR lpWindowName, DWORD dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, HMENU hMenu, HINSTANCE hInstance, LPVOID lpParam);
LRESULT DefWindowProcA(HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
BOOL PeekMessageA(MSG* lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax, UINT wRemoveMsg);
LRESULT DispatchMessageA(const MSG* lpMsg);
// user32.dll — Wide
UINT RegisterClassExW(const WNDCLASSEXW* lpwcx);
HWND CreateWindowExW(DWORD dwExStyle, LPCWSTR lpClassName, LPCWSTR lpWindowName, DWORD dwStyle, int X, int Y, int nWidth, int nHeight, HWND hWndParent, HMENU hMenu, HINSTANCE hInstance, LPVOID lpParam);
LRESULT DefWindowProcW(HWND hWnd, UINT Msg, WPARAM wParam, LPARAM lParam);
BOOL GetMessageW(MSG* lpMsg, HWND hWnd, UINT wMsgFilterMin, UINT wMsgFilterMax);
LRESULT DispatchMessageW(const MSG* lpMsg);
// user32.dll — shared
BOOL ShowWindow(HWND hWnd, int nCmdShow);
BOOL UpdateWindow(HWND hWnd);
BOOL TranslateMessage(const MSG* lpMsg);
void PostQuitMessage(int nExitCode);
HCURSOR LoadCursorW(HINSTANCE hInstance, int lpCursorName);
BOOL AdjustWindowRect(RECT* lpRect, DWORD dwStyle, BOOL bMenu);
HDC GetDC(HWND hWnd);
int ReleaseDC(HWND hWnd, HDC hDC);
BOOL InvalidateRect(HWND hWnd, const RECT* lpRect, BOOL bErase);
int FillRect(HDC hDC, const RECT* lprc, HBRUSH hbr);
// gdi32.dll — GDI rendering
typedef void* HPEN;
typedef void* HGDIOBJ;
typedef unsigned int COLORREF;
COLORREF SetPixel(HDC hdc, int x, int y, COLORREF color);
HBRUSH CreateSolidBrush(COLORREF color);
BOOL DeleteObject(HGDIOBJ ho);
HGDIOBJ SelectObject(HDC hdc, HGDIOBJ h);
BOOL Rectangle(HDC hdc, int left, int top, int right, int bottom);
HPEN CreatePen(int iStyle, int cWidth, COLORREF color);
BOOL MoveToEx(HDC hdc, int x, int y, POINT* lppt);
BOOL LineTo(HDC hdc, int x, int y);
BOOL Polygon(HDC hdc, const POINT* apt, int cpt);
// d3d12.dll
HRESULT D3D12CreateDevice(void* pAdapter, int MinimumFeatureLevel, void* riid, void** ppDevice);
HRESULT D3D12GetDebugInterface(void* riid, void** ppvDebug);
// dxgi.dll
HRESULT CreateDXGIFactory1(void* riid, void** ppFactory);
HRESULT CreateDXGIFactory2(UINT Flags, void* riid, void** ppFactory);
}
"#;

// ================================================================
// fastos_wrl.h — ComPtr<T>
// ================================================================
const HEADER_FASTOS_WRL: &str = r#"
namespace Microsoft {
namespace WRL {
template<typename T>
class ComPtr {
public:
    T* ptr;
    ComPtr() : ptr(0) {}
    ~ComPtr() { if (ptr) { ptr->Release(); ptr = 0; } }
    T* Get() const { return ptr; }
    T** GetAddressOf() { return &ptr; }
    T* operator->() const { return ptr; }
    T** operator&() { return &ptr; }
    void Reset() { if (ptr) { ptr->Release(); ptr = 0; } }
    T* Detach() { T* tmp = ptr; ptr = 0; return tmp; }
    operator bool() const { return ptr != 0; }
};
}
}
using Microsoft::WRL::ComPtr;
"#;

// ================================================================
// fastos_d3d12.h — D3D12 interfaces (minimal for HelloTriangle)
// ================================================================
const HEADER_FASTOS_D3D12: &str = r#"
struct D3D12_COMMAND_QUEUE_DESC {
    UINT Type;
    INT Priority;
    UINT Flags;
    UINT NodeMask;
};

struct D3D12_DESCRIPTOR_HEAP_DESC {
    UINT Type;
    UINT NumDescriptors;
    UINT Flags;
    UINT NodeMask;
};

struct D3D12_CPU_DESCRIPTOR_HANDLE {
    UINT64 ptr;
};

struct D3D12_GPU_DESCRIPTOR_HANDLE {
    UINT64 ptr;
};

struct D3D12_VERTEX_BUFFER_VIEW {
    UINT64 BufferLocation;
    UINT SizeInBytes;
    UINT StrideInBytes;
};

struct D3D12_INPUT_ELEMENT_DESC {
    LPCSTR SemanticName;
    UINT SemanticIndex;
    UINT Format;
    UINT InputSlot;
    UINT AlignedByteOffset;
    UINT InputSlotClass;
    UINT InstanceDataStepRate;
};

struct D3D12_VIEWPORT {
    FLOAT TopLeftX;
    FLOAT TopLeftY;
    FLOAT Width;
    FLOAT Height;
    FLOAT MinDepth;
    FLOAT MaxDepth;
};

struct D3D12_RECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
};

struct D3D12_RESOURCE_BARRIER {
    UINT Type;
    UINT Flags;
};

struct D3D12_HEAP_PROPERTIES {
    UINT Type;
    UINT CPUPageProperty;
    UINT MemoryPoolPreference;
    UINT CreationNodeMask;
    UINT VisibleNodeMask;
};

struct D3D12_RESOURCE_DESC {
    UINT Dimension;
    UINT64 Alignment;
    UINT64 Width;
    UINT Height;
    UINT16 DepthOrArraySize;
    UINT16 MipLevels;
    UINT Format;
    UINT SampleCount;
    UINT SampleQuality;
    UINT Layout;
    UINT Flags;
};

struct ID3D12Object : public IUnknown {
    virtual HRESULT SetName(LPCWSTR Name) = 0;
};
struct ID3D12DeviceChild : public ID3D12Object {};
struct ID3D12Pageable : public ID3D12DeviceChild {};

struct ID3D12Resource : public ID3D12Pageable {
    virtual HRESULT Map(UINT Subresource, const void* pReadRange, void** ppData) = 0;
    virtual void Unmap(UINT Subresource, const void* pWrittenRange) = 0;
    virtual UINT64 GetGPUVirtualAddress() = 0;
};

struct ID3D12CommandAllocator : public ID3D12Pageable {
    virtual HRESULT Reset() = 0;
};

struct ID3D12Fence : public ID3D12Pageable {
    virtual UINT64 GetCompletedValue() = 0;
    virtual HRESULT SetEventOnCompletion(UINT64 Value, HANDLE hEvent) = 0;
    virtual HRESULT Signal(UINT64 Value) = 0;
};

struct ID3D12DescriptorHeap : public ID3D12Pageable {
    virtual D3D12_CPU_DESCRIPTOR_HANDLE GetCPUDescriptorHandleForHeapStart() = 0;
};

struct ID3D12RootSignature : public ID3D12DeviceChild {};
struct ID3D12PipelineState : public ID3D12Pageable {};
struct ID3D12CommandList : public ID3D12DeviceChild {};

struct ID3D12GraphicsCommandList : public ID3D12CommandList {
    virtual HRESULT Close() = 0;
    virtual HRESULT Reset(ID3D12CommandAllocator* pAllocator, ID3D12PipelineState* pInitialState) = 0;
    virtual void RSSetViewports(UINT NumViewports, const D3D12_VIEWPORT* pViewports) = 0;
    virtual void RSSetScissorRects(UINT NumRects, const D3D12_RECT* pRects) = 0;
    virtual void DrawInstanced(UINT VertexCountPerInstance, UINT InstanceCount, UINT StartVertexLocation, UINT StartInstanceLocation) = 0;
};

struct ID3D12CommandQueue : public ID3D12Pageable {
    virtual void ExecuteCommandLists(UINT NumCommandLists, ID3D12CommandList* const* ppCommandLists) = 0;
    virtual HRESULT Signal(ID3D12Fence* pFence, UINT64 Value) = 0;
};

struct ID3D12Device : public ID3D12Object {
    virtual HRESULT CreateCommandQueue(const D3D12_COMMAND_QUEUE_DESC* pDesc, REFIID riid, void** ppCommandQueue) = 0;
    virtual HRESULT CreateCommandAllocator(UINT type, REFIID riid, void** ppCommandAllocator) = 0;
    virtual HRESULT CreateFence(UINT64 InitialValue, UINT Flags, REFIID riid, void** ppFence) = 0;
    virtual HRESULT CreateDescriptorHeap(const D3D12_DESCRIPTOR_HEAP_DESC* pDesc, REFIID riid, void** ppvHeap) = 0;
    virtual UINT GetDescriptorHandleIncrementSize(UINT DescriptorHeapType) = 0;
    virtual HRESULT CreateRenderTargetView(ID3D12Resource* pResource, const void* pDesc, D3D12_CPU_DESCRIPTOR_HANDLE DestDescriptor) = 0;
};

namespace DirectX {
    struct XMFLOAT2 {
        float x, y;
        XMFLOAT2() : x(0), y(0) {}
        XMFLOAT2(float _x, float _y) : x(_x), y(_y) {}
    };
    struct XMFLOAT3 {
        float x, y, z;
        XMFLOAT3() : x(0), y(0), z(0) {}
        XMFLOAT3(float _x, float _y, float _z) : x(_x), y(_y), z(_z) {}
    };
    struct XMFLOAT4 {
        float x, y, z, w;
        XMFLOAT4() : x(0), y(0), z(0), w(0) {}
        XMFLOAT4(float _x, float _y, float _z, float _w) : x(_x), y(_y), z(_z), w(_w) {}
    };
}
using namespace DirectX;
"#;

// ================================================================
// fastos_dxgi.h — DXGI interfaces (minimal)
// ================================================================
const HEADER_FASTOS_DXGI: &str = r#"
struct DXGI_SAMPLE_DESC {
    UINT Count;
    UINT Quality;
};

struct DXGI_SWAP_CHAIN_DESC1 {
    UINT Width;
    UINT Height;
    UINT Format;
    BOOL Stereo;
    DXGI_SAMPLE_DESC SampleDesc;
    UINT BufferUsage;
    UINT BufferCount;
    UINT Scaling;
    UINT SwapEffect;
    UINT AlphaMode;
    UINT Flags;
};

struct DXGI_ADAPTER_DESC1 {
    WCHAR Description[128];
    UINT VendorId;
    UINT DeviceId;
    UINT SubSysId;
    UINT Revision;
    UINT64 DedicatedVideoMemory;
    UINT64 DedicatedSystemMemory;
    UINT64 SharedSystemMemory;
    LARGE_INTEGER AdapterLuid;
    UINT Flags;
};

struct IDXGIObject : public IUnknown {};
struct IDXGIAdapter : public IDXGIObject {};
struct IDXGIAdapter1 : public IDXGIAdapter {
    virtual HRESULT GetDesc1(DXGI_ADAPTER_DESC1* pDesc) = 0;
};
struct IDXGIOutput : public IDXGIObject {};

struct IDXGISwapChain : public IDXGIObject {
    virtual HRESULT Present(UINT SyncInterval, UINT Flags) = 0;
    virtual HRESULT GetBuffer(UINT Buffer, REFIID riid, void** ppSurface) = 0;
};
struct IDXGISwapChain1 : public IDXGISwapChain {};
struct IDXGISwapChain3 : public IDXGISwapChain1 {
    virtual UINT GetCurrentBackBufferIndex() = 0;
};

struct IDXGIFactory : public IDXGIObject {};
struct IDXGIFactory1 : public IDXGIFactory {
    virtual HRESULT EnumAdapters1(UINT Adapter, IDXGIAdapter1** ppAdapter) = 0;
};
struct IDXGIFactory4 : public IDXGIFactory1 {};
"#;
