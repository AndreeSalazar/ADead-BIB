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
        "type_traits" | "limits" | "concepts" => Some(HEADER_EMPTY),
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
        _ => None,
    }
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
