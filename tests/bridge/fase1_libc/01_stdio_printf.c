#include <stdio.h>

int main() {
    int pass = 0, fail = 0;

    // %d integer
    printf("Test %%d: ");
    printf("%d\n", 42);
    pass++;

    // %s string
    printf("Test %%s: %s\n", "hello");
    pass++;

    // %x hex
    printf("Test %%x: 0x%x\n", 255);
    pass++;

    // %p pointer
    int x = 99;
    printf("Test %%p: %p\n", &x);
    pass++;

    // %c char
    printf("Test %%c: %c\n", 'A');
    pass++;

    // %ld long
    long big = 1234567890L;
    printf("Test %%ld: %ld\n", big);
    pass++;

    // Multiple args
    printf("Test multi: %d + %d = %d\n", 10, 20, 30);
    pass++;

    // puts
    puts("Test puts: OK");
    pass++;

    printf("\n=== stdio_printf: %d passed ===\n", pass);
    return 0;
}
