#include <stdio.h>

int counter = 0;
int max_val = 100;

void inc() {
    counter = counter + 1;
}

int get() {
    return counter;
}

int main() {
    printf("initial=%d\n", counter);
    inc();
    inc();
    inc();
    printf("after 3 inc=%d\n", get());
    printf("max=%d\n", max_val);
    return 0;
}
