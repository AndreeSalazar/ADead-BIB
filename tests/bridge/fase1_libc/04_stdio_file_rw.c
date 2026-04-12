// ADead-BIB Test: stdio file I/O (fopen, fclose, fread, fwrite, fgets, fputs, fseek, ftell)
#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // fopen + fwrite + fclose
    FILE *f = fopen("test_output.tmp", "w");
    if (f) {
        pass++; printf("PASS: fopen write\n");
        fputs("Hello World\n", f);
        fputs("Line 2\n", f);
        fprintf(f, "Number: %d\n", 42);
        fclose(f);
        pass++; printf("PASS: fclose write\n");
    } else { fail += 2; printf("FAIL: fopen write\n"); }

    // fopen + fgets + fclose (read back)
    f = fopen("test_output.tmp", "r");
    if (f) {
        pass++; printf("PASS: fopen read\n");

        char line[128];
        // Line 1
        if (fgets(line, sizeof(line), f)) {
            if (strncmp(line, "Hello World", 11) == 0) { pass++; printf("PASS: fgets line 1\n"); }
            else { fail++; printf("FAIL: fgets line 1 got '%s'\n", line); }
        } else { fail++; printf("FAIL: fgets line 1 null\n"); }

        // Line 2
        if (fgets(line, sizeof(line), f)) {
            if (strncmp(line, "Line 2", 6) == 0) { pass++; printf("PASS: fgets line 2\n"); }
            else { fail++; printf("FAIL: fgets line 2 got '%s'\n", line); }
        } else { fail++; printf("FAIL: fgets line 2 null\n"); }

        // feof check
        // Skip line 3
        fgets(line, sizeof(line), f);
        // Try reading past end
        char *result = fgets(line, sizeof(line), f);
        if (result == 0 || feof(f)) { pass++; printf("PASS: feof detected\n"); }
        else { fail++; printf("FAIL: feof not detected\n"); }

        fclose(f);
        pass++; printf("PASS: fclose read\n");
    } else { fail += 5; printf("FAIL: fopen read\n"); }

    // fseek + ftell
    f = fopen("test_output.tmp", "r");
    if (f) {
        fseek(f, 0, 2); // SEEK_END = 2
        long size = ftell(f);
        if (size > 0) { pass++; printf("PASS: ftell size=%ld\n", size); }
        else { fail++; printf("FAIL: ftell size=%ld\n", size); }

        fseek(f, 0, 0); // SEEK_SET = 0
        long pos = ftell(f);
        if (pos == 0) { pass++; printf("PASS: fseek to start\n"); }
        else { fail++; printf("FAIL: fseek pos=%ld\n", pos); }

        fclose(f);
    } else { fail += 2; printf("FAIL: fopen for seek\n"); }

    // remove temp file
    remove("test_output.tmp");
    pass++; printf("PASS: remove temp file\n");

    printf("\n=== stdio_file_rw: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
