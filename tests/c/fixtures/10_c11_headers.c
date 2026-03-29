// Test: C11 standard headers — fenv.h, stdatomic.h, threads.h, stdnoreturn.h
// Expected: All preprocess + parse + lower correctly

#include <fenv.h>
#include <stdatomic.h>
#include <threads.h>
#include <stdalign.h>
#include <stdnoreturn.h>
#include <iso646.h>

int test_fenv() {
    feclearexcept(0);
    int round = fegetround();
    fesetround(0);
    return round;
}

int test_atomic() {
    atomic_int counter;
    return 0;
}

int test_threads() {
    mtx_t mutex;
    return 0;
}

int main() {
    test_fenv();
    test_atomic();
    test_threads();
    return 0;
}
