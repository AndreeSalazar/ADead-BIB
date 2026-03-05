// ============================================================
// Canon C99 — §6.5 Expresiones Complejas y Precedencia
// ============================================================
// Intención: C tiene reglas de precedencia estrictas.
// El compilador DEBE respetar el orden de evaluación
// definido por paréntesis y precedencia de operadores.
//
// Precedencia (de mayor a menor):
//   ()                     — Paréntesis
//   ++ -- (postfix)        — Post-incremento
//   ++ -- (prefix) ! ~ -   — Unarios
//   * / %                  — Multiplicativos
//   + -                    — Aditivos
//   << >>                  — Shifts
//   < <= > >=              — Relacionales
//   == !=                  — Igualdad
//   &                      — AND bitwise
//   ^                      — XOR bitwise
//   |                      — OR bitwise
//   &&                     — AND lógico
//   ||                     — OR lógico
//   ?:                     — Ternario
//   = += -= etc            — Asignación
//   ,                      — Comma
// ============================================================

#include <stdio.h>

int main() {
    printf("=== Canon C99: Expresiones Complejas ===\n\n");

    // --- Precedencia multiplicativo > aditivo ---
    int a = 2 + 3 * 4;
    printf("2 + 3 * 4 = %d (esperado: 14)\n", a);

    int b = (2 + 3) * 4;
    printf("(2 + 3) * 4 = %d (esperado: 20)\n", b);

    // --- Precedencia de shift ---
    int c = 1 << 4 + 1;
    printf("1 << 4 + 1 = %d (+ tiene menor prec que <<? No: + > <<, = 1<<5=32)\n", c);

    // --- Ternario ---
    int x = 10;
    int d = (x > 5) ? 100 : 200;
    printf("\n(10 > 5) ? 100 : 200 = %d\n", d);

    int e = (x < 5) ? 100 : 200;
    printf("(10 < 5) ? 100 : 200 = %d\n", e);

    // --- Ternario anidado ---
    int score = 85;
    int grade = (score >= 90) ? 4 : (score >= 80) ? 3 : (score >= 70) ? 2 : 1;
    printf("score=%d grade=%d\n", score, grade);

    // --- Operadores compuestos ---
    printf("\nCompound assignments:\n");
    int v = 100;
    v += 50;  printf("  += 50: %d\n", v);
    v -= 30;  printf("  -= 30: %d\n", v);
    v *= 2;   printf("  *= 2:  %d\n", v);
    v /= 3;   printf("  /= 3:  %d\n", v);
    v %= 7;   printf("  %%= 7:  %d\n", v);

    // --- Pre/post incremento ---
    printf("\nIncrement/Decrement:\n");
    int n = 10;
    printf("  n = %d\n", n);
    printf("  n++ = %d (post)\n", n++);
    printf("  n ahora = %d\n", n);
    printf("  ++n = %d (pre)\n", ++n);
    printf("  n ahora = %d\n", n);
    printf("  n-- = %d (post)\n", n--);
    printf("  n ahora = %d\n", n);

    // --- Lógico short-circuit ---
    printf("\nShort-circuit:\n");
    int t = 1;
    int f = 0;
    printf("  1 && 1 = %d\n", t && t);
    printf("  1 && 0 = %d\n", t && f);
    printf("  0 || 1 = %d\n", f || t);
    printf("  0 || 0 = %d\n", f || f);
    printf("  !0 = %d\n", !f);
    printf("  !1 = %d\n", !t);

    // --- Expresión compleja combinada ---
    int result = (3 + 4) * 2 - 1;
    printf("\n(3 + 4) * 2 - 1 = %d\n", result);

    int complex = (10 > 5) && (3 < 7) ? 42 : 0;
    printf("(10>5) && (3<7) ? 42 : 0 = %d\n", complex);

    // --- Comma operator ---
    int comma_result = (1, 2, 3, 42);
    printf("\n(1, 2, 3, 42) = %d\n", comma_result);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (a == 14)           { pass++; } else { printf("FAIL: precedence\n"); }
    total++; if (b == 20)           { pass++; } else { printf("FAIL: parens\n"); }
    total++; if (d == 100)          { pass++; } else { printf("FAIL: ternary true\n"); }
    total++; if (e == 200)          { pass++; } else { printf("FAIL: ternary false\n"); }
    total++; if (grade == 3)        { pass++; } else { printf("FAIL: nested ternary\n"); }
    total++; if (result == 13)      { pass++; } else { printf("FAIL: complex expr\n"); }
    total++; if (complex == 42)     { pass++; } else { printf("FAIL: logic+ternary\n"); }
    total++; if (comma_result == 42){ pass++; } else { printf("FAIL: comma\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
