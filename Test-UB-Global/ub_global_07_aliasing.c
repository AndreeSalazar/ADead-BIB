#include <stdio.h>

void strict_aliasing_violation() {
    float f = 1.0f;
    int* p = (int*)&f; // UB
    *p = 0;
}

void safe_aliasing() {
    float f = 1.0f;
    char* p = (char*)&f; // Safe
    *p = 0;
}

int main() {
    strict_aliasing_violation();
    return 0;
}
