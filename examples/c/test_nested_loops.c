#include <stdio.h>

int main() {
    int total = 0;
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5; j++) {
            if (j == 3) break;
            if (i == 2 && j == 1) continue;
            total += 1;
        }
    }
    printf("total=%d\n", total);
    return 0;
}
