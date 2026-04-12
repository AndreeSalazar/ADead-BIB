// ADead-BIB Test: wchar.h basic functions (wcslen, wcscpy, wcscat, wcscmp, mbstowcs, wcstombs)
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

// Minimal wchar_t support via msvcrt
typedef unsigned short wchar_t_compat;

int main() {
    int pass = 0, fail = 0;

    // wcslen
    int wlen = wcslen(L"Hello");
    if (wlen == 5) { pass++; printf("PASS: wcslen(L\"Hello\")=%d\n", wlen); }
    else { fail++; printf("FAIL: wcslen=%d\n", wlen); }

    // wcscpy + wcscmp
    wchar_t buf[64];
    wcscpy(buf, L"Hello World");
    if (wcscmp(buf, L"Hello World") == 0) { pass++; printf("PASS: wcscpy+wcscmp\n"); }
    else { fail++; printf("FAIL: wcscpy+wcscmp\n"); }

    // wcsncmp
    if (wcsncmp(L"Hello", L"Help", 3) == 0) { pass++; printf("PASS: wcsncmp\n"); }
    else { fail++; printf("FAIL: wcsncmp\n"); }

    // wcscat
    wchar_t catbuf[64];
    wcscpy(catbuf, L"Hello");
    wcscat(catbuf, L" World");
    if (wcscmp(catbuf, L"Hello World") == 0) { pass++; printf("PASS: wcscat\n"); }
    else { fail++; printf("FAIL: wcscat\n"); }

    // mbstowcs
    wchar_t wbuf[32];
    int converted = mbstowcs(wbuf, "Test", 32);
    if (converted == 4) { pass++; printf("PASS: mbstowcs converted=%d\n", converted); }
    else { fail++; printf("FAIL: mbstowcs converted=%d\n", converted); }

    // wcstombs
    char mbuf[32];
    converted = wcstombs(mbuf, L"Test", 32);
    if (converted == 4 && strcmp(mbuf, "Test") == 0) { pass++; printf("PASS: wcstombs='%s'\n", mbuf); }
    else { fail++; printf("FAIL: wcstombs converted=%d\n", converted); }

    // towupper / towlower
    int upper = towupper(L'a');
    if (upper == L'A') { pass++; printf("PASS: towupper('a')='%c'\n", (char)upper); }
    else { fail++; printf("FAIL: towupper=%d\n", upper); }

    int lower = towlower(L'Z');
    if (lower == L'z') { pass++; printf("PASS: towlower('Z')='%c'\n", (char)lower); }
    else { fail++; printf("FAIL: towlower=%d\n", lower); }

    printf("\n=== wchar_basic: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
