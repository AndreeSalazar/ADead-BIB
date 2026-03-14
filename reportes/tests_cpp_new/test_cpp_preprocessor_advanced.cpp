#include <header_main.h>

// === #elif support ===
#define VERSION 2

#if VERSION == 1
    #define MSG "v1"
#elif VERSION == 2
    #define MSG "v2"
#elif VERSION == 3
    #define MSG "v3"
#else
    #define MSG "unknown"
#endif

// === Multi-line macros with backslash ===
#define SWAP(a, b) \
    do { \
        int tmp = (a); \
        (a) = (b); \
        (b) = tmp; \
    } while(0)

// === #if with defined() && ===
#define FEATURE_A
#define FEATURE_B

#if defined(FEATURE_A) && defined(FEATURE_B)
    #define BOTH_FEATURES 1
#endif

// === #if with comparison operators ===
#define LEVEL 5
#if LEVEL >= 3
    #define HIGH_LEVEL 1
#endif

// === #if with arithmetic ===
#if (2 + 3) == 5
    #define MATH_OK 1
#endif

// === Nested #elif ===
#define PLATFORM 3
#if PLATFORM == 1
    #define PLATFORM_NAME "Windows"
#elif PLATFORM == 2
    #define PLATFORM_NAME "Linux"
#elif PLATFORM == 3
    #define PLATFORM_NAME "macOS"
#else
    #define PLATFORM_NAME "Unknown"
#endif

// === #if !defined ===
#if !defined(NONEXISTENT_MACRO)
    #define NOT_DEFINED_OK 1
#endif

int main() {
    printf("Version: %s\n", MSG);
    printf("Both features: %d\n", BOTH_FEATURES);
    printf("High level: %d\n", HIGH_LEVEL);
    printf("Math: %d\n", MATH_OK);
    printf("Platform: %s\n", PLATFORM_NAME);
    printf("Not defined: %d\n", NOT_DEFINED_OK);

    int x = 10, y = 20;
    SWAP(x, y);
    printf("After swap: x=%d y=%d\n", x, y);

    return 0;
}
