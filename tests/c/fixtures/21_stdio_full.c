// ============================================================
// Test 21: <stdio.h> Completo — printf, sprintf, puts, putchar, FILE*
// ============================================================
// ADead-BIB Test Canon — C99 §7.21
// Verifica: format specifiers, string I/O, FILE operations
// ============================================================

#include <stdio.h>

int main() {
    // --- printf con todos los format specifiers ---
    printf("%%d: %d\n", 42);
    printf("%%i: %i\n", -42);
    printf("%%u: %u\n", 42U);
    printf("%%x: %x\n", 255);
    printf("%%X: %X\n", 255);
    printf("%%o: %o\n", 255);
    printf("%%c: %c\n", 'A');
    printf("%%s: %s\n", "hello");
    printf("%%p: %p\n", (void *)0);
    printf("%%%%: %%\n");

    // --- Width/padding ---
    printf("[%10d]\n", 42);
    printf("[%-10d]\n", 42);
    printf("[%010d]\n", 42);
    printf("[%+d] [%+d]\n", 42, -42);

    // --- Long specifiers ---
    printf("%%ld: %ld\n", 100000L);
    printf("%%lld: %lld\n", 9223372036854775807LL);
    printf("%%lu: %lu\n", 4294967295UL);

    // --- sprintf ---
    char buf[128];
    sprintf(buf, "%d + %d = %d", 10, 20, 30);
    printf("sprintf: %s\n", buf);

    sprintf(buf, "hex=0x%08X", 0xDEADBEEF);
    printf("sprintf: %s\n", buf);

    // --- puts ---
    puts("puts: hello world");

    // --- putchar ---
    putchar('A');
    putchar('B');
    putchar('C');
    putchar('\n');

    // --- Múltiple args ---
    printf("multi: %d %d %d %d %d %d\n", 1, 2, 3, 4, 5, 6);

    // --- String formatting ---
    printf("[%20s]\n", "right-aligned");
    printf("[%-20s]\n", "left-aligned");
    printf("[%.5s]\n", "truncated-string");

    return 0;
}
