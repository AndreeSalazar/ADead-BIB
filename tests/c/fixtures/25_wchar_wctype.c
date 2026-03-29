// Test: <wchar.h> + <wctype.h> — Wide characters
// Expected: Compile OK — wchar_t, wchar functions, wctype functions

#include <wchar.h>
#include <wctype.h>
#include <stdio.h>

int main() {
    printf("=== wchar/wctype test ===\n");

    // wchar_t type
    wchar_t wc = L'A';
    printf("wchar_t size=%d\n", (int)sizeof(wchar_t));

    // wctype classification
    int wa = iswalpha(wc);
    int wu = iswupper(wc);
    int wl = iswlower(wc);
    int wd = iswdigit(wc);
    int ws = iswspace(wc);
    printf("iswalpha(A)=%d iswupper(A)=%d\n", wa != 0, wu != 0);

    // wchar conversion
    int up = towupper(L'a');
    int lo = towlower(L'A');
    printf("towupper(a)=%d towlower(A)=%d\n", up, lo);

    // wcslen, wcscmp declarations
    const wchar_t *ws1 = L"hello";
    size_t len = wcslen(ws1);
    printf("wcslen parsed OK\n");

    printf("=== wchar/wctype OK ===\n");
    return 0;
}
