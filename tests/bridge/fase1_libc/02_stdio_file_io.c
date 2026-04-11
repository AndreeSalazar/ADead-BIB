#include <stdio.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;
    const char *filename = "adead_test_tmp.txt";
    const char *test_data = "Hello ADead-BIB file I/O!\n";

    // fopen + fwrite
    FILE *f = fopen(filename, "w");
    if (f) {
        size_t written = fwrite(test_data, 1, strlen(test_data), f);
        if (written == strlen(test_data)) { pass++; printf("PASS: fwrite %d bytes\n", (int)written); }
        else { fail++; printf("FAIL: fwrite\n"); }
        fclose(f);
        pass++;
        printf("PASS: fopen+fclose write\n");
    } else {
        fail++;
        printf("FAIL: fopen write\n");
    }

    // fopen + fread
    f = fopen(filename, "r");
    if (f) {
        char buf[256];
        memset(buf, 0, 256);
        size_t rd = fread(buf, 1, 255, f);
        if (rd > 0 && strcmp(buf, test_data) == 0) { pass++; printf("PASS: fread content matches\n"); }
        else { fail++; printf("FAIL: fread content mismatch\n"); }

        // fseek + ftell
        fseek(f, 0, 2); // SEEK_END
        long sz = ftell(f);
        if (sz == (long)strlen(test_data)) { pass++; printf("PASS: ftell = %ld\n", sz); }
        else { fail++; printf("FAIL: ftell = %ld\n", sz); }

        fclose(f);
    } else {
        fail += 2;
        printf("FAIL: fopen read\n");
    }

    // remove
    if (remove(filename) == 0) { pass++; printf("PASS: remove\n"); }
    else { fail++; printf("FAIL: remove\n"); }

    printf("\n=== stdio_file_io: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
