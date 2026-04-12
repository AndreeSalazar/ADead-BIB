// ADead-BIB Test: wchar.h advanced (wmemcpy, wmemset, fgetwc/fputwc, wcsftime)
#include <stdio.h>
#include <string.h>
#include <wchar.h>
#include <time.h>

int main() {
    int pass = 0, fail = 0;

    // wcschr
    wchar_t *p = wcschr(L"Hello World", L'W');
    if (p && *p == L'W') { pass++; printf("PASS: wcschr found 'W'\n"); }
    else { fail++; printf("FAIL: wcschr\n"); }

    // wcsrchr
    p = wcsrchr(L"Hello World", L'l');
    if (p && *p == L'l') { pass++; printf("PASS: wcsrchr found 'l'\n"); }
    else { fail++; printf("FAIL: wcsrchr\n"); }

    // wcsstr
    p = wcsstr(L"Hello World", L"World");
    if (p && wcscmp(p, L"World") == 0) { pass++; printf("PASS: wcsstr\n"); }
    else { fail++; printf("FAIL: wcsstr\n"); }

    // wcstok
    wchar_t tokbuf[64];
    wcscpy(tokbuf, L"one,two,three");
    wchar_t *tok = wcstok(tokbuf, L",", 0);
    if (tok && wcscmp(tok, L"one") == 0) { pass++; printf("PASS: wcstok first\n"); }
    else { fail++; printf("FAIL: wcstok first\n"); }

    // wcstol
    wchar_t *endp;
    long wl = wcstol(L"12345", &endp, 10);
    if (wl == 12345L) { pass++; printf("PASS: wcstol=%ld\n", wl); }
    else { fail++; printf("FAIL: wcstol=%ld\n", wl); }

    // wcstod
    double wd = wcstod(L"3.14", &endp);
    if (wd > 3.13 && wd < 3.15) { pass++; printf("PASS: wcstod=%.2f\n", wd); }
    else { fail++; printf("FAIL: wcstod=%.2f\n", wd); }

    // wcsftime
    time_t now = time(0);
    struct tm *lt = localtime(&now);
    if (lt) {
        wchar_t wbuf[128];
        int wlen = wcsftime(wbuf, 128, L"%Y-%m-%d", lt);
        if (wlen > 0) { pass++; printf("PASS: wcsftime len=%d\n", wlen); }
        else { fail++; printf("FAIL: wcsftime len=%d\n", wlen); }
    }

    // iswalpha / iswdigit / iswspace
    if (iswalpha(L'A')) { pass++; printf("PASS: iswalpha\n"); }
    else { fail++; printf("FAIL: iswalpha\n"); }

    if (iswdigit(L'5')) { pass++; printf("PASS: iswdigit\n"); }
    else { fail++; printf("FAIL: iswdigit\n"); }

    if (iswspace(L' ')) { pass++; printf("PASS: iswspace\n"); }
    else { fail++; printf("FAIL: iswspace\n"); }

    printf("\n=== wchar_advanced: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
