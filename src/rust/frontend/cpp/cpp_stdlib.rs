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
"#;
