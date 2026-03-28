// ============================================================
// Test 02: <ctype.h> — Extended Classification + Conversion
// ============================================================
// ADead-BIB Test Canon — C99 §7.4
// Verifica: isprint, isgraph, iscntrl, ispunct, isxdigit,
//           isblank, isascii, toupper, tolower, toascii
// Expected: Compila OK — cobertura completa de ctype.h
// ============================================================
#include <stdio.h>
#include <ctype.h>

int main() {
    // --- isprint / isgraph ---
    int p1 = isprint('A');    // → non-zero (printable)
    int p2 = isprint(' ');    // → non-zero (space is printable)
    int p3 = isprint('\0');   // → 0 (NUL is control)
    int g1 = isgraph('A');    // → non-zero (printable, not space)
    int g2 = isgraph(' ');    // → 0 (space is NOT graph)
    int g3 = isgraph('!');    // → non-zero (punctuation is graph)

    // --- iscntrl ---
    int c1 = iscntrl('\0');   // → non-zero (NUL)
    int c2 = iscntrl('\n');   // → non-zero (newline)
    int c3 = iscntrl(0x1F);  // → non-zero (unit separator)
    int c4 = iscntrl(0x7F);  // → non-zero (DEL)
    int c5 = iscntrl('A');   // → 0

    // --- ispunct ---
    int q1 = ispunct('!');    // → non-zero
    int q2 = ispunct('@');    // → non-zero
    int q3 = ispunct('.');    // → non-zero
    int q4 = ispunct('~');    // → non-zero
    int q5 = ispunct('A');    // → 0 (alpha, not punct)
    int q6 = ispunct('5');    // → 0 (digit, not punct)
    int q7 = ispunct(' ');    // → 0 (space, not punct)

    // --- isxdigit ---
    int x1 = isxdigit('0');   // → non-zero
    int x2 = isxdigit('9');   // → non-zero
    int x3 = isxdigit('a');   // → non-zero
    int x4 = isxdigit('f');   // → non-zero
    int x5 = isxdigit('A');   // → non-zero
    int x6 = isxdigit('F');   // → non-zero
    int x7 = isxdigit('g');   // → 0
    int x8 = isxdigit('G');   // → 0

    // --- isblank (C99) ---
    int b1 = isblank(' ');    // → non-zero
    int b2 = isblank('\t');   // → non-zero
    int b3 = isblank('\n');   // → 0 (newline is NOT blank)

    // --- isascii (POSIX) ---
    int a1 = isascii(65);     // → non-zero (0x41 = 'A')
    int a2 = isascii(127);    // → non-zero (max ASCII)
    int a3 = isascii(128);    // → 0 (beyond ASCII)

    // --- toupper / tolower ---
    int tu1 = toupper('a');   // → 'A' (65)
    int tu2 = toupper('z');   // → 'Z' (90)
    int tu3 = toupper('A');   // → 'A' (already upper)
    int tu4 = toupper('5');   // → '5' (not a letter)
    int tl1 = tolower('A');   // → 'a' (97)
    int tl2 = tolower('Z');   // → 'z' (122)
    int tl3 = tolower('a');   // → 'a' (already lower)

    // --- toascii (POSIX) ---
    int ta1 = toascii(0xFF);  // → 0x7F (mask to 7 bits)
    int ta2 = toascii(0x80);  // → 0x00

    printf("isprint: A=%d sp=%d NUL=%d\n", p1 != 0, p2 != 0, p3 != 0);
    printf("isgraph: A=%d sp=%d !=%d\n", g1 != 0, g2 != 0, g3 != 0);
    printf("iscntrl: NUL=%d nl=%d 1F=%d DEL=%d A=%d\n",
           c1 != 0, c2 != 0, c3 != 0, c4 != 0, c5 != 0);
    printf("ispunct: !=%d @=%d .=%d ~=%d A=%d 5=%d sp=%d\n",
           q1 != 0, q2 != 0, q3 != 0, q4 != 0, q5 != 0, q6 != 0, q7 != 0);
    printf("isxdigit: 0=%d 9=%d a=%d f=%d A=%d F=%d g=%d G=%d\n",
           x1 != 0, x2 != 0, x3 != 0, x4 != 0, x5 != 0, x6 != 0, x7 != 0, x8 != 0);
    printf("isblank: sp=%d tab=%d nl=%d\n", b1 != 0, b2 != 0, b3 != 0);
    printf("toupper: a→%c z→%c A→%c 5→%c\n", tu1, tu2, tu3, tu4);
    printf("tolower: A→%c Z→%c a→%c\n", tl1, tl2, tl3);
    printf("toascii: 0xFF→0x%02X 0x80→0x%02X\n", ta1, ta2);

    return 0;
}
// Expected output:
// isprint: A=1 sp=1 NUL=0
// isgraph: A=1 sp=0 !=1
// iscntrl: NUL=1 nl=1 1F=1 DEL=1 A=0
// ispunct: !=1 @=1 .=1 ~=1 A=0 5=0 sp=0
// isxdigit: 0=1 9=1 a=1 f=1 A=1 F=1 g=0 G=0
// isblank: sp=1 tab=1 nl=0
// toupper: a→A z→Z A→A 5→5
// tolower: A→a Z→z a→a
// toascii: 0xFF→0x7F 0x80→0x00
