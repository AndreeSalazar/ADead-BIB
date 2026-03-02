#include <stdio.h>

int dummy() { return 1; }

int main() {
    int i = 0;
    int sum = 0;
    while (i < 3) {
        sum = sum + 1;
        i = i + 1;
    }
    printf("sum=%d\n", sum);
    return 0;
}
