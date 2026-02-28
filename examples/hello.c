#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

int main() {
    int x = 10;
    int y = 20;
    int result = add(x, y);

    printf("ADead-BIB C Compiler!\n");
    printf("Result: %d\n", result);

    for (int i = 0; i < 5; i++) {
        printf("i = %d\n", i);
    }

    if (result > 25) {
        printf("Greater than 25\n");
    } else {
        printf("Less or equal to 25\n");
    }

    return 0;
}
