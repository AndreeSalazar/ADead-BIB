#include <header_main.h>
#include <iostream>
#include <vector>
#include <string>

#define VERSION_MAJOR 8
#define VERSION_MINOR 0
#define VERSION_STR "8.0"

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define CLAMP(x, lo, hi) MIN(MAX(x, lo), hi)
#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof(arr[0]))
#define STRINGIFY(x) #x
#define CONCAT(a, b) a ## b
#define UNUSED(x) (void)(x)

#define LOG(fmt, ...) printf("[LOG] " fmt "\n", ##__VA_ARGS__)
#define ASSERT(cond) if (!(cond)) { printf("ASSERT FAILED: %s at line %d\n", #cond, __LINE__); }

#ifdef __cplusplus
#define EXTERN_C extern "C"
#else
#define EXTERN_C
#endif

#ifndef NULL
#define NULL ((void*)0)
#endif

#define MAKE_PAIR(a, b) std::make_pair(a, b)

int main() {
    printf("ADead-BIB v%s\n", VERSION_STR);
    printf("Version: %d.%d\n", VERSION_MAJOR, VERSION_MINOR);
    int a = 10, b = 20;
    printf("MAX(%d,%d)=%d\n", a, b, MAX(a, b));
    printf("MIN(%d,%d)=%d\n", a, b, MIN(a, b));
    printf("CLAMP(25,0,20)=%d\n", CLAMP(25, 0, 20));
    int arr[] = {1, 2, 3, 4, 5};
    printf("ARRAY_SIZE=%d\n", ARRAY_SIZE(arr));
    LOG("hello %s", "world");
    LOG("value=%d", 42);
    ASSERT(1 == 1);
    ASSERT(a < b);
    int CONCAT(my, Var) = 999;
    printf("myVar=%d\n", myVar);
    UNUSED(a);
    printf("file: %s line: %d\n", __FILE__, __LINE__);
    #ifdef __cplusplus
    printf("C++ mode\n");
    #endif
    auto p = MAKE_PAIR(1, 2.0);
    printf("pair: %d, %.1f\n", p.first, p.second);
    std::vector<int> v = {1, 2, 3};
    for (auto& x : v) printf("%d ", x);
    printf("\n");
    return 0;
}
