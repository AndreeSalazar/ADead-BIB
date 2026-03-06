#include <stdio.h>
#include <stdlib.h>

void uaf_inmediato() {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 100;
    free(ptr);
    printf("%d\n", *ptr);       // UB: UseAfterFree
}

void uaf_en_if(int condition) {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 200;
    if (condition) {
        free(ptr);
    }
    printf("%d\n", *ptr);       // UB: ptr puede estar libre
}

void uaf_en_loop() {
    int *ptrs[5];
    for (int i = 0; i < 5; i++) {
        ptrs[i] = (int*)malloc(sizeof(int));
        *ptrs[i] = i * 10;
    }
    for (int i = 0; i < 5; i++) {
        free(ptrs[i]);
    }
    for (int i = 0; i < 5; i++) {
        printf("%d\n", *ptrs[i]); // UB: todos liberados
    }
}

void uaf_alias() {
    int *original = (int*)malloc(sizeof(int));
    int *alias = original;      // mismo bloque
    *original = 42;
    free(original);
    printf("%d\n", *alias);     // UB: alias también inválido
}

void libera(int *p) {
    free(p);
}

void uaf_cross_function() {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 999;
    libera(ptr);
    printf("%d\n", *ptr);       // UB: liberado en libera()
}

void uaf_realloc() {
    int *ptr = (int*)malloc(sizeof(int) * 5);
    int *old = ptr;             // guarda puntero viejo
    ptr = (int*)realloc(ptr, sizeof(int) * 10);
    old[0] = 42;                // UB: old puede ser inválido
}

typedef struct {
    int *data;
    int size;
} Buffer;

void uaf_struct() {
    Buffer buf;
    buf.data = (int*)malloc(sizeof(int) * 10);
    buf.size = 10;

    free(buf.data);
    buf.data[0] = 42;           // UB: data liberado
}

void double_free_basic() {
    int *ptr = (int*)malloc(sizeof(int));
    *ptr = 42;
    free(ptr);
    free(ptr);                  // UB: DoubleFree
}

void double_free_alias() {
    int *a = (int*)malloc(sizeof(int));
    int *b = a;                 // mismo bloque
    free(a);
    free(b);                    // UB: mismo bloque, segunda vez
}

void uaf_limpio_1() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == NULL) return;
    *ptr = 100;
    printf("limpio: %d\n", *ptr);  // uso ANTES de free ✅
    free(ptr);
    ptr = NULL;                     // NULL después ✅
}

void uaf_limpio_alias() {
    int *original = (int*)malloc(sizeof(int));
    if (original == NULL) return;
    *original = 42;
    printf("limpio alias: %d\n", *original);
    free(original);
    original = NULL;               // ambos inválidos ✅
    // alias nunca se usa después ✅
}

void double_free_limpio() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == NULL) return;
    *ptr = 42;
    free(ptr);
    ptr = NULL;                    // ptr = NULL ✅
}

int main() {
    uaf_limpio_1();
    double_free_limpio();

    printf("main: ok\n");
    return 0;
}
