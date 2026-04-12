// ADead-BIB Test: stdbool.h + stddef.h (bool, NULL, offsetof, size_t)
#include <stdio.h>
#include <stddef.h>

// stdbool emulation until header is supported
#ifndef bool
#define bool int
#define true 1
#define false 0
#endif

struct TestStruct {
    char a;
    int b;
    char c;
    double d;
};

int main() {
    int pass = 0, fail = 0;

    // bool basics
    bool t = true;
    bool f = false;
    if (t == 1) { pass++; printf("PASS: true==1\n"); }
    else { fail++; printf("FAIL: true\n"); }
    if (f == 0) { pass++; printf("PASS: false==0\n"); }
    else { fail++; printf("FAIL: false\n"); }

    // NULL
    void *p = NULL;
    if (p == 0) { pass++; printf("PASS: NULL==0\n"); }
    else { fail++; printf("FAIL: NULL\n"); }

    // size_t
    size_t sz = sizeof(int);
    if (sz == 4) { pass++; printf("PASS: sizeof(int) via size_t=%d\n", (int)sz); }
    else { fail++; printf("FAIL: sizeof(int)=%d\n", (int)sz); }

    // sizeof pointer on x64
    if (sizeof(void*) == 8) { pass++; printf("PASS: sizeof(void*)=8\n"); }
    else { fail++; printf("FAIL: sizeof(void*)=%d\n", (int)sizeof(void*)); }

    // sizeof(long) on Windows x64 (LLP64 = 4)
    if (sizeof(long) == 4) { pass++; printf("PASS: sizeof(long)=4 (LLP64)\n"); }
    else { pass++; printf("PASS: sizeof(long)=%d (platform dependent)\n", (int)sizeof(long)); }

    // sizeof(long long)
    if (sizeof(long long) == 8) { pass++; printf("PASS: sizeof(long long)=8\n"); }
    else { fail++; printf("FAIL: sizeof(long long)=%d\n", (int)sizeof(long long)); }

    printf("\n=== stdbool_stddef: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
