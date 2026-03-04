#include <stdio.h>

int main() {
    int count = 0;
    do {
        count++;
    } while (count < 5);
    printf("count=%d\n", count);

    int x = 10;
    do {
        x = x - 3;
    } while (x > 0);
    printf("x=%d\n", x);
    return 0;
}
