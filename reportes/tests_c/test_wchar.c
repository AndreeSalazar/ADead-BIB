#include <wchar.h>
#include <stdio.h>

int main() {
    wchar_t ws[] = L"Hello Wide";
    int len = wcslen(ws);
    wchar_t dest[64];
    wcscpy(dest, ws);
    int cmp = wcscmp(ws, dest);
    wchar_t *found = wcschr(ws, L'W');
    wchar_t *sub = wcsstr(ws, L"Wide");
    wprintf(L"Wide string: %ls, len=%d\n", ws, len);
    return 0;
}
