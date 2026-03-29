// Test: Preprocessor — #include, #define, #ifdef, macros, variadic macros
// Expected: All preprocess + parse + lower correctly

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

#define MAX_SIZE 100
#define SQUARE(x) ((x) * (x))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define ABS(x) ((x) < 0 ? -(x) : (x))

#ifdef __STDC__
int stdc_available = 1;
#else
int stdc_available = 0;
#endif

#ifndef CUSTOM_FLAG
#define CUSTOM_FLAG 42
#endif

int main() {
    int arr[MAX_SIZE];
    int sq = SQUARE(5);
    int mn = MIN(10, 20);
    int ab = ABS(-7);
    int flag = CUSTOM_FLAG;
    int stdc = stdc_available;

    bool b = true;
    uint32_t u = 0xDEADBEEF;
    int32_t s = -12345;

    return 0;
}
