#include <stdio.h>

int classify(int n) {
    switch (n) {
        case 0: return 0;
        case 1: return 10;
        case 2: return 20;
        case 3: return 30;
        default: return -1;
    }
}

int main() {
    printf("classify(0)=%d\n", classify(0));
    printf("classify(1)=%d\n", classify(1));
    printf("classify(2)=%d\n", classify(2));
    printf("classify(3)=%d\n", classify(3));
    printf("classify(99)=%d\n", classify(99));
    return 0;
}
