// ============================================================
// Test 20: <ctype.h> Completo — todas las funciones de clasificación
// ============================================================
// ADead-BIB Test Canon — C99 §7.4
// Verifica: isalpha..ispunct, toupper, tolower
// ============================================================

#include <stdio.h>
#include <ctype.h>

void classify_char(int ch) {
    printf("'%c'(%3d): ", (ch >= 32 && ch < 127) ? ch : '?', ch);
    if (isalpha(ch)) printf("alpha ");
    if (isdigit(ch)) printf("digit ");
    if (isalnum(ch)) printf("alnum ");
    if (isupper(ch)) printf("upper ");
    if (islower(ch)) printf("lower ");
    if (isspace(ch)) printf("space ");
    if (ispunct(ch)) printf("punct ");
    if (isprint(ch)) printf("print ");
    if (iscntrl(ch)) printf("cntrl ");
    if (isxdigit(ch)) printf("xdigit ");
    printf("\n");
}

int main() {
    // --- Letras ---
    classify_char('A');
    classify_char('z');
    classify_char('M');

    // --- Dígitos ---
    classify_char('0');
    classify_char('9');

    // --- Hex digits ---
    classify_char('a');
    classify_char('F');

    // --- Espacio ---
    classify_char(' ');
    classify_char('\t');
    classify_char('\n');

    // --- Puntuación ---
    classify_char('!');
    classify_char('.');
    classify_char('@');

    // --- Control ---
    classify_char(0);
    classify_char(127);

    // --- toupper/tolower ---
    printf("\ntoupper: a->%c z->%c A->%c 5->%c\n",
           toupper('a'), toupper('z'), toupper('A'), toupper('5'));
    printf("tolower: A->%c Z->%c a->%c 5->%c\n",
           tolower('A'), tolower('Z'), tolower('a'), tolower('5'));

    // --- Rango completo de letras ---
    int upper_count = 0, lower_count = 0, digit_count = 0;
    int ch;
    for (ch = 0; ch < 128; ch++) {
        if (isupper(ch)) upper_count++;
        if (islower(ch)) lower_count++;
        if (isdigit(ch)) digit_count++;
    }
    printf("\nCounts: upper=%d lower=%d digit=%d\n",
           upper_count, lower_count, digit_count);

    return 0;
}
// Expected counts: upper=26 lower=26 digit=10
