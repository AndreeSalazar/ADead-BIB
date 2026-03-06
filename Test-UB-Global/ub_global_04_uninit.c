// ============================================================
// ADead-BIB — Ejemplos: UB vs LIMPIO
// 04 - Variables No Inicializadas (Uninitialized Variables)
// ============================================================
// Compilar con UB:    adeadc cc ub_global_04_uninit.c
// ============================================================

#include <stdio.h>

// 1. Uso básico sin inicializar
void pat1_ub() {
    int x;
    printf("%d\n", x); // UB
}

// 2. Uso en una de las ramas
void pat2_ub() {
    int y;
    int flag = 1;
    if (flag) {
        y = 10;
    }
    printf("%d\n", y); // UB (y)
}

// 3. Seguro en ambas ramas (Limpio)
void pat3_limpio() {
    int z;
    int flag = 1;
    if (flag) {
        z = 10;
    } else {
        z = 20;
    }
    printf("%d\n", z); // Limpio
}

// 4. Dirección pasada a función (Limpio)
void scanf_sim(int *p) {
    *p = 42;
}

void pat4_limpio() {
    int a;
    scanf_sim(&a);  // Pasa la dirección, consideramos inicializado
    printf("%d\n", a); // Limpio
}

// 5. Asignación a sí mismo (UB)
void pat5_ub() {
    int b = b; // UB
    printf("%d\n", b);
}

// 6. Condicionales en bucles
void pat6_ub() {
    int c;
    while (0) {
        c = 100;
    }
    printf("%d\n", c); // UB
}

// 7. Retorno de no inicializado
int pat7_ub() {
    int d;
    return d; // UB
}

// 8. Puntero no inicializado
void pat8_ub() {
    int *p;
    *p = 10; // UB (Deref de uninit flaggeado)
}

int main() {
    pat1_ub();
    pat2_ub();
    pat3_limpio();
    pat4_limpio();
    pat5_ub();
    pat6_ub();
    pat7_ub();
    pat8_ub();
    return 0;
}
