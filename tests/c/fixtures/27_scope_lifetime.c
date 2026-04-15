// ============================================================
// Test 27: Scope y Lifetime — local, global, static, block scope
// ============================================================
// ADead-BIB Test Canon — C99 §6.2.1, §6.2.4
// Verifica: scope rules, static locals, shadowing
// ============================================================

#include <stdio.h>

// --- Global variable ---
int global_counter = 0;

// --- Static function-scope ---
int get_next_id() {
    static int id = 0;
    id++;
    return id;
}

int call_count() {
    static int count = 0;
    count++;
    return count;
}

// --- Static accumulator ---
int accumulate(int value) {
    static int total = 0;
    total += value;
    return total;
}

// --- Shadowing ---
int shadow_test() {
    int x = 10;
    {
        int x = 20;
        {
            int x = 30;
            if (x != 30) return -1;
        }
        if (x != 20) return -2;
    }
    return x;
}

int main() {
    // --- Global ---
    global_counter = 42;
    printf("global=%d\n", global_counter);

    // --- Static local (persiste entre llamadas) ---
    printf("id: %d %d %d\n", get_next_id(), get_next_id(), get_next_id());

    // --- Call count ---
    int i;
    for (i = 0; i < 5; i++) {
        call_count();
    }
    printf("call_count=%d\n", call_count());

    // --- Accumulate ---
    accumulate(10);
    accumulate(20);
    accumulate(30);
    printf("accumulated=%d\n", accumulate(0));

    // --- Block scope ---
    {
        int block_var = 100;
        printf("block_var=%d\n", block_var);
    }

    // --- Shadowing ---
    int result = shadow_test();
    printf("shadow=%d\n", result);

    // --- For-loop scope (C99) ---
    int total = 0;
    for (int j = 0; j < 5; j++) {
        total += j;
    }
    printf("for_scope total=%d\n", total);

    // --- Nested blocks ---
    int outer = 1;
    {
        int middle = 2;
        {
            int inner = 3;
            printf("nested: %d %d %d\n", outer, middle, inner);
        }
    }

    return 0;
}
// Expected:
// global=42
// shadow=10
// for_scope total=10
// nested: 1 2 3
