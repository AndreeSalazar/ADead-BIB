// ADead-BIB Test: stdio formatting functions (sprintf, fprintf, puts, putchar, getchar)
#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // sprintf
    char buf[128];
    sprintf(buf, "num=%d str=%s", 42, "ok");
    if (strcmp(buf, "num=42 str=ok") == 0) { pass++; printf("PASS: sprintf\n"); }
    else { fail++; printf("FAIL: sprintf got '%s'\n", buf); }

    // sprintf with hex
    sprintf(buf, "0x%x", 255);
    if (strcmp(buf, "0xff") == 0) { pass++; printf("PASS: sprintf hex\n"); }
    else { fail++; printf("FAIL: sprintf hex got '%s'\n", buf); }

    // puts
    puts("PASS: puts output");
    pass++;

    // putchar
    printf("PASS: putchar: ");
    putchar('A');
    putchar('B');
    putchar('C');
    putchar('\n');
    pass++;

    // fprintf to stdout
    fprintf(stdout, "PASS: fprintf to stdout\n");
    pass++;

    // sscanf
    int a = 0, b = 0;
    sscanf("10 20", "%d %d", &a, &b);
    if (a == 10 && b == 20) { pass++; printf("PASS: sscanf a=%d b=%d\n", a, b); }
    else { fail++; printf("FAIL: sscanf a=%d b=%d\n", a, b); }

    printf("\n=== stdio_format: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
