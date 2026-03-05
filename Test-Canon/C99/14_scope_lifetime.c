// ============================================================
// Canon C99 — §6.2.1 Scope y Lifetime de Variables
// ============================================================
// Intención: Cada variable tiene un scope (dónde es visible)
// y un lifetime (cuándo existe en memoria).
//
// C99 §6.2.1: "An identifier can denote an object; a
// function; a tag or member of a structure, union, or
// enumeration; a typedef name; a label name."
//
// Tipos de scope:
//   Block scope:    { int x; } — x vive en el stack frame
//   File scope:     int global; — vive toda la ejecución
//   Function scope: labels
//
// static: lifetime del programa, scope local
// ============================================================

#include <stdio.h>

// --- File scope (global) ---
int global_counter = 0;

// --- Static function-level: persiste entre llamadas ---
int next_id() {
    static int id = 0;
    id = id + 1;
    return id;
}

int call_counter() {
    static int count = 0;
    count = count + 1;
    return count;
}

// --- Block scope ---
void demonstrate_block_scope() {
    int x = 10;
    printf("  outer x = %d\n", x);
    {
        int x = 20;
        printf("  inner x = %d (shadows outer)\n", x);
        {
            int x = 30;
            printf("  innermost x = %d\n", x);
        }
    }
    printf("  outer x still = %d\n", x);
}

// --- Global mutation ---
void increment_global() {
    global_counter = global_counter + 1;
}

// --- Local vs global ---
int shadowing_test() {
    int global_counter = 999;
    return global_counter;
}

int main() {
    printf("=== Canon C99: Scope y Lifetime ===\n\n");

    // --- Global variable ---
    printf("Global:\n");
    printf("  global_counter = %d\n", global_counter);
    increment_global();
    increment_global();
    increment_global();
    printf("  después de 3 incrementos = %d\n", global_counter);

    // --- Static local (persiste entre llamadas) ---
    printf("\nStatic local:\n");
    printf("  next_id() = %d\n", next_id());
    printf("  next_id() = %d\n", next_id());
    printf("  next_id() = %d\n", next_id());

    printf("  call_counter() = %d\n", call_counter());
    printf("  call_counter() = %d\n", call_counter());

    // --- Block scope ---
    printf("\nBlock scope:\n");
    demonstrate_block_scope();

    // --- For loop scope ---
    printf("\nFor loop scope:\n");
    int i;
    for (i = 0; i < 3; i++) {
        int temp = i * 10;
        printf("  i=%d temp=%d\n", i, temp);
    }

    // --- Shadowing ---
    printf("\nShadowing:\n");
    printf("  global_counter (global) = %d\n", global_counter);
    printf("  shadowing_test (local) = %d\n", shadowing_test());
    printf("  global_counter still = %d\n", global_counter);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (global_counter == 3) { pass++; } else { printf("FAIL: global\n"); }
    total++; if (next_id() == 4)      { pass++; } else { printf("FAIL: static id\n"); }
    total++; if (call_counter() == 3) { pass++; } else { printf("FAIL: call counter\n"); }
    total++; if (shadowing_test()==999){ pass++; } else { printf("FAIL: shadow\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
