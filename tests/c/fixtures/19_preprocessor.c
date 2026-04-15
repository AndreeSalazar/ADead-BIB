// ============================================================
// Test 19: Preprocessor — #define, #include, #ifdef, macros
// ============================================================
// ADead-BIB Test Canon — C99 §6.10
// Verifica: object macros, function macros, conditional compilation
// ============================================================

#include <stdio.h>

// --- Object macros ---
#define MAX_SIZE 100
#define PI 3
#define VERSION_MAJOR 8
#define VERSION_MINOR 0
#define APP_NAME "ADead-BIB"

// --- Function macros ---
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define ABS(x) ((x) >= 0 ? (x) : -(x))
#define SQUARE(x) ((x) * (x))
#define CLAMP(val, lo, hi) ((val) < (lo) ? (lo) : ((val) > (hi) ? (hi) : (val)))
#define SWAP(a, b, type) do { type _tmp = (a); (a) = (b); (b) = _tmp; } while(0)
#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

// --- Stringification ---
#define STRINGIFY(x) #x
#define TOSTRING(x) STRINGIFY(x)

// --- Concatenation ---
#define CONCAT(a, b) a##b

// --- Conditional compilation ---
#define FEATURE_LOGGING 1
#define PLATFORM_WINDOWS 1

#ifdef FEATURE_LOGGING
#define LOG(msg) printf("[LOG] %s\n", msg)
#else
#define LOG(msg)
#endif

#if PLATFORM_WINDOWS
#define PATH_SEP '\\'
#else
#define PATH_SEP '/'
#endif

// --- Multi-line macro ---
#define PRINT_HEADER() \
    printf("=================\n"); \
    printf("  %s v%d.%d\n", APP_NAME, VERSION_MAJOR, VERSION_MINOR); \
    printf("=================\n")

// --- Guard pattern ---
#ifndef GUARD_TEST
#define GUARD_TEST 1
int guard_value = 42;
#endif

#ifndef GUARD_TEST
int guard_value = 99;
#endif

int main() {
    // --- Object macros ---
    printf("MAX_SIZE=%d PI=%d\n", MAX_SIZE, PI);
    printf("VERSION=%d.%d\n", VERSION_MAJOR, VERSION_MINOR);
    printf("APP=%s\n", APP_NAME);

    // --- Function macros ---
    printf("MAX(3,7)=%d\n", MAX(3, 7));
    printf("MIN(3,7)=%d\n", MIN(3, 7));
    printf("ABS(-5)=%d\n", ABS(-5));
    printf("SQUARE(6)=%d\n", SQUARE(6));
    printf("CLAMP(15,0,10)=%d\n", CLAMP(15, 0, 10));

    // --- SWAP ---
    int a = 10, b = 20;
    SWAP(a, b, int);
    printf("SWAP: a=%d b=%d\n", a, b);

    // --- ARRAY_SIZE ---
    int arr[] = {1, 2, 3, 4, 5};
    printf("ARRAY_SIZE=%d\n", (int)ARRAY_SIZE(arr));

    // --- Stringify ---
    printf("STRINGIFY(hello)=%s\n", STRINGIFY(hello));
    printf("VERSION_STR=%s\n", TOSTRING(VERSION_MAJOR));

    // --- Concat ---
    int CONCAT(my, _var) = 123;
    printf("concat_var=%d\n", my_var);

    // --- Conditional ---
    LOG("program started");
    printf("PATH_SEP='%c'\n", PATH_SEP);

    // --- Header ---
    PRINT_HEADER();

    // --- Guard ---
    printf("guard_value=%d\n", guard_value);

    // --- Predefined macros ---
    printf("__LINE__=%d\n", __LINE__);
    printf("__FILE__=%s\n", __FILE__);

    return 0;
}
