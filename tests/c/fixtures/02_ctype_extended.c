// ============================================================
// Test 02: <ctype.h> — Extended Classification + Conversion
// ============================================================
// ADead-BIB Test Canon — C99 §7.4
// Verifica: isprint, isgraph, iscntrl, ispunct, isxdigit,
//           toupper, tolower
// Expected: Compila OK + Ejecuta OK
// ============================================================
#include <stdio.h>
#include <ctype.h>

int main() {
    // isprint / isgraph / iscntrl
    printf("isprint: A=%d sp=%d NUL=%d\n",
           isprint('A') != 0, isprint(' ') != 0, isprint('\0') != 0);
    printf("isgraph: A=%d sp=%d !=%d\n",
           isgraph('A') != 0, isgraph(' ') != 0, isgraph('!') != 0);
    printf("iscntrl: NUL=%d nl=%d A=%d\n",
           iscntrl('\0') != 0, iscntrl('\n') != 0, iscntrl('A') != 0);

    // ispunct / isxdigit
    printf("ispunct: !=%d A=%d 5=%d\n",
           ispunct('!') != 0, ispunct('A') != 0, ispunct('5') != 0);
    printf("isxdigit: 0=%d a=%d g=%d\n",
           isxdigit('0') != 0, isxdigit('a') != 0, isxdigit('g') != 0);

    // toupper / tolower
    int tu = toupper('a');
    int tl = tolower('A');
    printf("toupper(a)=%d tolower(A)=%d\n", tu, tl);

    return 0;
}
