// ADead-BIB Test: MSVC CRT extensions (_stricmp, _strdup, _itoa, etc.)
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    int pass = 0, fail = 0;

    // _stricmp (case-insensitive compare)
    if (_stricmp("Hello", "hello") == 0) { pass++; printf("PASS: _stricmp equal\n"); }
    else { fail++; printf("FAIL: _stricmp\n"); }

    if (_stricmp("abc", "ABD") < 0) { pass++; printf("PASS: _stricmp less\n"); }
    else { fail++; printf("FAIL: _stricmp less\n"); }

    // _strnicmp
    if (_strnicmp("Hello World", "HELLO EARTH", 5) == 0) { pass++; printf("PASS: _strnicmp\n"); }
    else { fail++; printf("FAIL: _strnicmp\n"); }

    // _strdup
    char *dup = _strdup("Hello ADead");
    if (dup && strcmp(dup, "Hello ADead") == 0) { pass++; printf("PASS: _strdup='%s'\n", dup); }
    else { fail++; printf("FAIL: _strdup\n"); }
    if (dup) free(dup);

    // _strlwr / _strupr
    char buf1[32];
    strcpy(buf1, "Hello World");
    _strlwr(buf1);
    if (strcmp(buf1, "hello world") == 0) { pass++; printf("PASS: _strlwr='%s'\n", buf1); }
    else { fail++; printf("FAIL: _strlwr='%s'\n", buf1); }

    strcpy(buf1, "Hello World");
    _strupr(buf1);
    if (strcmp(buf1, "HELLO WORLD") == 0) { pass++; printf("PASS: _strupr='%s'\n", buf1); }
    else { fail++; printf("FAIL: _strupr='%s'\n", buf1); }

    // _itoa
    char numbuf[32];
    _itoa(12345, numbuf, 10);
    if (strcmp(numbuf, "12345") == 0) { pass++; printf("PASS: _itoa decimal='%s'\n", numbuf); }
    else { fail++; printf("FAIL: _itoa decimal='%s'\n", numbuf); }

    _itoa(255, numbuf, 16);
    if (strcmp(numbuf, "ff") == 0) { pass++; printf("PASS: _itoa hex='%s'\n", numbuf); }
    else { fail++; printf("FAIL: _itoa hex='%s'\n", numbuf); }

    // _access (check if current dir is accessible)
    if (_access(".", 0) == 0) { pass++; printf("PASS: _access current dir\n"); }
    else { fail++; printf("FAIL: _access\n"); }

    // _getcwd
    char cwdbuf[256];
    if (_getcwd(cwdbuf, sizeof(cwdbuf))) { pass++; printf("PASS: _getcwd='%s'\n", cwdbuf); }
    else { fail++; printf("FAIL: _getcwd\n"); }

    printf("\n=== msvc_extensions: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
