// ADead-BIB Test: errno.h + assert basics
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

int main() {
    int pass = 0, fail = 0;

    // errno initial value
    errno = 0;
    if (errno == 0) { pass++; printf("PASS: errno initial=0\n"); }
    else { fail++; printf("FAIL: errno initial=%d\n", errno); }

    // strerror for known codes
    char *e0 = strerror(0);
    if (e0) { pass++; printf("PASS: strerror(0)='%s'\n", e0); }
    else { fail++; printf("FAIL: strerror(0) null\n"); }

    char *e2 = strerror(2);
    if (e2) { pass++; printf("PASS: strerror(2)='%s'\n", e2); }
    else { fail++; printf("FAIL: strerror(2) null\n"); }

    // errno set by invalid operation
    FILE *f = fopen("__nonexistent_file_adead_test__", "r");
    if (f == 0) {
        if (errno != 0) { pass++; printf("PASS: errno set after failed fopen=%d\n", errno); }
        else { fail++; printf("FAIL: errno not set after fopen\n"); }
    } else {
        fclose(f);
        fail++; printf("FAIL: nonexistent file opened?\n");
    }

    // EDOM, ERANGE constants
    if (EDOM > 0) { pass++; printf("PASS: EDOM=%d\n", EDOM); }
    else { fail++; printf("FAIL: EDOM\n"); }

    if (ERANGE > 0) { pass++; printf("PASS: ERANGE=%d\n", ERANGE); }
    else { fail++; printf("FAIL: ERANGE\n"); }

    printf("\n=== errno_assert: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
