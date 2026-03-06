#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void bounds_fijo() {
    int arr[5] = {1, 2, 3, 4, 5};
    arr[5]  = 10;               // UB: index = size
    arr[10] = 20;               // UB: muy fuera
    arr[-1] = 30;               // UB: negativo
}

void bounds_off_by_one() {
    int arr[10] = {0};
    for (int i = 0; i <= 10; i++) {  // UB: <= en vez de <
        arr[i] = i;             // arr[10] fuera de bounds
    }
}

void bounds_index_overflow() {
    int arr[100] = {0};
    int base = 90;
    int offset = 20;
    arr[base + offset] = 42;    // UB: 110 >= 100
}

void bounds_ptr_arith() {
    int arr[5] = {1, 2, 3, 4, 5};
    int *ptr = arr;
    *(ptr + 10) = 42;           // UB: 10 >= size
}

void bounds_string() {
    char buf[5];
    strcpy(buf, "hello world");  // UB: "hello world" > 5
}

void bounds_stack_overflow() {
    char small[4];
    char big[] = "overflow here";
    memcpy(small, big, sizeof(big));  // UB: big > small
}

int* dangle_retorna_local() {
    int local = 42;
    return &local;              // UB: local destruido al retornar
}

int* dangle_scope_interno() {
    int *saved = NULL;
    {
        int inner = 99;
        saved = &inner;         // inner sale de scope
    }
    return saved;               // UB: inner destruido
}

int* dangle_array_local() {
    int arr[5] = {1, 2, 3, 4, 5};
    return &arr[2];             // UB: arr destruido al retornar
}

void dangle_realloc() {
    int *ptr = (int*)malloc(sizeof(int) * 5);
    int *old_ptr = ptr;         // guarda dirección vieja
    ptr = (int*)realloc(ptr, sizeof(int) * 100);
    old_ptr[0] = 42;            // UB: old_ptr puede ser inválido
    free(ptr);
}

void type_int_to_ptr() {
    long addr = 0x12345678;
    int *ptr = (int*)addr;      // UB: dirección arbitraria
    *ptr = 42;
}

void type_aliasing() {
    float f = 3.14f;
    int *ip = (int*)&f;         // UB: strict aliasing
    printf("%d\n", *ip);        // interpreta bits de float como int
}

union Mezcla {
    int   entero;
    float flotante;
    char  bytes[4];
};

void type_union() {
    union Mezcla m;
    m.entero = 42;
    printf("%f\n", m.flotante);
}

typedef struct { int tipo; } Base;
typedef struct { int tipo; int extra; } Derivado;

void type_downcast() {
    Base b;
    b.tipo = 1;
    Derivado *d = (Derivado*)&b;  // UB: b es Base, no Derivado
    d->extra = 99;                // accede memoria inválida
}

void type_char_aliasing_ub() {
    int x = 42;
    float *fp = (float*)&x;     // UB: float* aliasing int
    *fp = 3.14f;
}

void bounds_limpio() {
    int arr[5] = {1, 2, 3, 4, 5};
    int size = 5;

    for (int i = 0; i < size; i++) {    // < no <= ✅
        arr[i] = i * 2;
    }

    int index = 3;
    if (index >= 0 && index < size) {   // check ✅
        printf("bounds limpio: arr[%d] = %d\n", index, arr[index]);
    }

    char buf[16];
    strncpy(buf, "hello", sizeof(buf) - 1);
    buf[sizeof(buf) - 1] = '\0';        // null terminator ✅
    printf("bounds limpio str: %s\n", buf);
}

int dangle_limpio_malloc() {
    int *ptr = (int*)malloc(sizeof(int));
    if (ptr == NULL) return 0;
    *ptr = 42;
    int valor = *ptr;           // copia el valor ✅
    free(ptr);
    ptr = NULL;
    return valor;               // retorna valor, no puntero ✅
}

void type_limpio() {
    int x = 42;
    long lx = (long)x;          // widening ✅
    printf("type limpio int->long: %ld\n", lx);

    int val = 99;
    void *vp = (void*)&val;
    int *ip = (int*)vp;         // mismo tipo original ✅
    printf("type limpio void*: %d\n", *ip);

    int num = 0x41424344;
    unsigned char *bytes = (unsigned char*)&num;
    printf("type limpio char* alias: %02X\n", bytes[0]);
}

int main() {
    bounds_limpio();
    int v = dangle_limpio_malloc();
    printf("dangle limpio valor: %d\n", v);
    type_limpio();

    printf("\nmain: ok\n");
    return 0;
}
