// ADead-BIB Bridge Test 14 — File I/O
// Level: INTERMEDIATE
// Tests: fopen, fwrite, fread, fclose, fprintf, fseek, ftell, remove

#include <stdio.h>

int main() {
    printf("=== ADead-BIB Bridge Test 14: File I/O ===\n");
    int pass = 0, fail = 0;

    const char *filename = "adead_test_tmp.txt";

    // 1. fopen for writing
    FILE *fp = fopen(filename, "w");
    if (fp) { pass++; } else { fail++; printf("FAIL: fopen write\n"); return fail; }

    // 2. fprintf several lines
    int written = 0;
    written += fprintf(fp, "Line one: hello\n");
    written += fprintf(fp, "Line two: world\n");
    written += fprintf(fp, "Line three: 12345\n");
    if (written > 0) { pass++; } else { fail++; printf("FAIL: fprintf returned %d\n", written); }

    // 3. fclose after writing
    int rc = fclose(fp);
    if (rc == 0) { pass++; } else { fail++; printf("FAIL: fclose write rc=%d\n", rc); }

    // 4. fopen for reading
    fp = fopen(filename, "r");
    if (fp) { pass++; } else { fail++; printf("FAIL: fopen read\n"); return fail; }

    // 5. fseek to end + ftell to get file size
    fseek(fp, 0, 2); // SEEK_END = 2
    long size = ftell(fp);
    if (size == (long)written) { pass++; } else { fail++; printf("FAIL: ftell size=%ld expected=%d\n", size, written); }

    // 6. fseek back to beginning
    fseek(fp, 0, 0); // SEEK_SET = 0
    long pos = ftell(fp);
    if (pos == 0) { pass++; } else { fail++; printf("FAIL: fseek/ftell rewind pos=%ld\n", pos); }

    // 7. fread content back and verify first line
    char buf[256];
    int nread = (int)fread(buf, 1, (unsigned long)size, fp);
    buf[nread] = '\0';
    if (nread == (int)size) { pass++; } else { fail++; printf("FAIL: fread nread=%d expected=%ld\n", nread, size); }

    // 8. Verify content contains expected strings
    int found = 0;
    // Simple substring search for "hello" in buf
    for (int i = 0; i < nread - 4; i++) {
        if (buf[i]=='h' && buf[i+1]=='e' && buf[i+2]=='l' && buf[i+3]=='l' && buf[i+4]=='o') {
            found = 1;
            break;
        }
    }
    if (found) { pass++; } else { fail++; printf("FAIL: content missing 'hello'\n"); }

    // 9. fclose after reading
    rc = fclose(fp);
    if (rc == 0) { pass++; } else { fail++; printf("FAIL: fclose read rc=%d\n", rc); }

    // 10. remove temp file
    rc = remove(filename);
    if (rc == 0) { pass++; } else { fail++; printf("FAIL: remove rc=%d\n", rc); }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 14: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
