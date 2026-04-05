// ADead-BIB Bridge Test 01 — Console Hello
// Level: BASIC
// Tests: printf, return codes, string literals
// Expected: exit code 0

#include <stdio.h>

int main() {
    printf("=== ADead-BIB Bridge Test 01: Console ===\n");
    printf("Hello from ADead-BIB + ASM-BIB!\n");
    
    int x = 42;
    int y = 58;
    int sum = x + y;
    printf("Arithmetic: %d + %d = %d\n", x, y, sum);
    
    if (sum == 100) {
        printf("PASS: basic arithmetic\n");
    } else {
        printf("FAIL: expected 100, got %d\n", sum);
        return 1;
    }
    
    printf("=== Test 01: PASS ===\n");
    return 0;
}
