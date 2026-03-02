#include <stdio.h>

int sum_to(int n) {
    int s = 0;
    int i = 0;
    while (i < n) {
        s = s + i;
        i = i + 1;
    }
    return s;
}

int main() {
    printf("sum_to(5)=%d\n", sum_to(5));
    return 0;
}
