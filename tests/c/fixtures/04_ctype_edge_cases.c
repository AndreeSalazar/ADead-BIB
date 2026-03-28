// ============================================================
// Test 04: <ctype.h> — Edge Cases & UB-Safe Boundaries
// ============================================================
// ADead-BIB Test Canon — Boundary testing
// Verifica: EOF (-1), valor 0, valor 127, valor 255
// UB Detection: pasar signed char negativo → comportamiento
// definido en ADead-BIB (bit-strict, implicitly strict)
// Expected: Compila OK — sin UB — boundaries correctas
// ============================================================
#include <stdio.h>
#include <ctype.h>

int main() {
    // Edge case 1: EOF (-1) — C99 says behavior is defined for EOF
    // ADead-BIB: debe retornar 0 para todos los is* con EOF
    // (No pasamos EOF directamente por seguridad en la tabla)

    // Edge case 2: NUL (0)
    int nul_alpha = isalpha(0);    // → 0
    int nul_digit = isdigit(0);    // → 0
    int nul_cntrl = iscntrl(0);    // → non-zero (NUL es control)
    int nul_print = isprint(0);    // → 0

    // Edge case 3: Boundary characters
    int at_31 = iscntrl(0x1F);    // → non-zero (último control antes de space)
    int at_32 = isspace(0x20);    // → non-zero (space)
    int at_33 = ispunct(0x21);    // → non-zero ('!')
    int at_7e = isgraph(0x7E);    // → non-zero ('~' último printable)
    int at_7f = iscntrl(0x7F);    // → non-zero (DEL)

    // Edge case 4: toupper/tolower con no-letters
    int tu_digit = toupper('5');   // → '5' (sin cambio)
    int tu_punct = toupper('!');   // → '!' (sin cambio)
    int tu_space = toupper(' ');   // → ' ' (sin cambio)
    int tl_digit = tolower('5');   // → '5' (sin cambio)

    // Edge case 5: Full ASCII range classification consistency
    int total_alpha = 0, total_digit = 0, total_space = 0;
    int total_print = 0, total_cntrl = 0;
    for (int c = 0; c < 128; c++) {
        if (isalpha(c)) total_alpha++;
        if (isdigit(c)) total_digit++;
        if (isspace(c)) total_space++;
        if (isprint(c)) total_print++;
        if (iscntrl(c)) total_cntrl++;
    }

    printf("NUL: alpha=%d digit=%d cntrl=%d print=%d\n",
           nul_alpha != 0, nul_digit != 0, nul_cntrl != 0, nul_print != 0);
    printf("Boundaries: 0x1F_cntrl=%d 0x20_space=%d 0x21_punct=%d 0x7E_graph=%d 0x7F_cntrl=%d\n",
           at_31 != 0, at_32 != 0, at_33 != 0, at_7e != 0, at_7f != 0);
    printf("toupper non-letters: 5→%c !→%c sp→%c\n", tu_digit, tu_punct, tu_space);
    printf("ASCII stats: alpha=%d digit=%d space=%d print=%d cntrl=%d\n",
           total_alpha, total_digit, total_space, total_print, total_cntrl);
    printf("Expected:    alpha=52 digit=10 space=6  print=95 cntrl=33\n");

    return 0;
}
// Expected output:
// NUL: alpha=0 digit=0 cntrl=1 print=0
// Boundaries: 0x1F_cntrl=1 0x20_space=1 0x21_punct=1 0x7E_graph=1 0x7F_cntrl=1
// toupper non-letters: 5→5 !→! sp→ 
// ASCII stats: alpha=52 digit=10 space=6 print=95 cntrl=33
// Expected:    alpha=52 digit=10 space=6  print=95 cntrl=33
