// ============================================================
// Test 01: <ctype.h> — Character Classification Basic
// ============================================================
// ADead-BIB Test Canon — C99 §7.4
// Verifica: isalpha, isdigit, isalnum, isspace, isupper, islower
// Expected: Compila OK — todas las funciones reconocidas
// ============================================================
#include <stdio.h>
#include <ctype.h>

int main() {
    // --- isalpha ---
    int a1 = isalpha('A');   // → non-zero (uppercase letter)
    int a2 = isalpha('z');   // → non-zero (lowercase letter)
    int a3 = isalpha('5');   // → 0 (digit, not alpha)
    int a4 = isalpha(' ');   // → 0 (space, not alpha)

    // --- isdigit ---
    int d1 = isdigit('0');   // → non-zero
    int d2 = isdigit('9');   // → non-zero
    int d3 = isdigit('a');   // → 0
    int d4 = isdigit('\n');  // → 0

    // --- isalnum ---
    int n1 = isalnum('A');   // → non-zero (alpha)
    int n2 = isalnum('5');   // → non-zero (digit)
    int n3 = isalnum('!');   // → 0 (punctuation)

    // --- isspace ---
    int s1 = isspace(' ');   // → non-zero
    int s2 = isspace('\t');  // → non-zero
    int s3 = isspace('\n');  // → non-zero
    int s4 = isspace('A');   // → 0

    // --- isupper / islower ---
    int u1 = isupper('A');   // → non-zero
    int u2 = isupper('a');   // → 0
    int l1 = islower('a');   // → non-zero
    int l2 = islower('A');   // → 0

    printf("isalpha: A=%d z=%d 5=%d sp=%d\n", a1 != 0, a2 != 0, a3 != 0, a4 != 0);
    printf("isdigit: 0=%d 9=%d a=%d nl=%d\n", d1 != 0, d2 != 0, d3 != 0, d4 != 0);
    printf("isalnum: A=%d 5=%d !=%d\n", n1 != 0, n2 != 0, n3 != 0);
    printf("isspace: sp=%d tab=%d nl=%d A=%d\n", s1 != 0, s2 != 0, s3 != 0, s4 != 0);
    printf("isupper: A=%d a=%d  islower: a=%d A=%d\n", u1 != 0, u2 != 0, l1 != 0, l2 != 0);

    return 0;
}
// Expected output:
// isalpha: A=1 z=1 5=0 sp=0
// isdigit: 0=1 9=1 a=0 nl=0
// isalnum: A=1 5=1 !=0
// isspace: sp=1 tab=1 nl=1 A=0
// isupper: A=1 a=0  islower: a=1 A=0
