#include <stdio.h>

int main() {
    int i = 0;
    int sum = 0;
    while (i < 3) {
        sum = sum + 1;
        i = i + 1;
    }
    printf("sum=%d i=%d\n", sum, i);
    return 0;
}
