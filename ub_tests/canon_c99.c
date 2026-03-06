#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

// C99: inline functions
inline int add(int a, int b) {
    return a + b;
}

struct Point {
    int x;
    int y;
};

int main() {
    // C99: Mixed declarations and code
    int sum = add(10, 20);
    
    // C99: Loop variable declaration inside for
    int total = 0;
    for (int i = 0; i < 5; i++) {
        total += i; 
    }
    
    // C99: Struct initialization
    struct Point p = {10, 20};

    // C99: stdbool.h and stdint.h
    bool is_valid = true;
    int32_t val = 100;

    printf("Sum: %d, Total: %d, Point: %d, Valid: %d, Val: %d\n", sum, total, p.x, is_valid, val);
    
    return 0;
}
