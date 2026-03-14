#include <header_main.h>

int safe_divide(int a, int b) {
    if (b == 0) {
        printf("error: division by zero\n");
        return 0;
    }
    return a / b;
}

int main() {
    // Basic error handling pattern
    int result = safe_divide(10, 2);
    printf("10/2 = %d\n", result);

    result = safe_divide(10, 0);
    printf("10/0 = %d\n", result);

    // Try/catch pattern (converted to error codes by ADead-BIB)
    try {
        int x = safe_divide(20, 4);
        printf("20/4 = %d\n", x);
    } catch (...) {
        printf("caught error\n");
    }

    // Multiple operations
    int a = safe_divide(100, 5);
    int b = safe_divide(42, 7);
    printf("100/5=%d 42/7=%d\n", a, b);

    printf("exception handling complete\n");
    return 0;
}
