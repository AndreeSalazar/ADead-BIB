#include <stdio.h>

int main() {
    int x = 5;
    x++;
    printf("x++ -> %d\n", x);
    x--;
    printf("x-- -> %d\n", x);
    ++x;
    printf("++x -> %d\n", x);
    --x;
    printf("--x -> %d\n", x);
    
    int i = 0;
    while (i < 5) {
        printf("i=%d ", i);
        i++;
    }
    printf("\n");
    
    for (int j = 10; j > 0; j--) {
        if (j <= 3) {
            printf("j=%d ", j);
        }
    }
    printf("\n");
    return 0;
}
